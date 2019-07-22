#![allow(non_snake_case)]
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

// Since the RPC_Module.rs code is generated from proto files the rpc call fn
// is implemented here so it does not get overwritten;
impl RPC_Module::Module {
    // Macro?
    pub fn fn_call(&self, fn_name: String, params: Vec<JsValue>) -> Result<JsValue, JsValue>{

        Err(JsValue::from_str("Not Implimented yet!"))
    }
}

// Create Global Module list
lazy_static!{
    static ref GLOBAL_MODULE_LIST: Mutex<Vec::<RPC_Module::Module>>
        = Mutex::new(Vec::<RPC_Module::Module>::new());
    static ref GLOBAL_MODULE_NAMES: Mutex<Vec::<String>> 
        = Mutex::new(Vec::<String>::new());
}

// To get rid of the borrowing and lifetime problems that making this a
// function causes it is a macro so that the get is done in the same scope. 
macro_rules! get_module {
    ($name:expr, $returned_var:ident) => {
        use std::ops::DerefMut;
        let list = &mut GLOBAL_MODULE_LIST.lock().unwrap();
        let list = DerefMut::deref_mut(list);
        let mut loop_num: u32 = 0;

        for item in list{
            if item.get_module_name() == $name {
                break;
            }
            loop_num += 1
        }
        let $returned_var = &GLOBAL_MODULE_LIST.lock().unwrap()[loop_num as usize];
    };
}

#[wasm_bindgen]
pub fn init_file(module_file_name: String) -> Result<(), JsValue> {

    if module_file_name.contains(".wasm") {
        init_pure_wasm(module_file_name)

    } else if module_file_name.contains(".js"){ // check if it's a node module
        Err(JsValue::from_str("No binaries or modules could be found"))
    } else {
        Err(JsValue::from_str("No binaries or modules could be found"))
    }
    
    // attempt to autodetect packager
        // MVP: Go, Rust, C's
    // set init script
        // init the JS
    // set additional boilerplate to call funtions
        //

    // set up functions

    // set any meta data
}

// Return a web assembly instance
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

    // add wasm and how to get to it

    Ok(())
    
}

//Todo
#[wasm_bindgen]
pub fn init_proto(buf: Vec<u8>) -> Result<(), JsValue>{
    // https://github.com/stepancheg/rust-protobuf/pull/118/commits/e501dc72a74fc8939c7696a75961ab2bafad215f
    let mut read_buf = std::io::Cursor::new(buf);
    let result = protobuf::parse_from_reader::<RPC_Module::Module>(&mut read_buf);

    match result {
        Ok(proto) => {
            // Add module
            add_module(proto)?;
            Ok(())
        }
        Err(_) => return Err(JsValue::from_str("Module Protobuf couldn't be read!"))
    }
}

fn add_module(module: RPC_Module::Module ) -> Result <(), &'static str> {
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
