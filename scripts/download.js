const fs    = require('fs');
const path  = require('path');
const https = require('https');

const TARGET_DIR        = path.join(__dirname, '..', 'download');
const VK_XML_URL        = `https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/{TAG}/xml/vk.xml`;
const VULKAN_CORE_H_URL = `https://raw.githubusercontent.com/KhronosGroup/Vulkan-Docs/{TAG}/include/vulkan/vulkan_core.h`;

async function getFile(url) {
    return new Promise((resolve, reject) => {
        console.log(`GET ${url}`);
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

async function main(tag) {
    const vkXml = await getFile(VK_XML_URL.replace('{TAG}', tag));
    const vulkanCoreH = await getFile(VULKAN_CORE_H_URL.replace('{TAG}', tag));
    
    if (!fs.existsSync(TARGET_DIR)) {
        fs.mkdirSync(TARGET_DIR);
    }
    
    fs.writeFileSync(path.join(TARGET_DIR, 'vk.xml'), vkXml, 'utf8');
    fs.writeFileSync(path.join(TARGET_DIR, 'vulkan_core.h'), vulkanCoreH, 'utf8');
}

module.exports = main;