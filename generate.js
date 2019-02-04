#!/usr/bin/env node

const downloadSourceFiles = require('./scripts/download');
const { generateFiles, removeFiles } = require('./scripts/generate');

const HELP = `
Download the necessary files from the Vulkan documentation repository in the \`download\` folder, and then generate the library.

--tag <tag>, -t <tag>   Download the files for the specified branch or tag of the \`Vulkan-Docs\` repository (e.g "v1.1.80").
                        Defaults to "master".
--no-download, -nd      Do not download the files from the Vulkan repository (assume they are already there).
--no-generate, -ng      Do not generate the library.
--delete, -d            Remove the library files before re-generating them.
`.replace(/^--.*  /gm, str => '\033[1m' + str + '\033[0m');

main();

async function main() {
    const argv = process.argv.slice(2);

    if (argv.includes('--help') || argv.includes('-h')) {
        console.log(HELP);
        process.exit();
    }

    if (!argv.includes('--no-download') && !argv.includes('-nd')) {
        const tagOptionIndex = Math.max(argv.indexOf('--tag'), argv.indexOf('-t'));
        const tag = tagOptionIndex !== -1 ? argv[tagOptionIndex + 1] : 'master';

        await downloadSourceFiles(tag);
    }

    if (argv.includes('--delete') || argv.includes('-d')) {
        console.log('Removing current source files...');
        removeFiles();
    }

    if (!argv.includes('--no-generate') && !argv.includes('-ng')) {
        console.log('Generating source files...');
        generateFiles();
    }
}