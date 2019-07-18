//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

// Load and test pure wasm binary
#[wasm_bindgen_test]
fn test_binary(){
    
}

// Load and test wat
// Load and test Enscripten module
// Load and test wasm-pack module
// Load and test go module
