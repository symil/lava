class HandleList {
    constructor() {
        this._handles = {};
    }

    get(name) {
        return this._handles[name] || (this._handles[name] = new Handle(name));
    }
}

class Handle {
    constructor(name) {
        this._name = name;
        this._parent = null;
        this._toDestroy = [];
        this._methods = [];
    }

    setParent(parent) {
        this._parent = parent;
    }

    addHandleToDestroy(handleName, destroyFunction) {
        this._toDestroy.push({handleName, destroyFunction});
    }

    addMethod(method) {
        this._methods.push(method);
    }
}

exports.HandleList = HandleList;