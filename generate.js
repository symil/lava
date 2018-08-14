#!/usr/bin/env node

const downloadSourceFiles = require('./scripts/download');
const generateFiles = require('./scripts/generate');

const HELP = `
Download the necessary files from the Vulkan documentation repository in the \`download\` folder,
and generate a Rust wrapper for the Vulkan API.

Available options:

--no-download : Do not download the files from the Vulkan repository, assume they are already there.
--no-generate : Do not generate the Rust wrapper.
`;

main();

async function main() {
    const argv = process.argv.slice(2);

    if (argv.includes('--help')) {
        console.log(HELP);
        process.exit();
    }

    if (!argv.includes('--no-download')) {
        await downloadSourceFiles();
    }

    if (!argv.includes('--no-generate')) {
        console.log('Generating source files...');
        generateFiles();
    }
}