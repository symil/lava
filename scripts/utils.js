const { getEnumByName, getBitFlagsByName, getStructByName, getHandleByName, isHandle } = require('./vulkan_header');

const PRIMITIVE_TYPES = {
    uint64_t: 'u64',
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int64_t: 'i64',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    char: 'c_char',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    ssize_t: 'isize',
    void: 'c_void',
    VkAllocationCallbacks: 'c_void',
    VkDeviceSize: 'u64'
};

const INDENT = '    ';

Array.prototype.last = function() {
    return this[this.length - 1];
};

Array.prototype.beforeLast = function() {
    return this[this.length - 2];
};

function toSnakeCase(str) {
    return str
        .replace(/[A-Z0-9]+/g, str => str.charAt(0) + str.substring(1).toLowerCase())
        .split(/(?=[A-Z])/).join('_').toLowerCase()
        .replace(/_+/g, '_');
}

function toPascalCase(str) {
    return str.split('_')
    .map(word => word.charAt(0).toUpperCase() + word.substring(1).toLowerCase()).join('')
    .replace(/\d[a-z](?=\d)/g, str => str[0] + str[1].toUpperCase());
}

function findEnumPrefix(typeName) {
    if (typeName === 'VkResult') {
        return 'VK_';
    }

    return typeName
        .replace(/[A-Z]+$/, '')
        .replace(/[A-Z]+/g, `_$&`)
        .toUpperCase()
        .substring(1);
}

function blockToString(block) {
    if (typeof block === 'string') {
        return block;
    } else {
        return _blocksToString(block);
    }
}

function _blocksToString(blocks) {
    let result = '';

    let isFirst = true;

    for (let block of blocks) {
        if (typeof block === 'string') {
            if (!isFirst) {
                result += '\n';
            }
            result += block;
        } else if (isFirst) {
            result += _blocksToString(block);
        } else {
            result += ' {\n';
            result += _blocksToString(block).split('\n').map(str => INDENT + str).join('\n');
            result += '\n}';
        }

        isFirst = false;
    }

    return result;
}

function isCount(arg) {
    return arg && arg.isPointer && !arg.isConst && arg.typeName === 'uint32_t';
}

function areFieldsCountAndArray(field1, field2) {
    return isCount(field1) && field2 && field2.isPointer && field2.typeName !== 'char';
}

function isPlural(arg) {
    return arg && !arg.typeName.endsWith('s') && arg.name.endsWith('s');
}

function cToRustVarName(name) {
    name = name.replace(/^(p{1,2})[A-Z]/, str => str[str.length - 1]);

    return toSnakeCase(name);
}

function argToString(arg) {
    return arg.name ? `${arg.name}: ${arg.type}` : arg.type;
}

function doesFieldRepresentVersion(field) {
    return field.typeName === 'uint32_t' && /Version$/.test(field.name);
}

function doesFieldRepresentIndex(field) {
    return field.typeName === 'uint32_t' && /Index|Indices/.test(field.name);
}

function createStaticArray(typeName, arraySize, varName, functionName) {
    return `unsafe { let mut dst_array : [${typeName}, ${arraySize}] = mem::uninitialized(); ${functionName}(&${varName}, &mut dst_array); dst_array }`;
}

function getRawVkTypeName(cTypeName) {
    return `Raw${cTypeName}`;
}

function getWrappedVkTypeName(cTypeName) {
    return cTypeName;
}

function getFieldRawTypeName(field) {
    return PRIMITIVE_TYPES[field.typeName] || getRawVkTypeName(field.typeName);
}

function getFieldWrappedTypeName(field) {
    if (doesFieldRepresentVersion(field)) {
        return `VkVersion`;
    } else if (doesFieldRepresentIndex(field)) {
        return `usize`;
    }

    return PRIMITIVE_TYPES[field.typeName] || getWrappedVkTypeName(field.typeName);
}

class LifetimeSet {
    constructor() {
        this._nb = 0;
        this._types = {};
    }

    add(outlives) {
        const letter = "'" + String.fromCharCode('a'.charCodeAt(0) + this._nb);
        this._types[letter] = outlives;
        this._nb += 1;

        return letter;
    }

    getTypes() {
        return Object.keys(this._types);
    }

    getDefinitions() {
        return Object.values(this._types);
    }
}

function getFieldLifetime(field, genericsSet) {
    const def = getStructByName(field);

    if (field.fullType === 'const char* const*') {
        const sliceLifetime = genericsSet.add();
        const strLifetime = genericsSet.add(sliceLifetime);

        return [sliceLifetime, strLifetime];
    } else if (field.fullType === 'const char*') {
        return [genericsSet.add()];
    }
}


function getFieldsInformation(fields) {
    const genericsSet = new LifetimeSet();
    const infos = [];

    for (let i = 0; i < fields.length; ++i) {
        const field = fields[i];
        const prevField = fields[i - 1];
        const nextField = fields[i + 1];

        const rawTypeName = getFieldRawTypeName(field);
        const wrappedTypeName = getFieldWrappedTypeName(field);

        const arraySize = field.arraySize;
        const isPointerArray = field.isPointer && areFieldsCountAndArray(prevField, field);
        const isPointerValue = field.isPointer && !isPointerArray;
        const isStaticArray = !field.isPointer && !!arraySize;
        const isPrimitiveType = rawTypeName === wrappedTypeName;
        const isHandleType = !isPrimitiveType && isHandle(field.typeName);
        const isCount = areFieldsCountAndArray(field, nextField);

        const varName = '[varName]';
        const prevVarName = '[prevVarName]';
        const nextVarName = '[nextVarName]';

        let rawType = null;
        let wrappedType = null;
        let toRaw = null;
        let toWrapped = null;
        let defValue = null;

        if (field.fullType === 'const void*') {
            rawType = `*const c_void`;
            wrappedType = null;
            toRaw = `ptr::null()`;
            toWrapped = null;
            defValue = null;
        } else if (isCount) {
            rawType = `u32`;
            toRaw = `${nextVarName}.len() as u32`;
        } else if (field.fullType === 'const char* const*') {
            // rawType = `*const *const c_char`;
            rawType = `VkPtr<*mut c_char>`;
            wrappedType = genericsSet.add(`Deref<Target=[str]>`);
            toRaw = `VkPtr::new_string_array(&${varName})`;
            toWrapped = `Vec::new()`; // Should never be used
            defValue = `Vec::new()`;
        } else if (field.fullType === 'const char*') {
            // rawType = `*const c_char`;
            rawType = `VkPtr<c_char>`;
            wrappedType = genericsSet.add(`Deref<Target=str>`);
            toRaw = `VkPtr::new_string(&${varName})`;
            toWrapped = `new_string(${varName}.as_ptr())`; // Should never be used
            defValue = `vk_null::<String>()`;
        } else if (field.fullType === 'char' && field.arraySize) {
            rawType = `[c_char; ${arraySize}]`;
            wrappedType = genericsSet.add(`Deref<Target=str>`);
            toRaw = createStaticArray(rawTypeName, arraySize, varName, 'string_to_byte_array');
            toWrapped = `new_string(&${varName}[0] as *const c_char)`;
            defValue = `""`;
        } else if (isPrimitiveType) {
            if (isPointerArray) {
                rawType = `VkPtr<${rawTypeName}>`;
                wrappedType = genericsSet.add(`Deref<Target=[${wrappedTypeName}]>`);
                toRaw = `VkPtr::new_array(&${varName})`;
                toWrapped = `new_array(${prevVarName}, ${varName})`;
                defValue = `Vec::new()`;
            } else if (isPointerValue) {
                rawType = `VkPtr<${rawTypeName}>`;
                wrappedType = genericsSet.add(`Deref<Target=${wrappedTypeName}>`);
                toRaw = `VkPtr::new_value(${varName})`;
                toWrapped = varName; // Should never be used
                defValue = `vk_null()`;
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;
                wrappedType = genericsSet.add(`Deref<Target=[${wrappedTypeName}]>`);
                toRaw = createStaticArray(rawTypeName, arraySize, varName, 'to_array');
                toWrapped = `new_array(${prevVarName}, &${varName}[0] as *const ${rawTypeName})`;
                defValue = `Vec::new()`;
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = varName;
                toWrapped = varName;
                defValue = `0`;
            }
        } else {
            if (isPointerArray) {
                rawType = `VkPtr<${rawTypeName}>`;
                wrappedType = genericsSet.add(`Deref<Target=[${wrappedTypeName}]>`);
                toRaw = `VkPtr::new_vk_array(&${varName})`;
                toWrapped = `new_vk_array(${prevVarName}, ${varName})`;
                defValue = `Vec::new()`;
            } else if (isPointerValue) {
                rawType = `VkPtr<${rawTypeName}>`;
                wrappedType = genericsSet.add(`Deref<Target=${wrappedTypeName}>`);
                toRaw = `VkPtr::new_vk_value(&${varName})`;
                toWrapped = `${rawTypeName}::vk_to_wrapped(${varName}.as_ref().unwrap())`; // Pointer should never be null; if that happens, we should use an `Option`
                defValue = `vk_null()`;
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;
                wrappedType = genericsSet.add(`Deref<Target=[${wrappedTypeName}]>`);
                toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                toWrapped = `new_vk_array(${prevVarName}, &${varName}[0] as *const ${rawTypeName})`;
                defValue = `Vec::new()`;
            } else if (isHandleType) {
                rawType = rawTypeName;
                wrappedType = genericsSet.add(`Deref<Target=${wrappedTypeName}>`);
                toRaw = `vk_to_raw_value(&${varName})`;
                toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;
                defValue = `vk_null()`;
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = `vk_to_raw_value(&${varName})`;
                toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;
                defValue = `${wrappedTypeName}::vk_default()`;
            }
        }

        const info = {
            rawType: rawType,
            wrappedType: wrappedType,
            rawTypeName: rawTypeName,
            wrappedTypeName: wrappedTypeName,
            toRaw: stringToFunction(toRaw, varName, prevVarName, nextVarName),
            toWrapped: stringToFunction(toWrapped, varName, prevVarName, nextVarName),
            defaultValue: defValue,
            varName: cToRustVarName(field.name)
        };

        infos.push(info);
    }

    return infos;
}

function stringToFunction(statement, varName, prevVarName, nextVarName) {
    if (!statement) {
        return null;
    }

    return new Function('varName', 'prevVarName', 'nextVarName', [
        `var str = '${statement}';`,
        `str = str.replace('${varName}', varName);`,
        `str = str.replace('${prevVarName}', prevVarName);`,
        `str = str.replace('${nextVarName}', nextVarName);`,
        `return str;`
    ].join('\n'));
}

module.exports = {
    toSnakeCase,
    toPascalCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    blockToString,
    isCount,
    areFieldsCountAndArray,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldInformation,
    findEnumPrefix,
    LifetimeSet
};