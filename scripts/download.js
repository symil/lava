#!/usr/bin/env node

const fs    = require('fs');
const path  = require('path');
const https = require('https');

const TARGET_DIR        = path.join(__dirname, '..', 'download');
const VK_XML_URL        = `https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/master/xml/vk.xml`;
const VULKAN_CORE_H_URL = `https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/master/include/vulkan/vulkan_core.h`;

async function getFile(url) {
    return new Promise((resolve, reject) => {
        https.get(url, res => {
            const { statusCode } = res;
        
            if (statusCode !== 200) {
                console.log(`GET request to "${url}" failed with status code ${statusCode}`);
                process.exit(1);
            }
        
            res.setEncoding('utf8');

            let fileContent = '';
            res.on('data', chunk => { fileContent += chunk; });
            res.on('end', () => resolve(fileContent));
            res.on('error', reject);
        });
    });
}

async function main() {
    const vkXml = await getFile(VK_XML_URL);
    const vulkanCoreH = await getFile(VULKAN_CORE_H_URL);
    
    if (!fs.existsSync(TARGET_DIR)) {
        fs.mkdirSync(TARGET_DIR);
    }
    
    fs.writeFileSync(path.join(TARGET_DIR, 'vk.xml'), vkXml, 'utf8');
    fs.writeFileSync(path.join(TARGET_DIR, 'vulkan_core.h'), vulkanCoreH, 'utf8');
}

main();