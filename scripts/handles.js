const {
    toSnakeCase,
    toPascalCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isPlural,
    removeSuffix,
    cToRustVarName,
    argToString,
    getConstVkValueName,
    getFieldsInformation,
    addUsesToSet,
    isStructOrHandle
} = require('./utils');

const VK_SUCCESS = 0;
const VK_NULL_HANDLE = 0;

function generateVkHandleDefinition(def) {
    def.rawTypeName = getRawVkTypeName(def.name);
    def.wrappedTypeName = getWrappedVkTypeName(def.name);

    for (let func of def.functions) {
        if (!func.argsInfo) {
            func.argsInfo = getFieldsInformation(func.args);
        }
    }

    makeMethodNames(def, def.functions)

    return [
        genUses(def),
        genRawType(def),
        genWrappedType(def),
        genVkRawTypeTrait(def),
        genVkWrappedTypeTrait(def),
        genDefaultTrait(def),
        genVkSetupTrait(def),
        genMethods(def)
    ];
}

function genUses(def) {
    const uses = new Set([
        `utils::c_bindings::*`,
        `utils::vk_traits::*`,
        `utils::vk_ptr::*`,
        `utils::vk_convert::*`,
        'std::os::raw::c_char',
        'std::ptr',
        'std::mem',
        'std::cmp',
        `vk::*`,
        `glfw::*`
    ]);

    // if (def.name !== 'VkInstance') {
    //     uses.add('vk::vk_instance::*');
    // }

    // if (def.name !== 'VkDevice') {
    //     uses.add('vk::vk_device::*');
    // }

    // if (def.parent && def.name !== 'VkInstance') {
    //     let use = `vk::`;
    //     if (def.parent.extension) use += `::${def.parent.extension}`;
    //     use += `${toSnakeCase(def.parent.name)}::*`;

    //     uses.add(use);
    // }

    // for (let func of def.functions) {
    //     addUsesToSet(uses, def, func.argsInfo);
    // }

    return Array.from(uses.values()).map(str => `use ${str};`);
}

function makeMethodNames(handle, functions) {
    const assigned = new Set();

    for (const func of functions) {
        const toReplace = handle ? handle.name.replace('Vk', '') : '';

        let extension = '';
        let methodName = func.name
            .replace(/^vk/g, '')
            .replace(toReplace, '')
            .replace(/[A-Z]+$/, ext => { extension = ext.toLowerCase(); return ''; })
            .toSnakeCase();

        if (assigned.has(methodName)) {
            methodName += `_${extension}`;
        }

        assigned.add(methodName);
        func.methodName = methodName;
    }
}

function genRawType(def) {
    return `pub type ${def.rawTypeName} = u64;`;
}

function genWrappedType(def) {
    return [
        `#[derive(Debug, Copy, Clone)]`,
        `pub struct ${def.wrappedTypeName}`, [
            `_handle: ${def.rawTypeName},`,
            `_parent_instance: RawVkInstance,`,
            `_parent_device: RawVkDevice,`,
            `_fn_table: *mut VkInstanceFunctionTable`
        ]
    ];
}

function genVkRawTypeTrait(def) {
    return [
        `impl VkRawType<${def.wrappedTypeName}> for ${def.rawTypeName}`, [
            `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName, [
                    `_handle: *src,`,
                    `_parent_instance: ${VK_NULL_HANDLE},`,
                    `_parent_device: ${VK_NULL_HANDLE},`,
                    `_fn_table: ptr::null_mut()`
                ]
            ],
        ]
    ];
}

function genVkWrappedTypeTrait(def) {
    return [
        `impl VkWrappedType<${def.rawTypeName}> for ${def.wrappedTypeName}`, [
            `fn vk_to_raw(src: &${def.wrappedTypeName}, dst: &mut ${def.rawTypeName})`, [
                `*dst = src._handle`
            ]
        ]
    ];
}

function genDefaultTrait(def) {
    return [
        `impl Default for ${def.wrappedTypeName}`, [
            `fn default() -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName, [
                    `_handle: ${VK_NULL_HANDLE},`,
                    `_parent_instance: ${VK_NULL_HANDLE},`,
                    `_parent_device: ${VK_NULL_HANDLE},`,
                    `_fn_table: ptr::null_mut()`
                ]
            ]
        ]
    ];
}

function genVkSetupTrait(def) {
    return [
        `impl VkSetup for ${def.wrappedTypeName}`, [
            `fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice)`, [
                `self._parent_instance = instance;`,
                `self._parent_device = device;`,
                `self._fn_table = fn_table;`
            ]
        ]
    ]
}

function genMethods(def) {
    const handleMethod = [
        `\npub fn handle(&self) -> u64`, [
            `self._handle`
        ],
    ];
    return [
        `impl ${def.wrappedTypeName}`,
        def.functions.map(func => functionToMethod(def, func)).reduce((acc, block) => acc.concat(block), handleMethod)
    ];
}

function getRawVarName(varName) {
    return `raw_${varName}`;
}

function idName(name) {
    return name;
}

function functionToMethod(handle, func) {
    // if (func.name === 'vkGetValidationCacheDataEXT') console.log(func.args)

    const lastArg = func.args.last();
    const beforeLastArg = func.args.beforeLast();
    const createSomething = lastArg.isPointer && !lastArg.isConst && lastArg.fullType !== 'void**';
    const beforeLastArgIsCountPtr = createSomething && beforeLastArg && !!beforeLastArg.countFor.length && beforeLastArg.isPointer;
    const createList = createSomething && lastArg.countField;
    const returnVkResult = func.type === 'VkResult';

    const methodName = func.methodName;
    const methodArgs = handle ? [{type: '&self'}] : [];
    const statements = [];
    
    const functionRustArgs = func.argsInfo.map((arg, index) => {
        const rawArg = func.args[index];

        if (createList && rawArg.countFor.includes(lastArg.name) && !beforeLastArgIsCountPtr) {
            const countVarName = getRawVarName(arg.varName);
            statements.push(`let ${countVarName} = ${arg.toRaw(idName)};`)
            return countVarName;
        } else if (index === func.args.length - 1 && createSomething) {
            return getRawVarName(arg.varName);
        } else if (index === func.args.length - 2 && createSomething && beforeLastArgIsCountPtr) {
            return getRawVarName(arg.varName);
        } else if (arg.varName === 'allocator') {
            return 'ptr::null()';
        } else if (handle && index <= 1 && arg.typeName === handle.name) {
            return `self._handle`;
        } else if (handle && handle.parent && index === 0 && arg.typeName === handle.parent.name) {
            return handle.parent.name === 'VkInstance' ? `self._parent_instance` : `self._parent_device`;
        } else {
            const methodArgName = arg.varName;
            const functionArgName = getRawVarName(methodArgName);
            
            if (arg.wrappedType) {
                const methodArg = { name: methodArgName, type: arg.wrappedType };

                if (arg.extension) {
                    methodArg.type = arg.wrappedType.replace(arg.wrappedTypeName, `${arg.extension}::${arg.wrappedTypeName}`)
                }

                methodArgs.push(methodArg);
            }

            statements.push(`let ${functionArgName} = ${arg.toRaw(idName, true)};`);

            return functionArgName;
        }
    });

    if (createList && !lastArg.countField) {
        throw new Error(`function ${func.name} returns an array but does not take a count, need manual review`);
    }

    const funcNameValue = handle ? `((&*self._fn_table).${func.name})` : func.name;
    const functionCall = `${funcNameValue}(${functionRustArgs.join(', ')})`;

    let returnType = null;

    if (createSomething) {
        const wrappedFunctionCall = returnVkResult
            ? `let vk_result = ${functionCall};\nif vk_result != ${VK_SUCCESS} { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }`
            : `${functionCall};`;

        const createdType = func.argsInfo.last();
        const createdRawTypeName = prefixWithExtension(createdType.extension, createdType.rawTypeName);
        const createdWrappedTypeName = prefixWithExtension(createdType.extension, createdType.wrappedTypeName);

        const setupResult = createdType.typeName === 'VkInstance' || (handle && isStructOrHandle(createdType));

        const wrappedResultVarName = createdType.varName;
        const rawResultName = getRawVarName(createdType.varName);

        if (!createList) {
            statements.push(
                `let ${rawResultName} = &mut mem::uninitialized() as *mut ${createdRawTypeName};`,
                ``,
                wrappedFunctionCall,
                ``,
                `let${setupResult ? ' mut' : ''} ${wrappedResultVarName} = ${createdType.toWrapped(getRawVarName)};`,
            );

            if (setupResult) {
                const isInstance = createdType.typeName === 'VkInstance';
                const isDevice = createdType.typeName === 'VkDevice';

                const functionTable = isInstance ? `Box::into_raw(Box::new(VkInstanceFunctionTable::new(*${rawResultName})))` : `self._fn_table`;
                const parentInstance = isInstance ? `*${rawResultName}` : `self._parent_instance`;
                const parentDevice = isDevice ? `*${rawResultName}` : (isInstance ? VK_NULL_HANDLE : `self._parent_device`);

                statements.push(
                    `let fn_table = ${functionTable};`,
                    `let parent_instance = ${parentInstance};`,
                    `let parent_device = ${parentDevice};`,
                    `VkSetup::vk_setup(&mut ${wrappedResultVarName}, fn_table, parent_instance, parent_device);`
                );
            }
        } else {
            const countArg = func.argsInfo[func.args.findIndex(arg => arg.countFor.includes(lastArg.name))];
            const rawCountName = getRawVarName(countArg.varName);

            if (beforeLastArgIsCountPtr) {
                statements.push(
                    `let mut ${rawResultName} : *mut ${createdRawTypeName} = ptr::null_mut();`,
                    `let ${rawCountName} = &mut mem::uninitialized() as *mut ${func.argsInfo.beforeLast().rawTypeName};`,
                    wrappedFunctionCall
                );
            }

            statements.push(
                `${beforeLastArgIsCountPtr ? '' : 'let '}${rawResultName} = malloc((${beforeLastArgIsCountPtr ? '*' : ''}${rawCountName} as usize) * mem::size_of::<${createdRawTypeName}>()) as *mut ${createdRawTypeName};`,
                ``,
                wrappedFunctionCall,
                ``,
                `let${setupResult ? ' mut' : ''} ${wrappedResultVarName} = ${createdType.toWrapped(getRawVarName)};`
            );

            if (setupResult) {
                statements.push(
                    `for elt in &mut ${wrappedResultVarName} { VkSetup::vk_setup(elt, self._fn_table, self._parent_instance, self._parent_device); }`
                );
            }
        }

        returnType = createdWrappedTypeName;

        if (createList) {
            returnType = `Vec<${returnType}>`;
        }

        if (returnVkResult) {
            returnType = `Result<${returnType}, VkResult>`;
        }

        
        statements.push(returnVkResult ? `Ok(${wrappedResultVarName})` : wrappedResultVarName);
    } else if (returnVkResult) {
        returnType = 'VkResult';
        statements.push(
            `let vk_result = ${functionCall};`,
            `RawVkResult::vk_to_wrapped(&vk_result)`
        );
    } else {
        statements.push(`${functionCall};`);
    }

    if (func.name === 'vkDestroyInstance') {
        statements.push(`Box::from_raw(self._fn_table);`);
    }

    const argList = methodArgs.map(argToString).join(', ');
    const returnInfo = returnType ? ` -> ${returnType}` : '';
    
    return [
        `\npub fn ${methodName}(${argList})${returnInfo}`,
        [`unsafe`, statements]
    ];
}

function prefixWithExtension(ext, name) {
    return ext ? `${ext}::${name}` : name;
}

module.exports = {
    generateVkHandleDefinition,
    makeMethodNames,
    functionToMethod
};