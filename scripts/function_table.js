const {
    getFieldsInformation,
} = require('./utils');

function generateFunctionTableDefinition(functions) {
    for (let func of functions) {
        func.argsInfo = getFieldsInformation(func.args);
    }

    return [
        `#![allow(non_snake_case)]`,
        genUses(functions),
        genDefinition(functions),
        genNewMethod(functions),
        genNullFunctions(functions)
    ];
}

function genUses() {
    return [
        `std::os::raw::c_char`,
        `std::mem`,
        `utils::vk_ptr::c_void`,
        `utils::vk_convert::get_vk_instance_function_pointer`,
        'vk::*'
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

    return `extern fn(${args.join(', ')})${returnType}`;
}

function genNullFunctions(functions) {
    return functions.map(func => {
        const args = func.argsInfo.map(arg => `${arg.varName}: ${getArgType(arg)}`);
        const returnType = func.type === 'VkResult' ? ' -> RawVkResult' : '';
        const funcName = `null_${func.name}`;

        return [
            `extern fn ${funcName}(${args.join(', ')})${returnType}`, [
                `panic!("\\"vkGetInstanceProcAddr\\" returned NULL for \\"${func.name}\\"");`
            ]
        ];
    }).reduce((acc, value) => acc.concat(value), []);
}

function genNewMethod(functions) {
    return [
        `impl VkInstanceFunctionTable`, [
            `fn new(instance: RawVkInstance) -> Self`, [
                `unsafe`, [
                    `Self`,
                    functions.map(func => `${func.name}: { let fn_ptr = get_vk_instance_function_pointer(instance, "${func.name}"); if fn_ptr.is_null() { null_${func.name} } else { mem::transmute(fn_ptr) } },`)
                ]
            ]
        ]
    ]
}

module.exports = {
    generateFunctionTableDefinition
};