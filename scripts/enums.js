const {
    toPascalCase,
    toUpperCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    findEnumPrefix,
    getStaticVkValueName,
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

function generateVkEnumDefinition(cDef) {
    cDef.rawTypeName = getRawVkTypeName(cDef.name);
    cDef.wrappedTypeName = getWrappedVkTypeName(cDef.name);

    const prefix = findEnumPrefix(cDef.name);
    const extSuffix = `_${cDef.extension.toUpperCase()}`;

    for (let field of cDef.fields) {
        const suffix = field.name.endsWith(extSuffix) ? extSuffix : '';
        const strippedName = field.name.substring(prefix.length, field.name.length - suffix.length);
        
        field.rustName = formatEnumFieldName(strippedName);
    }

    return [
        genUses(),
        genRawType(cDef),
        genWrappedType(cDef),
        genImplVkRawType(cDef),
        genImplVkWrappedType(cDef),
        genImplVkDefault(cDef),
    ];
}


function genUses() {
    return [
        `utils::vk_type::*`
    ].map(str => `use ${str};`);
}

function genRawType(def) {
    return `pub type ${def.rawTypeName} = i32;`;
}

function genWrappedType(def) {
    return [
        `#[repr(i32)]`,
        `#[derive(Debug, PartialEq, Copy, Clone)]`,
        `pub enum ${def.wrappedTypeName}`,
        def.fields.map(({rustName, value}) => `${rustName} = ${value},`)
    ];
}

function genImplVkRawType(def) {
    return [
        `impl VkRawType<${def.wrappedTypeName}> for ${def.rawTypeName}`, [
            `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                `unsafe`, [
                    `*((src as *const i32) as *const ${def.wrappedTypeName})`
                ]
            ],
        ]
    ];
}

function genImplVkWrappedType(def) {
    return [
        `impl VkWrappedType<${def.rawTypeName}> for ${def.wrappedTypeName}`, [
            `fn vk_to_raw(src: &${def.wrappedTypeName}, dst: &mut ${def.rawTypeName})`, [
                `*dst = *src as i32`
            ]
        ]
    ];
}

function genImplVkDefault(def) {
    const staticValueName = getStaticVkValueName(def.wrappedTypeName);

    return [
        `pub static ${staticValueName} : ${def.wrappedTypeName} = ${def.wrappedTypeName}::${def.fields[0].rustName};`,
        `\nimpl VkDefault for ${def.wrappedTypeName}`, [
            `fn vk_default() -> ${def.wrappedTypeName}`, [
                staticValueName
            ]
        ]
    ];
}

function formatEnumFieldName(name) {
    return toPascalCase(name)
        .replace(/^(\d)/, '_$1');
}

module.exports = {
    generateVkEnumDefinition
};