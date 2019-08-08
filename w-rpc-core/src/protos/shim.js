// This shim is here to keep track of Iniated Wasm binaries and wat files
// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/catch.html
// 

export class WasmBinList {
    constructor() {
        this.arr = [];
    }

    // Search for instance/module in arr
    get_instance(){}

    // Add the module to this list
    add_module(){}

    // Do you really need this comment to understand this one buddy...
    remove_module(){}

}