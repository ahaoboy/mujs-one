#![feature(c_variadic, extern_types, label_break_value)]
#![allow(
  dead_code,
  mutable_transmutes,
  non_camel_case_types,
  non_snake_case,
  non_upper_case_globals,
  unused_assignments,
  unused_mut
)]

use std::ffi::{CStr, CString};
use std::ptr::null_mut;
use mujs_one::{
  js_State, js_dostring, js_newcfunction, js_newstate, js_pushundefined, js_setglobal,
  js_tostring, JS_STRICT,
};
unsafe extern "C" fn log(j: *mut js_State) {
    let name = js_tostring(j as *mut js_State, JS_STRICT.try_into().unwrap());
    let s = CStr::from_ptr(name).to_string_lossy();
    println!("{}", s);
    js_pushundefined(j as *mut js_State);
}

fn eval_code(code: &str) -> String {
    unsafe {
        let j = js_newstate(None, null_mut(), JS_STRICT.try_into().unwrap());
        let log_fn: Option<unsafe extern "C" fn(*mut js_State)> = Some(log);
        let log_name = CString::new("log").unwrap();
        js_newcfunction(j as *mut js_State, log_fn, log_name.as_ptr(), 1);
        js_setglobal(j as *mut js_State, log_name.as_ptr());
        let s: CString = CString::new(code).unwrap();
        js_dostring(j, s.as_ptr() as *const i8);
        let p = js_tostring(j, -1);
        let s = CStr::from_ptr(p).to_string_lossy();
        return s.to_string();
    }
}
fn main() {
    let code = "log(1+1)";
    eval_code(code);
}
