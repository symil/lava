const { toSnakeCase, getRawVkTypeName, getWrappedVkTypeName, getFieldsInformation } = require('./utils');
const { getStruct } = require('./vulkan_src');

function generateVkStructDefinition(cDef) {
    const def = {
        typeName: cDef.name,
        extension: cDef.extension,
        rawTypeName: getRawVkTypeName(cDef.name),
        wrappedTypeName: getWrappedVkTypeName(cDef.name),
        fields: getFieldsInformation(cDef.fields)
    };

    const lifetimeTree = assignLifetimes(def.fields);

    def.lifetimes = lifetimeTree.getSpecs();
    def.lifetimesRestrictions = lifetimeTree.getRestrictions();
    def.staticLifetimes = lifetimeTree.getStatics();

    return [
        genUses(def),
        genRawStructDeclaration(def),
        getWrappedStructDeclaration(def),
        genImplVkRawType(def),
        genImplVkWrappedType(def),
        genImplVkDefault(def)
    ];
}

function genUses(def) {
    const uses = new Set([
        'std::os::raw::c_char',
        'std::string::String',
        'std::vec::Vec',
        'std::ops::Deref',
        'std::ptr',
        'std::cmp',
        'std::mem',
        'utils::vk_convert::*',
        'utils::vk_null::*',
        'utils::vk_ptr::*',
        'utils::vk_type::*'
    ]);

    for (let field of def.fields) {
        const typeName = field.wrappedTypeName;

        if (typeName.startsWith('Vk') && typeName !== def.wrappedTypeName) {
            let use = `vk::`;
            if (field.extension) { use += `${field.extension}::`; }
            use += toSnakeCase(typeName);
            use += `::*`;

            uses.add(use);
        }
    }

    return Array.from(uses).map(str => `use ${str};`);
}

function genRawStructDeclaration(cDef) {
    return [
        `#[repr(C)]`,
        `pub struct ${cDef.rawTypeName}`,
            cDef.fields.map(field => `${field.varName}: ${field.rawType},`)
    ];
}

function getWrappedStructDeclaration(def) {
    const fields = getWrappedFields(def);
    const hasCopy = canHaveStaticValue(def);
    const derivedTraits = ['Debug', 'Clone'];

    if (hasCopy) {
        derivedTraits.push('Copy');
    }
    
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
    return def.fields.every(field => SPECIAL_NON_RAW_TYPES.includes(field.name) && (!field.wrappedType || field.toWrapped)) && !def.lifetimes;
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

function genImplVkDefault(def) {
    if (isConvertibleFromWrappedToRaw(def)) {
        const wrappedFields = getWrappedFields(def);

        return [
            `impl VkDefault for ${def.wrappedTypeName}${def.staticLifetimes}`, [
                `fn vk_default() -> ${def.wrappedTypeName}${def.staticLifetimes}`, [
                    def.wrappedTypeName,
                    wrappedFields.map(field => `${field.varName}: ${field.defaultValue},`)
                ]
            ]
        ];
    }
}

function canHaveStaticValue(def) {
    if (def.typeName === 'VkDisplayProperties') {
        return false;
    }

    if (def.arraySize) {
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

                if (field.wrappedType.endsWith(`[&str]`)) {
                    tree = tree.add(counter);
                    field.wrappedType = field.wrappedType.replace('[&str]', `[&${tree.letter()} str]`);
                }
            }

            if (field.wrappedTypeName.startsWith('Vk')) {
                const structDef = getStruct(field);

                if (structDef) {
                    const structFields = getFieldsInformation(structDef.fields);
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