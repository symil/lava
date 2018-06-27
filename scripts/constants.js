const path = require('path');
const fs = require('fs');

const ROOT = path.join(__dirname, '..');
const DST_DIR_NAME = 'vk';
const DST_DIR_PATH = path.join(ROOT, 'src', DST_DIR_NAME);
const VULKAN_SDK_PATH = process.env.VULKAN_SDK;
const VULKAN_H = fs.readFileSync(path.join(VULKAN_SDK_PATH, `include`, `vulkan`, `vulkan_core.h`), 'utf8');

const PRIMITIVE_TYPE = {
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    char: 'i8',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    void: 'void',
    VkBool32: 'u32',
    VkDeviceSize: 'u64',
    VkSampleCountFlags: 'u32',
    VkStructureType: 'VkStructureType'
};

module.exports = {
    ROOT,
    DST_DIR_NAME,
    DST_DIR_PATH,
    VULKAN_H,
    PRIMITIVE_TYPE
};