#!/usr/bin/env node

Array.prototype.last = function() {
    return this[this.length - 1];
};

const { parseType, parseFunction } = require('./parse');

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
    void: 'void'
};

const SCHEMA = {
    VkInstance: {
        new: {
            static: true,
            cMethod: 'vkCreateInstance'
        },
        // getSupportedExtensions: {
        //     args: [],
        //     methodName: 'vkEnumerateInstanceExtensionProperties',
        //     params: {
        //         pLayerName: null
        //     }
        // }
    },
    VkBuffer: {
        new: {
            static: true,
            cMethod: 'vkCreateBuffer'
        }
    }
};

function writeType(typeName) {
    const { type, fields } = parseType(typeName);

    if (type === 'handle') {

    }
}
generateHandle('VkBuffer', SCHEMA['VkBuffer']);

function generateHandle(typeName, definition = {}) {
    const uses = [
        `std::convert::From`,
        `std::ops::Drop`,
        `std::vec::Vec`,
        `std::ptr::null`,
        `vk::RawVkHandle`
    ];

    const rawTypeName = toRawTypeName(typeName);
    const trueTypeName = toTrueTypeName(typeName);
    const additionalFields = [];
    const fields = [{name: '_handle', type: rawTypeName}];

    const output = {};

    const methods = Object.entries(definition).filter(([key]) => !key.startsWith('_'));

    if (methods.length) {
        const methodDefinitions = methods.map(([methodName, def]) => {
            let statements = [];
            let returnType = null;
            let methodArguments = [];
            let unsafe = false;

            if (!def.static) {
                methodArguments.push('&self');
            }

            if (def.cMethod) {
                unsafe = true;

                const cMethod = parseFunction(def.cMethod);
                const defArgs = def.cMethod.args;
                const cMethodArgs = cMethod.args;
                const lastArg = cMethodArgs[cMethodArgs.length - 1];
                const beforeLastArg = cMethodArgs[cMethodArgs.length - 2];
                const createSomething = lastArg.isPointer && !lastArg.isConst;
                const createList = createSomething && beforeLastArg.isPointer && !beforeLastArg.isConst && beforeLastArg.typeName === 'uint32_t';
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
                        const trueArgType = toTrueTypeName(arg.typeName);

                        if (arg.isPointer && arg.isConst) {
                            // Structure that needs to be passed to the rust method
        
                            const rustArgName = toSnakeCase(arg.name.substring(1));
                            const rawVarName = `raw_${rustArgName}`;
                            const ptrVarName = `${rawVarName}_ptr`;
        
                            statements.push(
                                `let mut ${rawVarName} = ${rawArgType}::from(${rustArgName});`,
                                `let ${ptrVarName} = &mut ${rawVarName} as *mut ${rawArgType};`
                            );
        
                            methodArguments.push(`${rustArgName}: &${trueArgType}`);
        
                            return ptrVarName;
                        } else if (!arg.isPointer) {
                            const rustArgName = toSnakeCase(arg.name);

                            methodArguments.push(`${rustArgName}: &${trueArgType}`);

                            return `${rustArgName}.handle()`;
                        }
                    }

                    return '<missing>';
                });
    
                if (createSomething) {
                    returnType = toTrueTypeName(lastArg.typeName);
    
                    if (createList) {
                        returnType = `Vec<${returnType}>`;
                    }
    
                    if (returnVkResult) {
                        returnType = `Result<${returnType}, VkResult>`;
                    }
    
                    const vkWrapperCallName = `vk_call_retrieve_${createList ? 'list' : 'single'}${returnVkResult ? '' : '_unchecked'}`;
                    const lambdaArgs = createList ? '|count, ptr|' : '|ptr|';
                    const callback = `|x| {}`;
                    statements.push(`${vkWrapperCallName}(${lambdaArgs} ${cMethod.name}(${argsForCMethod.join(', ')}), ${callback})`);
                } else {
                    throw new Error(`method ${cMethod.name} does not seem to retrieve anything`);
                }
            }

            const functionPrototype = `pub fn ${toSnakeCase(methodName)}(${methodArguments.join(', ')})${returnType ? ` -> ${returnType}` : ''}`;
            const body = unsafe ? ['unsafe', statements] : statements;

            const functionDef = [
                functionPrototype, body
            ];

            console.log(blockToStr(functionDef));
        });
    }

    output.rawDefinition = `pub type ${rawTypeName}`;

    output.trueDefinition = [
        `pub struct ${trueTypeName}`,
        fields.map(({name, type}) => `${name}: ${type}`)
    ];

    output.fromRawToTrue = [
        `impl<'a> From<&'a ${rawTypeName}> for ${trueTypeName}`, [
            `fn from(raw: &'a ${rawTypeName}) -> Self`, [
                `Self`, 
                    fields.map(({name}) => `${name}: ${name === '_handle' ? '*raw' : 'VK_NULL_HANDLE'},`)
            ]
        ]
    ];

    output.fromTrueToRaw = [
        `impl<'a> From<&'a ${trueTypeName}> for ${rawTypeName}`, [
            `fn from(value: &'a ${trueTypeName}) -> Self`, [
                `value._handle`
            ]
        ]
    ];
}

function toRawTypeName(typeName) {
    return PRIMITIVE_TYPES[typeName] || `Raw${typeName}`;
}

function toTrueTypeName(typeName) {
    return PRIMITIVE_TYPES[typeName] || typeName;
}

function blockToStr(block, indent) {
    const spaces = indentToSpaces(indent);
    let result;

    if (typeof block === 'string') {
        result = `\n${spaces}${block} `;
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
    return str.split(/(?=[A-Z])/).join('_').toLowerCase();
}