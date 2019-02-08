const { getHandle, getStruct } = require('./parse');

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

const OUTPUT_HANDLE_STRUCTS = [
    'VkDisplayProperties',
    'VkDisplayPlaneProperties',
    'VkDisplayModeProperties'
];

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

String.prototype.capitalize = function() {
    return capitalize(this);
};

function capitalize(str) {
    if (!str.length) {
        return '';
    }

    return str.charAt(0).toUpperCase() + str.substring(1);
}

function isStructOrHandle(field) {
    return !!(getHandle(field) || getStruct(field));
}

function isStruct(field) {
    return !!getStruct(field);
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
    if (field.name === 'pCode') {
        return 'u8';
    } else if (field.typeName === 'VkBool32') {
        return 'u32';
    }

    return PRIMITIVE_TYPES[field.typeName] || getRawVkTypeName(field.typeName);
}

const INT_TYPES = ['uint32_t', 'uint64_t', 'int32_t', 'int64_t', 'VkDeviceSize']

// const INT_FIELD_NAMES = ['x', 'y', 'z', 'width', 'height', 'layers', 'timeout'];
const INT_FIELD_NAMES = ['layers', 'timeout'];

function fieldNameIsInt(field) {
    return INT_FIELD_NAMES.includes(field.name) ||
        /(mask|version|(bits$))/i.test(field.name);
}

function getFieldWrappedTypeName(field) {
    if (field.name === 'pCode') {
        return `u8`;
    } else if (doesFieldRepresentVersion(field)) {
        return `VkVersion`;
    } else if (INT_TYPES.includes(field.typeName) && !fieldNameIsInt(field)) {
        return field.typeName.startsWith('int') ? `isize` : `usize`;
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

function isOutputHandleStruct(typeName) {
    return OUTPUT_HANDLE_STRUCTS.includes(typeName);
}

function getCountVarNameValue(countFieldName, getVarName) {
    const isNested = countFieldName.includes('::');

    if (isNested) {
        const path = countFieldName.split('::');
        return `(&*${getVarName(cToRustVarName(path[0]))}).${path.slice(1).map(cToRustVarName).join('.')}`;
    } else {
        return getVarName(cToRustVarName(countFieldName));
    }
}

function getFieldsInformation(fields, structName) {
    const infos = [];

    for (const field of fields) {
        const rawTypeName = getFieldRawTypeName(field);
        const wrappedTypeName = getFieldWrappedTypeName(field);

        const arraySize = field.arraySize;
        const isCount = !!(field.countFor && field.countFor.length);
        const isDoublePointer = field.isDoublePointer;
        const isPointerArray = field.isPointer && (field.countField || field.name === 'pSampleMask');
        const isPointerValue = field.isPointer && !isPointerArray;
        const isStaticArray = !field.isPointer && !!arraySize;
        const isPrimitiveType = rawTypeName === wrappedTypeName;
        const isOptional = field.isOptional && !!(structName || !field.isPointer || field.isConst || field !== fields.last());

        const varName = '[VarName]';
        const countVarName = '[CountVarName]';
        const arrayVarName = '[ArrayVarName]';

        const varNameValue = cToRustVarName(field.name);
        const countVarNameValue = field.countField ? field.countField : null;
        const arrayVarNameValue = isCount ? cToRustVarName(field.countFor[0]) : null;
        const countField = (field.countField && fields.find(f => f.name === field.countField)) || {};
        const countVarIsPointer = countField.isPointer;

        let rawType = null;
        let wrappedType = null;
        let toRaw = null;
        let toWrapped = null;
        let defValue = null;
        let freeRaw = null;

        if (field.name === 'pResults' && structName === 'VkPresentInfo') {
            rawType = '*mut RawVkResult';
            wrappedType = 'bool';
            toRaw = getVarName => `if ${getVarName(varNameValue)} { unsafe { calloc(${getVarName(`swapchains`)}.len(), mem::size_of::<RawVkResult>()) as *mut RawVkResult } } else { ptr::null_mut() }`;
            toWrapped = `!${varName}.is_null()`;
            defValue = `false`;
            freeRaw = `free_ptr(${varName})`;
        } else if (field.name === 'pAllocator') {
            rawType = '*const c_void';
            wrappedType = '*const c_void';
            toRaw = varName;
            toWrapped = varName;
            defValue = 'ptr::null()';
        } else if (field.name === 'sType' && field.values) {
            rawType = 'RawVkStructureType';
            toRaw = () => `vk_to_raw_value(&VkStructureType::${toPascalCase(field.values.substring('VK_STRUCTURE_TYPE_'.length))})`;
        } else if (field.name === 'pNext') {
            rawType = `*mut c_void`;
            toRaw = `ptr::null_mut()`;
        } else if (field.name === 'pCode') {
            rawType = '*mut u8';
            wrappedType = '&[u8]';
            toRaw = `get_vec_ptr(${varName})`;
            toWrapped = `&[]`;
            defValue = '&[]';
        } else if (field.fullType === 'void**') {
            rawType = `*mut *mut c_void`;
            wrappedType = `*mut *mut c_void`;
            toRaw = varName;
            toWrapped = varName;
            defValue = 'ptr::null_mut()'
        } else if (/(const )?void\*$/.test(field.fullType)) {
            if (field.countField) {
                rawType = `*mut c_void`;
                toRaw = `get_vec_ptr(${varName})`;

                if (countVarIsPointer) {
                    wrappedType = `Vec<c_void>`;
                    toWrapped = `Vec::from_raw_parts(${varName}, *${countVarName}, *${countVarName})`;
                    defValue = 'Vec::new()';
                } else {
                    wrappedType = `&[c_void]`;
                    toWrapped = `slice_from_ptr(${countVarName} as usize, ${varName})`;
                    defValue = '&[]';
                }
            } else {
                rawType = `*mut c_void`;
                wrappedType = `*mut c_void`;
                toRaw = varName;
                toWrapped = varName;
                defValue = 'ptr::null_mut()';
            }
        } else if (isCount) {
            rawType = isPointerValue ? `*mut ${rawTypeName}` : rawTypeName;

            let lenValue = '';

            for (let i = 0; i < field.countFor.length; ++i) {
                const arrayFieldName = field.countFor[i];
                const arrayField = fields.find(f => f.name === arrayFieldName);
                if (i === 0 || arrayField.isConst || !arrayField.isPointer) {
                    // Don't use created array into account
                    const arrayName =`makeVarName(${cToRustVarName(arrayFieldName)})`; 
                    const otherLen = arrayField.isOptional ? `get_array_option_len(&${arrayName})` : `${arrayName}.len()`;
                    lenValue = !lenValue ? otherLen : `cmp::max(${lenValue}, ${otherLen})`;
                }
            }

            // if (field.name === 'codeSize') {
            //     lenValue = `${lenValue} * 4`
            // }

            if (rawTypeName !== 'usize') {
                lenValue += ` as ${rawTypeName}`;
            }

            toRaw = new Function('makeVarName', [
                `return '${lenValue}'.replace(/makeVarName\\((\\w+)\\)/g, function(_, name) { return makeVarName(name); });`,
            ].join('\n'));
        } else if (field.fullType === 'const char* const*') {
            rawType = `*mut *mut c_char`;
            freeRaw = `free_ptr(${varName})`;
            wrappedType = `Vec<&str>`;
            toRaw = `new_ptr_string_array(&${varName})`;
            toWrapped = `new_string_ref_vec(${countVarName}, ${varName} as *const *const c_char)`;
            defValue = `Vec::new()`;
        } else if (field.fullType === 'const char*') {
            rawType = `*mut c_char`;
            freeRaw = `free_ptr(${varName})`;

            if (isOptional) {
                wrappedType =`Option<&str>`;
                toRaw = `new_ptr_string_checked(&${varName})`;
                toWrapped = `new_string_ref_checked(${varName})`;
                defValue = `None`;
            } else {
                wrappedType =`&str`;
                toRaw = `new_ptr_string(${varName})`;
                toWrapped = `new_string_ref(${varName})`;
                defValue = `""`;
            }
        } else if (field.fullType === 'char' && field.arraySize) {
            rawType = `[c_char; ${arraySize}]`;
            wrappedType = `String`;
            toRaw = createStaticArray(rawTypeName, arraySize, varName, 'string_to_byte_array');
            toWrapped = `new_string(&${varName}[0] as *const c_char)`;
            defValue = `String::new()`;
        } else if (isPrimitiveType) {
            const primitiveDefaultValue = getPrimitiveDefaultValue(wrappedTypeName);

            if (isPointerArray) {
                let vecLength = `${countVarIsPointer ? '*' : ''}${countVarName}`;
                if (countField.typeName !== 'size_t') {
                    vecLength += ' as usize';
                }

                rawType = `*mut ${rawTypeName}`;
                // freeRaw = `free_ptr(${varName})`;

                if (isOptional) {
                    wrappedType = `Option<Vec<${wrappedTypeName}>>`;
                    toRaw = `get_vec_ptr_checked(&${varName})`;
                    toWrapped = `vec_from_ptr_checked(${vecLength}, ${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toRaw = `get_vec_ptr(&${varName})`;
                    toWrapped = `vec_from_ptr(${vecLength}, ${varName})`;
                    defValue = `Vec::new()`;
                }

                if (!field.countField) {
                    // This line is specifically for the field `pSampleMask` of `VkPipelineMultisampleStateCreateInfo`
                    // Might need to change it
                    toWrapped = isOptional ? `None` : `vec![]`;
                }
            } else if (isPointerValue) {
                rawType = `*mut ${rawTypeName}`;
                freeRaw = `free_ptr(${varName})`;
                wrappedType = wrappedTypeName;
                toRaw = `&${varName} as *const ${rawTypeName}`;
                toWrapped = `*${varName}`;
                defValue = primitiveDefaultValue;
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;

                if (field.countField) {
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
            const fieldIsStruct = isStruct(field);

            if (isDoublePointer) {
                const countPrefix = countField.isPointer ? '*' : '';
                const countSuffix = countField.typeName === 'size_t' ? '' : ' as usize';

                rawType = `*mut *mut ${rawTypeName}`;
                if (fieldIsStruct) {
                    freeRaw = `free_vk_ptr_array_array(${countPrefix}${countVarName}${countSuffix}, ${varName})`;
                } else {
                    freeRaw = `free_ptr(${varName})`;
                }
                wrappedType = `Vec<${wrappedTypeName}>`;
                toRaw = `new_ptr_vk_array_array(&${varName})`;
                defValue = `Vec::new()`;
            } else if (isPointerArray) {
                const countPrefix = countField.isPointer ? '*' : '';
                const countSuffix = countField.typeName === 'size_t' ? '' : ' as usize';

                rawType = `*mut ${rawTypeName}`;
                if (fieldIsStruct) {
                    freeRaw = `free_vk_ptr_array(${countPrefix}${countVarName}${countSuffix}, ${varName})`;
                } else {
                    freeRaw = `free_ptr(${varName})`;
                }

                if (isOptional) {
                    wrappedType = `Option<Vec<${wrappedTypeName}>>`;
                    toWrapped = `new_vk_array_checked(${countVarIsPointer ? '*' : ''}${countVarName}, ${varName})`;
                    toRaw = `new_ptr_vk_array_checked(&${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toWrapped = `new_vk_array(${countVarIsPointer ? '*' : ''}${countVarName}, ${varName})`;
                    toRaw = `new_ptr_vk_array(&${varName})`;
                    defValue = `Vec::new()`;
                }
            } else if (isPointerValue) {
                rawType = `*mut ${rawTypeName}`;

                if (fieldIsStruct) {
                    freeRaw = `free_vk_ptr(${varName})`;
                } else {
                    freeRaw = `free_ptr(${varName})`;
                }

                if (isOptional) {
                    wrappedType = `Option<${wrappedTypeName}>`;
                    toWrapped = `new_vk_value_checked(${varName})`;
                    toRaw = `new_ptr_vk_value_checked(&${varName})`;
                    defValue = `None`;
                } else {
                    wrappedType = `${wrappedTypeName}`;
                    toWrapped = `new_vk_value(${varName})`;
                    toRaw = `new_ptr_vk_value(&${varName})`;
                    defValue = `Default::default()`;
                }
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;

                if (fieldIsStruct) {
                    freeRaw = `for elt in ${varName}.iter() { ${rawTypeName}::vk_free(elt); }`;
                }

                if (field.countField) {
                    wrappedType = `Vec<${wrappedTypeName}>`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                    toWrapped = `new_vk_array(${countVarName}, ${varName}.as_ptr())`;
                    defValue = `Vec::new()`;
                } else {
                    wrappedType = `[${wrappedTypeName}; ${arraySize}]`;
                    toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                    toWrapped = createStaticArray(wrappedTypeName, arraySize, varName, 'vk_to_wrapped_array');
                    defValue = fillStaticArray(wrappedTypeName, arraySize);
                }
            } else {
                rawType = rawTypeName;

                if (isOptional && !wrappedTypeName.endsWith('Flags') && wrappedTypeName.startsWith('Vk')) {
                    wrappedType = `Option<${wrappedTypeName}>`;
                    toRaw = `vk_to_raw_value_checked(&${varName})`;
                    toWrapped = `Some(${rawTypeName}::vk_to_wrapped(&${varName}))`;
                    defValue = `None`;
                } else {
                    wrappedType = wrappedTypeName;
                    toRaw = `vk_to_raw_value(&${varName})`;
                    toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;
                    defValue = getPrimitiveDefaultValue(wrappedTypeName) || `Default::default()`;
                }
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
            // let use = `vk::`;
            // if (field.extension) { use += `${field.extension}::`; }
            // use += toSnakeCase(typeName);
            // use += `::*`;

            let types = [field.wrappedTypeName];

            if (field.rawTypeName.startsWith('Raw')) {
                types.push(field.rawTypeName);
            }

            set.add(`vulkan::${field.extension || 'vk'}::{${types.join(',')}}`);
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
        `var str = '${statement}';`,
        `str = str.replace('${varName}', makeVarName("${varNameValue}"));`,
        `str = str.replace('${varName}', makeVarName("${varNameValue}"));`
    ];

    if (countVarNameValue) {
        body.push(`str = str.replace('${countVarName}', makeCountVar("${countVarNameValue}", makeVarName));`);
        body.push(`str = str.replace('${countVarName}', makeCountVar("${countVarNameValue}", makeVarName));`);
    }

    if (arrayVarNameValue) {
        body.push(`str = str.replace('${arrayVarName}', makeVarName("${arrayVarNameValue}"));`);
    }

    body.push(
        `if (removeUnsafe) str = str.replace('unsafe {', '{');`,
        `return str;`
    );

    const baseFunc = new Function('makeCountVar', 'makeVarName', 'removeUnsafe', body.join('\n'));

    return function(makeVarName, removeUnsafe) {
        return baseFunc(getCountVarNameValue, makeVarName, removeUnsafe);
    };
}

function documentType(def, precision) {
    const typeName = (def.typeName || def.name).replace(/Flags$/, 'FlagBits') + (def.extension || '').toUpperCase();
    let str = `/// Wrapper for [${typeName}](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/${typeName}.html).`;

    if (precision) {
        str += `\n///\n${precision}`;
    }

    return str;
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
    isStructOrHandle,
    isStruct,
    isOutputHandleStruct,
    getCountVarNameValue,
    documentType
};