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
        'std::ops::Deref',
        'utils::vk_convert::*',
        'utils::vk_null::*',
        'utils::vk_ptr::*',
        'utils::vk_type::*'
    ]);

    return [
        uses.map(str => `use ${str};`).join('\n'),
        genRawStructDeclaration(cDef),
        getWrappedStructDeclaration(cDef),
        genImplDeref(cDef),
        genImplVkType(cDef)
    ];
}

function startsWith(str, prefix) {
    return str && str.startsWith(prefix);
}

function replaceGenericTypes(fields) {
    const startCode = 'A'.charCodeAt(0);
    const letters = [];
    const specs = [];
    
    for (let field of fields) {
        if (startsWith(field.info.wrappedType, 'T : ')) {
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

function genRawStructDeclaration(cDef) {
    return [
        `pub struct ${cDef.rawTypeName}`,
            cDef.fields.map(field => `${field.info.varName}: ${field.info.rawType},`)
    ];
}

function getWrappedStructDeclaration(def) {
    const fields = def.fields.filter(field => field.info.wrappedType && field.name !== 'sType');
    
    return [
        `pub struct ${def.wrappedTypeName}${def.generics.types}${def.generics.specs}`,
            fields.map(field => `pub ${field.info.varName}: ${field.info.wrappedType},`)
    ];
}

function genImplDeref(def) {
    return [
        `impl Deref for ${def.wrappedTypeName}`, [
            `type Target = ${def.wrappedTypeName};`,
            `fn deref(&self) -> &${def.wrappedTypeName}`, [
                `self`
            ]
        ]
    ];
}

function isSelfDescribingStructureType(field, index) {
    return field.name === 'sType' && index === 0;
}

function genImplVkType(def) {
    const rawFields = def.fields;
    const wrappedFields = def.fields.filter((field, index) => field.info.wrappedType && !isSelfDescribingStructureType(field, index));

    if (isSelfDescribingStructureType(def.fields[0], 0)) {
        def.fields[0].info.toRaw = () => `RawVkStructure::${def.wrappedTypeName}`;
    }

    return [
        `impl VkType<${def.rawTypeName}> for ${def.wrappedTypeName}`, [
            `\nfn vk_to_raw(src: &${def.wrappedTypeName}, dst: &mut ${def.rawTypeName})`,
            rawFields.map((field, index) => `dst.${field.info.varName} = ${genConvertStatement('toRaw', 'src', rawFields, index)};`),

            `\nfn vk_from_raw(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                wrappedFields.map((field, index) => `${field.info.varName}: ${genConvertStatement('toWrapped', 'src', wrappedFields, index)},`)
            ],

            `\nfn vk_default() -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                wrappedFields.map(field => `${field.info.varName}: ${field.info.defaultValue},`)
            ]
        ]
    ];
}

function getVarName(prefix, field) {
    return field && `${prefix}.${field.info.varName}`;
}

function genConvertStatement(key, prefix, fields, index) {
    const field = fields[index];
    const convertFunction = field.info[key];
    const varName = getVarName(prefix, field);
    const prevVarName = getVarName(prefix, fields[index - 1]);
    const nextVarName = getVarName(prefix, fields[index + 1]);

    return convertFunction(varName, prevVarName, nextVarName);
}

module.exports = {
    generateVkStructDefinition
};