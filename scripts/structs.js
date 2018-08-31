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

    const lifetimeTree = assignLifetimes(def.fields);

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

class LetterCounter {
    constructor() {
        this._counter = 0;
        this._initialized = false;
    }

    next() {
        return "'" + String.fromCharCode('a'.charCodeAt(0) + this._counter++);
    }
}

class LifetimeTree {
    constructor(letter, parentLetter) {
        this._letter = letter;
        this._parentLetter = parentLetter;
        this._children = [];
    }

    add(counter) {
        const child = new LifetimeTree(counter && counter.next(), this._letter || this._parentLetter);
        this._children.push(child);

        return child;
    }

    _collect(obj = {}) {
        if (this._letter) {
            obj[this._letter] = this._parentLetter;
        }

        this._children.forEach(child => child._collect(obj));

        return obj;
    }

    letter() {
        return this._letter;
    }

    getSpecs() {
        const letters = Object.keys(this._collect());

        if (!letters.length) {
            return '';
        }

        return `<${letters.join(', ')}>`;
    }

    getRestrictions() {
        const restrictions = Object.entries(this._collect()).filter(entry => entry[1]);

        if (!restrictions.length) {
            return '';
        }

        return `\n    where\n${restrictions.map(([key, value]) => `        ${key}: ${value},\n`).join('')}`
    }

    getStatics() {
        const letters = Object.keys(this._collect());

        if (!letters.length) {
            return '';
        }

        return `<${letters.map(() => "'static").join(', ')}>`;
    }
}

function assignLifetimes(fields, counter, root) {
    counter = counter || new LetterCounter();
    root = root || new LifetimeTree();

    for (let field of fields) {
        let tree = root;

        if (field.wrappedType) {
            if (field.wrappedType.includes('&')) {
                tree = tree.add(counter);
                field.wrappedType = field.wrappedType.replace('&', `&${tree.letter()} `);

                if (field.wrappedType.includes(`[&str]`)) {
                    tree = tree.add(counter);
                    field.wrappedType = field.wrappedType.replace('[&str]', `[&${tree.letter()} str]`);
                }

                if (field.wrappedType.includes(`[&${field.wrappedTypeName}]`)) {
                    tree = tree.add(counter);
                    field.wrappedType = field.wrappedType.replace(`[&${field.wrappedTypeName}]`, `[&${tree.letter()} ${field.wrappedTypeName}]`);
                }
            }

            if (field.wrappedTypeName.startsWith('Vk')) {
                const structDef = getStruct(field);

                if (structDef) {
                    const structFields = getFieldsInformation(structDef.fields, structDef.name);
                    tree = assignLifetimes(structFields, counter, tree.add(null));
                    field.wrappedType = field.wrappedType.replace(field.wrappedTypeName, field.wrappedTypeName + tree.getSpecs());
                }
            }
        }
    }

    return root;
}

module.exports = {
    generateVkStructDefinition
};