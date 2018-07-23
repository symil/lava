const { isHandle } = require('./parse_vulkan_h');

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

function getFullRawType(arg) {
    if (arg.fullType === 'const char* const*') {
        return `*const *const c_char`;
    } else if (arg.fullType === 'const char*') {
        return `*const RawVkString`;
    } else {
        const rawTypeName = getRawTypeName(arg.typeName);

        if (!arg.isPointer) {
            return rawTypeName;
        } else if (arg.isConst) {
            return `*const ${rawTypeName}`;
        } else {
            return `*mut ${rawTypeName}`;
        }
    }
}

function getFullWrappedType(arg) {
    if (arg.fullType === 'const char* const*') {
        return `&[&str]`;
    } else if (arg.fullType === 'const char*') {
        return `&str`;
    } else {
        const rawTypeName = getWrappedTypeName(arg.typeName);

        if (!arg.isPointer) {
            return rawTypeName;
        } else if (arg.isConst) {
            return `&${rawTypeName}`;
        } else {
            return `&mut ${rawTypeName}`;
        }
    }
}

function blockToString(block) {
    if (typeof block === 'string') {
        return block;
    } else {
        let result = block[0];

        if (!result.endsWith(' ') && !result.endsWith('\n')) {
            result += ' ';
        }

        result += _blockToString(block[1], 0);

        return result;
    }
}

function _blockToString(block, indent) {
    const spaces = indentToSpaces(indent);
    
    const result = typeof block === 'string'
        ? `\n${block.split('\n').map(line => `${spaces}${line}`).join('\n')}`
        : ` {${block.filter(x => x !== null).map(b => _blockToString(b, inc(indent))).join('')}\n${spaces}}`;

    return result.replace(/(^|\n) {/g, '$1{');
}

function inc(value) {
    return value === undefined ? 0 : value + 1;
}

function indentToSpaces(indent) {
    if (!indent) return '';

    return new Array(indent).fill('    ').join('');
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

function getFieldInformation(field, prevField, nextField) {
    // TODO: default values

    const rawTypeName = getFieldRawTypeName(field);
    const wrappedTypeName = getFieldWrappedTypeName(field);

    const arraySize = field.arraySize;
    const isPointerArray = field.isPointer && areFieldsCountAndArray(prevField, field);
    const isPointerValue = field.isPointer && !isPointerArray;
    const isStaticArray = !field.isPointer && arraySize;
    const isPrimitiveType = rawTypeName !== wrappedTypeName;
    const isHandleType = !isPrimitiveType && isHandle(field.typeName);
    const isCount = areFieldsCountAndArray(field, nextField);

    const varName = 'varName';
    const prevVarName = 'prevVarName';
    const nextVarName = 'nextVarName';

    let rawType = null;
    let wrappedType = null;
    let toRaw = null;
    let toWrapped = null;
    let defValue = null;

    if (field.fullType === 'const void*') {
        rawType = `*const c_void`;
        wrappedType = `*const c_void`;
        toRaw = varName;
        toWrapped = varName;
        defValue = `ptr::null()`;
    } else if (isCount) {
        rawType = `u32`;
        toRaw = `${nextVarName}.len() as u32`;
    } else if (field.fullType === 'const char* const*') {
        rawType = `*const *const c_char`;
        wrappedType = `Vec<String>`;
        toRaw = `VkPtr::new_string_array(&${varName})`;
        toWrapped = `Vec::new()`; // Should never be used
        defValue = `Vec::new()`;
    } else if (field.fullType === 'const char*') {
        rawType = `*const c_char`;
        wrappedType = `T : Deref<Target=str>`;
        toRaw = `VkPtr::new_string(&${varName})`;
        toWrapped = `""`; // Should never be used
        defValue = `vk_null()`;
    } else if (field.fullType === 'char' && field.arraySize) {
        rawType = `[c_char; ${arraySize}]`;
        wrappedType = `T : Deref<Target=str>`;
        toRaw = createStaticArray(rawTypeName, arraySize, varName, 'string_to_byte_array');
        toWrapped = `new_string(&${varName}[0] as *const c_char)`;
        defValue = `""`;
    } else if (isPrimitiveType) {
        if (isPointerArray) {
            rawType = `VkPtr<${rawTypeName}>`;
            wrappedType = `T : Deref<Target=[${wrappedTypeName}]>`;
            toRaw = `VkPtr::new_array(&${varName})`;
            toWrapped = `new_array(${prevVarName}, ${varName})`;
            defValue = `Vec::new()`;
        } else if (isPointerValue) {
            rawType = `VkPtr<${rawTypeName}>`;
            wrappedType = `T : Deref<Target=${wrappedTypeName}>`;
            toRaw = `VkPtr::new_value(${varName})`;
            toWrapped = varName; // Should never be used
            defValue = `vk_null()`;
        } else if (isStaticArray) {
            rawType = `[${rawTypeName}; ${arraySize}]`;
            wrappedType = `T : Deref<Target=[${wrappedTypeName}]>`;
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
            wrappedType = `T : Deref<Target=[${wrappedTypeName}]>`;
            toRaw = `VkPtr::new_vk_array(&${varName})`;
            toWrapped = `new_vk_array(${prevVarName}, ${varName})`;
            defValue = `Vec::new()`;
        } else if (isPointerValue) {
            rawType = `VkPtr<${rawTypeName}>`;
            wrappedType = `T : Deref<Target=${wrappedTypeName}>`;
            toRaw = `VkPtr::new_vk_value(&${varName})`;
            toWrapped = `${wrappedTypeName}::vk_from_raw(${varName}.as_ref().unwrap())`; // Pointer should never be null; if that happens, we should use an `Option`
            defValue = `vk_null()`;
        } else if (isStaticArray) {
            rawType = `[${rawTypeName}; ${arraySize}]`;
            wrappedType = `T : Deref<Target=[${wrappedTypeName}]>`;
            toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
            toWrapped = `new_vk_array(${prevVarName}, &${varName}[0] as *const ${rawTypeName})`;
            defValue = `Vec::new()`;
        } else if (isHandleType) {
            rawType = rawTypeName;
            wrappedType = `T : Deref<Target=${wrappedTypeName}>`;
            toRaw = `vk_to_raw_value(&${varName})`;
            toWrapped = `${wrappedTypeName}::vk_from_raw(&${varName})`;
            defValue = `vk_null()`;
        } else {
            rawType = rawTypeName;
            wrappedType = wrappedTypeName;
            toRaw = `vk_to_raw_value(&${varName})`;
            toWrapped = `${wrappedTypeName}::vk_from_raw(&${varName})`;
            defValue = `${wrappedTypeName}::vk_default()`;
        }
    }

    const toRawFunction = toRaw ? new Function(varName, prevVarName, nextVarName, `return '${toRaw}';`) : null;
    const toWrappedFunction = toWrapped ? new Function(varName, prevVarName, nextVarName, `return '${toWrapped}';`) : null;

    return {
        rawType: rawType,
        wrappedType: wrappedType,
        toRaw: toRawFunction,
        toWrapped: toWrappedFunction,
        defValue: defValue
    };
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
    getFieldInformation
};