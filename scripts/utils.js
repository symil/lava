Array.prototype.last = function() {
    return this[this.length - 1];
};

Array.prototype.beforeLast = function() {
    return this[this.length - 2];
};

function toSnakeCase(str) {
    return str
        .replace(/[A-Z0-9]+/g, str => str.charAt(0) + str.substring(1).toLowerCase())
        .split(/(?=[A-Z])/).join('_').toLowerCase()
        .replace(/_+/g, '_');
}

function toPascalCase(str) {
    return str.split('_')
    .map(word => word.charAt(0).toUpperCase() + word.substring(1).toLowerCase()).join('')
    .replace(/\d[a-z](?=\d)/g, str => str[0] + str[1].toUpperCase());
}

module.exports = { toSnakeCase, toPascalCase };