const {
    toSnakeCase,
    toPascalCase,
    getRawTypeName,
    getWrappedTypeName,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isCount,
    isPlural,
    removeSuffix,
    cToRustVarName,
    argToString
} = require('./utils');

class HandleList {
    constructor() {
        this._handles = {};
    }

    get(name) {
        return this._handles[name] || (this._handles[name] = new Handle(name));
    }
}

class Handle {
    constructor(name) {
        this._name = name;
        this._parent = null;
        this._toDestroy = [];
        this._methods = [];
    }

    setParent(parent) {
        this._parent = parent;
    }

    addHandleToDestroy(handleName, destroyFunction) {
        this._toDestroy.push({handleName, destroyFunction});
    }

    addMethod(cFunction) {
        this._methods.push(cFunction);
    }

    toString() {
        const rawTypeName = getRawTypeName(this._name);
        const wrappedTypeName = getWrappedTypeName(this._name);

        const block = [
            `use std::string::String;`,
            `use std::vec::Vec;`,
            `use std::*;`,
            `use vk::*;`,
            ``,
            `pub type ${rawTypeName} = u64;`,
            ``,
            `pub struct ${wrappedTypeName}`, [
                `_handle: ${rawTypeName},`,
                this._parent ? `_parent: ${getRawTypeName(this._parent)}` : null
            ],
            ``,
            `impl<'a> VkFrom<&'a ${wrappedTypeName}> for ${rawTypeName}`, [
                `fn vk_from(value: &'a ${wrappedTypeName}) -> Self`, [
                    `value.raw_handle()`
                ]
            ],
            ``,
            `impl VkFrom<${rawTypeName}> for ${wrappedTypeName}`, [
                `fn vk_from(value: &'a ${wrappedTypeName}) -> Self`, [
                    `Self`, [
                        `_handle: value`,
                        this._parent ? `_parent: VK_NULL_HANDLE` : null
                    ]
                ]
            ],
            ``,
            `impl ${wrappedTypeName}`,
                this._methods.slice(0, 3).map(cFunction => this._methodToBlock(cFunction)).reduce((acc, f) => acc.concat([``, ...f]), []),
            ``,
            `extern`,
                this._methods.map(cFunction => this._methodToExternDeclaration(cFunction))
            
        ];

        return blockToString(block);
    }

    _methodToExternDeclaration(cFunction) {
        return `${cFunction.name}();`
    }

    _methodToBlock(cFunction) {
        const lastArg = cFunction.args.last();
        const beforeLastArg = cFunction.args.beforeLast();
        const instanceVarName = toSnakeCase(lastArg.typeName.substring(2));
        const createSomething = lastArg.isPointer && !lastArg.isConst;
        const beforeLastArgIsCount = isCount(beforeLastArg);
        const createList = createSomething && (beforeLastArgIsCount || isPlural(lastArg));
        const returnVkResult = cFunction.type === 'VkResult';

        const methodName = toSnakeCase(removeSuffix(cFunction.name.substring(2)));
        const methodArgs = [];
        const statements = [];
        const functionRustArgs = cFunction.args.map((arg, index) => {
            if (index === cFunction.args.length - 1 && createSomething) {
                return 'raw_result_ptr';
            } else if (index === cFunction.args.length - 2 && createSomething && beforeLastArgIsCount) {
                return 'count_ptr';
            } else if (arg.typeName === 'VkAllocationCallbacks') {
                return 'ptr::null()';
            } else {
                const rawArgType = getFullRawType(arg);
                const wrappedArgType = getFullWrappedType(arg);

                const methodArgName = cToRustVarName(arg.name);
                const functionArgName = `raw_${methodArgName}`;

                let wrappedValue;

                if (arg.fullType === this._name) {
                    wrappedValue = 'self';
                } else {
                    wrappedValue = methodArgName;
                    methodArgs.push({name: methodArgName, type: wrappedArgType});
                }

                statements.push(`let ${functionArgName} : ${rawArgType} = VkFrom::vk_from(${wrappedValue});`);

                return functionArgName;
            }
        });

        if (createList && !beforeLastArgIsCount) {
            throw new Error(`function ${cFunction.name} returns an array but does not takes a count, need manual review`);
        }

        if (createSomething) {
            const functionCall = `${cFunction.name}(${functionRustArgs.join(', ')})`;
            const createdRawTypeName = getRawTypeName(lastArg.typeName);
            const createdWrappedTypeName = getWrappedTypeName(lastArg.typeName);

            if (createList) {
                statements.push(
                    `let mut raw_result : Vec<${createdRawTypeName}> = Vec::new();`,
                    `let mut raw_result_ptr = ptr:null_mut();`,
                    `let mut count : u32 = 0;`,
                    `let count_ptr = &mut count as *mut u32;`,
                );

                if (returnVkResult) {
                    statements.push(
                        `let vk_result_1 = ${functionCall};`,
                        `if (vk_result_1 != VK_SUCCESS)`, [
                            `return Err(VkResult::vk_from(vk_result_1))`
                        ]
                    )
                } else {
                    statements.push(`${functionCall};`);
                }

                statements.push(
                    ``,
                    `raw_result.reserve(count as usize);`,
                    `raw_result.set_len(count as usize);`,
                    `raw_result_ptr = raw_result.as_mut_ptr();`,
                    `${functionCall};`,
                );

                if (returnVkResult) {
                    statements.push(
                        `let vk_result_2 = ${functionCall};`,
                        `if (vk_result_2 != VK_SUCCESS)`, [
                            `return Err(VkResult::vk_from(vk_result_2))`
                        ]
                    )
                } else {
                    statements.push(`${functionCall};`);
                }

                statements.push(
                    ``,
                    `let wrapped_result = raw_result.iter().map(|raw| { ${createdWrappedTypeName}::vk_from(raw) }).collect();`,
                    `Ok(wrapped_result)`
                );

            } else {
                statements.push(

                );
            }
        }

        const argList = methodArgs.map(argToString).join(', ');

        return [`pub fn ${methodName}(${argList})`, statements];
    }
}

exports.HandleList = HandleList;