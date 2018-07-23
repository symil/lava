const {
    toSnakeCase,
    toPascalCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isCount,
    areCountAndArray,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldInformation
} = require('./utils');

function generateVkStructDefinition(cDef) {
    cDef.rawTypeName = getRawVkTypeName(cDef.name);
    cDef.wrappedTypeName = getWrappedVkTypeName(cDef.name);

    for (let i = 0; i < cDef.fields.length; ++i) {
        const field = cDef.fields[i];
        const prevField = cDef.fields[i - 1];
        const nextField = cDef.fields[i + 1];

        field.info = getFieldInformation(field, prevField, nextField);
    }

    cDef.generics = replaceGenericTypes(cDef.fields);

    const uses = new Set([
        'std::string::String',
        'std::vec::Vec',
        'std::ops::Deref'
    ]);

    const blocks = [
        generateRawStruct(cDef),
        generateWrappedStruct(cDef)
    ];

    return blocks.map(blockToString).join('\n\n');
}

function generateRawStruct(cDef) {
    return [
        `pub struct ${cDef.rawTypeName}`,
            cDef.fields.map(field => `${cToRustVarName(field.name)}: ${field.info.rawType},`)
    ];
}

function replaceGenericTypes(fields) {
    const startCode = 'A'.charCodeAt(0);
    const letters = [];
    const specs = [];
    
    for (let field of fields) {
        if (field.info.wrappedType.startsWith('T : ')) {
            const letter = String.fromCharCode(startCode + specs.length);
            const typeSpec = field.info.wrappedType.replace('T : ', `${letter} : `);

            field.info.wrappedType = letter;
            letters.push(letter);
            specs.push(typeSpec);
        }
    }

    if (!letters.length) {
        return { types: '', specs: '' };
    }

    return {
        types: `<${letters.join(', ')}>`,
        specs: `\n    where\n${specs.map(spec => `        ${spec},\n`).join('')}`
    };
}

function generateWrappedStruct(def) {
    const fields = def.fields.filter(field => field.info.wrappedType && field.name !== 'sType');
    
    return [
        `pub struct ${def.wrappedTypeName}${def.generics.types}${def.generics.specs}`,
            fields.map(field => `${cToRustVarName(field.name)}: ${field.info.wrappedType},`)
    ];
}

module.exports = {
    generateVkStructDefinition
};