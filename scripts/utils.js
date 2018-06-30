

function cToRustVarName(name) {
    return name
        .replace(/[A-Z]+/g, str => `_${str.toLowerCase()}`)
        .replace(/__/g, '_')
        .replace(/^_/, '')
        .replace(/(\d)_d$/, '_$1d');
}

function capitalizeVarName(name) {
    return name.replace(/[A-Z]/g, '_$&').toUpperCase().substring(1);
}

function cToRustEnumValue(name) {
    return `_${name}`.toLowerCase().replace(/_[0-9a-z]/g, str => str.charAt(1).toUpperCase());
}

function toRawTypeName(name) {
    return `Raw${formatVkTypeName(name)}`;
}

function toTrueTypeName(name) {
    return formatVkTypeName(name);
}

function formatVkTypeName(name) {
    return name
        .replace(/KHR$/, '')
        .replace(/FlagBits$/, 'Flags');
}

module.exports = {
    cToRustVarName,
    capitalizeVarName,
    cToRustEnumValue,
    toRawTypeName,
    toTrueTypeName,
    formatVkTypeName
};