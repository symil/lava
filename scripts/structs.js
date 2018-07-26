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

    if (isSelfDescribingStructureType(cDef.fields[0], 0)) {
        cDef.fields[0].info.toRaw = () => `vk_to_raw_value(&VkStructureType::${cDef.wrappedTypeName.substring(2)})`;
    }

    return [
        genUses(cDef),
        genRawStructDeclaration(cDef),
        getWrappedStructDeclaration(cDef),
        genImplDeref(cDef),
        genImplVkRawType(cDef),
        genImplVkWrappedType(cDef),
        genImplVkDefault(cDef)
    ];
}

function genUses(cDef) {
    const uses = new Set([
        'std::os::raw::c_char',
        'std::string::String',
        'std::vec::Vec',
        'std::ops::Deref',
        'std::ptr',
        'utils::vk_convert::*',
        'utils::vk_null::*',
        'utils::vk_ptr::*',
        'utils::vk_type::*'
    ]);

    for (let field of cDef.fields) {
        const typeName = field.info.wrappedTypeName;

        if (typeName.startsWith('Vk')) {
            let use = `vk::`;
            if (field.extension) { use += `${field.extension}::`; }
            use += toSnakeCase(typeName);
            use += `::*`;

            uses.add(use);
        }
    }

    return Array.from(uses).map(str => `use ${str};`);
}

function startsWith(str, prefix) {
    return str && str.startsWith(prefix);
}

function replaceGenericTypes(fields) {
    const startCode = 'A'.charCodeAt(0);
    const letters = [];
    const specs = [];
    const staticTypes = [];
    const createdTypeNames = [];
    
    for (let field of fields) {
        if (startsWith(field.info.wrappedType, 'T : ')) {
            const letter = String.fromCharCode(startCode + specs.length);
            const typeSpec = field.info.wrappedType.replace('T : ', `${letter} : `);
            const refTypeName = field.info.wrappedType.match(/T : Deref<Target=(.*)>/)[1];
            let targetTypeName = refTypeName;
            let isVec = false;

            if (targetTypeName.startsWith(']')) {
                targetTypeName = targetTypeName.substring(1, targetTypeName.length - 1);
                isVec = true;
            }

            let typeName = targetTypeName === 'str' ? 'String' : targetTypeName;

            if (isVec) {
                typeName = `Vec<${typeName}>`;
            }

            field.info.wrappedType = letter;
            letters.push(letter);
            specs.push(typeSpec);
            staticTypes.push(`&'static ${refTypeName}`)
            createdTypeNames.push(typeName);
        }
    }

    if (!letters.length) {
        return { types: '', specs: '', static: '', created: '' };
    }

    return {
        types: `<${letters.join(', ')}>`,
        specs: `\n    where\n${specs.map(spec => `        ${spec},\n`).join('')}`,
        static: `<${staticTypes.join(', ')}>`,
        created: `<${createdTypeNames.join(', ')}>`
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
        `impl${def.generics.types} Deref for ${def.wrappedTypeName}${def.generics.types}${def.generics.specs}`, [
            `type Target = ${def.wrappedTypeName}${def.generics.types};`,
            `fn deref(&self) -> &${def.wrappedTypeName}${def.generics.types}`, [
                `self`
            ]
        ]
    ];
}

function isSelfDescribingStructureType(field, index) {
    return field.name === 'sType' && index === 0;
}

function genImplVkRawType(def) {
    const wrappedFields = def.fields.filter((field, index) => field.info.wrappedType && !isSelfDescribingStructureType(field, index));

    return [
        `impl VkRawType<${def.wrappedTypeName}${def.generics.created}> for ${def.rawTypeName}`, [
            `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}${def.generics.created}`, [
                def.wrappedTypeName,
                wrappedFields.map((field, index) => `${field.info.varName}: ${genConvertStatement('toWrapped', 'src', wrappedFields, index)},`)
            ],
        ]
    ];
}

function genImplVkWrappedType(def) {
    const rawFields = def.fields;

    return [
        `impl${def.generics.types} VkWrappedType<${def.rawTypeName}> for ${def.wrappedTypeName}${def.generics.types}${def.generics.specs}`, [
            `fn vk_to_raw(src: &${def.wrappedTypeName}${def.generics.types}, dst: &mut ${def.rawTypeName})`,
            rawFields.map((field, index) => `dst.${field.info.varName} = ${genConvertStatement('toRaw', 'src', rawFields, index)};`)
        ]
    ];
}

function genImplVkDefault(def) {
    const wrappedFields = def.fields.filter((field, index) => field.info.wrappedType && !isSelfDescribingStructureType(field, index));

    return [
        `impl VkDefault for ${def.wrappedTypeName}${def.generics.static}`, [
            `fn vk_default() -> ${def.wrappedTypeName}${def.generics.static}`, [
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