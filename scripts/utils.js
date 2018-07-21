const REMOVE_SUFFIXES = true;

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

function getRawTypeName(name) {
    return PRIMITIVE_TYPES[name] || `Raw${name}`;
}

function getWrappedTypeName(name) {
    return PRIMITIVE_TYPES[name] || name;
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

function blockToString(block, indent) {
    const spaces = indentToSpaces(indent);
    let result;

    if (typeof block === 'string') {
        result = `\n${block.split('\n').map(line => `${spaces}${line}`).join('\n')}`;
    } else {
        result = ` {${block.filter(x => x !== null).map(b => blockToString(b, inc(indent))).join('')}\n${spaces}}`
    }

    if (indent === undefined) {
        result = result.substring(2, result.length - 2).trim();
    }

    return result;
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

function isPlural(arg) {
    return arg && !arg.typeName.endsWith('s') && arg.name.endsWith('s');
}

function removeSuffix(str) {
    if (REMOVE_SUFFIXES) {
        str = str
            .replace(/_?KHR$/, '')
            .replace(/_?EXT$/, '');
    }

    return str;
}

function cToRustVarName(name) {
    return toSnakeCase(name.replace(/^(p{1,2})[A-Z]/, str => str[str.length - 1]));
}

function argToString(arg) {
    return arg.name ? `${arg.name}: ${arg.type}` : arg.type;
}

function isPrimitiveType(typeName) {
    return !!PRIMITIVE_TYPES[typeName];
}

function convertToRaw(cArg, isArray) {
    const argName = cToRustVarName(cArg.name);
    const rawTypeName = getRawTypeName(cArg.typeName);
    const wrappedTypeName = getWrappedTypeName(cArg.typeName);

    if (arg.fullType === 'const char* const*') {
        return `VkPtr::new_string_array(${argName})`;
    } else if (arg.fullType === 'const char*') {
        return `VkPtr::new_string(${argName})`;
    } else {
        const isPrimitiveType = isPrimitiveType(arg.typeName);
        const isPointerArray = arg.isPointer && isArray;
        const isPointerValue = arg.isPointer && !isArray;
        const isStaticArray = !arg.isPointer && arg.arraySize;

        if (isPrimitiveType) {
            if (isPointerArray) {
                return `VkPtr::new_array(${argName})`;
            } else if (isPointerValue) {
                return `VkPtr::new_value(${argName})`;
            } else if (isStaticArray) {
                return `${argName}.clone()`;
            } else {
                return argName;
            }
        } else {
            if (isPointerArray) {
                return `VkPtr::new_vk_array(${argName})`;
            } else if (isPointerValue) {
                return `VkPtr::new_vk_value(${argName})`;
            } else if (isStaticArray) {
                return `unsafe { let mut dst_array : [${rawTypeName}, ${arg.arraySize}] = mem::uninitialized(); vk_to_raw_array(${argName}, &mut dst_array); dst_array }`;
            } else {
                return `vk_to_raw_value(&${argName})`;
            }
        }
    }
}

module.exports = {
    toSnakeCase,
    toPascalCase,
    getRawTypeName,
    getWrappedTypeName,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isCount,
    isPlural,
    removeSuffix,
    cToRustVarName,
    argToString
};