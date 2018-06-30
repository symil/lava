#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const ROOT = path.join(__dirname, '..', 'src');
const DIRS_TO_REFRESH = ['vk', 'glfw'];

main();

function main() {
    DIRS_TO_REFRESH.forEach(name => refreshModRs(path.join(ROOT, name)));
}

function refreshModRs(dirPath) {
    const filePath = path.join(dirPath, 'mod.rs');
    const moduleNames = fs.readdirSync(dirPath).filter(name => name !== 'mod.rs').map(str => str.replace('.rs', ''));
    const content = [
        ...moduleNames.map(name => `mod ${name};`),
        ``,
        ...moduleNames.map(name => `pub use self::${name}::*;`),
    ].join('\n');

    fs.writeFileSync(filePath, content);
}