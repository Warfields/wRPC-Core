mod utils;
mod protos;

use wasm_bindgen::prelude::*;
use protos::init;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn initModule(moduleFileName: String) -> init::Module  {
    let module: init::Module = Default::default();
    return module;
}

#[wasm_bindgen]
pub struct InitiatedModule {
    name: String,
    functions: String,
}

impl InitiatedModule {
    pub fn call(functName: String) -> JsValue {
        return JsValue::from("ohea")
    }
}