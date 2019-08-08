pub mod RPC_Module;
pub mod packager;
pub mod result;
pub mod request;

use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, WebAssembly};

// Import Functions and classes from the Shim

// Since the RPC_Module.rs code is generated from proto files the rpc call fn
// is implemented here so it does not get overwritten;

impl RPC_Module::Module {
    // Create Global Module List?
    // Refactor everything...
    // Add a u32 as a pointer
    // Associate the type

    pub fn fn_call(&self, fn_name: String, params: Vec<JsValue>) -> Result<JsValue, JsValue>{
        // check if function exists
        let mut exists = false;
        let mut funct: &RPC_Module::Function = &RPC_Module::Function::new();

        for function in self.functions.iter() {
            if function.get_name() == fn_name {
                exists = true;
                funct = function;
                break;
            }
        }
        if !exists {
            return Err(JsValue::from_str("Function not found!"));
        }

        // Generate Request
        let mut request_proto = request::FnRequest::new();
        request_proto.set_params(funct.parameter.clone());

        // Set parameter values
        if funct.get_parameter().len() != params.len(){
            return Err(JsValue::from_str("Parameter Length Missmatch"));
        }

        for i in 0..request_proto.get_params().len() {
            let param_temp = Uint8Array::from(params.get(i).unwrap().clone());
            let mut param_data: Vec<u8> = Vec::new();

            param_temp.copy_to(&mut param_data);
            request_proto.mut_params().get_mut(i).unwrap().set_data(param_data);
        }

        // Run RPC (Send Proto to Remote module)
        if self.pure_wasm {
            // Somehow the parameter objects need to get sent through to the
            // eval environment. JSON?
            js_sys::eval(self.gen_boilerplate(fn_name))
        } else {
            self.wasm_call(fn_name, params)
        }
        // Two options: Stubs or send handling code in intial code?
        // pros / cons...
        // use js-sys::eval()
    }

    fn wasm_call(&self, fn_name: String, params: Vec<JsValue>) -> Result<JsValue, JsValue>{
        Err(JsValue::from_str("Not implimented"))
    }

    fn gen_boilerplate(&self, function: String) -> &'static str{
        "Howdy"
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        if self.pure_wasm {
            // Init Wasm Module
            let result = super::init_pure_wasm_bytes(&self.wasm_binary);
            match result {
                Ok(instance) => {
                    // TODO do shit here
                    // Global list of initiated Wasm Modules that exists purely
                    // in JS
                    Ok(())
                }
                Err(_) => Err("Module Could not be found")
            }
            
        } else {
            match js_sys::eval(self.init_script.as_str()){
                Ok(_) => Ok(()),
                Err(_) => Err("Init script threw an exception")
            }
        }
    }
}

