// This shim is here to keep track of Iniated Wasm binaries and wat files
// https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-js-imports/catch.html
// 

export class WasmBinList {
    constructor() {
        this.arr = [];
    }

    // Search for instance/module in arr
    get_instance(name){
        for (instance in this.arr){
            if (instance[1] == name) {
                return instance[0];
            }
        }
    }

    // Add the module to this list
    add_module(instance, name){
        this.arr.push((instance, name));
    }

    // Do you really need this comment to understand this one buddy...
    remove_module(name){
        for (var i = 0; i < this.arr.length; i++){
            if (this.arr[i][1] == name) {
                this.arr.splice(i, 1);
                return;
            }
        }
        throw "Module not found";
    }

    //Call instance
    call(module_name, fn_name, )

}