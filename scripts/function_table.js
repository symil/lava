const {
    getFieldsInformation,
} = require('./utils');

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
        genNewMethod(functions),
        genNullFunctions(functions),
        genExterns(functions)
    ];
}

function genUses() {
    return [
        `std::os::raw::c_char`,
        `std::mem`,
        `utils::c_bindings::*`,
        `utils::vk_convert::get_vk_instance_function_pointer`,
        'vk::*',
        `glfw::*`
    ].map(x => `use ${x};`);
}

function genDefinition(functions) {
    return [
        `pub struct VkInstanceFunctionTable`,
        functions.map(func => `pub ${func.name}: ${functionToDeclaration(func)},`)
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
    return functions.filter(func => func.name.startsWith('vk')).map(func => {
        const funcName = `null_${func.name}`;

        return [
            `unsafe extern fn ${funcName}${getVkFunctionPrototype(func)}`, [
                `panic!("\\"vkGetInstanceProcAddr\\" returned NULL for \\"${func.name}\\"");`
            ]
        ];
    }).reduce((acc, value) => acc.concat(value), []);
}

function getFunctionPointer(func) {
    if (func.name.startsWith('vk')) {
        return `{ let fn_ptr = get_vk_instance_function_pointer(instance, "${func.name}"); if fn_ptr.is_null() { null_${func.name} } else { mem::transmute(fn_ptr) } }`;
    } else {
        return func.name;
    }
}

function genNewMethod(functions) {
    return [
        `impl VkInstanceFunctionTable`, [
            `pub fn new(instance: RawVkInstance) -> Self`, [
                `unsafe`, [
                    `Self`,
                    functions.map(func => `${func.name}: ${getFunctionPointer(func)},`)
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