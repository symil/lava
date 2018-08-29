const { toSnakeCase, getRawVkTypeName, getWrappedVkTypeName, getFieldsInformation, addUsesToSet, isStructOrHandle, isOutputHandleStruct } = require('./utils');
const { getStruct } = require('./parse');
const { genImplFlags } = require('./bit_flags');

function generateVkStructDefinition(cDef) {
    const def = {
        typeName: cDef.name,
        extension: cDef.extension,
        rawTypeName: getRawVkTypeName(cDef.name),
        wrappedTypeName: getWrappedVkTypeName(cDef.name),
        fields: getFieldsInformation(cDef.fields, cDef.name),
        cFields: cDef.fields
    };

    const lifetimeTree = assignGenerics(def.fields);

    def.lifetimes = lifetimeTree.getSpecs();
    def.lifetimesRestrictions = lifetimeTree.getRestrictions();
    def.staticLifetimes = lifetimeTree.getStatics();

    // if (def.typeName === 'VkDisplayProperties') {
    //     def.fields.forEach(field => {
    //         console.log(`--> ${field.varName}`)
    //         console.log(!!field.toRaw)
    //         console.log(!field.wrappedType);
    //         console.log(!!field.defaultValue)
    //     })
    // }

    const isFlags = cDef.name === 'VkPhysicalDeviceFeatures';

    return [
        genUses(def),
        genRawStructDeclaration(def),
        getWrappedStructDeclaration(def),
        genImplVkRawType(def),
        genImplVkWrappedType(def),
        genImplDefault(def),
        genImplVkSetup(def),
        genImplVkFree(def),
        isFlags ? genImplFlags(def) : null
    ];
}

function genUses(def) {
    const uses = new Set([
        'std::os::raw::c_char',
        'std::ops::Deref',
        'std::ptr',
        'std::cmp',
        'std::mem',
        `utils::c_bindings::*`,
        'utils::vk_convert::*',
        'utils::vk_null::*',
        'utils::vk_ptr::*',
        'utils::vk_traits::*',
        `vk::vk_instance_function_table::*`,
        `vk::vk_instance::*`,
        `vk::vk_device::*`
    ]);

    addUsesToSet(uses, def, def.fields);

    return Array.from(uses).map(str => `use ${str};`);
}

function genRawStructDeclaration(cDef) {
    const derivedTraits = [];
    
    if (!cDef.cFields.some(field => (field.arraySize && +field.arraySize > 32) || field.typeName === 'VkPhysicalDeviceProperties')) {
        derivedTraits.push('Debug', 'Copy', 'Clone');
    }

    return [
        `#[repr(C)]`,
        derivedTraits.length ? `#[derive(${derivedTraits.join(', ')})]` : null,
        `pub struct ${cDef.rawTypeName}`,
            cDef.fields.map(field => `pub ${field.varName}: ${field.rawType},`)
    ];
}

function getWrappedStructDeclaration(def) {
    const fields = getWrappedFields(def);
    const derivedTraits = ['Debug', 'Clone'];
    
    return [
        `#[derive(${derivedTraits.join(', ')})]`,
        `pub struct ${def.wrappedTypeName}${def.lifetimes}${def.lifetimesRestrictions}`,
            fields.map(field => `pub ${field.varName}: ${field.wrappedType},`)
    ];
}

function getWrappedFields(def) {
    return def.fields.filter(field => field.wrappedType);
}

const SPECIAL_NON_RAW_TYPES = ['VkClearValue', 'VkClearColorValue'];

function isConvertibleFromRawToWrapped(def) {
    return def.fields.every(field => !SPECIAL_NON_RAW_TYPES.includes(field.typeName) && (!field.wrappedType || field.toWrapped)) && !def.lifetimes;
}

function isConvertibleFromWrappedToRaw(def) {
    return def.fields.every(field => field.toRaw && (!field.wrappedType || field.defaultValue)) && canHaveStaticValue(def);
}

function genImplVkRawType(def) {
    if (isConvertibleFromRawToWrapped(def)) {
        const wrappedFields = getWrappedFields(def);

        return [
            `impl VkRawType<${def.wrappedTypeName}> for ${def.rawTypeName}`, [
                `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                    def.wrappedTypeName,
                    wrappedFields.map(field => `${field.varName}: ${field.toWrapped(varName => `src.${varName}`)},`)
                ],
            ]
        ];
    }
}

function genImplVkWrappedType(def) {
    if (isConvertibleFromWrappedToRaw(def)) {
        const rawFields = def.fields;

        return [
            `impl${def.lifetimes} VkWrappedType<${def.rawTypeName}> for ${def.wrappedTypeName}${def.lifetimes}${def.lifetimesRestrictions}`, [
                `fn vk_to_raw(src: &${def.wrappedTypeName}, dst: &mut ${def.rawTypeName})`,
                rawFields.map(field => `dst.${field.varName} = ${field.toRaw(varName => `src.${varName}`)};`)
            ]
        ];
    }
}

function genImplDefault(def) {
    if (isConvertibleFromWrappedToRaw(def)) {
        const wrappedFields = getWrappedFields(def);

        return [
            `impl Default for ${def.wrappedTypeName}${def.staticLifetimes}`, [
                `fn default() -> ${def.wrappedTypeName}${def.staticLifetimes}`, [
                    def.wrappedTypeName,
                    wrappedFields.map(field => `${field.varName}: ${field.defaultValue},`)
                ]
            ]
        ];
    }
}

function genImplVkSetup(def) {
    return [
        `impl${def.lifetimes} VkSetup for ${def.wrappedTypeName}${def.lifetimes}${def.lifetimesRestrictions}`, [
            `fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice)`,
            def.fields.map(field => {
                if (isStructOrHandle(field) && field.wrappedType === field.wrappedTypeName) {
                    return `VkSetup::vk_setup(&mut self.${field.varName}, fn_table, instance, device);`
                }
            }).filter(x => x)
        ]
    ]
}

function genImplVkFree(def) {
    const fieldsToFree = def.fields.filter(field => field.freeRaw);

    return [
        `impl VkFree for ${def.rawTypeName}`, [
            `fn vk_free(&mut self)`,
            fieldsToFree.map(field => `${field.freeRaw(varName => `self.${varName}`)};`)
        ]
    ]
}

function canHaveStaticValue(def) {
    if (isOutputHandleStruct(def.typeName)) {
        return false;
    }

    if (def.arraySize && def.typeName === 'char') {
        return false;
    } else {
        const struct = getStruct(def);

        if (struct) {
            return struct.fields.every(field => field.typeName === def.typeName || canHaveStaticValue(field));
        } else {
            return true;
        }
    }
}

class GenericsIdCounter {
    constructor() {
        this._lifetimeCounter = 0;
        this._typeCounter = 0;
    }

    nextLifetimeId() {
        return "'" + String.fromCharCode('a'.charCodeAt(0) + this._lifetimeCounter++);
    }

    nextTypeId() {
        return String.fromCharCode('A'.charCodeAt(0) + this._typeCounter++);
    }
}

function getGenericsInfo(generics) {
    const keys = Object.keys(generics);
    const lifetimes = keys.filter(key => key.startsWith("'"));
    const types = keys.filter(key => !key.startsWith("'"));

    const ids = lifetimes.concat(types);
    const restrictions = ids.filter(id => generics[id]).map(id => `        ${id}: ${generics[id]},\n`);

    return  {
        specs: ids.length ? `<${ids.join(', ')}>` : '',
        restrictions: restrictions.length ? `\n    where\n${restrictions.join('')}` : '',
        typeSpecs: types.length ? `<${types.join(', ')}>` : '',
        typeRestrictions: types.length ? `\n    where\n${types.map(type => `        ${type}: ${generics[type]},\n`).join('')}` : ''
    };
}

function assignGenerics(fields, counter, parentLifetime) {
    counter = counter || new GenericsIdCounter();

    const generics = {};
    const baseParentLifetime = parentLifetime;

    for (let field of fields) {
        parentLifetime = baseParentLifetime;

        if (field.wrappedType) {
            // let strippedType = field.wrappedType.replace(/^Option<(.*)>$/, '$1');

            if (field.wrappedType.includes('&')) {
                const lifetime = counter.nextLifetimeId();
                generics[lifetime] = parentLifetime;
                field.wrappedType = field.wrappedType.replace('&', `&${lifetime} `);
                parentLifetime = lifetime

                if (field.wrappedType.includes(`[&str]`)) {
                    const type = counter.nextTypeId();
                    generics[type] = `AsRef<str>`;
                    field.wrappedType = field.wrappedType.replace('[&str]', `[${type}]`);
                }

                if (field.wrappedType.includes(`[&${field.wrappedTypeName}]`)) {
                    const type = counter.nextTypeId();
                    generics[type] = `AsRef<${field.wrappedTypeName}>`;
                    field.wrappedType = field.wrappedType.replace(`[&${field.wrappedTypeName}]`, `[${type}]`);
                }
            }

            if (field.wrappedTypeName.startsWith('Vk')) {
                const structDef = getStruct(field);

                if (structDef) {
                    const structFields = getFieldsInformation(structDef.fields, structDef.name);
                    const fieldGenerics = assignGenerics(structFields, counter, parentLifetime);
                    field.wrappedType = field.wrappedType.replace(field.wrappedTypeName, field.wrappedTypeName + getGenericsInfo(fieldGenerics).specs);
                    Object.assign(generics, fieldGenerics);
                }
            }
        }
    }

    return generics;
}

module.exports = {
    generateVkStructDefinition
};