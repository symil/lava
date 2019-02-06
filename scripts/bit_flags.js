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
        genImplFlags(cDef, 'u32')
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

function genFlagBitsDoc(def) {
    const fields = def.fields.slice(0, 2).map(f => f.varName);

    return [
        `Use the macro \`${def.wrappedTypeName}!\` as an alternative method to create a structure. For example, these two snippets return the same value:`,
        '```',
        `${def.wrappedTypeName}!(${fields.join(', ')})`,
        '```',
        '```',
        `${def.wrappedTypeName} {`,
        ...fields.map(f => `    ${f}: true,`),
        def.fields.length <= 2 ? '' : `    ..${def.wrappedTypeName}::none()`,
        `}`,
        '```'
    ].filter(x => x).map(x => `/// ${x}`).join('\n');
}

function genWrappedType(def) {
    return [
        documentType(def, genFlagBitsDoc(def)),
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

const NONE_DOC = '/// Return a structure with all flags to `false`.\n'
const ALL_DOC = '/// Return a structure with all flags to `true`.\n'
const TO_U32_DOC = '/// Return the numerical bit flags corresponding to the structure (as described in the Vulkan specs).\n';
const FROM_U32_DOC = '/// Create a structure corresponding to the specified numerical bit flags.\n';

function genImplFlags(def, intType) {
    let macroSuffix = '';

    // Manually solve conflicts for now
    if (def.wrappedTypeName === 'VkExternalMemoryFeatureFlags' || def.wrappedTypeName === 'VkExternalMemoryHandleTypeFlags') {
        macroSuffix = def.extension.capitalize();
    }

    const toIntDoc = intType === 'u32' ? TO_U32_DOC : '';
    const fromIntDoc = intType === 'u32' ? FROM_U32_DOC : '';

    return [
        `impl ${def.wrappedTypeName}`, [
            `\n${NONE_DOC}pub fn none() -> Self`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: false,`)
            ],
            `\n${ALL_DOC}pub fn all() -> Self`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: true,`)
            ],
            `\n${toIntDoc}pub fn to_${intType}(&self) -> ${intType}`, [
                `0${def.fields.map(field => `\n+ if self.${field.varName} { ${field.value} } else { 0 }`).join('')}`
            ],
            `\n${fromIntDoc}pub fn from_${intType}(value: ${intType}) -> Self`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.varName}: value & ${field.value} > 0,`)
            ]
        ],
        ``,
        `#[doc(hidden)]`,
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
    genImplFlags,
    genFlagBitsDoc
};