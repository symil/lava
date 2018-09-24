const {
    getRawVkTypeName,
    getWrappedVkTypeName,
    argToString,
    getFieldsInformation,
    isStructOrHandle,
    isStruct,
    getCountVarNameValue
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

    makeMethodNames(def, def.functions);

    return [
        genUses(def),
        genRawType(def),
        genWrappedType(def),
        genVkRawTypeTrait(def),
        genVkWrappedTypeTrait(def),
        genDefaultTrait(def),
        genPartialEqTrait(def),
        genVkSetupTrait(def),
        genSpecialMethods(def),
        genMethods(def)
    ];
}

function genUses() {
    const uses = new Set([
        `utils::c_bindings::*`,
        `utils::vk_traits::*`,
        `utils::vk_ptr::*`,
        `utils::vk_convert::*`,
        'std::os::raw::c_char',
        'std::ptr',
        'std::mem',
        'std::cmp',
        'std::slice',
        `vk::*`
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
        const singularToReplace = handle ? handle.name.replace('Vk', '') : '';
        const pluralToReplace = handle ? singularToReplace + 's' : '';

        let extension = '';
        let methodName = func.name
            .replace(/^vk/g, '')
            .replace(pluralToReplace, '')
            .replace(singularToReplace, '')
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

function genPartialEqTrait(def) {
    return [
        `impl PartialEq for ${def.wrappedTypeName}`, [
            `fn eq(&self, other: &${def.wrappedTypeName}) -> bool`, [
                `self._handle == other._handle`
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

function genSpecialMethods(def) {
    if (def.name === 'VkInstance') {
        return [
            `impl ${def.wrappedTypeName}`, [
                `pub fn create_surface<F : Fn(u64, *const c_void, *mut u64) -> i32>(&self, create_fn: F) -> Result<khr::VkSurface, VkResult>`, [
                    `unsafe`, [
                        `let raw_surface = &mut mem::uninitialized() as *mut khr::RawVkSurface;`,
                        `let vk_result = create_fn(self._handle, ptr::null(), raw_surface);`,
                        `if vk_result != 0 { return Err(RawVkResult::vk_to_wrapped(&vk_result)) }`,
                        `let mut surface = new_vk_value(raw_surface);`,
                        `let fn_table = self._fn_table;`,
                        `let parent_instance = self._parent_instance;`,
                        `let parent_device = self._parent_device;`,
                        `VkSetup::vk_setup(&mut surface, fn_table, parent_instance, parent_device);`,
                        `Ok(surface)`
                    ]
                ]
            ]
        ]
    }
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

function idFunction(name) {
    return name;
}

function functionToMethod(handle, func) {
    // if (func.name === 'vkGetValidationCacheDataEXT') console.log(func.args)

    const lastArg = func.args.last();
    const beforeLastArg = func.args.beforeLast();
    const createSomething = lastArg.isPointer && !lastArg.isConst;
    const beforeLastArgIsCountPtr = createSomething && beforeLastArg && !!beforeLastArg.countFor.length && beforeLastArg.isPointer;
    const createList = createSomething && lastArg.countField;
    const returnVkResult = func.type === 'VkResult';

    const methodName = func.methodName;
    const methodArgs = handle ? [{type: '&self'}] : [];
    const statements = [];
    const freeStataments = [];
    let returnStatement = null;
    
    const functionRustArgs = func.argsInfo.map((arg, index) => {
        const rawArg = func.args[index];

        if (createList && rawArg.countFor.includes(lastArg.name) && !beforeLastArgIsCountPtr) {
            const countVarName = getRawVarName(arg.varName);
            statements.push(`let ${countVarName} = ${arg.toRaw(idFunction)};`)
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

            statements.push(`let ${functionArgName} = ${arg.toRaw(idFunction, true)};`);

            if (arg.freeRaw) {
                freeStataments.push(`${arg.freeRaw(getRawVarName)};`);
            }

            return functionArgName;
        }
    });

    if (createList && !lastArg.countField) {
        throw new Error(`function ${func.name} returns an array but does not take a count, need manual review`);
    }

    const isVkMapMemory = func.name === 'vkMapMemory';
    const funcNameValue = handle ? `((&*self._fn_table).${func.name})` : func.name;
    const functionCall = `${funcNameValue}(${functionRustArgs.join(', ')})`;

    let returnType = null;
    let generics = '';

    if (createSomething) {
        if (returnVkResult) {
            statements.push(`let mut vk_result = ${VK_SUCCESS};`);
        }

        const wrappedFunctionCall = returnVkResult ? `vk_result = ${functionCall};` : `${functionCall};`;
        const createdType = func.argsInfo.last();
        let createdRawTypeName = prefixWithExtension(createdType.extension, createdType.rawTypeName);
        let createdWrappedTypeName = prefixWithExtension(createdType.extension, createdType.wrappedTypeName);

        if (isVkMapMemory) {
            createdRawTypeName = '*mut c_void';
            createdWrappedTypeName = `&'a mut [c_void]`;
            generics = `<'a>`;
        }

        const setupResult = createdType.typeName === 'VkInstance' || (handle && isStructOrHandle(createdType));
        const resutlIsStruct = isStruct(createdType);

        const wrappedResultVarName = createdType.varName;
        const rawResultName = getRawVarName(createdType.varName);

        if (!createList) {
            statements.push(
                `let ${rawResultName} = &mut mem::zeroed() as *mut ${createdRawTypeName};`,
                ``,
                wrappedFunctionCall,
                ``,
                `let${setupResult ? ' mut' : ''} ${wrappedResultVarName} = ${createdType.toWrapped(getRawVarName)};`,
            );

            if (isVkMapMemory) {
                statements[statements.length - 1] = `let ${wrappedResultVarName} = slice::from_raw_parts_mut(*${rawResultName}, size);`
            }

            if (setupResult) {
                const isInstance = createdType.typeName === 'VkInstance';
                const isDevice = createdType.typeName === 'VkDevice';

                const functionTable = isInstance ? `Box::into_raw(Box::new(VkInstanceFunctionTable::new(*${rawResultName})))` : `self._fn_table`;
                const parentInstance = isInstance ? `*${rawResultName}` : `self._parent_instance`;
                const parentDevice = isDevice ? `*${rawResultName}` : (isInstance ? VK_NULL_HANDLE : `self._parent_device`);

                const setupStatements = [
                    `let fn_table = ${functionTable};`,
                    `let parent_instance = ${parentInstance};`,
                    `let parent_device = ${parentDevice};`,
                    `VkSetup::vk_setup(&mut ${wrappedResultVarName}, fn_table, parent_instance, parent_device);`
                ];

                if (returnVkResult) {
                    statements.push(`if vk_result == ${VK_SUCCESS}`, setupStatements);
                } else {
                    for (const statement of setupStatements) {
                        statements.push(statement);
                    }
                }
            }

            if (resutlIsStruct) {
                freeStataments.push(
                    `${createdRawTypeName}::vk_free(${rawResultName}.as_mut().unwrap());`
                );
            }
        } else {
            const rawCountName = getCountVarNameValue(lastArg.countField, getRawVarName);

            if (beforeLastArgIsCountPtr) {
                statements.push(
                    `let mut ${rawResultName} : *mut ${createdRawTypeName} = ptr::null_mut();`,
                    `let ${rawCountName} = &mut mem::zeroed() as *mut ${func.argsInfo.beforeLast().rawTypeName};`,
                    wrappedFunctionCall
                );
            }

            statements.push(
                `${beforeLastArgIsCountPtr ? '' : 'let '}${rawResultName} = calloc(${beforeLastArgIsCountPtr ? '*' : ''}${rawCountName} as usize, mem::size_of::<${createdRawTypeName}>()) as *mut ${createdRawTypeName};`,
                ``,
                wrappedFunctionCall,
                ``,
                `let${setupResult ? ' mut' : ''} ${wrappedResultVarName} = ${createdType.toWrapped(getRawVarName)};`
            );

            if (setupResult) {
                const setupStatement = `for elt in &mut ${wrappedResultVarName} { VkSetup::vk_setup(elt, self._fn_table, self._parent_instance, self._parent_device); }`;

                if (returnVkResult) {
                    statements.push(`if vk_result == ${VK_SUCCESS}`, [ setupStatement ]);
                } else {
                    statements.push(setupStatement);
                }
            }

            if (createdType.freeRaw) {
                freeStataments.push(`${createdType.freeRaw(getRawVarName)};`);
            }
        }

        returnType = createdWrappedTypeName;

        if (createList) {
            returnType = `Vec<${returnType}>`;
        }

        if (returnVkResult) {
            returnType = `Result<${returnType}, (VkResult, ${returnType})>`;
            returnStatement = `if vk_result == ${VK_SUCCESS} { Ok(${wrappedResultVarName}) } else { Err((RawVkResult::vk_to_wrapped(&vk_result), ${wrappedResultVarName})) }`
        } else {
            returnStatement = wrappedResultVarName;
        }
    } else if (returnVkResult) {
        statements.push(`let vk_result = ${functionCall};`);

        if (methodName === 'get_status') {
            returnType = `VkResult`;
            returnStatement = `RawVkResult::vk_to_wrapped(&vk_result)`;
        } else {
            returnType = 'Result<(), VkResult>';
            returnStatement = `if vk_result == ${VK_SUCCESS} { Ok(()) } else { Err(RawVkResult::vk_to_wrapped(&vk_result)) }`
        }
    } else {
        statements.push(`${functionCall};`);
    }

    if (func.name === 'vkDestroyInstance') {
        freeStataments.push(`Box::from_raw(self._fn_table);`);
    }

    const argList = methodArgs.map(argToString).join(', ');
    const returnInfo = returnType ? ` -> ${returnType}` : '';
    
    const allStatements = statements.concat(freeStataments);

    if (returnStatement) {
        allStatements.push(returnStatement);
    }

    return [
        `\npub fn ${methodName}${generics}(${argList})${returnInfo}`,
        [`unsafe`, allStatements]
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