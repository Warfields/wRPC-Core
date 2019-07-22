pub mod RPC_Module;
pub mod packager;
pub mod result;
use wasm_bindgen::prelude::*;

// Since the RPC_Module.rs code is generated from proto files the rpc call fn
// is implemented here so it does not get overwritten;
impl RPC_Module::Module {
    // Macro?
    pub fn fn_call(&self, fn_name: String, params: Vec<JsValue>) -> Result<JsValue, JsValue>{

        Err(JsValue::from_str("Not Implimented yet!"))
    }
}

