mod utils;
mod protos;

use wasm_bindgen::prelude::*;
use protos::init;
use js_sys::{Function, Object, Reflect, Uint8Array, WebAssembly};

#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

// Create Global Module list
lazy_static!{
    static ref GLOBAL_MODULE_LIST: Mutex<Vec::<init::Module>> = Mutex::new(Vec::<init::Module>::new());
    static ref GLOBAL_MODULE_NAMES: Mutex<Vec::<String>> = Mutex::new(Vec::<String>::new());
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn init_module(module_file_name: String) -> init::Module  {
    use init::Module;
    let mut new_module = Module::new();

    new_module.set_module_name(module_file_name.split(".").collect());
    
    // attempt to autodetect packager
        // MVP: Go, Rust, C's
    // set init script
        // init the JS
    // set additional boilerplate to call funtions
        //

    // set up functions

    // set any meta data


    return new_module;
}

pub fn get_module(name: String) -> init::Module {

    return init::Module::new();
}

fn add_module_to_list(new_module: init::Module) -> Result <String, String> {
    // TODO Make sure that there can only be one module per name
    let mut ohea = &GLOBAL_MODULE_NAMES.lock().unwrap().clone();
    for name in ohea {
        if name == new_module.get_module_name() {
            return Err(String::from("Module with that name exists"))
        }
    }

    // Add to global
    &GLOBAL_MODULE_LIST.lock().unwrap().push(new_module.clone());
    &GLOBAL_MODULE_NAMES.lock().unwrap().push(String::from(new_module.get_module_name()));

    Ok(String::from("Good"))
}

fn remove_module_from_list(module_name: String){

}