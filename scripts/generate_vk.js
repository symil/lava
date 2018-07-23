#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const { parseStructs, parseFunctions, isHandle } = require('./parse_vulkan_h');
const { toSnakeCase, toPascalCase, getRawTypeName, getWrappedTypeName } = require('./utils');
const { HandleList } = require('./handles');
const { generateVkStructDefinition } = require('./structs');

const ROOT = path.join(__dirname, '..');
const DST_DIR_NAME = 'vk';
const DST_DIR_PATH = path.join(ROOT, 'src', DST_DIR_NAME);

const HEADER = '// Generated by `scripts/generate_vk.js`';

main();

function main() {
    // generateHandles();
    generateStructs();
}

function generateStructs() {
    const cStructs = parseStructs().slice(0, 1);

    return cStructs.map(cDef => {
        const rustDefinition = generateVkStructDefinition(cDef);

        console.log(rustDefinition)

        return {
            name: cDef.name,
            extension: cDef.extension,
            definition: rustDefinition
        };
    });
}

function generateHandles() {
    const cFunctions = parseFunctions();
    const handles = new HandleList();

    const destroyFunctions = cFunctions.filter(func => func.name.includes('Destroy'));
    const otherFunctions = cFunctions.filter(func => !func.name.includes('Destroy'));

    destroyFunctions.forEach(func => {
        const handleName = func.args.beforeLast().typeName;
        const hasParent = func.args.length > 2;
        const parent = hasParent ? func.args[0].typeName : 'VkInstance';

        handles.get(parent).addHandleToDestroy(handleName, func);

        if (hasParent) {
            handles.get(handleName).setParent(parent);
        }
    });

    otherFunctions.forEach(func => {
        const firstArg = func.args[0];
        const isFirstArgHandle = !firstArg.isPointer && isHandle(firstArg.typeName);
        const handleName = isFirstArgHandle ? firstArg.typeName : 'VkInstance';

        handles.get(handleName).addMethod(func);
    });

    console.log(handles.get('VkInstance').toString());

    return handles;
}