const { getFieldsInformation } = require('./utils');
const { getVkFunctionPrototype } = require('./function_table');
const { functionToMethod } = require('./handles');

function generateRootTypeDefinition(functions) {
    for (let func of functions) {
        if (!func.argsInfo) {
            func.argsInfo = getFieldsInformation(func.args);
        }
    }

    return [
        genUses(functions),
        genType(functions),
        genMethods(functions),
        genExterns(functions)
    ];
}

function genUses() {
    return [
        `utils::c_bindings::*`,
        `utils::vk_type::*`,
        `utils::vk_ptr::*`,
        `utils::vk_convert::*`,
        'std::os::raw::c_char',
        'std::ptr',
        'std::mem',
        `vk::*`
    ].map(x => `use ${x};`);
}

function genType(functions) {
    return `pub struct Vk;`
}

function genMethods(functions) {
    return [
        `impl Vk`,
        functions.map(func => functionToMethod(null, func)).reduce((acc, block) => acc.concat(block), [])
    ];
}

function genExterns(functions) {
    return [
        `extern`,
        functions.map(func => `fn ${func.name}${getVkFunctionPrototype(func)};`)
    ]
}

module.exports = {
    generateRootTypeDefinition
}