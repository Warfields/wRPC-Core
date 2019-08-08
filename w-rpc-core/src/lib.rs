#![allow(non_snake_case)]
#![allow(dead_code)]

mod utils;
mod protos;

use wasm_bindgen::prelude::*;
use protos::RPC_Module;
use protobuf::Message;
use js_sys::{Uint8Array, WebAssembly, Object};
use std::io::prelude::*;
use std::fs;
use std::alloc::{GlobalAlloc, Layout, alloc};
use std::ptr::null_mut;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cfg_if;
use std::sync::Mutex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Make bindings to external JS Module for tracking of
// TODO: Refactor to use the JS Module to do all of the pure WASM interactions 
#[wasm_bindgen(module = "src/js/shim.js")]
extern "C" {
    type WasmBinList;

    #[wasm_bindgen(constructor)]
    fn new() -> WasmBinList;

    
    #[wasm_bindgen(method, indexing_setter)]
    fn add_module(this: &WasmBinList, instance: WebAssembly::Instance, name: String);


}

// Create Global Module list
lazy_static!{
    static ref GLOBAL_MODULE_LIST: Mutex<Vec::<RPC_Module::Module>>
        = Mutex::new(Vec::<RPC_Module::Module>::new());

    static ref GLOBAL_MODULE_NAMES: Mutex<Vec::<String>> 
        = Mutex::new(Vec::<String>::new());

    static ref bin_list: WasmBinList
        = WasmBinList::new();
}

// To get rid of the borrowing and lifetime problems that making this a
// function causes it is a macro so that the get is done in the same scope. 
macro_rules! get_module {
    ($name:expr, $returned_var:ident) => {
        use std::ops::DerefMut;
        let list = &mut GLOBAL_MODULE_LIST.lock().unwrap();
        let list = DerefMut::deref_mut(list);
        let mut loop_num: u32 = 0;
        let mut found = false;

        for item in list{
            if item.get_module_name() == $name {
                found = true;
                break;
            }
            loop_num += 1
        }
        if !found {
            return Err(JsValue::from_str("Module Not Found"));
        }

        let $returned_var = &GLOBAL_MODULE_LIST.lock().unwrap()[loop_num as usize];
    };
}

/*
macro_rules! get_module_mut {
    ($name:expr, $returned_var:ident) => {
        use std::ops::DerefMut;
        let list = &mut GLOBAL_MODULE_LIST.lock().unwrap();
        let list = DerefMut::deref_mut(list);
        let mut loop_num: u32 = 0;
        let mut found = false;

        for item in list{
            if item.get_module_name() == $name {
                found = true;
                break;
            }
            loop_num += 1
        }
        if !found {
            return Err(JsValue::from_str("Module Not Found"));
        }

        let $returned_var = &mut GLOBAL_MODULE_LIST.lock().unwrap()[loop_num as usize];
    };
}
*/

// TODO Creates a WRPC Module & Proto based on a read .wasm file
#[wasm_bindgen]
pub fn init_pure_wasm(file_name: String) -> Result<(), JsValue>{
    // get bytes from wasm
    let binary = fs::File::open(&file_name);
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

    // Todo finish packing module params
    create_module(&file_name)?;
    // get new module

    get_module!(file_name, ohea);
    println!("{}", ohea.get_module_name());

    Err(JsValue::from_str("Not implemented!"))

    // add wasm and how to get to it


    
}

// TODO Creates a WRPC Module & Proto based on bytes
fn init_pure_wasm_bytes(bytes: &Vec<u8>) -> Result<WebAssembly::Instance, JsValue>{

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

    let instance = WebAssembly::Instance::new(&wasm, &Object::new())?;
    Ok(instance)
}

//Todo
#[wasm_bindgen]
pub fn init_proto(buf: Vec<u8>) -> Result<(), JsValue>{
    // https://github.com/stepancheg/rust-protobuf/pull/118/commits/e501dc72a74fc8939c7696a75961ab2bafad215f
    let result = protobuf::parse_from_bytes(&buf);

    match result {
        Ok(proto) => {
            // Add module
            add_module(proto)?;
            Ok(())
        }
        Err(_) => return Err(JsValue::from_str("Module Protobuf couldn't be read!"))
    }

}

fn add_module(mut module: RPC_Module::Module ) -> Result <(), &'static str> {
    // Initiate The Module
    module.init()?;

    let name = module.get_module_name().clone();
    let module_names = &GLOBAL_MODULE_NAMES.lock().unwrap().clone();

    for existing_name in module_names {
        if *name == *existing_name {
            return Err("Module with that name exists")
        }
    }

    // Add to global
    &GLOBAL_MODULE_NAMES.lock().unwrap().push(name.to_string().clone());
    &GLOBAL_MODULE_LIST.lock().unwrap().push(module);

    Ok(())
}

fn create_module(name:&String) -> Result <(), &'static str> {
    let module_names = &GLOBAL_MODULE_NAMES.lock().unwrap().clone();
    for existing_name in module_names {
        if *name == *existing_name {
            return Err("Module with that name exists")
        }
    }

    let new_module = RPC_Module::Module::new();

    // Add to global
    &GLOBAL_MODULE_LIST.lock().unwrap().push(new_module);
    &GLOBAL_MODULE_NAMES.lock().unwrap().push((*name).clone());
    Ok(())
}

fn remove_module_from_list<'a>(module_name: String) -> Result <(), &'a str>{
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
        return Err("Module Does not Exist")
    }

    // This next section uses a for loop to avoid making a copy of every module
    let num_modules = &GLOBAL_MODULE_LIST.lock().unwrap().len();
    for i in 0..*num_modules {
        if &GLOBAL_MODULE_LIST.lock().unwrap()[i].get_module_name() == &&module_name.to_string() {
            &GLOBAL_MODULE_LIST.lock().unwrap().remove(i);
            return Ok(());
        }
    }

    Err("Not Good")
}

#[wasm_bindgen]
pub fn rpc_call(module: String, function: String, params: Vec<JsValue>) -> Result<JsValue, JsValue> {
    get_module!(module, callee);
    callee.fn_call(function, params)
}

#[wasm_bindgen]
pub fn find_module_proto(module_name: String) -> Result<Vec<u8>, JsValue> {
    get_module!(module_name, found_module);
    let mut buf : Vec<u8> = Vec::new();
    found_module.write_to_writer(&mut buf)
        .expect("Somehow The proto could not be converted to Message");
    Ok(buf)
}

// Returns True or False on if a module exists
#[wasm_bindgen]
pub fn find_module_bool(name: String) -> bool {
        use std::ops::DerefMut;
        let list = &mut GLOBAL_MODULE_LIST.lock().unwrap();
        let list = DerefMut::deref_mut(list);
        let mut found = false;

        for item in list{
            if item.get_module_name() == name {
                found = true;
                break;
            }
        }
        found
}
