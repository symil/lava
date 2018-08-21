const path = require('path');
const fs = require('fs');
const XML = require('pixl-xml')

const DOWNLOAD_DIR_PATH = path.join(__dirname, '..', 'download');
const VULKAN_CORE_H_PATH = path.join(DOWNLOAD_DIR_PATH, `vulkan_core.h`);
const VK_XML_PATH = path.join(DOWNLOAD_DIR_PATH, `vk.xml`);

const EXTENSIONS = ['KHR', 'EXT', 'GOOGLE', 'NV', 'NVX', 'AMD'];

let VULKAN_H = null;
let VK_XML_STR = null;
let VK_XML = null;
let ENUMS = null;
let BIT_FLAGS = null;
let STRUCTS = null;
let HANDLES = null;
let FUNCTIONS = null;
let BOOTSTRAP_DONE = false;

function bootstrap() {
    if (!BOOTSTRAP_DONE) {
        VULKAN_H = fs.readFileSync(VULKAN_CORE_H_PATH, 'utf8');
        VK_XML_STR = fs.readFileSync(VK_XML_PATH, 'utf8');
        VK_XML = XML.parse(VK_XML_STR);
        ENUMS = parseEnums();
        BIT_FLAGS = parseBitFlags();
        STRUCTS = parseStructs();
        HANDLES = parseHandles();
        FUNCTIONS = parseFunctions();
        BOOTSTRAP_DONE = true;
    }
}

function getByName(obj, typeName) {
    const { name, extension } = parseName(typeName);

    return obj[extension][name];
}

function getAll(obj) {
    bootstrap();
    return Object.values(obj).reduce((acc, subObj) => acc.concat(Object.values(subObj)), []);
}

function get(obj, type) {
    return (obj[type.extension] || {})[type.typeName];
}

function getAllEnums() { return getAll(ENUMS); }
function getAllBitFlags() { return getAll(BIT_FLAGS); }
function getAllStructs() { return getAll(STRUCTS); }
function getAllHandles() { return getAll(HANDLES); }
function getAllFunctions() { bootstrap(); return FUNCTIONS.slice(); }

function getEnumByName(name) { return getByName(ENUMS, name); }
function getBitFlagsByName(name) { return getByName(BIT_FLAGS, name); }
function getStructByName(name) { return getByName(STRUCTS, name); }
function getHandleByName(name) { return getByName(HANDLES, name); }

function getEnum(type) { return get(ENUMS, type); }
function getBitFlags(type) { return get(BIT_FLAGS, type); }
function getStruct(type) { return get(STRUCTS, type); }
function getHandle(type) { return get(HANDLES, type); }

function isHandle(name) { return !!getByName(HANDLES, name); }

function parseName(str) {
    let extension = '';

    for (let ext of EXTENSIONS) {
        if (str.endsWith(ext)) {
            extension = ext.toLowerCase();
        }
    }

    return {
        extension: extension,
        name: str.substring(0, str.length - extension.length)
    };
}

function listToObj(array) {
    const types = {};

    array.forEach(type => {
        const byExtension = types[type.extension] || (types[type.extension] = {});
        byExtension[type.name] = type;
    });

    return types;
}

function parseField(str) {
    const match = str.match(/\s*([\w* ]+)\s+(\w+)(?:\[(\w+)\])?;?\s*$/);

    if (!match) {
        return null;
    }

    const fullType = match[1].replace('FlagBits', 'Flags').replace(' struct ', ' ').trim();
    const name = match[2].trim();
    const fieldName = fullType.replace(/\*\*$/, '').replace(/ const\*$/, '').replace(/(?:const )?(\w+)\*?/, '$1');
    const fieldTypeNameInfo = parseName(fieldName);
    const typeName = fieldTypeNameInfo.name;
    const extension = fieldTypeNameInfo.extension;
    const isPointer = fullType.endsWith('*');
    const isDoublePointer = fullType.endsWith(' const*') || fullType.endsWith('**');
    const isConst = fullType.startsWith('const ');
    const arraySizeIdentifier = match[3];
    const arraySize = parseConstant(arraySizeIdentifier);
    const countFor = [];

    return { name, extension, fullType, typeName, isPointer, isDoublePointer, isConst, arraySize, countFor };
}

function parseEnums() {
    const regexp = /typedef enum \w+ {\n([^}]+)\n}/gmi;
    const match = VULKAN_H.match(regexp);

    const enums = match.map(str => {
        const structName = str.split(' ', 3)[2];
        const structNameInfo = parseName(structName);
        const name = structNameInfo.name;
        const extension = structNameInfo.extension;
        const fieldsStr = str.substring(str.indexOf('{') + 2, str.indexOf('}') - 1);

        if (name.endsWith('FlagBits')) {
            return null;
        }

        const fields = fieldsStr.split('\n').map(line => {
            const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(-?(?:0x)?\d+),?$/);

            if (!match) {
                return null;
            }

            return {
                name: match[1].trim(),
                value: match[2].trim()
            };
        }).filter(x => x);

        return { name, extension, fields };
    }).filter(x => x);

    return listToObj(enums);
}

function parseBitFlags() {
    const defined = {};
    const flagBitsRegexp = /typedef enum \w+FlagBits[A-Z]* {\n([^}]+)\n}/gmi;
    const match = VULKAN_H.match(flagBitsRegexp);

    match.forEach(str => {
        const name = str.split(' ', 3)[2];
        const fieldsStr = str.substring(str.indexOf('{') + 2, str.indexOf('}') - 1);

        const fields = fieldsStr.split('\n').map(line => {
            const match = line.match(/^\s*([0-9A-Z_]+)\s*=\s*(0x[\dA-F]{8})|([A-Z_]+)|(0),?\s*$/);

            if (!match) {
                throw new Error(`for enum ${name}: unexpected field "${line}"`);
            }

            return {
                name: match[1],
                value: match[2] || match[3] || match[4]
            };
        }).filter(({value}) => value !== '0x7FFFFFFF' && value.startsWith('0x'));

        defined[name] = fields;
    });

    const flagsRegexp = /typedef VkFlags \w+;/g
    const match2 = VULKAN_H.match(flagsRegexp);

    const bitFlags = match2.map(str => {
        const fullName = str.substring(str.lastIndexOf(' ') + 1, str.indexOf(';'));
        const flagBitsName = fullName.replace('Flags', 'FlagBits');
        const nameInfo = parseName(fullName);
        const fields = defined[flagBitsName] || [];

        return {
            name: nameInfo.name,
            extension: nameInfo.extension,
            fields: fields
        };
    });

    return listToObj(bitFlags);
}

function parseHandles() {
    const regexp = /(VK_DEFINE_HANDLE|VK_DEFINE_NON_DISPATCHABLE_HANDLE)\(\w+\)\n/gm;
    const match = VULKAN_H.match(regexp);

    const handles = match.map(line => {
        const handleName = line.substring(line.indexOf('(') + 1, line.indexOf(')'));
        const nameInfo = parseName(handleName);

        return {
            name: nameInfo.name,
            extension: nameInfo.extension
        };
    });

    return listToObj(handles);
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

function parseStructs() {
    const regexp = /typedef struct \w+ {\n([^}]+)\n}/gmi;
    const match = VULKAN_H.match(regexp);

    const structs = match.map(str => {
        const structName = str.split(' ', 3)[2];
        const structNameInfo = parseName(structName);
        const name = structNameInfo.name;
        const extension = structNameInfo.extension;
        const fieldsStr = str.substring(str.indexOf('{') + 2, str.indexOf('}') - 1);

        const xmlDef = VK_XML.types.type.find(def => def.name === structName);

        if (!Array.isArray(xmlDef.member)) {
            xmlDef.member = [xmlDef.member];
        }

        const fields = fieldsStr.split('\n').filter(x => x).map(line => {
            const fieldInfo = parseField(line);

            if (!fieldInfo) {
                throw new Error(`unexpected line for struct ${structName}: "${line}"`);
            }

            return fieldInfo;
        });

        let lastField = null;
        for (let field of fields) {
            const xmlMember = xmlDef.member.find(member => member.name === field.name);

            field.values = xmlMember.values;

            if (xmlMember.name === 'pCode') {
                xmlMember.len = "codeSize";
            }

            field.isOptional = !!xmlMember.optional;
            if (field.typeName !== 'void') {
                field.countField = (xmlMember.len || '').split(',').find(str => fields.some(field => field.name === str));

                if (areCountAndArray(lastField, field)) {
                    field.countField = lastField.name;
                }
            }

            lastField = field;
        }

        for (let field of fields) {
            field.countFor = fields.filter(otherField => otherField.countField === field.name).map(f => f.name);
        }

        return { name, extension, fields };
    });

    return listToObj(structs);
}

function areCountAndArray(field1, field2) {
    return field1
        && field2
        && field1.name.startsWith(field2.name.substring(0, field2.name.length - 1))
        && field1.name.endsWith('Count')
        && field1.fullType === 'uint32_t';
}

function parseFunctions() {
    const regexp = /(?:VKAPI_ATTR\s+)?(VkResult|void)\s+(?:VKAPI_CALL\s+)?(\w+)\s*\(([^;]+)\)/gm;
    const match = VULKAN_H.match(regexp);

    const functions = match.map(str => {
        const words = str.replace(/VKAPI_ATTR|VKAPI_CALL/g, '').trim().split(/\W+/, 2);
        const type = words[0];
        const name = words[1];
        
        const args = str.substring(str.indexOf('(') + 1, str.indexOf(')')).split(',').map(x => x.trim()).map(argStr => {
            const argInfo = parseField(argStr);

            if (!argInfo) {
                throw new Error(`unexpected arg "${argStr}" for function "${name}"`);
            }

            return argInfo;
        });

        let xml = VK_XML.commands.command.find(c => (c.proto && c.proto.name === name) || c.name === name);
        
        if (xml) {
            if (xml.alias) {
                return null;
                xml = VK_XML.commands.command.find(c => c.proto && c.proto.name === xml.alias)
            }
    
            const xmlParams = Array.isArray(xml.param) ? xml.param : [xml.param];
    
            if (!xmlParams) {
                throw new Error(`function ${name} does not have a xml`)
            }

            let lastArg = null;
            for (let arg of args) {
                const xmlParam = xmlParams.find(p => p.name === arg.name);
    
                if (!xmlParam) {
                    throw new Error(`function "${name}": missing xml parameter ${arg.name}`);
                }
    
                arg.values = xmlParam.values;
    
                arg.isOptional = !!xmlParam.optional;
                // if (arg.typeName !== 'void') {
                    arg.countField = (xmlParam.len || '').split(',').find(str => args.some(arg => arg.name === str || str.startsWith(`${arg.name}::`)));
    
                    if (areCountAndArray(lastArg, arg)) {
                        arg.countField = lastArg.name;
                    }
                // }
    
                lastArg = arg;
            }
    
            for (let arg of args) {
                arg.countFor = args.filter(otherArg => otherArg.countField === arg.name).map(a => a.name);
            }
        }

        return { name, type, args };
    }).filter(f => f);

    return functions;
}

function getExtensions() {
    return EXTENSIONS.slice();
}

module.exports = {
    getAllEnums,
    getAllBitFlags,
    getAllStructs,
    getAllHandles,
    getAllFunctions,
    getEnumByName,
    getBitFlagsByName,
    getStructByName,
    getHandleByName,
    getEnum,
    getBitFlags,
    getStruct,
    getHandle,
    isHandle,
    getExtensions
};