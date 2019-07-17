#![allow(dead_code)]
mod utils;
mod protos;

use wasm_bindgen::prelude::*;
use protos::RPC_Module;
use js_sys::{Uint8Array, WebAssembly};
use std::io::prelude::*;
use std::fs;

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Create Global Module list
lazy_static!{
    static ref GLOBAL_MODULE_LIST: Mutex<Vec::<RPC_Module::Module>> = Mutex::new(Vec::<RPC_Module::Module>::new());
    static ref GLOBAL_MODULE_NAMES: Mutex<Vec::<String>> = Mutex::new(Vec::<String>::new());
}

#[wasm_bindgen]
pub fn init_module(module_file_name: String) -> Result<(), JsValue> {


    use RPC_Module::Module;
    let mut new_module = Module::new();

    if module_file_name.contains(".wasm"){
        init_pure_wasm(module_file_name, &mut new_module)?;

    } else { // check if it's a node module

        // If it's not a node/JS module then return error
        return Err(JsValue::from_str("No binaries or modules could be found"));
    }
    
    // attempt to autodetect packager
        // MVP: Go, Rust, C's
    // set init script
        // init the JS
    // set additional boilerplate to call funtions
        //

    // set up functions

    // set any meta data

    match add_module_to_list(new_module) {
        Err(reason) => return Err(JsValue::from_str(reason.as_str())),
        Ok(_) => return Ok(()),
    };
}

// Return a web assembly instance
pub fn init_pure_wasm(file_name: String, module: &mut RPC_Module::Module) -> Result<(), JsValue>{
    // get bytes from wasm
    let binary = fs::File::open(file_name);
    let mut binary = match binary {
        Ok(file) => file,
        Err(_) => return Err(JsValue::from_str("Binary file could be found"))
    };

    let mut bytes: Vec<u8> = Vec::new();
    let read_success = binary.read_to_end(bytes.as_mut());

    match read_success {
        Err(_) => return Err(JsValue::from_str("Binary file could read, were the permission set right?")),
        Ok(_) => (),
    };

    // Note that `Uint8Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Uint8Array` to be invalid.
    //
    // As a result, after `Uint8Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.

    let wasm = unsafe {
        let array = Uint8Array::view(bytes.as_slice());
        WebAssembly::Module::new(array.as_ref())?
    };

    // Todo finish packing module param

    Ok(())
}

fn add_module_to_list(new_module: RPC_Module::Module) -> Result <String, String> {
    // TODO Make sure that there can only be one module per name
    let module_names = &GLOBAL_MODULE_NAMES.lock().unwrap().clone();
    for name in module_names {
        if name == new_module.get_module_name() {
            return Err(String::from("Module with that name exists"))
        }
    }

    // Add to global
    &GLOBAL_MODULE_LIST.lock().unwrap().push(new_module.clone());
    &GLOBAL_MODULE_NAMES.lock().unwrap().push(String::from(new_module.get_module_name()));

    Ok(String::from("Good"))
}

fn remove_module_from_list(module_name: String) -> Result <String, String>{
    // TODO Make sure that there can only be one module per name
    let module_names = &GLOBAL_MODULE_NAMES.lock().unwrap().clone();
    let mut exists: bool = false;
    let mut index = 0;
    for name in module_names {
        if *name == module_name {
            exists = true;
            if GLOBAL_MODULE_NAMES.lock().unwrap().remove(index) != module_name{
                panic!("You implimented this wrong!");
            }
        }
        index += 1;
    }

    if !exists {
        return Err(String::from("Module Does not Exist"))
    }

    // This next section uses a for loop to avoid making a copy of every module
    let num_modules = &GLOBAL_MODULE_LIST.lock().unwrap().len();
    for i in 0..*num_modules {
        if &GLOBAL_MODULE_LIST.lock().unwrap()[i].get_module_name() == &&module_name.to_string() {
            &GLOBAL_MODULE_LIST.lock().unwrap().remove(i);
            return Ok(String::from("Good"))
        }
    }

    Err(String::from("Not Good"))
}

//TODO
pub fn get_module(name: String) -> RPC_Module::Module {

    return RPC_Module::Module::new();
}