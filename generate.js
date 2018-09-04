#!/usr/bin/env node

const downloadSourceFiles = require('./scripts/download');
const { generateFiles, removeFiles } = require('./scripts/generate');

const HELP = `
Download the necessary files from the Vulkan documentation repository in the \`download\` folder,
and generate a Rust wrapper for the Vulkan API.

Available options:

--tag <tag>   : Download the files for the specified branch or tag of the \`Vulkan-Docs\` repository (e.g "v1.1.80").
                Defaults to "master".
--no-download : Do not download the files from the Vulkan repository, assume they are already there.
--no-generate : Do not generate the Rust wrapper.
--remove      : Remove \`src/vk/\` before generating the files
`;

main();

async function main() {
    const argv = process.argv.slice(2);

    if (argv.includes('--help')) {
        console.log(HELP);
        process.exit();
    }

    if (!argv.includes('--no-download') && !argv.includes('-n')) {
        const tagOptionIndex = argv.indexOf('--tag');
        const tag = tagOptionIndex !== -1 ? argv[tagOptionIndex + 1] : 'master';

        await downloadSourceFiles(tag);
    }

    if (argv.includes('--remove')) {
        console.log('Removing current source files...');
        removeFiles();
    }

    if (!argv.includes('--no-generate')) {
        console.log('Generating source files...');
        generateFiles();
    }
}