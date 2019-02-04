const {
    toPascalCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    getConstVkValueName,
    findEnumPrefix,
    toSnakeCase,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isCount,
    areCountAndArray,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldInformation,
    documentType
} = require('./utils');

function generateVkBitFlagsDefinition(cDef) {
    cDef.rawTypeName = getRawVkTypeName(cDef.name);
    cDef.wrappedTypeName = getWrappedVkTypeName(cDef.name);

    const prefix = findEnumPrefix(cDef.name.replace('Flags', ''));
    const extSuffix = `_${cDef.extension.toUpperCase()}`;

    for (let field of cDef.fields) {
        const name = field.name.replace('_BIT', '');
        const suffix = name.endsWith(extSuffix) ? extSuffix : '';
        const strippedName = name.substring(prefix.length + 1, name.length - suffix.length);
        
        field.varName = formatBitFlagsFieldName(strippedName);
    }

    return [
        genUses(),
        genWrappedType(cDef),
        genRawType(cDef),
        genImplVkWrappedType(cDef),
        genImplVkRawType(cDef),
        genImplDefault(cDef),
        genImplFlags(cDef),
        genImplAsUint(cDef)
    ];
}

function genUses() {
    return [
        `utils::vk_traits::*`
    ].map(str => `use ${str};`);
}

function genRawType(def) {
    return [
        `#[doc(hidden)]`,
        `pub type ${def.rawTypeName} = u32;`
    ];
}

function genWrappedType(def) {
    return [
        documentType(def),
        `#[derive(Debug, Clone, Copy)]`,
        `pub struct ${def.wrappedTypeName}`,
        def.fields.map(field => `pub ${field.varName}: bool,`)
    ];
}

function genImplVkRawType(def) {
    return [
        `impl VkRawType<${def.wrappedTypeName}> for ${def.rawTypeName}`, [
            `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: (src & ${field.value}) != 0,`)
            ]
        ]
    ];
}

function genImplVkWrappedType(def) {
    return [
        `impl VkWrappedType<${def.rawTypeName}> for ${def.wrappedTypeName}`, [
            `fn vk_to_raw(src: &${def.wrappedTypeName}, dst: &mut ${def.rawTypeName})`, [
                `*dst = 0;`,
                ...def.fields.map(field => `if src.${field.varName} { *dst |= ${field.value}; }`),
            ]
        ]
    ];
}

function genImplDefault(def) {
    return [
        `impl Default for ${def.wrappedTypeName}`, [
            `fn default() -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: false,`)
            ]
        ]
    ];
}

function formatBitFlagsFieldName(name) {
    return toSnakeCase(name)
        .replace(/^(\d)/, '_$1');
}

function genImplAsUint(def) {
    return [
        `impl ${def.wrappedTypeName}`, [
            `\npub fn to_u32(&self) -> u32`, [
                `0${def.fields.map(field => `\n+ if self.${field.varName} { ${field.value} } else { 0 }`).join('')}`
            ],
            `\npub fn from_u32(value: u32) -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: value & ${field.value} > 0,`)
            ]
        ]
    ];
}

function genImplFlags(def) {
    let macroSuffix = '';

    // Manually solve conflicts for now
    if (def.wrappedTypeName === 'VkExternalMemoryFeatureFlags' || def.wrappedTypeName === 'VkExternalMemoryHandleTypeFlags') {
        macroSuffix = def.extension.capitalize();
    }

    return [
        `impl ${def.wrappedTypeName}`, [
            `\npub fn none() -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: false,`)
            ],
            `\npub fn all() -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: true,`)
            ]
        ],
        ``,
        `#[macro_export]`,
        `macro_rules! ${def.wrappedTypeName}${macroSuffix}`, [
            `( $( $x:ident ),* ) =>`, [
                `${def.wrappedTypeName}`, [
                    `$($x: true,)*`,
                    `..${def.wrappedTypeName}::none()`
                ]
            ]
        ]
    ];
}

module.exports = {
    generateVkBitFlagsDefinition,
    genImplFlags
};