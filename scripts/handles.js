const { toSnakeCase, removeSuffix, getRawTypeName, getWrappedTypeName, blockToString, isCount, isPlural } = require('./utils');

const COUNT_VAR_NAME = 'count';
const RESULT_VAR_NAME = 'raw_result';
const RESULT_PTR_VAR_NAME = 'raw_result_ptr';

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
                this._methods.map(cFunction => this._methodToBlock(cFunction)).reduce((acc, f) => acc.concat([``, ...f]), [])
            
        ];

        return blockToString(block);
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
        const functionRustArgs = cFunction.args.map((arg, index) => {
            if (index === cFunction.args.length - 1 && createSomething) {
                return RESULT_PTR_VAR_NAME;
            } else if (index === cFunction.args.length - 2 && createSomething && beforeLastArgIsCount) {
                return COUNT_VAR_NAME;
            } else if (arg.typeName === 'VkAllocationCallbacks') {
                return 'ptr::null()';
            } else {
                const rawArgType = toRawTypeName(arg.typeName);
                const wrappedArgType = toWrappedTypeName(arg.typeName);

                if (arg.isPointer && arg.isConst) {
                    // Structure that needs to be passed to the rust method

                    const rustArgName = toSnakeCase(arg.name.substring(1));
                    const rawVarName = `raw_${rustArgName}`;
                    const ptrVarName = `${rawVarName}_ptr`;

                    statements.push(
                        `let mut ${rawVarName} = ${rawArgType}::vk_from(${rustArgName});`, // TODO: properly convert VkBool32
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

        return [`pub fn ${methodName}()`, []]
    }
}

exports.HandleList = HandleList;