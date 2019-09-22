const {
    getFieldsInformation
} = require('./utils');

const { EXTENSIONS } = require('./data');

function generateFunctionTableDefinition(functions) {
    for (let func of functions) {
        if (!func.argsInfo) {
            func.argsInfo = getFieldsInformation(func.args);
        }
    }

    return [
        `#![allow(non_snake_case)]`,
        genUses(functions),
        genDefinition(functions),
        genFromMethods(functions),
        genNullFunctions(functions),
        genExterns(functions)
    ];
}

const FUNCTION_TABLE_NAME = 'VkFunctionTable';

function genUses() {
    return [
        `std::os::raw::c_char`,
        `std::mem`,
        `utils::c_bindings::*`,
        `utils::vk_convert::{get_vk_instance_function_pointer, get_vk_device_function_pointer}`,
        'vulkan::vk::*',
        `vulkan::{${EXTENSIONS.map(str => str.toLowerCase()).join(',')}}`
    ].map(x => `use ${x};`);
}

function genDefinition(functions) {
    return [
        `#[doc(hidden)]`,
        `pub struct ${FUNCTION_TABLE_NAME}`, [
            `pub instance: RawVkInstance,`,
            `pub device: RawVkDevice,`,
            ...functions.map(func => `pub ${func.name}: ${functionToDeclaration(func)},`)
        ]
    ];
}

function getArgType(arg) {
    let type = arg.rawType;

    if (arg.extension) {
        type = type.replace(arg.rawTypeName, `${arg.extension}::${arg.rawTypeName}`)
    }

    return type;
}

function functionToDeclaration(func) {
    const args = func.argsInfo.map(getArgType);
    const returnType = func.type === 'VkResult' ? ' -> RawVkResult' : '';

    return `unsafe extern fn(${args.join(', ')})${returnType}`;
}

function getVkFunctionPrototype(func) {
    const args = func.argsInfo.map(arg => `${arg.varName}: ${getArgType(arg)}`);
    const returnType = func.type === 'VkResult' ? ' -> RawVkResult' : '';

    return `(${args.join(', ')})${returnType}`;
}

function genNullFunctions(functions) {
    return genNullFunctionsForParent(functions, 'instance').concat(genNullFunctionsForParent(functions, 'device'));
}

function genNullFunctionsForParent(functions, parent) {
    return functions.filter(func => func.name.startsWith('vk')).map(func => {
        const funcName = `null_${parent}_${func.name}`;

        return [
            `unsafe extern fn ${funcName}${getVkFunctionPrototype(func)}`, [
                `panic!("\\"vkGet${parent.capitalize()}ProcAddr\\" returned NULL for \\"${func.name}\\"");`
            ]
        ];
    }).reduce((acc, value) => acc.concat(value), []);
}

function getFunctionPointer(func, parent) {
    if (func.name.startsWith('vk')) {
        return `{ let fn_ptr = get_vk_${parent}_function_pointer(${parent}, "${func.name}"); if fn_ptr.is_null() { null_${parent}_${func.name} } else { mem::transmute(fn_ptr) } }`;
    } else {
        return func.name;
    }
}

function genFromMethods(functions) {
    return [
        `impl ${FUNCTION_TABLE_NAME}`, [
            `pub fn from_instance(instance: RawVkInstance) -> Self`, [
                `unsafe`, [
                    `Self`, [
                        `instance: instance,`,
                        `device: 0,`,
                        ...functions.map(func => `${func.name}: ${getFunctionPointer(func, 'instance')},`)
                    ]
                ]
            ],
            `pub fn from_device(device: RawVkDevice) -> Self`, [
                `unsafe`, [
                    `Self`, [
                        `instance: 0,`,
                        `device: device,`,
                        ...functions.map(func => `${func.name}: ${getFunctionPointer(func, 'device')},`)
                    ]
                ]
            ]
        ]
    ]
}

function genExterns(functions) {
    const externs = functions.filter(func => !func.name.startsWith('vk'));

    if (externs.length) {
        return [
            `extern`,
            externs.map(func => `fn ${func.name}${getVkFunctionPrototype(func)};`)
        ]
    }
}

module.exports = {
    generateFunctionTableDefinition,
    getVkFunctionPrototype
};