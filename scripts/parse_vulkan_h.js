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

function parseSpecial(typeName) {
    return SPECIAL_TYPES.includes(typeName) ? {} : null;
}

function parseType(name) {
    const entries = Object.entries(POSSIBLE_TYPES);

    for (let i = 0; i < entries.length; ++i) {
        const [type, parseFunction] = entries[i];
        const fields = parseFunction(name);

        if (fields) {
            return { name, type, fields };
        }
    }

    throw new Error(`unable to parse type ${name}`);
}

function parseStruct(structName) {
    const match = VULKAN_H.match(new RegExp(`typedef struct ${structName} {\n([^}]+)\n}`, 'mi'));

    if (!match) {
        return null;
    }

    return match[1].split('\n').map(line => {
        const match = line.match(/\s*([\w* ]+)\s+(\w+)(?:\[(\w+)\])?;\s*$/);

        if (!match) {
            throw new Error(`unexpected line for struct ${structName}: "${line}"`);
        }

        const name = match[2].trim();
        const fullType = match[1].trim();
        const typeName = fullType.replace(/(?:const )?(\w+)\*?/, '$1');
        const isPointer = fullType.endsWith('*');
        const isConst = fullType.startsWith('const ');
        const arraySize = parseConstant(match[3]);

        return { name, fullType, typeName, isPointer, isConst, arraySize };
    });
}

function parseEnum(typeName) {
    const match = VULKAN_H.match(new RegExp(`typedef enum ${typeName}\\s+{\n([^}]+)\n}`, 'mi'));

    if (!match) {
        return null;
    }

    return match[1].split('\n').map(line => {
        const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(-?\d+),?$/);

        if (!match) return null;

        return {
            name: match[1].trim(),
            value: match[2].trim()
        };
    }).filter(x => x);
}

function parseBitFlags(typeName) {
    const bitsFlagTypeName = typeName.replace('Flags', 'FlagBits');
    const match = VULKAN_H.match(new RegExp(`typedef enum ${bitsFlagTypeName}\\s+{\n([^}]+)\n}`, 'mi'));

    if (!match) {
        return typeName.includes('Flags') ? [] : null;
    }

    return match[1].split('\n').map(line => {
        const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(0x[\dA-F]{8})|([A-Z_]+),?\s*$/);

        if (!match) {
            throw new Error(`for enum ${typeName}: unexpected field "${line}"`);
        }

        return {
            name: match[1],
            value: match[2] || match[3]
        };
    }).filter(({value}) => value !== '0x7FFFFFFF' && value.startsWith('0x'));
}

function parseHandle(typeName) {
    const match = VULKAN_H.match(new RegExp(`\n(?:VK_DEFINE_HANDLE|VK_DEFINE_NON_DISPATCHABLE_HANDLE)\\(${typeName}\\)`, 'mi'));

    return match ? {} : null;
}

function isHandle(typeName) {
    return new RegExp(`\n(?:VK_DEFINE_HANDLE|VK_DEFINE_NON_DISPATCHABLE_HANDLE)\\(${typeName}\\)`, 'mi').test(VULKAN_H);
}

function parseFunction(name) {
    const match = VULKAN_H.match(new RegExp(`\nVKAPI_ATTR (VkResult|void) VKAPI_CALL ${name}\\(([^;]+)\\)`, 'm'));

    if (!match) {
        throw new Error(`unable to parse function ${name}`);
    }

    const returnType = match[1];

    const args = match[2].trim().split('\n').map(line => {
        const spaceIndex = line.lastIndexOf(' ');

        const name = line.substring(spaceIndex + 1).replace(',', '');
        const fullType = line.substring(0, spaceIndex).trim();
        const typeName = fullType.replace(/(?:const )?(\w+)\*?/, '$1');
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

parseFunction('vkCreateInstance');

exports.parseType = parseType;
exports.parseFunction = parseFunction;
exports.isHandle = isHandle;