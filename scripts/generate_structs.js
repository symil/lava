#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const rimraf = require('rimraf');

const ROOT = path.join(__dirname, '..');
const DST_STRUCT_DIR = path.join(ROOT, 'src', 'vk_types');
const VULKAN_SDK_PATH = process.env.VULKAN_SDK;
const VULKAN_H = fs.readFileSync(path.join(__dirname, `vulkan.h`), 'utf8');

const PRIMITIVE_TYPE = {
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    char: 'i8',
    float: 'float',
    double: 'double',
    size_t: 'usize',
    VkBool32: 'u32',
    VkDeviceSize: 'u64'
};

const typesToGenerate = ['VkPhysicalDeviceProperties'];
const alreadyGenerated = new Set();

rimraf.sync(DST_STRUCT_DIR);
fs.mkdirSync(DST_STRUCT_DIR);
generateTypes(['VkPhysicalDeviceProperties']);

function generateTypes(types) {
    types.forEach(generateType);
}

function writeVkType({name, rawDefinition, trueDefinition, fromDefinition}) {
    const moduleName = cToRustVarName(name);
    const filePath = path.join(DST_STRUCT_DIR, `${moduleName}.rs`);
    const fileContent = ['use std;', rawDefinition, trueDefinition, fromDefinition].filter(x => x).join('\n\n');

    const rootFileName = path.join(DST_STRUCT_DIR, 'mod.rs');
    const existingRootContent = fs.existsSync(rootFileName) ? fs.readFileSync(rootFileName, 'utf8') : '';
    const newRootContent = `${existingRootContent}pub mod ${moduleName};\n`

    fs.writeFileSync(filePath, fileContent, 'utf8');
    fs.writeFileSync(rootFileName, newRootContent, 'utf8');
}

function generateStruct(name) {
    const match = VULKAN_H.match(new RegExp(`typedef struct ${name} {\n([^}]+)\n}`, 'm'));

    if (!match) return false;

    const rawTypeName = toRawTypeName(name);
    const trueTypeName = toTrueTypeName(name);

    const rawDefLines = [];
    const trueDefLines = [];
    const fromLines = []

     match[1].split('\n').forEach(line => {
        let [type, name] = line.split(' ').filter(x => x);
        let rustPrimitiveType = PRIMITIVE_TYPE[type];
        let rawRustType = rustPrimitiveType ? rustPrimitiveType : toRawTypeName(type);
        let trueRustType = rustPrimitiveType ? rustPrimitiveType : toTrueTypeName(type);
        let isArray = name.includes('[');

        if (type === 'VkBool32') {
            trueRustType = 'bool';
        }

        generateType(type);

        name = name.substring(0, name.length - 1);

        if (isArray) {
            const start = name.indexOf('[');
            const end = name.indexOf(']');
            const constantName = name.substring(start + 1, end);
            const constantValue = isNaN(+constantName) ? findConstant(constantName) : constantName;

            name = name.substring(0, start);
            rawRustType = `[${rawRustType}; ${constantValue}]`;
            trueRustType = `[${trueRustType}; ${constantValue}]`;
        }

        // TODO: check if field is flag
        // TODO: check if field is VkBool

        let rustName = cToRustVarName(name);

        rawDefLines.push(`${rustName}: ${rawRustType}`);
        trueDefLines.push(`${rustName}: ${trueRustType}`);

        const sourceField = `value.${rustName}`;
        let fieldConvertion;

        if (rustPrimitiveType) {
            if (type === 'VkBool32') {
                fieldConvertion = `${sourceField} != 0`
            } else {
                fieldConvertion = sourceField;
            }
        } else {
            fieldConvertion = `${trueRustType}::from(${sourceField})`;
        }

        fromLines.push(`${rustName}: ${fieldConvertion}`);
    });

    const rawDefinition = [
        `pub struct ${rawTypeName} {`,
        rawDefLines.map(line => `    ${line}`).join(',\n'),
        `}`
    ].join('\n');

    const trueDefinition = [
        `pub struct ${trueTypeName} {`,
        trueDefLines.map(line => `    ${line}`).join(',\n'),
        `}`
    ].join('\n');

    const fromDefinition = [
        `impl std::convert::From<${rawTypeName}> for ${trueTypeName} {`,
        `    fn from(value: ${rawTypeName}) {`,
        `        ${trueTypeName} {`,
        fromLines.map(line => `            ${line}`).join(',\n'),
        `        }`,
        `    }`,
        `}`
    ].join('\n');

    writeVkType({name, rawDefinition, trueDefinition, fromDefinition});

    return true;
}

function generateEnum(name) {
    const match = VULKAN_H.match(new RegExp(`typedef enum ${name}\\s+{\n([^}]+)\n}`, 'm'));

    if (!match) return false;

    const rawTypeName = toRawTypeName(name);
    const rawDefinition = `pub type ${rawTypeName} = i32;`;
    const trueTypeName = toTrueTypeName(name);
    const enumPrefix = name.replace(/[a-z]+/g, str => `${str.toUpperCase()}_`);

    const trueDefFields = [];
    const fromLines = [];

    match[1].split('\n').forEach(line => {
        const match = line.match(/^\s*([A-Z_]+)\s*=\s*(-?\d+),?$/);

        if (!match) return;

        const valueName = match[1];
        const valueInt = match[2];

        if (!valueName.startsWith(enumPrefix)) {
            throw new Error(`enum value ${valueName} does not start with prefix ${enumPrefix}`);
        }

        const rustValue = cToRustEnumValue(valueName.substring(enumPrefix.length));

        trueDefFields.push(rustValue);
        fromLines.push(`${valueInt} => ${trueTypeName}::${rustValue}`);
    });

    const trueDefinition = [
        `#[derive(PartialEq)]`,
        `pub enum ${trueTypeName} {`,
        trueDefFields.map(l => `    ${l}`).join(',\n'),
        `}`
    ].join('\n');

    const fromDefinition = [
        `impl From<${rawTypeName}> for ${trueTypeName} {`,
        `    fn from(value: ${rawTypeName}) {`,
        `        match(value) {`,
        fromLines.map(line => `            ${line},`).join('\n'),
        `            _ => panic!("Vulkan wrapper error: unable to convert int32 {} into ${trueTypeName} value", value)`,
        `        }`,
        `    }`,
        `}`
    ].join('\n');

    return true;
}

function findConstant(name) {
    const match = VULKAN_H.match(new RegExp(`#define\\s+${name}\\s+([0-9.]+)`));

    if (!match) {
        throw new Error(`cannot find constant ${name}`);
    }

    return match[1];
}

function generateType(name) {
    if (PRIMITIVE_TYPE[name] || alreadyGenerated.has(name)) {
        return;
    }

    alreadyGenerated.add(name);

    const generated = generateStruct(name) || generateEnum(name);

    if (!generated) {
        throw new Error(`cannot find type ${name}`);
    }
}

function cToRustVarName(name) {
    return name.replace(/[A-Z]+/g, str => `_${str.toLowerCase()}`).replace(/__/g, '_').replace(/^_/, '');
}

function cToRustEnumValue(name) {
    return `_${name}`.toLowerCase().replace(/_[a-z]/g, str => str.charAt(1).toUpperCase());
}

function toRawTypeName(name) {
    return `Raw${name}`;
}

function toTrueTypeName(name) {
    return name;
}