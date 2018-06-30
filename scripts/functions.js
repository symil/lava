#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const {
    DST_DIR_NAME,
    DST_DIR_PATH,
    VULKAN_H,
    PRIMITIVE_TYPE
} = require('./constants');

const {
    cToRustVarName,
    capitalizeVarName,
    cToRustEnumValue,
    toRawTypeName,
    toTrueTypeName
} = require('./utils');

const FILE_NAME = 'c_bindings';

const FUNCTIONS_TO_GENERATE = [
    'vkCreateInstance',
    'vkDestroyInstance',
    'vkEnumeratePhysicalDevices',
    'vkGetPhysicalDeviceProperties',
    'vkGetPhysicalDeviceQueueFamilyProperties',
    'vkCreateDevice',
    'vkDestroyDevice',
    'vkGetDeviceQueue',
    'vkCreateBuffer',
    'vkDestroyBuffer',
    'vkGetPhysicalDeviceSurfaceSupportKHR',
    'vkDestroySurfaceKHR'
];

main();

function main() {
    writeFunctions();
}

function writeFunctions() {
    const match = VULKAN_H.match(/\nVKAPI_ATTR (VkResult|void) VKAPI_CALL \w+\([^;]+\)/g);
    const functions = match.map(block => {
        const lines = block.split('\n').slice(1).map(removeTrailing);
        const [VKAPI_ATTR, returnType, VKAPI_CALL, functionName] = lines[0].split(' ');

        if (!FUNCTIONS_TO_GENERATE.includes(functionName)) return null;

        const args = lines.slice(1).map(line => {
            const spaceIndex = line.lastIndexOf(' ');
            const type = line.substring(0, spaceIndex).trim();
            const name = line.substring(spaceIndex + 1);

            const rustType = getRustType(type);
            const rustName = cToRustVarName(name);

            return `${rustName}: ${rustType}`;
        });

        return `    pub fn ${functionName}(${args.join(', ')})${returnType !== 'void' ? ` -> ${returnType}` : ``};`
    }).filter(x => x).join('\n');

    const fileContent = `use vk::*;\n\nextern {\n${functions}\n}`;
    const filePath = path.join(DST_DIR_PATH, `${FILE_NAME}.rs`);

    fs.writeFileSync(filePath, fileContent, 'utf8');
}

function removeTrailing(str) {
    return str.substring(0, str.length - 1);
}

function getRustType(type) {
    if (type === 'const VkAllocationCallbacks*') {
        return `*const VkAllocator`;
    } else if (type.endsWith('*')) {
        const isConst = type.startsWith('const ');
        const typeName = type.substring(isConst ? 'count '.length : 0, type.length - 1)
        const targetTypeName = PRIMITIVE_TYPE[typeName] || `Raw${typeName}`;

        return `*${isConst ? 'const' : 'mut'} ${targetTypeName}`;
    } else {
        return PRIMITIVE_TYPE[type] || `Raw${type}`;
    }
}