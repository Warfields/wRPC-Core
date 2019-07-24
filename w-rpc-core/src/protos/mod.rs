pub mod RPC_Module;
pub mod packager;
pub mod result;
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

// Since the RPC_Module.rs code is generated from proto files the rpc call fn
// is implemented here so it does not get overwritten;
impl RPC_Module::Module {
    pub fn fn_call(&mut self, fn_name: String, params: Vec<JsValue>) -> Result<JsValue, JsValue>{
        // check if function exists
        let mut exists = false;
        let mut funct: &mut RPC_Module::Function = &mut RPC_Module::Function::new();

        for function in self.functions.iter_mut() {
            if function.get_name() == fn_name {
                exists = true;
                funct = function;
                break;
            }
        }
        if !exists {
            return Err(JsValue::from_str("Function not found!"));
        }
        
        // Set parameter values
        if funct.get_parameter().len() != params.len(){
            return Err(JsValue::from_str("Parameter Length Missmatch"));
        }

        for i in 0..funct.get_parameter().len() {
            let param_temp = Uint8Array::from(params.get(i).unwrap().clone());
            let mut param_data: Vec<u8> = Vec::new();

            param_temp.copy_to(&mut param_data);

            funct.mut_parameter().get_mut(i).unwrap().set_data(param_data);
        }

        // Run RPC (Send Proto to Remote module)


        Err(JsValue::from_str("Not Implimented yet!"))
    }
}

