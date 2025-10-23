#![feature(c_variadic, extern_types)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    unused_variables
)]

mod js_array;
mod js_boolean;
mod js_builtin;
mod js_compile;
mod js_date;
mod js_dtoa;
mod js_error;
mod js_function;
mod js_gc;
mod js_intern;
mod js_lex;
mod js_math;
mod js_number;
mod js_object;
mod js_parse;
mod js_property;
mod js_regexp;
mod js_repr;
mod js_run;
mod js_state;
mod js_string;
mod js_value;
mod json;
mod regexp;
mod utf;
mod utf_data;

use compact_str::CompactString;

pub use js_array::*;
pub use js_boolean::*;
pub use js_builtin::*;
pub use js_compile::*;
pub use js_date::*;
pub use js_dtoa::*;
pub use js_error::*;
pub use js_function::*;
pub use js_gc::*;
pub use js_intern::*;
pub use js_lex::*;
pub use js_math::*;
pub use js_number::*;
pub use js_object::*;
pub use js_parse::*;
pub use js_property::*;
pub use js_regexp::*;
pub use js_repr::*;
pub use js_run::*;
pub use js_state::*;
pub use js_string::*;
pub use js_value::*;
// pub use json::*;
pub use regexp::*;
pub use utf::*;
pub use utf_data::*;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn _setjmp(_: *mut __jmp_buf_tag) -> i32;
    fn longjmp(_: *mut __jmp_buf_tag, _: i32) -> !;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> i32;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> i32;
    fn snprintf(_: *mut libc::c_char, _: u32, _: *const libc::c_char, _: ...) -> i32;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: u32,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn putchar(__c: i32) -> i32;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> i32;
    fn fread(
        _: *mut libc::c_void,
        _: u32,
        _: u32,
        _: *mut FILE,
    ) -> u32;
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: i32) -> i32;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn atoi(__nptr: *const libc::c_char) -> i32;
    fn realloc(_: *mut libc::c_void, _: u32) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u32) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u32) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> i32;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: u32) -> i32;
    fn strchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: i32) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> u32;
    fn strerror(_: i32) -> *mut libc::c_char;
    fn acos(_: f64) -> f64;
    fn asin(_: f64) -> f64;
    fn atan(_: f64) -> f64;
    fn atan2(_: f64, _: f64) -> f64;
    fn cos(_: f64) -> f64;
    fn sin(_: f64) -> f64;
    fn tan(_: f64) -> f64;
    fn exp(_: f64) -> f64;
    fn log(_: f64) -> f64;
    fn pow(_: f64, _: f64) -> f64;
    fn sqrt(_: f64) -> f64;
    fn ceil(_: f64) -> f64;
    fn fabs(_: f64) -> f64;
    fn floor(_: f64) -> f64;
    fn fmod(_: f64, _: f64) -> f64;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> i32;
    fn __errno_location() -> *mut i32;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u32; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_State {
    pub actx: *mut libc::c_void,
    pub uctx: *mut libc::c_void,
    pub alloc: js_Alloc,
    pub report: js_Report,
    pub panic: js_Panic,
    pub strings: *mut js_StringNode,
    pub default_strict: i32,
    pub strict: i32,
    pub filename: *const libc::c_char,
    pub source: *const libc::c_char,
    pub line: i32,
    pub lexbuf: C2RustUnnamed_8,
    pub lexline: i32,
    pub lexchar: i32,
    pub lasttoken: i32,
    pub newline: i32,
    pub astdepth: i32,
    pub lookahead: i32,
    pub text: *const libc::c_char,
    pub number: f64,
    pub gcast: *mut js_Ast,
    pub Object_prototype: *mut js_Object,
    pub Array_prototype: *mut js_Object,
    pub Function_prototype: *mut js_Object,
    pub Boolean_prototype: *mut js_Object,
    pub Number_prototype: *mut js_Object,
    pub String_prototype: *mut js_Object,
    pub RegExp_prototype: *mut js_Object,
    pub Date_prototype: *mut js_Object,
    pub Error_prototype: *mut js_Object,
    pub EvalError_prototype: *mut js_Object,
    pub RangeError_prototype: *mut js_Object,
    pub ReferenceError_prototype: *mut js_Object,
    pub SyntaxError_prototype: *mut js_Object,
    pub TypeError_prototype: *mut js_Object,
    pub URIError_prototype: *mut js_Object,
    pub seed: libc::c_uint,
    pub scratch: [libc::c_char; 12],
    pub nextref: i32,
    pub R: *mut js_Object,
    pub G: *mut js_Object,
    pub E: *mut js_Environment,
    pub GE: *mut js_Environment,
    pub top: i32,
    pub bot: i32,
    pub stack: *mut js_Value,
    pub gcmark: i32,
    pub gccounter: libc::c_uint,
    pub gcthresh: libc::c_uint,
    pub gcenv: *mut js_Environment,
    pub gcfun: *mut js_Function,
    pub gcobj: *mut js_Object,
    pub gcstr: *mut js_String,
    pub gcroot: *mut js_Object,
    pub envtop: i32,
    pub envstack: [*mut js_Environment; 1024],
    pub tracetop: i32,
    pub trace: [js_StackTrace; 1024],
    pub trytop: i32,
    pub trybuf: [js_Jumpbuf; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Jumpbuf {
    pub buf: jmp_buf,
    pub E: *mut js_Environment,
    pub envtop: i32,
    pub tracetop: i32,
    pub top: i32,
    pub bot: i32,
    pub strict: i32,
    pub pc: *mut js_Instruction,
}
pub type js_Instruction = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Environment {
    pub outer: *mut js_Environment,
    pub variables: *mut js_Object,
    pub gcnext: *mut js_Environment,
    pub gcmark: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Object {
    pub type_0: js_Class,
    pub extensible: i32,
    pub properties: *mut js_Property,
    pub count: i32,
    pub prototype: *mut js_Object,
    pub u: C2RustUnnamed,
    pub gcnext: *mut js_Object,
    pub gcroot: *mut js_Object,
    pub gcmark: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub boolean: i32,
    pub number: f64,
    pub s: C2RustUnnamed_7,
    pub a: C2RustUnnamed_4,
    pub f: C2RustUnnamed_3,
    pub c: C2RustUnnamed_2,
    pub r: js_Regexp,
    pub iter: C2RustUnnamed_1,
    pub user: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tag: *const libc::c_char,
    pub data: *mut libc::c_void,
    pub has: js_HasProperty,
    pub put: js_Put,
    pub delete: js_Delete,
    pub finalize: js_Finalize,
}
pub type js_Finalize = Option<unsafe extern "C" fn(*mut js_State, *mut libc::c_void) -> ()>;
pub type js_Delete =
    Option<unsafe extern "C" fn(*mut js_State, *mut libc::c_void, *const libc::c_char) -> i32>;
pub type js_Put =
    Option<unsafe extern "C" fn(*mut js_State, *mut libc::c_void, *const libc::c_char) -> i32>;
pub type js_HasProperty =
    Option<unsafe extern "C" fn(*mut js_State, *mut libc::c_void, *const libc::c_char) -> i32>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub target: *mut js_Object,
    pub i: i32,
    pub n: i32,
    pub head: *mut js_Iterator,
    pub current: *mut js_Iterator,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Iterator {
    pub next: *mut js_Iterator,
    pub name: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Regexp {
    pub prog: *mut libc::c_void,
    pub source: *mut libc::c_char,
    pub flags: libc::c_ushort,
    pub last: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub name: *const libc::c_char,
    pub function: js_CFunction,
    pub constructor: js_CFunction,
    pub length: i32,
    pub data: *mut libc::c_void,
    pub finalize: js_Finalize,
}
pub type js_CFunction = Option<unsafe extern "C" fn(&mut js_State) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub function: *mut js_Function,
    pub scope: *mut js_Environment,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Function {
    pub name: *const libc::c_char,
    pub script: i32,
    pub lightweight: i32,
    pub strict: i32,
    pub arguments: i32,
    pub numparams: i32,
    pub code: *mut js_Instruction,
    pub codecap: i32,
    pub codelen: i32,
    pub funtab: *mut *mut js_Function,
    pub funcap: i32,
    pub funlen: i32,
    pub vartab: *mut *const libc::c_char,
    pub varcap: i32,
    pub varlen: i32,
    pub filename: *const libc::c_char,
    pub line: i32,
    pub lastline: i32,
    pub gcnext: *mut js_Function,
    pub gcmark: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub length: i32,
    pub simple: i32,
    pub flat_length: i32,
    pub flat_capacity: i32,
    pub array: *mut js_Value,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union js_Value {
    pub t: C2RustUnnamed_6,
    pub u: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub shrstr: [libc::c_char; 16],
    pub boolean: i32,
    pub number: f64,
    pub litstr: *const libc::c_char,
    pub memstr: *mut js_String,
    pub object: *mut js_Object,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_String {
    pub gcnext: *mut js_String,
    pub gcmark: libc::c_char,
    pub p: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub pad: [libc::c_char; 15],
    pub type_0: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub length: i32,
    pub string: *mut libc::c_char,
    pub shrstr: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Property {
    pub left: *mut js_Property,
    pub right: *mut js_Property,
    pub level: i32,
    pub atts: i32,
    pub value: js_Value,
    pub getter: *mut js_Object,
    pub setter: *mut js_Object,
    pub name: [libc::c_char; 1],
}
pub type js_Class = libc::c_uint;
pub const JS_CUSERDATA: js_Class = 15;
pub const JS_CITERATOR: js_Class = 14;
pub const JS_CARGUMENTS: js_Class = 13;
pub const JS_CJSON: js_Class = 12;
pub const JS_CMATH: js_Class = 11;
pub const JS_CDATE: js_Class = 10;
pub const JS_CREGEXP: js_Class = 9;
pub const JS_CSTRING: js_Class = 8;
pub const JS_CNUMBER: js_Class = 7;
pub const JS_CBOOLEAN: js_Class = 6;
pub const JS_CERROR: js_Class = 5;
pub const JS_CCFUNCTION: js_Class = 4;
pub const JS_CSCRIPT: js_Class = 3;
pub const JS_CFUNCTION: js_Class = 2;
pub const JS_CARRAY: js_Class = 1;
pub const JS_COBJECT: js_Class = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_StackTrace {
    pub name: *const libc::c_char,
    pub file: *const libc::c_char,
    pub line: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Ast {
    pub type_0: js_AstType,
    pub line: i32,
    pub parent: *mut js_Ast,
    pub a: *mut js_Ast,
    pub b: *mut js_Ast,
    pub c: *mut js_Ast,
    pub d: *mut js_Ast,
    pub number: f64,
    pub string: *const libc::c_char,
    pub jumps: *mut js_JumpList,
    pub casejump: i32,
    pub gcnext: *mut js_Ast,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_JumpList {
    pub type_0: js_AstType,
    pub inst: i32,
    pub next: *mut js_JumpList,
}
pub type js_AstType = libc::c_uint;
pub const STM_DEFAULT: js_AstType = 91;
pub const STM_CASE: js_AstType = 90;
pub const STM_LABEL: js_AstType = 89;
pub const STM_DEBUGGER: js_AstType = 88;
pub const STM_TRY: js_AstType = 87;
pub const STM_THROW: js_AstType = 86;
pub const STM_SWITCH: js_AstType = 85;
pub const STM_WITH: js_AstType = 84;
pub const STM_RETURN: js_AstType = 83;
pub const STM_BREAK: js_AstType = 82;
pub const STM_CONTINUE: js_AstType = 81;
pub const STM_FOR_IN_VAR: js_AstType = 80;
pub const STM_FOR_IN: js_AstType = 79;
pub const STM_FOR_VAR: js_AstType = 78;
pub const STM_FOR: js_AstType = 77;
pub const STM_WHILE: js_AstType = 76;
pub const STM_DO: js_AstType = 75;
pub const STM_IF: js_AstType = 74;
pub const STM_VAR: js_AstType = 73;
pub const STM_EMPTY: js_AstType = 72;
pub const STM_BLOCK: js_AstType = 71;
pub const EXP_VAR: js_AstType = 70;
pub const EXP_COMMA: js_AstType = 69;
pub const EXP_ASS_BITOR: js_AstType = 68;
pub const EXP_ASS_BITXOR: js_AstType = 67;
pub const EXP_ASS_BITAND: js_AstType = 66;
pub const EXP_ASS_USHR: js_AstType = 65;
pub const EXP_ASS_SHR: js_AstType = 64;
pub const EXP_ASS_SHL: js_AstType = 63;
pub const EXP_ASS_SUB: js_AstType = 62;
pub const EXP_ASS_ADD: js_AstType = 61;
pub const EXP_ASS_MOD: js_AstType = 60;
pub const EXP_ASS_DIV: js_AstType = 59;
pub const EXP_ASS_MUL: js_AstType = 58;
pub const EXP_ASS: js_AstType = 57;
pub const EXP_COND: js_AstType = 56;
pub const EXP_LOGOR: js_AstType = 55;
pub const EXP_LOGAND: js_AstType = 54;
pub const EXP_BITOR: js_AstType = 53;
pub const EXP_BITXOR: js_AstType = 52;
pub const EXP_BITAND: js_AstType = 51;
pub const EXP_EQ: js_AstType = 50;
pub const EXP_NE: js_AstType = 49;
pub const EXP_STRICTEQ: js_AstType = 48;
pub const EXP_STRICTNE: js_AstType = 47;
pub const EXP_LT: js_AstType = 46;
pub const EXP_GT: js_AstType = 45;
pub const EXP_LE: js_AstType = 44;
pub const EXP_GE: js_AstType = 43;
pub const EXP_INSTANCEOF: js_AstType = 42;
pub const EXP_IN: js_AstType = 41;
pub const EXP_SHL: js_AstType = 40;
pub const EXP_SHR: js_AstType = 39;
pub const EXP_USHR: js_AstType = 38;
pub const EXP_ADD: js_AstType = 37;
pub const EXP_SUB: js_AstType = 36;
pub const EXP_MUL: js_AstType = 35;
pub const EXP_DIV: js_AstType = 34;
pub const EXP_MOD: js_AstType = 33;
pub const EXP_LOGNOT: js_AstType = 32;
pub const EXP_BITNOT: js_AstType = 31;
pub const EXP_NEG: js_AstType = 30;
pub const EXP_POS: js_AstType = 29;
pub const EXP_PREDEC: js_AstType = 28;
pub const EXP_PREINC: js_AstType = 27;
pub const EXP_TYPEOF: js_AstType = 26;
pub const EXP_VOID: js_AstType = 25;
pub const EXP_DELETE: js_AstType = 24;
pub const EXP_POSTDEC: js_AstType = 23;
pub const EXP_POSTINC: js_AstType = 22;
pub const EXP_NEW: js_AstType = 21;
pub const EXP_CALL: js_AstType = 20;
pub const EXP_MEMBER: js_AstType = 19;
pub const EXP_INDEX: js_AstType = 18;
pub const EXP_FUN: js_AstType = 17;
pub const EXP_PROP_SET: js_AstType = 16;
pub const EXP_PROP_GET: js_AstType = 15;
pub const EXP_PROP_VAL: js_AstType = 14;
pub const EXP_OBJECT: js_AstType = 13;
pub const EXP_ARRAY: js_AstType = 12;
pub const EXP_THIS: js_AstType = 11;
pub const EXP_FALSE: js_AstType = 10;
pub const EXP_TRUE: js_AstType = 9;
pub const EXP_NULL: js_AstType = 8;
pub const EXP_ELISION: js_AstType = 7;
pub const EXP_REGEXP: js_AstType = 6;
pub const EXP_STRING: js_AstType = 5;
pub const EXP_NUMBER: js_AstType = 4;
pub const EXP_IDENTIFIER: js_AstType = 3;
pub const AST_IDENTIFIER: js_AstType = 2;
pub const AST_FUNDEC: js_AstType = 1;
pub const AST_LIST: js_AstType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub text: *mut libc::c_char,
    pub len: i32,
    pub cap: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_StringNode {
    pub left: *mut js_StringNode,
    pub right: *mut js_StringNode,
    pub level: i32,
    pub string: [libc::c_char; 1],
}
pub type js_Panic = Option<unsafe extern "C" fn(&mut js_State) -> ()>;
pub type js_Report = Option<unsafe extern "C" fn(&mut js_State, *const libc::c_char) -> ()>;
pub type js_Alloc =
    Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, i32) -> *mut libc::c_void>;
pub const JS_TUNDEFINED: js_Type = 1;
pub const JS_TLITSTR: js_Type = 5;
pub const JS_TOBJECT: js_Type = 7;
pub const JS_TNUMBER: js_Type = 4;
pub const JS_TBOOLEAN: js_Type = 3;
pub type Rune = i32;
pub const Bad: C2RustUnnamed_21 = 65533;
pub const Runemax: C2RustUnnamed_17 = 1114111;
pub const Rune3: C2RustUnnamed_21 = 65535;
pub const Rune4: C2RustUnnamed_21 = 2097151;
pub const Bitx: C2RustUnnamed_21 = 6;
pub const T5: C2RustUnnamed_21 = 248;
pub const Testx: C2RustUnnamed_21 = 192;
pub const Tx: C2RustUnnamed_21 = 128;
pub type uchar = libc::c_uchar;
pub const UTFmax: C2RustUnnamed_17 = 4;
pub const Rune2: C2RustUnnamed_21 = 2047;
pub const T4: C2RustUnnamed_21 = 240;
pub const Rune1: C2RustUnnamed_21 = 127;
pub const T2: C2RustUnnamed_21 = 192;
pub const T3: C2RustUnnamed_21 = 224;
pub const Runeself: C2RustUnnamed_17 = 128;
pub type size_t = u32;
pub const JS_TMEMSTR: js_Type = 6;
pub const JS_TSHRSTR: js_Type = 0;
pub type va_list = __builtin_va_list;
pub const JS_TNULL: js_Type = 2;
pub const JS_READONLY: C2RustUnnamed_12 = 1;
pub const OP_RETURN: js_OpCode = 84;
pub const OP_JFALSE: js_OpCode = 83;
pub const OP_JTRUE: js_OpCode = 82;
pub const OP_JUMP: js_OpCode = 81;
pub const OP_DEBUGGER: js_OpCode = 80;
pub const OP_ENDWITH: js_OpCode = 79;
pub const OP_WITH: js_OpCode = 78;
pub const OP_ENDCATCH: js_OpCode = 77;
pub const OP_CATCH: js_OpCode = 76;
pub const OP_ENDTRY: js_OpCode = 75;
pub const OP_TRY: js_OpCode = 74;
pub const OP_THROW: js_OpCode = 73;
pub const JS_HNUMBER: C2RustUnnamed_14 = 1;
pub const JS_REGEXP_M: C2RustUnnamed_11 = 4;
pub const JS_REGEXP_I: C2RustUnnamed_11 = 2;
pub const JS_REGEXP_G: C2RustUnnamed_11 = 1;
pub const Maskx: C2RustUnnamed_21 = 63;
pub const Runeerror: C2RustUnnamed_17 = 65533;
pub const JS_HSTRING: C2RustUnnamed_14 = 2;
pub const JS_HNONE: C2RustUnnamed_14 = 0;
pub const OP_BITOR: js_OpCode = 71;
pub const OP_BITXOR: js_OpCode = 70;
pub const OP_BITAND: js_OpCode = 69;
pub const OP_JCASE: js_OpCode = 68;
pub const OP_STRICTNE: js_OpCode = 67;
pub const OP_STRICTEQ: js_OpCode = 66;
pub const OP_NE: js_OpCode = 65;
pub const OP_EQ: js_OpCode = 64;
pub const OP_INSTANCEOF: js_OpCode = 72;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct diy_fp_t {
    pub f: u64,
    pub e: i32,
}
pub type uint64_t = __uint64_t;
pub type __uint64_t = u32;
pub type uint32_t = __uint32_t;
pub type __uint32_t = libc::c_uint;
pub type js_Type = libc::c_uint;
pub const OP_GE: js_OpCode = 63;
pub const OP_LE: js_OpCode = 62;
pub const OP_GT: js_OpCode = 61;
pub const OP_LT: js_OpCode = 60;
pub const OP_USHR: js_OpCode = 59;
pub const OP_SHR: js_OpCode = 58;
pub const OP_SHL: js_OpCode = 57;
pub const OP_SUB: js_OpCode = 56;
pub const OP_ADD: js_OpCode = 55;
pub const OP_MOD: js_OpCode = 54;
pub const OP_DIV: js_OpCode = 53;
pub const OP_MUL: js_OpCode = 52;
pub const OP_POSTDEC: js_OpCode = 51;
pub const OP_POSTINC: js_OpCode = 50;
pub const OP_DEC: js_OpCode = 49;
pub const OP_INC: js_OpCode = 48;
pub const OP_LOGNOT: js_OpCode = 47;
pub const OP_BITNOT: js_OpCode = 46;
pub const OP_NEG: js_OpCode = 45;
pub const OP_POS: js_OpCode = 44;
pub const OP_TYPEOF: js_OpCode = 43;
pub const OP_NEW: js_OpCode = 42;
pub const OP_CALL: js_OpCode = 41;
pub const OP_UNDEF: js_OpCode = 13;
pub const OP_POP: js_OpCode = 0;
pub const OP_DUP: js_OpCode = 1;
pub const OP_SETPROP_S: js_OpCode = 35;
pub const OP_ROT3: js_OpCode = 4;
pub const OP_SETPROP: js_OpCode = 34;
pub const OP_ROT4: js_OpCode = 5;
pub const OP_SETVAR: js_OpCode = 24;
pub const OP_SETLOCAL: js_OpCode = 20;
pub const OP_ROT2: js_OpCode = 3;
pub const OP_GETPROP_S: js_OpCode = 33;
pub const OP_GETPROP: js_OpCode = 32;
pub const OP_DUP2: js_OpCode = 2;
pub const OP_GETVAR: js_OpCode = 23;
pub const OP_GETLOCAL: js_OpCode = 19;
pub const OP_IN: js_OpCode = 26;
pub const OP_HASVAR: js_OpCode = 22;
pub const OP_DELPROP_S: js_OpCode = 37;
pub const OP_DELPROP: js_OpCode = 36;
pub const OP_DELVAR: js_OpCode = 25;
pub const OP_DELLOCAL: js_OpCode = 21;
pub const OP_EVAL: js_OpCode = 40;
pub const OP_CLOSURE: js_OpCode = 9;
pub const OP_INITARRAY: js_OpCode = 28;
pub const OP_SKIPARRAY: js_OpCode = 27;
pub const OP_NEWARRAY: js_OpCode = 10;
pub const OP_INITSETTER: js_OpCode = 31;
pub const OP_INITGETTER: js_OpCode = 30;
pub const OP_INITPROP: js_OpCode = 29;
pub const OP_NUMBER: js_OpCode = 7;
pub const OP_INTEGER: js_OpCode = 6;
pub const OP_STRING: js_OpCode = 8;
pub const OP_NEWOBJECT: js_OpCode = 11;
pub const OP_NEWREGEXP: js_OpCode = 12;
pub const OP_THIS: js_OpCode = 17;
pub const OP_FALSE: js_OpCode = 16;
pub const OP_TRUE: js_OpCode = 15;
pub const OP_NULL: js_OpCode = 14;
pub const OP_NEXTITER: js_OpCode = 39;
pub const OP_ITERATOR: js_OpCode = 38;
pub const OP_CURRENT: js_OpCode = 18;
pub const TK_IDENTIFIER: C2RustUnnamed_15 = 256;
pub const TK_BREAK: C2RustUnnamed_15 = 284;
pub const TK_XOR_ASS: C2RustUnnamed_15 = 281;
pub const TK_OR_ASS: C2RustUnnamed_15 = 280;
pub const TK_OR: C2RustUnnamed_15 = 270;
pub const TK_AND_ASS: C2RustUnnamed_15 = 279;
pub const TK_AND: C2RustUnnamed_15 = 269;
pub const TK_MOD_ASS: C2RustUnnamed_15 = 275;
pub const TK_MUL_ASS: C2RustUnnamed_15 = 273;
pub const TK_SUB_ASS: C2RustUnnamed_15 = 272;
pub const TK_DEC: C2RustUnnamed_15 = 283;
pub const TK_ADD_ASS: C2RustUnnamed_15 = 271;
pub const TK_INC: C2RustUnnamed_15 = 282;
pub const TK_NE: C2RustUnnamed_15 = 263;
pub const TK_STRICTNE: C2RustUnnamed_15 = 265;
pub const TK_EQ: C2RustUnnamed_15 = 262;
pub const TK_STRICTEQ: C2RustUnnamed_15 = 264;
pub const TK_GE: C2RustUnnamed_15 = 261;
pub const TK_SHR: C2RustUnnamed_15 = 267;
pub const TK_SHR_ASS: C2RustUnnamed_15 = 277;
pub const TK_USHR: C2RustUnnamed_15 = 268;
pub const TK_USHR_ASS: C2RustUnnamed_15 = 278;
pub const TK_LE: C2RustUnnamed_15 = 260;
pub const TK_SHL: C2RustUnnamed_15 = 266;
pub const TK_SHL_ASS: C2RustUnnamed_15 = 276;
pub const TK_NUMBER: C2RustUnnamed_15 = 257;
pub const TK_STRING: C2RustUnnamed_15 = 258;
pub const TK_DIV_ASS: C2RustUnnamed_15 = 274;
pub const TK_REGEXP: C2RustUnnamed_15 = 259;
pub const TK_TRUE: C2RustUnnamed_15 = 306;
pub const TK_THIS: C2RustUnnamed_15 = 304;
pub const TK_NULL: C2RustUnnamed_15 = 301;
pub const TK_FALSE: C2RustUnnamed_15 = 293;
pub const TK_THROW: C2RustUnnamed_15 = 305;
pub const TK_RETURN: C2RustUnnamed_15 = 302;
pub const TK_CONTINUE: C2RustUnnamed_15 = 287;
pub const TK_FUNCTION: C2RustUnnamed_15 = 296;
pub const TK_NEW: C2RustUnnamed_15 = 300;
pub const TK_TYPEOF: C2RustUnnamed_15 = 308;
pub const TK_VOID: C2RustUnnamed_15 = 310;
pub const TK_DELETE: C2RustUnnamed_15 = 290;
pub const TK_IN: C2RustUnnamed_15 = 298;
pub const TK_INSTANCEOF: C2RustUnnamed_15 = 299;
pub const TK_DEBUGGER: C2RustUnnamed_15 = 288;
pub const TK_DEFAULT: C2RustUnnamed_15 = 289;
pub const TK_CASE: C2RustUnnamed_15 = 285;
pub const TK_FINALLY: C2RustUnnamed_15 = 294;
pub const TK_CATCH: C2RustUnnamed_15 = 286;
pub const TK_TRY: C2RustUnnamed_15 = 307;
pub const TK_SWITCH: C2RustUnnamed_15 = 303;
pub const TK_WITH: C2RustUnnamed_15 = 312;
pub const TK_VAR: C2RustUnnamed_15 = 309;
pub const TK_FOR: C2RustUnnamed_15 = 295;
pub const TK_WHILE: C2RustUnnamed_15 = 311;
pub const TK_DO: C2RustUnnamed_15 = 291;
pub const TK_ELSE: C2RustUnnamed_15 = 292;
pub const TK_IF: C2RustUnnamed_15 = 297;
pub const JS_DONTENUM: C2RustUnnamed_12 = 2;
pub const JS_DONTCONF: C2RustUnnamed_12 = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Reprog {
    pub start: *mut Reinst,
    pub end: *mut Reinst,
    pub cclass: *mut Reclass,
    pub flags: i32,
    pub nsub: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Reclass {
    pub end: *mut Rune,
    pub spans: [Rune; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Reinst {
    pub opcode: libc::c_uchar,
    pub n: libc::c_uchar,
    pub c: Rune,
    pub cc: *mut Reclass,
    pub x: *mut Reinst,
    pub y: *mut Reinst,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cstate {
    pub prog: *mut Reprog,
    pub pstart: *mut Renode,
    pub pend: *mut Renode,
    pub source: *const libc::c_char,
    pub ncclass: i32,
    pub nsub: i32,
    pub sub: [*mut Renode; 16],
    pub lookahead: i32,
    pub yychar: Rune,
    pub yycc: *mut Reclass,
    pub yymin: i32,
    pub yymax: i32,
    pub error: *const libc::c_char,
    pub kaboom: jmp_buf,
    pub cclass: [Reclass; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Renode {
    pub type_0: libc::c_uchar,
    pub ng: libc::c_uchar,
    pub m: libc::c_uchar,
    pub n: libc::c_uchar,
    pub c: Rune,
    pub cc: i32,
    pub x: *mut Renode,
    pub y: *mut Renode,
}
pub const I_END: C2RustUnnamed_20 = 0;
pub const I_RPAR: C2RustUnnamed_20 = 16;
pub const I_REF: C2RustUnnamed_20 = 10;
pub const P_REF: C2RustUnnamed_19 = 14;
pub const I_NCCLASS: C2RustUnnamed_20 = 9;
pub const P_NCCLASS: C2RustUnnamed_19 = 13;
pub const I_CCLASS: C2RustUnnamed_20 = 8;
pub const P_CCLASS: C2RustUnnamed_19 = 12;
pub const REG_ICASE: C2RustUnnamed_16 = 1;
pub const I_CHAR: C2RustUnnamed_20 = 7;
pub const P_CHAR: C2RustUnnamed_19 = 11;
pub const I_ANY: C2RustUnnamed_20 = 6;
pub const P_ANY: C2RustUnnamed_19 = 10;
pub const I_NLA: C2RustUnnamed_20 = 4;
pub const P_NLA: C2RustUnnamed_19 = 9;
pub const I_PLA: C2RustUnnamed_20 = 3;
pub const P_PLA: C2RustUnnamed_19 = 8;
pub const I_LPAR: C2RustUnnamed_20 = 15;
pub const P_PAR: C2RustUnnamed_19 = 7;
pub const I_NWORD: C2RustUnnamed_20 = 14;
pub const P_NWORD: C2RustUnnamed_19 = 6;
pub const I_WORD: C2RustUnnamed_20 = 13;
pub const P_WORD: C2RustUnnamed_19 = 5;
pub const I_EOL: C2RustUnnamed_20 = 12;
pub const P_EOL: C2RustUnnamed_19 = 4;
pub const I_BOL: C2RustUnnamed_20 = 11;
pub const P_BOL: C2RustUnnamed_19 = 3;
pub const I_SPLIT: C2RustUnnamed_20 = 2;
pub const I_JUMP: C2RustUnnamed_20 = 1;
pub const P_REP: C2RustUnnamed_19 = 2;
pub const P_ALT: C2RustUnnamed_19 = 1;
pub const P_CAT: C2RustUnnamed_19 = 0;
pub const I_ANYNL: C2RustUnnamed_20 = 5;
pub const L_CHAR: C2RustUnnamed_18 = 256;
pub const L_NLA: C2RustUnnamed_18 = 261;
pub const L_PLA: C2RustUnnamed_18 = 260;
pub const L_NC: C2RustUnnamed_18 = 259;
pub const L_CCLASS: C2RustUnnamed_18 = 257;
pub const L_NCCLASS: C2RustUnnamed_18 = 258;
pub const L_COUNT: C2RustUnnamed_18 = 265;
pub const L_REF: C2RustUnnamed_18 = 264;
pub const L_NWORD: C2RustUnnamed_18 = 263;
pub const L_WORD: C2RustUnnamed_18 = 262;
pub const REG_NEWLINE: C2RustUnnamed_16 = 2;
pub type js_OpCode = libc::c_uint;
#[derive(Clone)]
#[repr(C)]
pub struct js_Buffer {
    pub s: CompactString,
}

pub type time_t = __time_t;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub sp: *const libc::c_char,
    pub ep: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Resub {
    pub nsub: i32,
    pub sub: [C2RustUnnamed_9; 16],
}
pub const REG_NOTBOL: C2RustUnnamed_16 = 4;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: i32,
    pub _unused2: [libc::c_char; 20],
}
pub type __off64_t = libc::c_long;
pub type _IO_lock_t = ();
pub type __off_t = libc::c_long;
pub const JS_STRICT: C2RustUnnamed_10 = 1;
pub type C2RustUnnamed_10 = libc::c_uint;
pub type C2RustUnnamed_11 = libc::c_uint;
pub type C2RustUnnamed_12 = libc::c_uint;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const JS_ISOBJECT: C2RustUnnamed_13 = 6;
pub const JS_ISFUNCTION: C2RustUnnamed_13 = 5;
pub const JS_ISSTRING: C2RustUnnamed_13 = 4;
pub const JS_ISNUMBER: C2RustUnnamed_13 = 3;
pub const JS_ISBOOLEAN: C2RustUnnamed_13 = 2;
pub const JS_ISNULL: C2RustUnnamed_13 = 1;
pub const JS_ISUNDEFINED: C2RustUnnamed_13 = 0;
pub type C2RustUnnamed_14 = libc::c_uint;
pub type C2RustUnnamed_15 = libc::c_uint;
pub type C2RustUnnamed_16 = libc::c_uint;
pub type C2RustUnnamed_17 = libc::c_uint;
pub const Runesync: C2RustUnnamed_17 = 128;
pub type C2RustUnnamed_18 = libc::c_uint;
pub type C2RustUnnamed_19 = libc::c_uint;
pub type C2RustUnnamed_20 = libc::c_uint;
pub type C2RustUnnamed_21 = libc::c_uint;
pub const T1: C2RustUnnamed_21 = 0;
pub const Bit5: C2RustUnnamed_21 = 2;
pub const Bit4: C2RustUnnamed_21 = 3;
pub const Bit3: C2RustUnnamed_21 = 4;
pub const Bit2: C2RustUnnamed_21 = 5;
pub const Bit1: C2RustUnnamed_21 = 7;
