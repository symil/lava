const {
    toSnakeCase,
    toPascalCase,
    getRawTypeName,
    getWrappedTypeName,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isCount,
    areCountAndArray,
    isPlural,
    cToRustVarName,
    argToString,
    getArgInformation
} = require('./utils');

function generateVkStructDefinition(cDef) {
    for (let i = 0; i < cDef.args.length; ++i) {
        const arg = cDef.args[i];
        const prevArg = cDef.args[i - 1];
        const countArg = areCountAndArray(prevArg, arg) ? prevArg : null;

        arg.info = getArgInformation(arg, countArg);
    }

    cDef.rawTypeName = getRawTypeName(cDef.name);
    cDef.wrappedTypeName = getWrappedTypeName(cDef.name);

    const uses = new Set([
        'std::string::String',
        'std::vec::Vec',
        'std::ops::Deref'
    ]);

    const blocks = [
        generateRawStruct(cDef)
    ];

    return blocks.map(blockToString).join('\n\n');
}

function generateRawStruct(cDef) {
    return [
        `pub struct ${cDef.rawTypeName}`,
            cDef.args.map(arg => `${cToRustVarName(arg.name)}: ${arg.info.rawType},`)
    ];
}

module.exports = {
    generateVkStructDefinition
};