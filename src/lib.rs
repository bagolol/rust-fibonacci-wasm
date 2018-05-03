#![feature(proc_macro, wasm_import_module, wasm_custom_section)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[no_mangle]
pub fn fib(n: i32) -> i32{
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}


#[test]
fn it_works() {
    assert_eq!(fib(10), 55);
}
