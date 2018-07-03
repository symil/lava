#!/usr/bin/env node

const { parseType, parseFunction, isHandle } = require('./parse');

const PRIMITIVE_TYPES = {
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    char: 'c_char',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    void: 'c_void'
};

const META_TYPE_TO_BUILD_FUNC = {
    handle: buildHandle,
    struct: buildStruct
};

const SCHEMA = {
    VkInstance: {

    },
    VkBuffer: {
        new: 'vkCreateBuffer',
        drop: 'vkDestroyBuffer'
    },
    VkPhysicalDevice: {
        // getList: 'vkEnumeratePhysicalDevices',
        // getSurfaceCapabilities: 'vkGetPhysicalDeviceSurfaceCapabilitiesKHR',
        // doesSupportSurface: 'vkGetPhysicalDeviceSurfaceSupportKHR',
        // getSurfacePresentModes: 'vkGetPhysicalDeviceSurfacePresentModesKHR',
        createLogicalDevice: 'VkDevice::new'
    },
    VkDevice: {
        new: 'vkCreateDevice'
    }
};

const DEFINITIONS = {};

// buildType('VkPhysicalDeviceProperties');
// buildType('VkInstanceCreateInfo');
buildType('VkSwapchainCreateInfoKHR');

function methodToStr(method) {
    const { name, args, returnType, body } = method;

    return blockToStr([
        `fn ${name}(${args.map(({name, type}) => (name ? `${name}: `: '') + type).join(', ')})` + (returnType ? ` -> ${returnType}` : ''),
            body
    ]);
}

function buildType(typeName) {
    if (DEFINITIONS[typeName]) {
        return;
    }

    const cDef = parseType(typeName);
    const metaType = cDef.type;
    const rawInfo = metaType === 'handle' ? (SCHEMA[typeName] || {}) : cDef;
    const definition = META_TYPE_TO_BUILD_FUNC[metaType](typeName, rawInfo);

    definition.metaType = metaType;
    DEFINITIONS[typeName] = definition;
}

function buildStruct(typeName, parsed) {
    const rawTypeName = toRawTypeName(typeName);
    const wrappedTypeName = toWrappedTypeName(typeName);

    const uses = [
        `std::ops::Drop`,
        `std::vec::Vec`,
        `std::ptr::null`,
        `vk::VkFrom`,
        `vk::RawVkHandle`,
        'libc::*'
    ];

    let lifetimeIdCounter = 0;
    const srcVar = 'value';
    const lifetimeIds = [];
    const rawFields = [];
    const wrappedFields = [];
    const fromRawToWrappedFields = [];
    const fromWrappedToRawFields = [];

    parsed.fields.forEach((field, index) => {
        const prevField = parsed.fields[index - 1];
        const nextField = parsed.fields[index + 1];

        const rustFieldName = getRustFieldName(field);
        const rawRustFieldType = getRawRustFieldType(field);
        const isCount = areCountAndArray(field, nextField);
        const isArrayPtr = areCountAndArray(prevField, field);
        const isStaticArray = !!field.arraySize;
        const isPointer = field.isPointer;
        const isTypeChar = field.typeName === 'char';
        const isPrimitiveType = !!PRIMITIVE_TYPES[field.typeName];
        const isHandleType = isHandle(field.typeName);
        const representIndex = doesFieldRepresentIndex(field);

        rawFields.push({ name: rustFieldName, type: rawRustFieldType });

        if (field.name === 'sType') {
            fromWrappedToRawFields.push(`${rustFieldName}: VkFrom::from(&VkStructure::${typeName.substring(2)})`);
        } else if (field.name === 'pNext') {
            fromWrappedToRawFields.push(`${rustFieldName}: null()`);
        } else if (isCount) {
            const vecFieldName = getRustFieldName(nextField);
            fromWrappedToRawFields.push(`${rustFieldName}: ${srcVar}.${vecFieldName}.len() as u32`);
        } else {
            const src = `${srcVar}.${rustFieldName}`;
            const ref = isHandleType ? '' : '&';
            const wrappedToRawCollectionConversion = isPrimitiveType
                ? (representIndex ? `.iter().map(|x| x as u32).collect()` : '')
                : `.iter().map(|x| VkFrom::from(x)).collect()`;
            const rawToWrappedSingleEltConversion = isPrimitiveType
                ? (representIndex ? `${src} as u32` : src)
                : `VkFrom::from(${ref}${src})`;
            const wrappedTypeName = representIndex ? 'usize' : toWrappedTypeName(field.typeName);
            let wrappedFieldType = null;

            if (isArrayPtr) {
                if (isStringArray(field)) {
                    wrappedFieldType = `Vec<String>`;
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_string_array(&${src})`);
                } else {
                    wrappedFieldType = `Vec<${wrappedTypeName}>`;
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_array(&${src}${wrappedToRawCollectionConversion})`);
                }
            } else if (isPointer) {
                if (isTypeChar) {
                    wrappedFieldType = 'String';
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_string(&${src})`);
                } else {
                    if (isHandleType) {
                        const lifetimeId = String.fromCharCode('a'.charCodeAt(0) + lifetimeIdCounter++);
                        lifetimeIds.push(lifetimeId);
                        wrappedFieldType = `&'${lifetimeId} ${wrappedTypeName}`
                    } else {
                        wrappedFieldType = wrappedTypeName;
                    }
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_ptr(&${rawToWrappedSingleEltConversion})`);
                }
            } else if (isStaticArray) {
                if (isTypeChar) {
                    wrappedFieldType = 'String';
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_string(&${src} as *const c_char)`);
                } else {
                    wrappedFieldType = `[${wrappedTypeName}, ${arraySize}]`;
                    fromWrappedToRawFields.push(`${rustFieldName}: ${src}${wrappedToRawCollectionConversion}`);
                }
            } else {
                if (isHandleType) {
                    const lifetimeId = String.fromCharCode('a'.charCodeAt(0) + lifetimeIdCounter++);
                    lifetimeIds.push(lifetimeId);
                    wrappedFieldType = `&'${lifetimeId} ${wrappedTypeName}`
                } else {
                    wrappedFieldType = wrappedTypeName;
                }
                fromWrappedToRawFields.push(`${rustFieldName}: ${rawToWrappedSingleEltConversion}`);
            }

            wrappedFields.push({ name: rustFieldName, type: wrappedFieldType });
        }
    });

    const fromWrappedToRaw = [rawTypeName, fromWrappedToRawFields.map(x => `${x},`)];

    // console.log(blockToStr(fromWrappedToRaw));
    console.log(blockToStr([rawTypeName, rawFields.map(({name, type}) => `${name}: ${type},`)]));
    console.log(blockToStr([wrappedTypeName, wrappedFields.map(({name, type}) => `${name}: ${type},`)]));

    return {};
}

function doesFieldRepresentIndex(field) {
    return field.typeName === 'uint32_t' && /Index|Indices/.test(field.name);
}

function isStringArray(field) {
    return field.fullType === 'const char* const*';
}

function getRawRustFieldType(field) {
    const { name, typeName, isPointer, arraySize } = field;

    if (name === 'pNext') {
        return `*const c_void`;
    } else if (isStringArray(field)) {
        return `*mut *mut c_char`;
    } else {
        const rawTypeName = toRawTypeName(typeName);

        if (isPointer) {
            return `*mut ${rawTypeName}`;
        } else if (arraySize) {
            return `[${rawTypeName}, ${arraySize}]`;
        } else {
            return rawTypeName;
        }
    }
}

function getRustFieldName({name}) {
    return toSnakeCase(name.replace(/^(p{1,2}|s)[A-Z]/, str => str[str.length - 1]));
}

function areCountAndArray(field1, field2) {
    return field1 && field2 && field1.name.endsWith('Count') && field1.fullType === 'uint32_t' && field2.isPointer && field2.typeName !== 'char';
}

function buildHandle(typeName, rawInfo) {
    const uses = [
        `std::ops::Drop`,
        `std::vec::Vec`,
        `std::ptr::null`,
        `vk::VkFrom`,
        `vk::RawVkHandle`
    ];

    const rawTypeName = toRawTypeName(typeName);
    const wrappedTypeName = toWrappedTypeName(typeName);
    const fields = [];
    const rawDefinition = { type: 'typedef', target: 'RawVkHandle' };
    const wrappedDefinition = { type: 'struct', fields: fields };
    const methods = [];
    const externFunctions = [];
    let wrappedDropMethod = null;
    let rawDropMethod = null;
    let fromWrappedToRaw = null;
    let fromRawToWrapped = null;

    fields.push({name: '_handle', type: rawTypeName});
    
    methods.push({
        name: 'handle',
        args: [{type: '&self'}],
        returnType: rawTypeName,
        body: ['self._handle']
    });

    if (rawInfo.drop) {
        const cMethodName = rawInfo.drop;
        const cMethod = parseFunction(cMethodName);
        const cArgs = cMethod.args.map(arg => {
            if (arg.name === 'pAllocator') {
                return 'null()';
            } else {
                const rawArgType = toRawTypeName(arg.typeName);
                let field = fields.find(({type}) => type === rawArgType);

                if (!field) {
                    field = {name: `_${arg.name}`, type: rawArgType};
                    fields.push(field);
                }

                return `self.${field.name}`;
            }
        });
        const body = ['unsafe', [`${cMethodName}(${cArgs.join(', ')});`]];
        
        dropMethod = {
            name: 'drop',
            args: [{type: '&mut self'}],
            returnType: null,
            body: body
        };

        externFunctions.push(cMethod);
    }

    Object.entries(rawInfo).forEach(([methodName, def]) => {
        if (methodName === 'drop') {
            return;
        }

        let statements = [];
        let returnType = null;
        let methodArguments = [];
        let static = false;
        let unsafe = false;
        let setAdditionalFields = [];

        if (!def.includes('::')) {
            const cMethod = parseFunction(def);
            const cMethodArgs = cMethod.args;
            const lastArg = cMethodArgs[cMethodArgs.length - 1];
            const instanceVarName = toSnakeCase(lastArg.typeName.substring(2));
            const beforeLastArg = cMethodArgs[cMethodArgs.length - 2];
            const createSomething = lastArg.isPointer && !lastArg.isConst;
            const createList = createSomething && beforeLastArg.isPointer && !beforeLastArg.isConst && beforeLastArg.typeName === 'uint32_t';
            const isConstructor = createSomething && lastArg.typeName === typeName;
            const returnVkResult = cMethod.returnType === 'VkResult';
            const argsForCMethod = cMethodArgs.map((arg, index) => {
                if (createSomething && index === cMethodArgs.length - 1) {
                    return 'ptr';
                } else if (createList && index === cMethodArgs.length - 2) {
                    return 'count';
                } else if (arg.name === 'pAllocator') {
                    return 'null()';
                } else {
                    const rawArgType = toRawTypeName(arg.typeName);
                    const wrappedArgType = toWrappedTypeName(arg.typeName);
    
                    if (arg.isPointer && arg.isConst) {
                        // Structure that needs to be passed to the rust method
    
                        const rustArgName = toSnakeCase(arg.name.substring(1));
                        const rawVarName = `raw_${rustArgName}`;
                        const ptrVarName = `${rawVarName}_ptr`;
    
                        statements.push(
                            `let mut ${rawVarName} = ${rawArgType}::from(${rustArgName});`, // TODO: properly convert VkBool32
                            `let ${ptrVarName} = &mut ${rawVarName} as *mut ${rawArgType};`
                        );
    
                        methodArguments.push({
                            name: rustArgName,
                            type: `&${wrappedArgType}`
                        });
    
                        return ptrVarName;
                    } else if (!arg.isPointer) {
                        if (arg.typeName.startsWith('Vk')) {
                            const field = fields.find(({type}) => type === rawArgType);
    
                            if (field && !isConstructor) {
                                return `self.${field.name}`;
                            } else {
                                const rustArgName = toSnakeCase(arg.name);
                                const handleName = `${rustArgName}_handle`;
    
                                statements.push(`let ${handleName} = ${rustArgName}.handle();`);
                                methodArguments.push({ name: rustArgName, type: `&${wrappedArgType}` });
    
                                if (isConstructor && field) {
                                    setAdditionalFields.push(`${instanceVarName}.${field.name} = ${handleName};`);
                                }
    
    
                                return handleName;
                            }
                        } else {
                            const rustArgName = toSnakeCase(arg.name);
                            const convertToUsize = rustArgName.endsWith('_index') && wrappedArgType === 'u32';
                            const rustArgType = convertToUsize ? 'usize' : wrappedArgType;
                            methodArguments.push({ name: rustArgName, type: rustArgType });
    
                            return rustArgName + (convertToUsize ? ' as u32' : '');
                        }
                    }
                }
    
                return '<missing>';
            });
    
            externFunctions.push(cMethod);

            unsafe = true;

            if (isConstructor) {
                static = true;
            }
    
            if (createSomething) {
                returnType = toWrappedTypeName(lastArg.typeName);
    
                if (createList) {
                    returnType = `Vec<${returnType}>`;
                }
    
                if (returnVkResult) {
                    returnType = `Result<${returnType}, VkResult>`;
                }
    
                const vkWrapperCallName = `vk_call_retrieve_${createList ? 'list' : 'single'}${returnVkResult ? '' : '_unchecked'}`;
                const lambdaArgs = createList ? '|count, ptr|' : '|ptr|';
                const callback = `|${instanceVarName}| { ${setAdditionalFields.join(' ')} }`;
                statements.push(`${vkWrapperCallName}(\n    ${lambdaArgs} ${cMethod.name}(${argsForCMethod.join(', ')}),\n    ${callback}\n)`);
            } else {
                throw new Error(`method ${cMethod.name} does not seem to retrieve anything`);
            }
        } else {
            const [targetTypeName, targetMethodName] = def.split('::');

            buildType(targetTypeName);

            const rustTargetMethodName = toSnakeCase(targetMethodName);
            const targetMethod = DEFINITIONS[targetTypeName].methods.find(m => m.name === rustTargetMethodName);
            const targetCallArgs = [];

            targetMethod.args.forEach(({type, name}) => {
                if (type === `&${wrappedTypeName}`) {
                    targetCallArgs.push('self');
                } else {
                    targetCallArgs.push(name);
                    methodArguments.push({name, type});
                }
            });

            returnType = targetMethod.returnType
            statements.push(`${targetTypeName}::${rustTargetMethodName}(${targetCallArgs.join(', ')})`);
        }

        if (!static) {
            methodArguments.unshift({type: '&self'});
        }
        
        const method = {
            name: toSnakeCase(methodName),
            args: methodArguments,
            returnType: returnType,
            body: unsafe ? ['unsafe', statements] : statements
        };

        methods.push(method);
    });

    fromWrappedToRaw = {
        name: 'from',
        args: [{name: 'value', type: `&${wrappedTypeName}`}],
        returnType: 'Self',
        body: [`value._handle`]
    };

    fromRawToWrapped = {
        name: 'from',
        args: [{name: 'value', type: `&${rawTypeName}`}],
        returnType: 'Self',
        body: fields.map(({name}) => `${name}: ${name === '_handle' ? '*raw' : 'VK_NULL_HANDLE'},`)
    }

    methods.forEach(method => console.log(methodToStr(method)));

    return { rawTypeName, wrappedTypeName, uses, rawDefinition, wrappedDefinition, methods, rawDropMethod, wrappedDropMethod, externFunctions, fromRawToWrapped, fromWrappedToRaw };
}

function toRawTypeName(typeName) {
    return PRIMITIVE_TYPES[typeName] || `Raw${typeName}`;
}

function toWrappedTypeName(typeName) {
    if (typeName === 'VkBool32') {
        return 'bool';
    }

    return PRIMITIVE_TYPES[typeName] || typeName.replace('FlagBits', 'Flags');
}

function blockToStr(block, indent) {
    const spaces = indentToSpaces(indent);
    let result;

    if (typeof block === 'string') {
        result = `\n${block.split('\n').map(line => `${spaces}${line}`).join('\n')} `;
    } else {
        result = `{${block.map(b => blockToStr(b, inc(indent))).join('')}\n${spaces}}`
    }

    if (indent === undefined) {
        result = result.substring(2, result.length - 2);
    }

    return result;
}

function inc(value) {
    return value === undefined ? 0 : value + 1;
}

function indentToSpaces(indent) {
    if (!indent) return '';

    return new Array(indent).fill('    ').join('');
}

function toSnakeCase(str) {
    return str
        .replace(/[A-Z0-9]+/g, str => str.charAt(0) + str.substring(1).toLowerCase())
        .split(/(?=[A-Z])/).join('_').toLowerCase();
}