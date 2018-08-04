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
    addUsesToSet
} = require('./utils');

const VK_SUCCESS = 0;

function generateVkHandleDefinition(def) {
    def.rawTypeName = getRawVkTypeName(def.name);
    def.wrappedTypeName = getWrappedVkTypeName(def.name);

    return [
        genUses(def),
        genRawType(def),
        genWrappedType(def),
        genVkRawTypeTrait(def),
        genVkWrappedTypeTrait(def),
        genVkDefaultTrait(def),
        genMethods(def),
        genExterns(def)
    ];
}

function genUses(def) {
    const uses = new Set([
        `utils::vk_type::*`,
        `utils::vk_ptr::*`,
        `utils::vk_convert::*`,
        'std::os::raw::c_char',
        'std::ptr',
        'std::mem',
        `vk::vk_result::*`
    ]);

    if (def.parent && def.name !== 'VkInstance') {
        let use = `vk::`;
        if (def.parent.extension) use += `::${def.parent.extension}`;
        use += `${toSnakeCase(def.parent.name)}::*`;

        uses.add(use);
    }

    for (let func of def.functions) {
        addUsesToSet(uses, def, func.argsInfo);
    }

    return Array.from(uses.values()).map(str => `use ${str};`);
}

function genRawType(def) {
    return `pub type ${def.rawTypeName} = u64;`;
}

function genWrappedType(def) {
    return [
        `#[derive(Debug, Copy, Clone)]`,
        `pub struct ${def.wrappedTypeName}`, [
            `_handle: ${def.rawTypeName},`,
            def.parent ? `_parent_handle: Raw${def.parent.name},` : null
        ]
    ];
}

function genVkRawTypeTrait(def) {
    return [
        `impl VkRawType<${def.wrappedTypeName}> for ${def.rawTypeName}`, [
            `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName, [
                    `_handle: *src,`,
                    def.parent ? `_parent_handle: 0,` : null
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

function genVkDefaultTrait(def) {
    return [
        `impl VkDefault for ${def.wrappedTypeName}`, [
            `fn vk_default() -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName, [
                    `_handle: 0,`,
                    def.parent ? `_parent_handle: 0,` : null
                ]
            ]
        ]
    ];
}

function genMethods(def) {
    if (def.name !== 'VkInstance') {
        return;
    }

    const handleMethod = [
        `\npub fn handle(&self) -> u64`, [
            `self._handle`
        ]
    ];
    return [
        `impl ${def.wrappedTypeName}`,
        def.functions.map(func => functionToMethod(def, func)).reduce((acc, block) => acc.concat(block), handleMethod)
    ];
}

function genExterns(def) {
    if (!def.functions.length || def.name !== 'VkInstance') {
        return;
    }

    return [
        `extern`,
        def.functions.map(func => functionToDeclaration(func)).reduce((acc, block) => acc.concat(block), [])
    ];
}

function functionToDeclaration(func) {
    const args = func.argsInfo.map(arg => `${arg.varName}: ${arg.rawType}`);
    const returnType = func.type === 'VkResult' ? ' -> RawVkResult' : '';

    return `fn ${func.name}(${args.join(', ')})${returnType};`;
}

function makeMethodName(functionName, handleName) {
    return functionName
        .replace(/^vk/g, '')
        .replace(handleName.replace('Vk', ''), '')
        .replace(/[A-Z]+$/, '')
        .toSnakeCase();
}

function functionToMethod(handle, func) {
    const lastArg = func.args.last();
    const beforeLastArg = func.args.beforeLast();
    const createSomething = lastArg.isPointer && !lastArg.isConst;
    const beforeLastArgIsCount = beforeLastArg && !!beforeLastArg.countFor.length;
    const createList = createSomething && lastArg.countField;
    const returnVkResult = func.type === 'VkResult';

    const methodName = makeMethodName(func.name, handle.name);
    const methodArgs = [{type: '&self'}];
    const statements = [];
    const functionRustArgs = func.argsInfo.map((arg, index) => {
        if (index === func.args.length - 1 && createSomething) {
            return 'raw_result_ptr';
        } else if (index === func.args.length - 2 && createSomething && beforeLastArgIsCount) {
            return 'count_ptr';
        } else if (arg.varName === 'allocator') {
            return 'ptr::null()';
        } else if (index <= 1 && arg.typeName === handle.name) {
            return `self._handle`;
        } else if (handle.parent && index === 0 && arg.typeName === handle.parent.name) {
            return `self._parent_handle`;
        } else {
            const argInfo = func.argsInfo[index];

            const methodArgName = argInfo.varName;
            const functionArgName = `raw_${methodArgName}`;

            methodArgs.push({name: methodArgName, type: argInfo.wrappedType});
            statements.push(`let ${functionArgName} = ${argInfo.toRaw(x => x)};`);

            return functionArgName;
        }
    });

    if (createList && !beforeLastArgIsCount) {
        throw new Error(`function ${func.name} returns an array but does not takes a count, need manual review`);
    }

    const functionCall = `${func.name}(${functionRustArgs.join(', ')})`;
    let returnType = null;

    if (createSomething) {
        const createdRawTypeName = func.argsInfo.last().rawTypeName;
        const createdWrappedTypeName = func.argsInfo.last().wrappedTypeName;

        if (createList) {
            statements.push(
                `let mut raw_result : Vec<${createdRawTypeName}> = Vec::new();`,
                `let mut raw_result_ptr : *mut ${createdRawTypeName} = ptr::null_mut();`,
                `let mut count : u32 = 0;`,
                `let count_ptr = &mut count as *mut u32;`,
            );

            if (returnVkResult) {
                statements.push(
                    ``,
                    `let vk_result_1 = ${functionCall};`,
                    `if vk_result_1 != ${VK_SUCCESS}`, [
                        `return Err(RawVkResult::vk_to_wrapped(&vk_result_1))`
                    ],
                    ``
                )
            } else {
                statements.push(`${functionCall};`);
            }

            statements.push(
                `raw_result.reserve(count as usize);`,
                `raw_result.set_len(count as usize);`,
                `raw_result_ptr = raw_result.as_mut_ptr();`,
            );

            if (returnVkResult) {
                statements.push(
                    ``,
                    `let vk_result_2 = ${functionCall};`,
                    `if vk_result_2 != ${VK_SUCCESS}`, [
                        `return Err(RawVkResult::vk_to_wrapped(&vk_result_2))`
                    ]
                );
            } else {
                statements.push(`${functionCall};`);
            }

            statements.push(
                ``,
                `let wrapped_result = raw_result.iter().map(|raw| { ${createdRawTypeName}::vk_to_wrapped(raw) }).collect();`
            );

            if (returnVkResult) {
                statements.push(`Ok(wrapped_result)`);
                returnType = `Result<Vec<${createdWrappedTypeName}>, VkResult>`
            } else {
                statements.push(`wrapped_result`);
                returnType = `Vec<${createdWrappedTypeName}>`
            }

        } else {
            statements.push(
                `let mut raw_result : ${createdRawTypeName} = mem::uninitialized();`,
                `let raw_result_ptr = &mut raw_result as *mut ${createdRawTypeName};`,
                ``
            );

            if (returnVkResult) {
                statements.push(
                    `let vk_result = ${functionCall};`,
                    `if vk_result != ${VK_SUCCESS}`, [
                        `return Err(RawVkResult::vk_to_wrapped(&vk_result))`
                    ]
                )
            } else {
                statements.push(`${functionCall};`);
            }

            statements.push(
                ``,
                `let wrapped_result = ${createdRawTypeName}::vk_to_wrapped(&raw_result);`
            );

            if (returnVkResult) {
                statements.push(`Ok(wrapped_result)`);
                returnType = `Result<${createdWrappedTypeName}, VkResult>`
            } else {
                statements.push(`wrapped_result`);
                returnType = createdWrappedTypeName;
            }
        }
    } else {
        statements.push(`${functionCall};`);
    }

    if (func.name === 'vkCreateInstance') {
        methodArgs.shift();
    }

    const argList = methodArgs.map(argToString).join(', ');
    const returnInfo = returnType ? ` -> ${returnType}` : '';
    
    return [
        `\npub fn ${methodName}(${argList})${returnInfo}`,
        [`unsafe`, statements]
    ];
}

module.exports = {
    generateVkHandleDefinition
};