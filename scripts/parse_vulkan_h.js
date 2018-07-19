#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const VULKAN_SDK_PATH = process.env.VULKAN_SDK;
const VULKAN_H = fs.readFileSync(path.join(VULKAN_SDK_PATH, `include`, `vulkan`, `vulkan_core.h`), 'utf8');

const POSSIBLE_TYPES = {
    struct: parseStruct,
    enum: parseEnum,
    flags: parseBitFlags,
    handle: parseHandle,
    special: parseSpecial
};

const SPECIAL_TYPES = ['VkBool32', 'VkDeviceSize'];

const REMOVE_SUFFIXES = true;
const SUFFIX = REMOVE_SUFFIXES ? `(?:KHR|EXT)?` : '';

const SPECIAL_FUNCTIONS = [
    'VkResult glfwCreateWindowSurface(VkInstance instance, GLFWwindow* window, const VkAllocationCallbacks* allocator, VkSurfaceKHR* surface);',
    'VkResult vkCreateDebugReportCallbackEXT(VkInstance instance, const VkDebugReportCallbackCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDebugReportCallbackEXT* pCallback);',
    'void vkDestroyDebugReportCallbackEXT(VkInstance instance, VkDebugReportCallbackEXT callback, const VkAllocationCallbacks* pAllocator);'
].join('\n');

function removeSuffix(str) {
    if (REMOVE_SUFFIXES) {
        return str.replace(/_?KHR$/, '').replace(/_?EXT$/, '');
    } else {
        return str;
    }
}

function parseType(name) {
    const typeName = removeSuffix(name);
    const entries = Object.entries(POSSIBLE_TYPES);

    for (let i = 0; i < entries.length; ++i) {
        const [type, parsingFunction] = entries[i];
        const fields = parsingFunction(typeName);

        if (fields) {
            return { name: typeName, type, fields };
        }
    }

    throw new Error(`unable to parse type ${name}`);
}

function parseSpecial(typeName) {
    return SPECIAL_TYPES.includes(typeName) ? {} : null;
}

function parseStruct(typeName) {
    const match = VULKAN_H.match(new RegExp(`typedef struct ${typeName}${SUFFIX} {\n([^}]+)\n}`, 'mi'));

    if (!match) {
        return null;
    }

    return match[1].split('\n').map(line => {
        const match = line.match(/\s*([\w* ]+)\s+(\w+)(?:\[(\w+)\])?;\s*$/);

        if (!match) {
            throw new Error(`unexpected line for struct ${typeName}: "${line}"`);
        }

        const name = removeSuffix(match[2].trim());
        const fullType = match[1].trim();
        const typeName = removeSuffix(fullType.replace(/(?:const )?(\w+)\*?/, '$1'));
        const isPointer = fullType.endsWith('*');
        const isConst = fullType.startsWith('const ');
        const arraySize = parseConstant(match[3]);

        return { name, fullType, typeName, isPointer, isConst, arraySize };
    });
}

function parseEnum(typeName) {
    const match = VULKAN_H.match(new RegExp(`typedef enum ${typeName}${SUFFIX}\\s+{\n([^}]+)\n}`, 'mi'));

    if (!match) {
        return null;
    }

    return match[1].split('\n').map(line => {
        const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(-?\d+),?$/);

        if (!match || match[1] === 'VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT') return null;

        return {
            name: removeSuffix(match[1].trim()),
            value: match[2].trim()
        };
    }).filter(x => x);
}

function parseBitFlags(typeName) {
    const bitsFlagTypeName = typeName.replace('Flags', 'FlagBits');
    const match = VULKAN_H.match(new RegExp(`typedef enum ${bitsFlagTypeName}${SUFFIX}\\s+{\n([^}]+)\n}`, 'mi'));

    if (!match) {
        return typeName.includes('Flags') ? [] : null;
    }

    const fields = [];

    match[1].split('\n').map(line => {
        const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(0x[\dA-F]{8})|([A-Z_]+),?\s*$/);

        if (!match) {
            throw new Error(`for enum ${typeName}: unexpected field "${line}"`);
        }

        const value =  match[2] || match[3];

        if (value !== '0x7FFFFFFF' && value.startsWith('0x')) {
            fields.push({
                name: removeSuffix(match[1]),
                value: value
            })
        }
    });

    return fields;
}

function parseHandle(typeName) {
    const match = VULKAN_H.match(new RegExp(`\n(?:VK_DEFINE_HANDLE|VK_DEFINE_NON_DISPATCHABLE_HANDLE)\\(${typeName}${SUFFIX}\\)`, 'mi'));

    return match ? {} : null;
}

function isHandle(typeName) {
    return new RegExp(`\n(?:VK_DEFINE_HANDLE|VK_DEFINE_NON_DISPATCHABLE_HANDLE)\\(${typeName}${SUFFIX}\\)`, 'mi').test(VULKAN_H);
}

function parseFunction(name) {
    const regexp = new RegExp(`(?:VKAPI_ATTR/\s+)?(VkResult|void)\\s+(?:VKAPI_CALL\\s+)?${name}\\s*\\(([^;]+)\\)`, 'm');
    
    const match = SPECIAL_FUNCTIONS.match(regexp) || VULKAN_H.match(regexp);

    if (!match) {
        throw new Error(`unable to parse function ${name}`);
    }

    const returnType = removeSuffix(match[1]);

    const args = match[2].trim().split(',').map(line => {
        line = line.trim();
        const spaceIndex = line.lastIndexOf(' ');

        const name = line.substring(spaceIndex + 1);
        const fullType = line.substring(0, spaceIndex).trim();
        const typeName = removeSuffix(fullType.replace(/(?:const )?(\w+)\*?/, '$1'));
        const isPointer = fullType.endsWith('*');
        const isConst = fullType.startsWith('const ');

        return { name, fullType, typeName, isPointer, isConst };
    });

    return { name, returnType, args };
}

function parseConstant(name) {
    if (!name) {
        return null;
    }

    if (!isNaN(+name)) {
        return name;
    }

    const match = VULKAN_H.match(new RegExp(`#define\\s+${name}\\s+([0-9.]+)`));

    if (!match) {
        throw new Error(`cannot find constant ${name}`);
    }

    return match[1];
}

function parseFunctions() {
    const regexp = /(?:VKAPI_ATTR\s+)?(VkResult|void)\s+(?:VKAPI_CALL\s+)?(\w+)\s*\(([^;]+)\)/gm;
    const match = VULKAN_H.match(regexp);
    const functions = match.map(str => {
        const words = str.split(/\W+/g);
        const type = words[1];
        const name = words[3];
        const args = str.substring(str.indexOf('(') + 1, str.indexOf(')')).split(',').map(x => x.trim()).map(argStr => {
            const spaceIndex = argStr.lastIndexOf(' ');
            const name = argStr.substring(spaceIndex + 1);
            const fullType = argStr.substring(0, spaceIndex).trim();
            const typeName = removeSuffix(fullType.replace(/(?:const )?(\w+)\*?/, '$1'));
            const isPointer = fullType.endsWith('*');
            const isConst = fullType.startsWith('const ');

            return { name, fullType, typeName, isPointer, isConst };
        });

        return { name, type, args };
    });

    return functions;
}

exports.parseType = parseType;
exports.parseFunction = parseFunction;
exports.isHandle = isHandle;

exports.parseFunctions = parseFunctions;