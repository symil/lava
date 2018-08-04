const { getHandle, getStruct } = require('./vulkan_src');

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

Array.prototype.first = function() {
    return this[0];
};

Array.prototype.last = function() {    
    return this[this.length - 1];
};

Array.prototype.beforeLast = function() {
    return this[this.length - 2];
};

String.prototype.toSnakeCase = function() {
    return toSnakeCase(this);
}

function isStructOrHandle(field) {
    return !!(getHandle(field) || getStruct(field));
}

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
    blocks = blocks.filter(b => b !== null);

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
    return field.typeName === 'uint32_t' && /(p?)apiversion$/i.test(field.name);
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
        const isCount = !!(field.countFor && field.countFor.length);
        const isDoublePointer = field.isDoublePointer;
        const isPointerArray = field.isPointer && (field.countField || field.name === 'pSampleMask');
        const isPointerValue = field.isPointer && !isPointerArray;
        const isStaticArray = !field.isPointer && !!arraySize;
        const isPrimitiveType = rawTypeName === wrappedTypeName;
        const isHandleType = !isPrimitiveType && !!getHandle(field);
        const isOptional = field.isOptional;
        const isConst = field.isConst;

        const varName = '[VarName]';
        const countVarName = '[CountVarName]';
        const arrayVarName = '[ArrayVarName]';

        const varNameValue = cToRustVarName(field.name);
        const countVarNameValue = field.countField ? cToRustVarName(field.countField) : null;
        const arrayVarNameValue = isCount ? cToRustVarName(field.countFor[0]) : null;
        const countVarIsPointer = field.countField && fields.find(f => f.name === field.countField).isPointer;

        let rawType = null;
        let wrappedType = null;
        let toRaw = null;
        let toWrapped = null;
        let defValue = null;
        let freeRaw = null;

        if (field.name === 'sType' && field.values) {
            rawType = 'RawVkStructureType';
            toRaw = () => `vk_to_raw_value(&VkStructureType::${toPascalCase(field.values.substring('VK_STRUCTURE_TYPE_'.length))})`;
        } else if (field.name === 'pNext') {
            rawType = `*const c_void`;
            toRaw = `ptr::null()`;
        } else if (field.fullType === 'void**') {
            rawType = `*mut *mut c_void`;
            wrappedType = `*mut *mut c_void`;
            toRaw = varName;
            toWrapped = varName;
            defValue = 'ptr::null_mut()'
        } else if (field.typeName === 'void' && field.isPointer) {
            const qualifier = field.isConst ? 'const' : 'mut';

            rawType = `*${qualifier} c_void`;
            wrappedType = `*${qualifier} c_void`;
            toRaw = varName;
            toWrapped = varName;
            defValue = field.isConst ? `ptr::null()` : `ptr::null_mut()`;
        } else if (isCount) {
            rawType = isPointerValue ? (isConst ? `*const u32` : `*mut u32`) : `u32`;

            let lenValue = `makeVarName(${cToRustVarName(field.countFor[0])}).len()`;

            for (let i = 1; i < field.countFor.length; ++i) {
                const arrayFieldName = field.countFor[i];
                const arrayField = fields.find(f => f.name === arrayFieldName);
                if (arrayField.isConst || !arrayField.isPointer) {
                    // Don't use created array into account
                    const otherLen = `makeVarName(${cToRustVarName(arrayFieldName)}).len()`;
                    lenValue = `cmp::max(${lenValue}, ${otherLen})`;
                }
            }

            if (field.name === 'codeSize') {
                lenValue = `(${lenValue} * 4)`
            }

            lenValue += ' as u32';

            toRaw = new Function('makeVarName', [
                `makeVarName = makeVarName || function(x) { return x; };`,
                `return '${lenValue}'.replace(/makeVarName\\((\\w+)\\)/g, function(_, name) { return makeVarName(name); });`,
            ].join('\n'));
        } else if (field.fullType === 'const char* const*') {
            rawType = `*mut *mut c_char`;
            wrappedType = `&[&str]`;
            toRaw = `new_ptr_string_array(${varName})`;
            defValue = `&[]`;
            freeRaw = `free_ptr(${varName})`
        } else if (field.fullType === 'const char*') {
            if (field.name === 'displayName') {
                rawType = '*const c_char';
                wrappedType =`Option<String>`;
                toWrapped = `new_string_checked(${varName})`;
                defValue = 'None';
            } else {
                rawType = `*mut c_char`;
                freeRaw = `free_ptr(${varName})`

                if (isOptional) {
                    wrappedType =`Option<&str>`;
                    toRaw = `new_ptr_string_checked(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType =`&str`;
                    toRaw = `new_ptr_string(${varName})`;
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
            const ptrQualifier = isConst ? 'const' : 'mut';

            if (isPointerArray) {
                rawType = `*${ptrQualifier} ${rawTypeName}`;
                wrappedType = `&[${wrappedTypeName}]`;
                toRaw = `${varName}.as_ptr()`;
                // toWrapped = `new_array(${prevVarName}, ${varName})`;
                defValue = `&[]`;
            } else if (isPointerValue) {
                rawType = `*${ptrQualifier} ${rawTypeName}`;
                wrappedType = `&${wrappedTypeName}`;
                toRaw = `${varName} as *${ptrQualifier} ${rawTypeName}`;
                toWrapped = varName;
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
            if (isDoublePointer) {
                rawType = `*mut *mut ${rawTypeName}`;
                wrappedType = `&[&${wrappedTypeName}]`;
                toRaw = `new_ptr_vk_array_array(${varName})`;
                defValue = `&[]`;
                freeRaw = `free_ptr(${varName})`
            } else if (isPointerArray) {
                rawType = `*mut ${rawTypeName}`;
                wrappedType = `&[${wrappedTypeName}]`;
                toRaw = `new_ptr_vk_array(${varName})`;
                toWrapped = `new_vk_array(${countVarIsPointer ? '*' : ''}${countVarName}, ${varName})`;
                defValue = `&[]`;
                freeRaw = `free_ptr(${varName})`
            } else if (isPointerValue) {
                rawType = `*mut ${rawTypeName}`;
                freeRaw = `free_ptr(${varName})`

                if (isOptional) {
                    wrappedType = `Option<&${wrappedTypeName}>`;
                    toRaw = `new_ptr_vk_value_checked(${varName})`;
                    toWrapped = `new_vk_value_checked(${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `&${wrappedTypeName}`;
                    toRaw = `new_ptr_vk_value(${varName})`;
                    toWrapped = `new_vk_value(${varName})`;
                    defValue = `vk_null_ref()`;
                }
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;

                if (countVarNameValue) {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toWrapped = `new_vk_array(${countVarName}, ${varName}.as_ptr())`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                    defValue = `Vec::new()`;
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
            freeRaw: stringToFunction(freeRaw, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            defaultValue: defValue,
            varName: varNameValue
        };

        infos.push(info);
    }

    return infos;
}

function addUsesToSet(set, rootType, fields) {
    for (let field of fields) {
        const typeName = field.wrappedTypeName;

        if (typeName.startsWith('Vk') && typeName !== rootType.wrappedTypeName) {
            let use = `vk::`;
            if (field.extension) { use += `${field.extension}::`; }
            use += toSnakeCase(typeName);
            use += `::*`;

            set.add(use);
        }
    }
}

function stringToFunction(statement, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue) {
    if (!statement) {
        return null;
    }

    if (typeof statement === 'function') {
        return statement;
    }

    let body = [
        `makeVarName = makeVarName || function(x) { return x; };`,
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
    getConstVkValueName,
    addUsesToSet,
    isStructOrHandle
};