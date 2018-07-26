const {
    toPascalCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    findEnumPrefix,
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
        genImplVkType(cDef)
    ];
}


function genUses() {
    return [
        `utils::vk_type::VkType`
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

function genImplVkType(def) {
    return [
        `impl VkType<${def.rawTypeName}> for ${def.wrappedTypeName}`, [
            `\nfn vk_to_raw(src: &${def.wrappedTypeName}, dst: &mut ${def.rawTypeName})`, [
                `*dst = *src as i32`
            ],

            `\nfn vk_from_raw(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                `unsafe`, [
                    `*((src as *const i32) as *const ${def.wrappedTypeName})`
                ]
            ],

            `\nfn vk_default() -> ${def.wrappedTypeName}`, [
                `${def.wrappedTypeName}::${def.fields[0].rustName}`
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