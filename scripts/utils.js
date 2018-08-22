const { getHandle, getStruct } = require('./parse');

const PRIMITIVE_TYPES = {
    bool: 'bool',
    uint64_t: 'u64',
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int64_t: 'i64',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    int: 'i32',
    char: 'c_char',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    ssize_t: 'isize',
    void: 'c_void',
    VkAllocationCallbacks: 'c_void',
    VkDeviceSize: 'u64',
    VkSampleMask: 'u32'
};

const INDENT = '    ';

const OUTPUT_HANDLE_STRUCTS = [
    'VkDisplayProperties',
    'VkDisplayPlaneProperties',
    'VkDisplayModeProperties'
];

Array.prototype.first = function() {
    return this[0];
};

Array.prototype.last = function() {    
    return this[this.length - 1];
};

Array.prototype.beforeLast = function() {
    return this[this.length - 2];
};

String.prototype.toSnakeCase = function() {
    return toSnakeCase(this);
}

function isStructOrHandle(field) {
    return !!(getHandle(field) || getStruct(field));
}

function isStruct(field) {
    return !!getStruct(field);
}

function toSnakeCase(str) {
    return str
        .replace(/([A-Z]+)|([0-9]+)/g, str => `_${str}`)
        .toLowerCase()
        .replace(/(^|_)(1|2|3)_d(_|$)/g, '$1$2d$3')
        .replace(/^_/, '')
        .replace(/_+/g, '_');
}

function toPascalCase(str) {
    return str.split('_')
        .map(word => word.charAt(0).toUpperCase() + word.substring(1).toLowerCase()).join('')
        .replace(/\d[a-z](?=\d)/g, str => str[0] + str[1].toUpperCase());
}

function toUpperCase(str) {
    return str
        .replace(/[a-z][A-Z]/g, str => `${str[0]}_${str[1]}`)
        .toUpperCase();
}

function findEnumPrefix(typeName) {
    if (typeName === 'VkResult') {
        return 'VK_';
    }

    return typeName
        .replace(/[A-Z]+$/, '')
        .replace(/[A-Z]+/g, `_$&`)
        .toUpperCase()
        .substring(1);
}

function blockToString(block) {
    if (typeof block === 'string') {
        return block;
    } else {
        return _blocksToString(block).replace(/\n {/g, '\n{');
    }
}

function _blocksToString(blocks) {
    blocks = blocks.filter(b => b !== null);

    let result = '';

    let isFirst = true;

    for (let block of blocks) {
        if (typeof block === 'string') {
            if (!isFirst) {
                result += '\n';
            }
            result += block;
        } else if (isFirst) {
            result += _blocksToString(block);
        } else {
            result += ' {\n';
            result += _blocksToString(block).split('\n').map(str => INDENT + str).join('\n');
            result += '\n}';
        }

        isFirst = false;
    }

    return result.replace(/\n;/g, ';\n');
}

function isPlural(arg) {
    return arg && !arg.typeName.endsWith('s') && arg.name.endsWith('s');
}

function cToRustVarName(name) {
    name = name.replace(/^(p{1,2})[A-Z]/, str => str[str.length - 1]);
    name = toSnakeCase(name);

    if (name === 'type') {
        name = 'type_';
    }

    return name;
}

function argToString(arg) {
    return arg.name ? `${arg.name}: ${arg.type}` : arg.type;
}

function doesFieldRepresentVersion(field) {
    return field.typeName === 'uint32_t' && /(p?)apiversion$/i.test(field.name);
}

function createStaticArray(typeName, arraySize, varName, functionName) {
    return `unsafe { let mut dst_array : [${typeName}; ${arraySize}] = mem::uninitialized(); ${functionName}(&${varName}, &mut dst_array); dst_array }`;
}

function fillStaticArray(typeName, arraySize) {
    return `unsafe { let mut dst_array : [${typeName}; ${arraySize}] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array }`;
}

function getRawVkTypeName(cTypeName) {
    return `Raw${cTypeName}`;
}

function getWrappedVkTypeName(cTypeName) {
    return cTypeName;
}

function getFieldRawTypeName(field) {
    if (field.name === 'pCode') {
        return 'u8';
    } else if (field.typeName === 'VkBool32') {
        return 'u32';
    }

    return PRIMITIVE_TYPES[field.typeName] || getRawVkTypeName(field.typeName);
}

const INT_TYPES = ['uint32_t', 'uint64_t', 'int32_t', 'int64_t', 'VkDeviceSize']

const INT_FIELD_NAMES = ['x', 'y', 'z', 'width', 'height', 'layers', 'timeout'];

function fieldNameIsInt(field) {
    return INT_FIELD_NAMES.includes(field.name) ||
        /(mask|version)/i.test(field.name) ||
        /subpass/i.test(field.name);
}

function getFieldWrappedTypeName(field) {
    if (field.name === 'pCode') {
        return `u8`;
    } else if (doesFieldRepresentVersion(field)) {
        return `VkVersion`;
    } else if (INT_TYPES.includes(field.typeName) && !fieldNameIsInt(field)) {
        return field.typeName.startsWith('int') ? `isize` : `usize`;
    } else if (field.typeName === 'VkBool32') {
        return 'bool';
    }

    return PRIMITIVE_TYPES[field.typeName] || getWrappedVkTypeName(field.typeName);
}

function getStaticVkValueName(wrappedTypeName) {
    return `STATIC_${toUpperCase(wrappedTypeName)}`;
}

function getConstVkValueName(wrappedTypeName) {
    return `CONST_${toUpperCase(wrappedTypeName)}`;
}

function getPrimitiveDefaultValue(typeName) {
    if (!Object.values(PRIMITIVE_TYPES).includes(typeName)) {
        return null;
    } if (typeName === 'bool') {
        return 'false';
    } else if (typeName === 'f32' || typeName === 'f64') {
        return '0.0';
    } else {
        return '0';
    }
}

function isOutputHandleStruct(typeName) {
    return OUTPUT_HANDLE_STRUCTS.includes(typeName);
}

function getCountVarNameValue(countFieldName, getVarName) {
    const isNested = countFieldName.includes('::');

    if (isNested) {
        const path = countFieldName.split('::');
        return `(&*${getVarName(cToRustVarName(path[0]))}).${path.slice(1).map(cToRustVarName).join('.')}`;
    } else {
        return getVarName(cToRustVarName(countFieldName));
    }
}

function getFieldsInformation(fields, structName) {
    const infos = [];

    const doesOutputHandle = isOutputHandleStruct(structName);

    for (let field of fields) {
        const rawTypeName = getFieldRawTypeName(field);
        const wrappedTypeName = getFieldWrappedTypeName(field);

        const arraySize = field.arraySize;
        const isCount = !!(field.countFor && field.countFor.length);
        const isPointer = field.isPointer;
        const isDoublePointer = field.isDoublePointer;
        const isPointerArray = field.isPointer && (field.countField || field.name === 'pSampleMask');
        const isPointerValue = field.isPointer && !isPointerArray;
        const isStaticArray = !field.isPointer && !!arraySize;
        const isPrimitiveType = rawTypeName === wrappedTypeName;
        const isHandleType = !isPrimitiveType && !!getHandle(field);
        const isOptional = field.isOptional;
        const isConst = field.isConst;

        const varName = '[VarName]';
        const countVarName = '[CountVarName]';
        const arrayVarName = '[ArrayVarName]';

        const varNameValue = cToRustVarName(field.name);
        const countVarNameValue = field.countField ? field.countField : null;
        const arrayVarNameValue = isCount ? cToRustVarName(field.countFor[0]) : null;
        const countField = (field.countField && fields.find(f => f.name === field.countField)) || {};
        const countVarIsPointer = countField.isPointer;
        const isInputHandle = isHandleType && !doesOutputHandle;

        let rawType = null;
        let wrappedType = null;
        let toRaw = null;
        let toWrapped = null;
        let defValue = null;
        let freeRaw = null;

        let freeMethod = null;
        let freeMemory = false;

        if (isStruct(field)) {
            if (isDoublePointer || isPointerArray) {
                const methodName = isDoublePointer ? 'free_vk_ptr_array_array' : 'free_vk_ptr_array';
                const countPrefix = countField.isPointer ? '*' : '';
                const countSuffix = countField.typeName === 'size_t' ? '' : ' as usize';

                freeMethod = `${methodName}(${countPrefix}${countVarName}${countSuffix}, ${varName})`;
            } else if (isPointer) {
                freeMethod = `free_vk_ptr(${varName})`;
            } else if (isStaticArray) {
                freeMethod = `for elt in ${varName}.iter_mut() { ${rawTypeName}::vk_free(elt); }`
            } else {
                freeMethod = `${rawTypeName}::vk_free(&mut ${varName})`;
            }
        } else if (isPointer) {
            freeMethod = `free_ptr(${varName})`;
        }

        if (field.name === 'sType' && field.values) {
            rawType = 'RawVkStructureType';
            toRaw = () => `vk_to_raw_value(&VkStructureType::${toPascalCase(field.values.substring('VK_STRUCTURE_TYPE_'.length))})`;
        } else if (field.name === 'pNext') {
            rawType = `*const c_void`;
            toRaw = `ptr::null()`;
        } else if (field.fullType === 'void**') {
            rawType = `*mut *mut c_void`;
            wrappedType = `*mut *mut c_void`;
            toRaw = varName;
            toWrapped = varName;
            defValue = 'ptr::null_mut()'
        } else if (field.fullType === 'void*' && !field.countField) {
            rawType = `*const c_void`;
            wrappedType = `*const c_void`;
            toRaw = varName;
            toWrapped = varName;
            defValue = 'ptr::null()';
        } else if (isCount) {
            rawType = isPointerValue ? (isConst ? `*const ${rawTypeName}` : `*mut ${rawTypeName}`) : rawTypeName;

            let lenValue = '';

            for (let i = 0; i < field.countFor.length; ++i) {
                const arrayFieldName = field.countFor[i];
                const arrayField = fields.find(f => f.name === arrayFieldName);
                if (i === 0 || arrayField.isConst || !arrayField.isPointer) {
                    // Don't use created array into account
                    const arrayName =`makeVarName(${cToRustVarName(arrayFieldName)})`; 
                    const otherLen = arrayField.isOptional ? `get_array_option_len(${arrayName})` : `${arrayName}.len()`;
                    lenValue = !lenValue ? otherLen : `cmp::max(${lenValue}, ${otherLen})`;
                }
            }

            // if (field.name === 'codeSize') {
            //     lenValue = `${lenValue} * 4`
            // }

            if (rawTypeName !== 'usize') {
                lenValue += ` as ${rawTypeName}`;
            }

            toRaw = new Function('makeVarName', [
                `return '${lenValue}'.replace(/makeVarName\\((\\w+)\\)/g, function(_, name) { return makeVarName(name); });`,
            ].join('\n'));
        } else if (field.fullType === 'const char* const*') {
            rawType = `*mut *mut c_char`;
            wrappedType = `&[&str]`;
            toRaw = `new_ptr_string_array(${varName})`;
            defValue = `&[]`;
            freeMemory = true;
        } else if (field.fullType === 'const char*') {
            if (doesOutputHandle) {
                rawType = '*const c_char';
                wrappedType =`Option<String>`;
                toWrapped = `new_string_checked(${varName})`;
                defValue = 'None';
            } else {
                rawType = `*mut c_char`;
                freeMemory = true;

                if (isOptional) {
                    wrappedType =`Option<&str>`;
                    toRaw = `new_ptr_string_checked(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType =`&str`;
                    toRaw = `new_ptr_string(${varName})`;
                    defValue = `""`;
                }
            }
        } else if (field.fullType === 'char' && field.arraySize) {
            rawType = `[c_char; ${arraySize}]`;
            wrappedType = `String`;
            // toRaw = createStaticArray(rawTypeName, arraySize, varName, 'string_to_byte_array');
            toWrapped = `new_string(&${varName}[0] as *const c_char)`;
            // defValue = `String::new()`;
        } else if (isPrimitiveType) {
            const primitiveDefaultValue = getPrimitiveDefaultValue(wrappedTypeName);
            let ptrQualifier = isConst ? 'const' : 'mut';
            let refQualifier = isConst ? '' : 'mut ';

            if (isPointerArray) {
                let vecLength = `${countVarIsPointer ? '*' : ''}${countVarName}`;
                if (countField.typeName !== 'size_t') {
                    vecLength += ' as usize';
                }

                rawType = `*${ptrQualifier} ${rawTypeName}`;
                toWrapped = `Vec::from_raw_parts(${varName}, ${vecLength}, ${vecLength})`; // Maybe risky

                if (isOptional) {
                    wrappedType = `Option<&${refQualifier}[${wrappedTypeName}]>`;
                    toRaw = `slice_option_to_ptr${isConst ? '' : '_mut'}(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `&${refQualifier}[${wrappedTypeName}]`;
                    toRaw = `${varName}.${isConst ? 'as_ptr' : 'as_mut_ptr'}()`;
                    defValue = `&[]`;
                }
            } else if (isPointerValue) {
                if (rawTypeName === 'c_void') {
                    ptrQualifier = 'const';
                }

                rawType = `*${ptrQualifier} ${rawTypeName}`;
                wrappedType = `&${wrappedTypeName}`;
                toRaw = `${varName} as *${ptrQualifier} ${rawTypeName}`;
                toWrapped = `*${varName}`;
                defValue = `&${primitiveDefaultValue}`;
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;

                if (field.countField) {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toWrapped = `new_array(${countVarName}, ${varName}.as_ptr())`;
                } else {
                    wrappedType = `[${wrappedTypeName}; ${arraySize}]`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'to_array');
                    toWrapped = createStaticArray(rawTypeName, arraySize, varName, 'to_array');
                    defValue = `[${primitiveDefaultValue}; ${arraySize}]`;
                }
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = varName;
                toWrapped = varName;
                defValue = primitiveDefaultValue;
            }
        } else {
            freeMemory = true;

            if (isDoublePointer) {
                rawType = `*mut *mut ${rawTypeName}`;
                wrappedType = `&[&${wrappedTypeName}]`;
                toRaw = `new_ptr_vk_array_array(${varName})`;
                defValue = `&[]`;
            } else if (isPointerArray) {
                const prefixRef = isInputHandle ? '&' : '';
                const refMethodsuffix = isInputHandle ? '_from_ref' : '';

                rawType = `*mut ${rawTypeName}`;
                toWrapped = `new_vk_array(${countVarIsPointer ? '*' : ''}${countVarName}, ${varName})`;

                if (isOptional) {
                    wrappedType = `Option<&[${prefixRef}${wrappedTypeName}]>`;
                    toRaw = `new_ptr_vk_array_checked${refMethodsuffix}(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `&[${prefixRef}${wrappedTypeName}]`;
                    toRaw = `new_ptr_vk_array${refMethodsuffix}(${varName})`;
                    defValue = `&[]`;
                }
            } else if (isPointerValue) {
                rawType = `*mut ${rawTypeName}`;
                toWrapped = `new_vk_value(${varName})`;

                if (isOptional) {
                    wrappedType = `Option<&${wrappedTypeName}>`;
                    toRaw = `new_ptr_vk_value_checked(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `&${wrappedTypeName}`;
                    toRaw = `new_ptr_vk_value(${varName})`;
                    defValue = `vk_null_ref()`;
                }
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;

                if (field.countField) {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                    toWrapped = `new_vk_array(${countVarName}, ${varName}.as_ptr())`;
                    defValue = `Vec::new()`;
                } else {
                    wrappedType = `[${wrappedTypeName}; ${arraySize}]`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                    toWrapped = createStaticArray(wrappedTypeName, arraySize, varName, 'vk_to_wrapped_array');
                    defValue = fillStaticArray(wrappedTypeName, arraySize);
                }
            } else if (isInputHandle) {
                rawType = rawTypeName;

                // toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;

                if (isOptional) {
                    wrappedType = `Option<&${wrappedTypeName}>`;
                    toRaw = `if ${varName}.is_some() { vk_to_raw_value(${varName}.unwrap()) } else { 0 }`;
                    defValue = `None`;
                } else {
                    wrappedType = `&${wrappedTypeName}`;
                    toRaw = `vk_to_raw_value(${varName})`;
                    defValue = `vk_null_ref()`;
                }
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = `vk_to_raw_value(&${varName})`;
                toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;
                defValue = getPrimitiveDefaultValue(wrappedTypeName) || `${wrappedTypeName}::default()`;
            }
        }

        if (freeMemory) {
            freeRaw = freeMethod;
        }

        const info = {
            typeName: field.typeName,
            extension: field.extension,
            rawType: rawType,
            wrappedType: wrappedType,
            rawTypeName: rawTypeName,
            wrappedTypeName: wrappedTypeName,
            toRaw: stringToFunction(toRaw, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            toWrapped: stringToFunction(toWrapped, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            freeRaw: stringToFunction(freeRaw, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            defaultValue: defValue,
            varName: varNameValue
        };

        infos.push(info);
    }

    return infos;
}

function addUsesToSet(set, rootType, fields) {
    for (let field of fields) {
        const typeName = field.wrappedTypeName;

        if (typeName.startsWith('Vk') && typeName !== rootType.wrappedTypeName) {
            let use = `vk::`;
            if (field.extension) { use += `${field.extension}::`; }
            use += toSnakeCase(typeName);
            use += `::*`;

            set.add(use);
        }
    }
}

function stringToFunction(statement, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue) {
    if (!statement) {
        return null;
    }

    if (typeof statement === 'function') {
        return statement;
    }

    let body = [
        `var str = '${statement}';`,
        `str = str.replace('${varName}', makeVarName("${varNameValue}"));`,
        `str = str.replace('${varName}', makeVarName("${varNameValue}"));`
    ];

    if (countVarNameValue) {
        body.push(`str = str.replace('${countVarName}', makeCountVar("${countVarNameValue}", makeVarName));`);
        body.push(`str = str.replace('${countVarName}', makeCountVar("${countVarNameValue}", makeVarName));`);
    }

    if (arrayVarNameValue) {
        body.push(`str = str.replace('${arrayVarName}', makeVarName("${arrayVarNameValue}"));`);
    }

    body.push(
        `if (removeUnsafe) str = str.replace('unsafe {', '{');`,
        `return str;`
    );

    const baseFunc = new Function('makeCountVar', 'makeVarName', 'removeUnsafe', body.join('\n'));

    return function(makeVarName, removeUnsafe) {
        return baseFunc(getCountVarNameValue, makeVarName, removeUnsafe);
    };
}

module.exports = {
    toSnakeCase,
    toPascalCase,
    toUpperCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    blockToString,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldsInformation,
    findEnumPrefix,
    getStaticVkValueName,
    getConstVkValueName,
    addUsesToSet,
    isStructOrHandle,
    isStruct,
    isOutputHandleStruct,
    getCountVarNameValue
};