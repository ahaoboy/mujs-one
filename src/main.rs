#![feature(c_variadic, extern_types)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use core::ptr::null_mut;
use mujs_one::{
    js_State, js_dostring, js_newcfunction, js_newstate, js_pushundefined, js_setglobal,
    js_tostring,
};
use std::ffi::{CStr, CString};

unsafe extern "C" fn print(j: &mut js_State) {
    let name = js_tostring(j, -1);
    let s = CStr::from_ptr(name).to_string_lossy();
    println!("{s}");
    js_pushundefined(j);
}

const CONSOLE_JS: &CStr = c"var console = { log: print, debug: print, warn: print, error: print };";

fn eval_code(code: &str) -> String {
    unsafe {
        let mut j = js_newstate(None, null_mut(), 0).unwrap();
        let j = j.as_mut();
        let log_fn: Option<unsafe extern "C" fn(&mut js_State)> = Some(print);
        let log_name = c"print";
        js_newcfunction(j, log_fn, log_name.as_ptr(), 1);
        js_setglobal(j, log_name.as_ptr());
        js_dostring(j, CONSOLE_JS.as_ptr() as *const i8);
        let s: CString = CString::new(code).unwrap();
        js_dostring(j, s.as_ptr() as *const i8);
        let p = js_tostring(j, -1);
        let s = CStr::from_ptr(p).to_string_lossy();
        s.to_string()
    }
}
fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let buffer = std::fs::read_to_string(&path).expect("failed to read file");
        eval_code(&buffer);
        return;
    }
    println!("mujs-one <PATH>");
}
