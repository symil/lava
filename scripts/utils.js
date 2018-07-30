const { isHandle } = require('./vulkan_header');

const PRIMITIVE_TYPES = {
    bool: 'bool',
    uint64_t: 'u64',
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int64_t: 'i64',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    int: 'i32',
    char: 'c_char',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    ssize_t: 'isize',
    void: 'c_void',
    VkAllocationCallbacks: 'c_void',
    VkDeviceSize: 'u64',
    VkSampleMask: 'u32'
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
        .replace(/([A-Z]+)|([0-9]+)/g, str => `_${str}`)
        .toLowerCase()
        .replace(/(^|_)(1|2|3)_d(_|$)/g, '$1$2d$3')
        .replace(/^_/, '')
        .replace(/_+/g, '_');
}

function toPascalCase(str) {
    return str.split('_')
        .map(word => word.charAt(0).toUpperCase() + word.substring(1).toLowerCase()).join('')
        .replace(/\d[a-z](?=\d)/g, str => str[0] + str[1].toUpperCase());
}

function toUpperCase(str) {
    return str
        .replace(/[a-z][A-Z]/g, str => `${str[0]}_${str[1]}`)
        .toUpperCase();
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
        return _blocksToString(block).replace(/\n {/g, '\n{');
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

    return result.replace(/\n;/g, ';\n');
}

function isPlural(arg) {
    return arg && !arg.typeName.endsWith('s') && arg.name.endsWith('s');
}

function cToRustVarName(name) {
    name = name.replace(/^(p{1,2})[A-Z]/, str => str[str.length - 1]);
    name = toSnakeCase(name);

    if (name === 'type') {
        name = 'type_';
    }

    return name;
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
    return `unsafe { let mut dst_array : [${typeName}; ${arraySize}] = mem::uninitialized(); ${functionName}(&${varName}, &mut dst_array); dst_array }`;
}

function fillStaticArray(typeName, arraySize) {
    return `unsafe { let mut dst_array : [${typeName}; ${arraySize}] = mem::uninitialized(); fill_vk_array(&mut dst_array); dst_array }`;
}

function getRawVkTypeName(cTypeName) {
    return `Raw${cTypeName}`;
}

function getWrappedVkTypeName(cTypeName) {
    return cTypeName;
}

function getFieldRawTypeName(field) {
    if (field.typeName === 'VkBool32') {
        return 'u32';
    }

    return PRIMITIVE_TYPES[field.typeName] || getRawVkTypeName(field.typeName);
}

function getFieldWrappedTypeName(field) {
    if (doesFieldRepresentVersion(field)) {
        return `VkVersion`;
    } else if (doesFieldRepresentIndex(field)) {
        return `usize`;
    } else if (field.typeName === 'VkBool32') {
        return 'bool';
    }

    return PRIMITIVE_TYPES[field.typeName] || getWrappedVkTypeName(field.typeName);
}

function getStaticVkValueName(wrappedTypeName) {
    return `STATIC_${toUpperCase(wrappedTypeName)}`;
}

function getConstVkValueName(wrappedTypeName) {
    return `CONST_${toUpperCase(wrappedTypeName)}`;
}

function getPrimitiveDefaultValue(typeName) {
    if (!Object.values(PRIMITIVE_TYPES).includes(typeName)) {
        return null;
    } if (typeName === 'bool') {
        return 'false';
    } else if (typeName === 'f32' || typeName === 'f64') {
        return '0.0';
    } else {
        return '0';
    }
}

function getFieldsInformation(fields) {
    const infos = [];

    for (let field of fields) {
        const rawTypeName = getFieldRawTypeName(field);
        const wrappedTypeName = getFieldWrappedTypeName(field);

        const arraySize = field.arraySize;
        const isCount = !!field.countFor.length;
        const isPointerArray = field.isPointer && (field.countField || field.name === 'pSampleMask');
        const isPointerValue = field.isPointer && !isPointerArray;
        const isStaticArray = !field.isPointer && !!arraySize;
        const isPrimitiveType = rawTypeName === wrappedTypeName;
        const isHandleType = !isPrimitiveType && isHandle(field.typeName);
        const isOptional = field.isOptional;

        const varName = '[VarName]';
        const countVarName = '[CountVarName]';
        const arrayVarName = '[ArrayVarName]';

        const varNameValue = cToRustVarName(field.name);
        const countVarNameValue = field.countField ? cToRustVarName(field.countField) : null;
        const arrayVarNameValue = isCount ? cToRustVarName(field.countFor[0]) : null;

        let rawType = null;
        let wrappedType = null;
        let toRaw = null;
        let toWrapped = null;
        let defValue = null;

        if (field.name === 'sType' && field.values) {
            rawType = 'RawVkStructureType';
            toRaw = () => `vk_to_raw_value(&VkStructureType::${toPascalCase(field.values.substring('VK_STRUCTURE_TYPE_'.length))})`;
        } else if (field.name === 'pNext') {
            rawType = `*const c_void`;
            toRaw = `ptr::null()`;
        } else if (field.typeName === 'void' && field.isPointer) {
            const qualifier = field.isConst ? 'const' : 'mut';

            rawType = `*${qualifier} c_void`;
            wrappedType = `*${qualifier} c_void`;
            toRaw = varName;
            toWrapped = varName;
            defValue = field.isConst ? `ptr::null()` : `ptr::null_mut()`;
        } else if (isCount) {
            rawType = `u32`;

            let lenValue = `makeVarName(${cToRustVarName(field.countFor[0])}).len()`;

            for (let i = 1; i < field.countFor.length; ++i) {
                const otherLen = `makeVarName(${cToRustVarName(field.countFor[i])}).len()`;
                lenValue = `cmp::max(${lenValue}, ${otherLen})`;
            }

            if (field.name === 'codeSize') {
                lenValue = `(${lenValue} * 4)`
            }

            lenValue += ' as u32';

            toRaw = new Function('makeVarName', `return '${lenValue}'.replace(/makeVarName\\((\\w+)\\)/g, function(_, name) { return makeVarName(name); })`);
        } else if (field.fullType === 'const char* const*') {
            rawType = `VkPtr<*mut c_char>`;
            wrappedType = `&[&str]`;
            toRaw = `VkPtr::new_string_array(${varName})`;
            defValue = `&[]`;
        } else if (field.fullType === 'const char*') {
            if (field.name === 'displayName') {
                rawType = '*const c_char';
                wrappedType =`Option<String>`;
                toWrapped = `new_string_checked(${varName})`;
                defValue = 'None';
            } else {
                rawType = `VkPtr<c_char>`;

                if (isOptional) {
                    wrappedType =`Option<&str>`;
                    toRaw = `VkPtr::new_string_checked(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType =`&str`;
                    toRaw = `VkPtr::new_string(${varName})`;
                    defValue = `""`;
                }
            }
        } else if (field.fullType === 'char' && field.arraySize) {
            rawType = `[c_char; ${arraySize}]`;
            wrappedType = `String`;
            // toRaw = createStaticArray(rawTypeName, arraySize, varName, 'string_to_byte_array');
            toWrapped = `new_string(&${varName}[0] as *const c_char)`;
            // defValue = `String::new()`;
        } else if (isPrimitiveType) {
            const primitiveDefaultValue = getPrimitiveDefaultValue(wrappedTypeName);

            if (isPointerArray) {
                rawType = `*const ${rawTypeName}`;
                wrappedType = `&[${wrappedTypeName}]`;
                toRaw = `${varName}.as_ptr()`;
                // toWrapped = `new_array(${prevVarName}, ${varName})`;
                defValue = `&[]`;
            } else if (isPointerValue) {
                // There's no reason to get there
                console.log(`NON ARRAY POINTER ON PRIMITIVE VALUE: ${field.name}`);
                rawType = `*const *${rawTypeName}`;
                wrappedType = `&${wrappedTypeName}`;
                toRaw = `${varName} as *const ${rawTypeName}`;
                // toWrapped = varName;
                defValue = `&${primitiveDefaultValue}`;
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;

                if (countVarNameValue) {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toWrapped = `new_array(${countVarName}, ${varName}.as_ptr())`;
                } else {
                    wrappedType = `[${wrappedTypeName}; ${arraySize}]`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'to_array');
                    toWrapped = createStaticArray(rawTypeName, arraySize, varName, 'to_array');
                    defValue = `[${primitiveDefaultValue}; ${arraySize}]`;
                }
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = varName;
                toWrapped = varName;
                defValue = primitiveDefaultValue;
            }
        } else {
            if (isPointerArray) {
                rawType = `VkPtr<${rawTypeName}>`;
                wrappedType = `&[${wrappedTypeName}]`;
                toRaw = `VkPtr::new_vk_array(${varName})`;
                // toWrapped = `new_vk_array(${prevVarName}, ${varName})`;
                defValue = `&[]`;
            } else if (isPointerValue) {
                rawType = `VkPtr<${rawTypeName}>`;
                // toWrapped = `${rawTypeName}::vk_to_wrapped(${varName}.as_ref().unwrap())`;

                if (isOptional) {
                    wrappedType = `Option<&${wrappedTypeName}>`;
                    toRaw = `VkPtr::new_vk_value_checked(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `&${wrappedTypeName}`;
                    toRaw = `VkPtr::new_vk_value(${varName})`;
                    defValue = `vk_null_ref()`;
                }
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;

                if (countVarNameValue) {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toWrapped = `new_vk_array(${countVarName}, ${varName}.as_ptr())`;
                } else {
                    wrappedType = `[${wrappedTypeName}; ${arraySize}]`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                    defValue = fillStaticArray(wrappedTypeName, arraySize);
                }
            } else if (isHandleType) {
                rawType = rawTypeName;
                // toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;

                if (isOptional) {
                    wrappedType = `Option<&${wrappedTypeName}>`;
                    toRaw = `if ${varName}.is_some() { vk_to_raw_value(${varName}.unwrap()) } else { 0 }`;
                    defValue = `None`;
                } else {
                    wrappedType = `&${wrappedTypeName}`;
                    toRaw = `vk_to_raw_value(${varName})`;
                    defValue = `vk_null_ref()`;
                }
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = `vk_to_raw_value(&${varName})`;
                toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;
                defValue = getPrimitiveDefaultValue(wrappedTypeName) || `${wrappedTypeName}::vk_default()`;
            }
        }

        const info = {
            typeName: field.typeName,
            extension: field.extension,
            rawType: rawType,
            wrappedType: wrappedType,
            rawTypeName: rawTypeName,
            wrappedTypeName: wrappedTypeName,
            toRaw: stringToFunction(toRaw, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            toWrapped: stringToFunction(toWrapped, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            defaultValue: defValue,
            varName: varNameValue
        };

        infos.push(info);
    }

    return infos;
}

function stringToFunction(statement, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue) {
    if (!statement) {
        return null;
    }

    if (typeof statement === 'function') {
        return statement;
    }

    let body = [
        `var str = '${statement}';`,
        `str = str.replace('${varName}', makeVarName("${varNameValue}"));`,
        `str = str.replace('${varName}', makeVarName("${varNameValue}"));`
    ];

    if (countVarNameValue) {
        body.push(`str = str.replace('${countVarName}', makeVarName("${countVarNameValue}"));`);
    }

    if (arrayVarNameValue) {
        body.push(`str = str.replace('${arrayVarName}', makeVarName("${arrayVarNameValue}"));`);
    }

    body.push(`return str;`);

    return new Function('makeVarName', body.join('\n'));
}

module.exports = {
    toSnakeCase,
    toPascalCase,
    toUpperCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    blockToString,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldsInformation,
    findEnumPrefix,
    getStaticVkValueName,
    getConstVkValueName
};