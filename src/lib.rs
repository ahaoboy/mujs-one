#![feature(c_variadic, extern_types)]
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    static_mut_refs,
    path_statements,
    unused_variables,
    unused_labels,
    clippy::useless_transmute,
    clippy::missing_safety_doc,
    clippy::match_wildcard_for_single_variants,
    clippy::needless_pass_by_ref_mut,
    clippy::unnecessary_cast,
    clippy::unnecessary_mut_passed,
    clippy::no_effect,
    clippy::if_same_then_else,
    clippy::legacy_numeric_constants,
    clippy::while_immutable_condition,
    clippy::unnecessary_unwrap,
    clippy::while_immutable_condition,
    clippy::wildcard_in_or_patterns,
    clippy::comparison_chain,
    clippy::only_used_in_recursion,
    clippy::approx_constant,
    clippy::needless_late_init,
    clippy::nonminimal_bool,
    clippy::unnecessary_cast
)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
    ) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abort() -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn acos(_: libc::c_double) -> libc::c_double;
    fn asin(_: libc::c_double) -> libc::c_double;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn exp(_: libc::c_double) -> libc::c_double;
    fn log(_: libc::c_double) -> libc::c_double;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn time(__timer: *mut time_t) -> time_t;
    fn mktime(__tp: *mut tm) -> time_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
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
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
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
    pub default_strict: libc::c_int,
    pub strict: libc::c_int,
    pub filename: *const libc::c_char,
    pub source: *const libc::c_char,
    pub line: libc::c_int,
    pub lexbuf: C2RustUnnamed_8,
    pub lexline: libc::c_int,
    pub lexchar: libc::c_int,
    pub lasttoken: libc::c_int,
    pub newline: libc::c_int,
    pub astdepth: libc::c_int,
    pub lookahead: libc::c_int,
    pub text: *const libc::c_char,
    pub number: libc::c_double,
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
    pub nextref: libc::c_int,
    pub R: *mut js_Object,
    pub G: *mut js_Object,
    pub E: *mut js_Environment,
    pub GE: *mut js_Environment,
    pub top: libc::c_int,
    pub bot: libc::c_int,
    pub stack: *mut js_Value,
    pub gcmark: libc::c_int,
    pub gccounter: libc::c_uint,
    pub gcthresh: libc::c_uint,
    pub gcenv: *mut js_Environment,
    pub gcfun: *mut js_Function,
    pub gcobj: *mut js_Object,
    pub gcstr: *mut js_String,
    pub gcroot: *mut js_Object,
    pub envtop: libc::c_int,
    pub envstack: [*mut js_Environment; 1024],
    pub tracetop: libc::c_int,
    pub trace: [js_StackTrace; 1024],
    pub trytop: libc::c_int,
    pub trybuf: [js_Jumpbuf; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Jumpbuf {
    pub buf: jmp_buf,
    pub E: *mut js_Environment,
    pub envtop: libc::c_int,
    pub tracetop: libc::c_int,
    pub top: libc::c_int,
    pub bot: libc::c_int,
    pub strict: libc::c_int,
    pub pc: *mut js_Instruction,
}
pub type js_Instruction = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Environment {
    pub outer: *mut js_Environment,
    pub variables: *mut js_Object,
    pub gcnext: *mut js_Environment,
    pub gcmark: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Object {
    pub type_0: js_Class,
    pub extensible: libc::c_int,
    pub properties: *mut js_Property,
    pub count: libc::c_int,
    pub prototype: *mut js_Object,
    pub u: C2RustUnnamed,
    pub gcnext: *mut js_Object,
    pub gcroot: *mut js_Object,
    pub gcmark: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub boolean: libc::c_int,
    pub number: libc::c_double,
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
pub type js_Delete = Option<
    unsafe extern "C" fn(*mut js_State, *mut libc::c_void, *const libc::c_char) -> libc::c_int,
>;
pub type js_Put = Option<
    unsafe extern "C" fn(*mut js_State, *mut libc::c_void, *const libc::c_char) -> libc::c_int,
>;
pub type js_HasProperty = Option<
    unsafe extern "C" fn(*mut js_State, *mut libc::c_void, *const libc::c_char) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub target: *mut js_Object,
    pub i: libc::c_int,
    pub n: libc::c_int,
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
    pub length: libc::c_int,
    pub data: *mut libc::c_void,
    pub finalize: js_Finalize,
}
pub type js_CFunction = Option<unsafe extern "C" fn(*mut js_State) -> ()>;
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
    pub script: libc::c_int,
    pub lightweight: libc::c_int,
    pub strict: libc::c_int,
    pub arguments: libc::c_int,
    pub numparams: libc::c_int,
    pub code: *mut js_Instruction,
    pub codecap: libc::c_int,
    pub codelen: libc::c_int,
    pub funtab: *mut *mut js_Function,
    pub funcap: libc::c_int,
    pub funlen: libc::c_int,
    pub vartab: *mut *const libc::c_char,
    pub varcap: libc::c_int,
    pub varlen: libc::c_int,
    pub filename: *const libc::c_char,
    pub line: libc::c_int,
    pub lastline: libc::c_int,
    pub gcnext: *mut js_Function,
    pub gcmark: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub length: libc::c_int,
    pub simple: libc::c_int,
    pub flat_length: libc::c_int,
    pub flat_capacity: libc::c_int,
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
    pub boolean: libc::c_int,
    pub number: libc::c_double,
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
    pub length: libc::c_int,
    pub string: *mut libc::c_char,
    pub shrstr: [libc::c_char; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Property {
    pub left: *mut js_Property,
    pub right: *mut js_Property,
    pub level: libc::c_int,
    pub atts: libc::c_int,
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
    pub line: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_Ast {
    pub type_0: js_AstType,
    pub line: libc::c_int,
    pub parent: *mut js_Ast,
    pub a: *mut js_Ast,
    pub b: *mut js_Ast,
    pub c: *mut js_Ast,
    pub d: *mut js_Ast,
    pub number: libc::c_double,
    pub string: *const libc::c_char,
    pub jumps: *mut js_JumpList,
    pub casejump: libc::c_int,
    pub gcnext: *mut js_Ast,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_JumpList {
    pub type_0: js_AstType,
    pub inst: libc::c_int,
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
    pub len: libc::c_int,
    pub cap: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct js_StringNode {
    pub left: *mut js_StringNode,
    pub right: *mut js_StringNode,
    pub level: libc::c_int,
    pub string: [libc::c_char; 1],
}
pub type js_Panic = Option<unsafe extern "C" fn(*mut js_State) -> ()>;
pub type js_Report = Option<unsafe extern "C" fn(*mut js_State, *const libc::c_char) -> ()>;
pub type js_Alloc = Option<
    unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, libc::c_int) -> *mut libc::c_void,
>;
pub const JS_TUNDEFINED: js_Type = 1;
pub const JS_TLITSTR: js_Type = 5;
pub const JS_TOBJECT: js_Type = 7;
pub const JS_TNUMBER: js_Type = 4;
pub const JS_TBOOLEAN: js_Type = 3;
pub type Rune = libc::c_int;
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
pub type size_t = libc::c_ulong;
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
    pub f: uint64_t,
    pub e: libc::c_int,
}
pub type uint64_t = __uint64_t;
pub type __uint64_t = libc::c_ulong;
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
    pub flags: libc::c_int,
    pub nsub: libc::c_int,
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
    pub ncclass: libc::c_int,
    pub nsub: libc::c_int,
    pub sub: [*mut Renode; 16],
    pub lookahead: libc::c_int,
    pub yychar: Rune,
    pub yycc: *mut Reclass,
    pub yymin: libc::c_int,
    pub yymax: libc::c_int,
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
    pub cc: libc::c_int,
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
    pub s: Vec<libc::c_int>,
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
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
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
    pub nsub: libc::c_int,
    pub sub: [C2RustUnnamed_9; 16],
}
pub const REG_NOTBOL: C2RustUnnamed_16 = 4;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
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
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
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
    pub _mode: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn js_getlength(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let mut len: libc::c_int = 0;
    js_getproperty(J, idx, b"length\0" as *const u8 as *const libc::c_char);
    len = js_tointeger(J, -(1 as libc::c_int));
    js_pop(J, 1 as libc::c_int);
    len
}
#[no_mangle]
pub unsafe extern "C" fn js_setlength(J: *mut js_State, idx: libc::c_int, len: libc::c_int) {
    js_pushnumber(J, len as libc::c_double);
    js_setproperty(
        J,
        if idx < 0 as libc::c_int {
            idx - 1 as libc::c_int
        } else {
            idx
        },
        b"length\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn jsB_new_Array(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    js_newarray(J);
    if top == 2 as libc::c_int {
        if js_isnumber(J, 1 as libc::c_int) != 0 {
            js_copy(J, 1 as libc::c_int);
            js_setproperty(
                J,
                -(2 as libc::c_int),
                b"length\0" as *const u8 as *const libc::c_char,
            );
        } else {
            js_copy(J, 1 as libc::c_int);
            js_setindex(J, -(2 as libc::c_int), 0 as libc::c_int);
        }
    } else {
        i = 1 as libc::c_int;
        while i < top {
            js_copy(J, i);
            js_setindex(J, -(2 as libc::c_int), i - 1 as libc::c_int);
            i += 1;
            i;
        }
    };
}
unsafe extern "C" fn Ap_concat(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    js_newarray(J);
    n = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < top {
        js_copy(J, i);
        if js_isarray(J, -(1 as libc::c_int)) != 0 {
            len = js_getlength(J, -(1 as libc::c_int));
            k = 0 as libc::c_int;
            while k < len {
                if js_hasindex(J, -(1 as libc::c_int), k) != 0 {
                    let fresh0 = n;
                    n += 1;
                    js_setindex(J, -(3 as libc::c_int), fresh0);
                }
                k += 1;
                k;
            }
            js_pop(J, 1 as libc::c_int);
        } else {
            let fresh1 = n;
            n += 1;
            js_setindex(J, -(2 as libc::c_int), fresh1);
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn Ap_join(J: *mut js_State) {
    let mut out: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut r: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut sep: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut seplen: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut rlen: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    if js_isdefined(J, 1 as libc::c_int) != 0 {
        sep = js_tostring(J, 1 as libc::c_int);
        seplen = strlen(sep) as libc::c_int;
    } else {
        sep = b",\0" as *const u8 as *const libc::c_char;
        seplen = 1 as libc::c_int;
    }
    if len <= 0 as libc::c_int {
        js_pushliteral(J, b"\0" as *const u8 as *const libc::c_char);
        return;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, out as *mut libc::c_void);
        js_throw(J);
    }
    n = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while k < len {
        js_getindex(J, 0 as libc::c_int, k);
        if js_iscoercible(J, -(1 as libc::c_int)) != 0 {
            ::core::ptr::write_volatile(
                &mut r as *mut *const libc::c_char,
                js_tostring(J, -(1 as libc::c_int)),
            );
            rlen = strlen(r) as libc::c_int;
        } else {
            rlen = 0 as libc::c_int;
        }
        if k == 0 as libc::c_int {
            ::core::ptr::write_volatile(
                &mut out as *mut *mut libc::c_char,
                js_malloc(J, rlen + 1 as libc::c_int) as *mut libc::c_char,
            );
            if rlen > 0 as libc::c_int {
                memcpy(
                    out as *mut libc::c_void,
                    r as *const libc::c_void,
                    rlen as libc::c_ulong,
                );
                n += rlen;
            }
        } else {
            if n + seplen + rlen > (1 as libc::c_int) << 28 as libc::c_int {
                js_rangeerror(
                    J,
                    b"invalid string length\0" as *const u8 as *const libc::c_char,
                );
            }
            ::core::ptr::write_volatile(
                &mut out as *mut *mut libc::c_char,
                js_realloc(
                    J,
                    out as *mut libc::c_void,
                    n + seplen + rlen + 1 as libc::c_int,
                ) as *mut libc::c_char,
            );
            if seplen > 0 as libc::c_int {
                memcpy(
                    out.offset(n as isize) as *mut libc::c_void,
                    sep as *const libc::c_void,
                    seplen as libc::c_ulong,
                );
                n += seplen;
            }
            if rlen > 0 as libc::c_int {
                memcpy(
                    out.offset(n as isize) as *mut libc::c_void,
                    r as *const libc::c_void,
                    rlen as libc::c_ulong,
                );
                n += rlen;
            }
        }
        js_pop(J, 1 as libc::c_int);
        k += 1;
        k;
    }
    js_pushlstring(J, out, n);
    js_endtry(J);
    js_free(J, out as *mut libc::c_void);
}
unsafe extern "C" fn Ap_pop(J: *mut js_State) {
    let mut n: libc::c_int = 0;
    n = js_getlength(J, 0 as libc::c_int);
    if n > 0 as libc::c_int {
        js_getindex(J, 0 as libc::c_int, n - 1 as libc::c_int);
        js_delindex(J, 0 as libc::c_int, n - 1 as libc::c_int);
        js_setlength(J, 0 as libc::c_int, n - 1 as libc::c_int);
    } else {
        js_setlength(J, 0 as libc::c_int, 0 as libc::c_int);
        js_pushundefined(J);
    };
}
unsafe extern "C" fn Ap_push(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    let mut n: libc::c_int = 0;
    n = js_getlength(J, 0 as libc::c_int);
    i = 1 as libc::c_int;
    while i < top {
        js_copy(J, i);
        js_setindex(J, 0 as libc::c_int, n);
        i += 1;
        i;
        n += 1;
        n;
    }
    js_setlength(J, 0 as libc::c_int, n);
    js_pushnumber(J, n as libc::c_double);
}
unsafe extern "C" fn Ap_reverse(J: *mut js_State) {
    let mut len: libc::c_int = 0;
    let mut middle: libc::c_int = 0;
    let mut lower: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    middle = len / 2 as libc::c_int;
    lower = 0 as libc::c_int;
    while lower != middle {
        let upper: libc::c_int = len - lower - 1 as libc::c_int;
        let haslower: libc::c_int = js_hasindex(J, 0 as libc::c_int, lower);
        let hasupper: libc::c_int = js_hasindex(J, 0 as libc::c_int, upper);
        if haslower != 0 && hasupper != 0 {
            js_setindex(J, 0 as libc::c_int, lower);
            js_setindex(J, 0 as libc::c_int, upper);
        } else if hasupper != 0 {
            js_setindex(J, 0 as libc::c_int, lower);
            js_delindex(J, 0 as libc::c_int, upper);
        } else if haslower != 0 {
            js_setindex(J, 0 as libc::c_int, upper);
            js_delindex(J, 0 as libc::c_int, lower);
        }
        lower += 1;
        lower;
    }
    js_copy(J, 0 as libc::c_int);
}
unsafe extern "C" fn Ap_shift(J: *mut js_State) {
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    if len == 0 as libc::c_int {
        js_setlength(J, 0 as libc::c_int, 0 as libc::c_int);
        js_pushundefined(J);
        return;
    }
    js_getindex(J, 0 as libc::c_int, 0 as libc::c_int);
    k = 1 as libc::c_int;
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_setindex(J, 0 as libc::c_int, k - 1 as libc::c_int);
        } else {
            js_delindex(J, 0 as libc::c_int, k - 1 as libc::c_int);
        }
        k += 1;
        k;
    }
    js_delindex(J, 0 as libc::c_int, len - 1 as libc::c_int);
    js_setlength(J, 0 as libc::c_int, len - 1 as libc::c_int);
}
unsafe extern "C" fn Ap_slice(J: *mut js_State) {
    let mut len: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sv: libc::c_double = 0.;
    let mut ev: libc::c_double = 0.;
    js_newarray(J);
    len = js_getlength(J, 0 as libc::c_int);
    sv = js_tointeger(J, 1 as libc::c_int) as libc::c_double;
    ev = (if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        len
    }) as libc::c_double;
    if sv < 0 as libc::c_int as libc::c_double {
        sv += len as libc::c_double;
    }
    if ev < 0 as libc::c_int as libc::c_double {
        ev += len as libc::c_double;
    }
    s = (if sv < 0 as libc::c_int as libc::c_double {
        0 as libc::c_int as libc::c_double
    } else if sv > len as libc::c_double {
        len as libc::c_double
    } else {
        sv
    }) as libc::c_int;
    e = (if ev < 0 as libc::c_int as libc::c_double {
        0 as libc::c_int as libc::c_double
    } else if ev > len as libc::c_double {
        len as libc::c_double
    } else {
        ev
    }) as libc::c_int;
    n = 0 as libc::c_int;
    while s < e {
        if js_hasindex(J, 0 as libc::c_int, s) != 0 {
            js_setindex(J, -(2 as libc::c_int), n);
        }
        s += 1;
        s;
        n += 1;
        n;
    }
}
unsafe extern "C" fn Ap_sort_cmp(
    J: *mut js_State,
    idx_a: libc::c_int,
    idx_b: libc::c_int,
) -> libc::c_int {
    let obj: *mut js_Object = (*js_tovalue(J, 0 as libc::c_int)).u.object;
    if (*obj).u.a.simple != 0 {
        let val_a: *mut js_Value = &mut *((*obj).u.a.array).offset(idx_a as isize) as *mut js_Value;
        let val_b: *mut js_Value = &mut *((*obj).u.a.array).offset(idx_b as isize) as *mut js_Value;
        let und_a: libc::c_int =
            ((*val_a).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int) as libc::c_int;
        let und_b: libc::c_int =
            ((*val_b).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int) as libc::c_int;
        if und_a != 0 {
            return und_b;
        }
        if und_b != 0 {
            return -(1 as libc::c_int);
        }
        if js_iscallable(J, 1 as libc::c_int) != 0 {
            let mut v: libc::c_double = 0.;
            js_copy(J, 1 as libc::c_int);
            js_pushundefined(J);
            js_pushvalue(J, *val_a);
            js_pushvalue(J, *val_b);
            js_call(J, 2 as libc::c_int);
            v = js_tonumber(J, -(1 as libc::c_int));
            js_pop(J, 1 as libc::c_int);
            if v.is_nan() as i32 != 0 {
                return 0 as libc::c_int;
            }
            if v == 0 as libc::c_int as libc::c_double {
                return 0 as libc::c_int;
            }
            if v < 0 as libc::c_int as libc::c_double {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }
        } else {
            let mut str_a: *const libc::c_char = std::ptr::null::<libc::c_char>();
            let mut str_b: *const libc::c_char = std::ptr::null::<libc::c_char>();
            let mut c: libc::c_int = 0;
            js_pushvalue(J, *val_a);
            js_pushvalue(J, *val_b);
            str_a = js_tostring(J, -(2 as libc::c_int));
            str_b = js_tostring(J, -(1 as libc::c_int));
            c = strcmp(str_a, str_b);
            js_pop(J, 2 as libc::c_int);
            c
        }
    } else {
        let mut und_a_0: libc::c_int = 0;
        let mut und_b_0: libc::c_int = 0;
        let has_a: libc::c_int = js_hasindex(J, 0 as libc::c_int, idx_a);
        let has_b: libc::c_int = js_hasindex(J, 0 as libc::c_int, idx_b);
        if has_a == 0 && has_b == 0 {
            return 0 as libc::c_int;
        }
        if has_a != 0 && has_b == 0 {
            js_pop(J, 1 as libc::c_int);
            return -(1 as libc::c_int);
        }
        if has_a == 0 && has_b != 0 {
            js_pop(J, 1 as libc::c_int);
            return 1 as libc::c_int;
        }
        und_a_0 = js_isundefined(J, -(2 as libc::c_int));
        und_b_0 = js_isundefined(J, -(1 as libc::c_int));
        if und_a_0 != 0 {
            js_pop(J, 2 as libc::c_int);
            return und_b_0;
        }
        if und_b_0 != 0 {
            js_pop(J, 2 as libc::c_int);
            return -(1 as libc::c_int);
        }
        if js_iscallable(J, 1 as libc::c_int) != 0 {
            let mut v_0: libc::c_double = 0.;
            js_copy(J, 1 as libc::c_int);
            js_pushundefined(J);
            js_copy(J, -(4 as libc::c_int));
            js_copy(J, -(4 as libc::c_int));
            js_call(J, 2 as libc::c_int);
            v_0 = js_tonumber(J, -(1 as libc::c_int));
            js_pop(J, 3 as libc::c_int);
            if v_0.is_nan() as i32 != 0 {
                return 0 as libc::c_int;
            }
            if v_0 == 0 as libc::c_int as libc::c_double {
                return 0 as libc::c_int;
            }
            if v_0 < 0 as libc::c_int as libc::c_double {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }
        } else {
            let str_a_0: *const libc::c_char = js_tostring(J, -(2 as libc::c_int));
            let str_b_0: *const libc::c_char = js_tostring(J, -(1 as libc::c_int));
            let c_0: libc::c_int = strcmp(str_a_0, str_b_0);
            js_pop(J, 2 as libc::c_int);
            c_0
        }
    }
}
unsafe extern "C" fn Ap_sort_swap(J: *mut js_State, idx_a: libc::c_int, idx_b: libc::c_int) {
    let obj: *mut js_Object = (*js_tovalue(J, 0 as libc::c_int)).u.object;
    if (*obj).u.a.simple != 0 {
        let tmp: js_Value = *((*obj).u.a.array).offset(idx_a as isize);
        *((*obj).u.a.array).offset(idx_a as isize) = *((*obj).u.a.array).offset(idx_b as isize);
        *((*obj).u.a.array).offset(idx_b as isize) = tmp;
    } else {
        let has_a: libc::c_int = js_hasindex(J, 0 as libc::c_int, idx_a);
        let has_b: libc::c_int = js_hasindex(J, 0 as libc::c_int, idx_b);
        if has_a != 0 && has_b != 0 {
            js_setindex(J, 0 as libc::c_int, idx_a);
            js_setindex(J, 0 as libc::c_int, idx_b);
        } else if has_a != 0 && has_b == 0 {
            js_delindex(J, 0 as libc::c_int, idx_a);
            js_setindex(J, 0 as libc::c_int, idx_b);
        } else if has_a == 0 && has_b != 0 {
            js_delindex(J, 0 as libc::c_int, idx_b);
            js_setindex(J, 0 as libc::c_int, idx_a);
        }
    };
}
unsafe extern "C" fn Ap_sort_leaf(
    J: *mut js_State,
    i: libc::c_int,
    end: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = i;
    let mut lc: libc::c_int = (j << 1 as libc::c_int) + 1 as libc::c_int;
    let mut rc: libc::c_int = (j << 1 as libc::c_int) + 2 as libc::c_int;
    while rc < end {
        if Ap_sort_cmp(J, rc, lc) > 0 as libc::c_int {
            j = rc;
        } else {
            j = lc;
        }
        lc = (j << 1 as libc::c_int) + 1 as libc::c_int;
        rc = (j << 1 as libc::c_int) + 2 as libc::c_int;
    }
    if lc < end {
        j = lc;
    }
    j
}
unsafe extern "C" fn Ap_sort_sift(J: *mut js_State, i: libc::c_int, end: libc::c_int) {
    let mut j: libc::c_int = Ap_sort_leaf(J, i, end);
    while Ap_sort_cmp(J, i, j) > 0 as libc::c_int {
        j = (j - 1 as libc::c_int) >> 1 as libc::c_int;
    }
    while j > i {
        Ap_sort_swap(J, i, j);
        j = (j - 1 as libc::c_int) >> 1 as libc::c_int;
    }
}
unsafe extern "C" fn Ap_sort_heapsort(J: *mut js_State, n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = n / 2 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        Ap_sort_sift(J, i, n);
        i -= 1;
        i;
    }
    i = n - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        Ap_sort_swap(J, 0 as libc::c_int, i);
        Ap_sort_sift(J, 0 as libc::c_int, i);
        i -= 1;
        i;
    }
}
unsafe extern "C" fn Ap_sort(J: *mut js_State) {
    let mut len: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    if len <= 1 as libc::c_int {
        js_copy(J, 0 as libc::c_int);
        return;
    }
    if js_iscallable(J, 1 as libc::c_int) == 0 && js_isundefined(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"comparison function must be a function or undefined\0" as *const u8
                as *const libc::c_char,
        );
    }
    if len >= 2147483647 as libc::c_int {
        js_rangeerror(
            J,
            b"array is too large to sort\0" as *const u8 as *const libc::c_char,
        );
    }
    Ap_sort_heapsort(J, len);
    js_copy(J, 0 as libc::c_int);
}
unsafe extern "C" fn Ap_splice(J: *mut js_State) {
    let top: libc::c_int = js_gettop(J);
    let mut len: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut del: libc::c_int = 0;
    let mut add: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    start = js_tointeger(J, 1 as libc::c_int);
    if start < 0 as libc::c_int {
        start = if len + start > 0 as libc::c_int {
            len + start
        } else {
            0 as libc::c_int
        };
    } else if start > len {
        start = len;
    }
    if js_isdefined(J, 2 as libc::c_int) != 0 {
        del = js_tointeger(J, 2 as libc::c_int);
    } else {
        del = len - start;
    }
    if del > len - start {
        del = len - start;
    }
    if del < 0 as libc::c_int {
        del = 0 as libc::c_int;
    }
    js_newarray(J);
    k = 0 as libc::c_int;
    while k < del {
        if js_hasindex(J, 0 as libc::c_int, start + k) != 0 {
            js_setindex(J, -(2 as libc::c_int), k);
        }
        k += 1;
        k;
    }
    js_setlength(J, -(1 as libc::c_int), del);
    add = top - 3 as libc::c_int;
    if add < del {
        k = start;
        while k < len - del {
            if js_hasindex(J, 0 as libc::c_int, k + del) != 0 {
                js_setindex(J, 0 as libc::c_int, k + add);
            } else {
                js_delindex(J, 0 as libc::c_int, k + add);
            }
            k += 1;
            k;
        }
        k = len;
        while k > len - del + add {
            js_delindex(J, 0 as libc::c_int, k - 1 as libc::c_int);
            k -= 1;
            k;
        }
    } else if add > del {
        k = len - del;
        while k > start {
            if js_hasindex(J, 0 as libc::c_int, k + del - 1 as libc::c_int) != 0 {
                js_setindex(J, 0 as libc::c_int, k + add - 1 as libc::c_int);
            } else {
                js_delindex(J, 0 as libc::c_int, k + add - 1 as libc::c_int);
            }
            k -= 1;
            k;
        }
    }
    k = 0 as libc::c_int;
    while k < add {
        js_copy(J, 3 as libc::c_int + k);
        js_setindex(J, 0 as libc::c_int, start + k);
        k += 1;
        k;
    }
    js_setlength(J, 0 as libc::c_int, len - del + add);
}
unsafe extern "C" fn Ap_unshift(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    k = len;
    while k > 0 as libc::c_int {
        let from: libc::c_int = k - 1 as libc::c_int;
        let to: libc::c_int = k + top - 2 as libc::c_int;
        if js_hasindex(J, 0 as libc::c_int, from) != 0 {
            js_setindex(J, 0 as libc::c_int, to);
        } else {
            js_delindex(J, 0 as libc::c_int, to);
        }
        k -= 1;
        k;
    }
    i = 1 as libc::c_int;
    while i < top {
        js_copy(J, i);
        js_setindex(J, 0 as libc::c_int, i - 1 as libc::c_int);
        i += 1;
        i;
    }
    js_setlength(J, 0 as libc::c_int, len + top - 1 as libc::c_int);
    js_pushnumber(J, (len + top - 1 as libc::c_int) as libc::c_double);
}
unsafe extern "C" fn Ap_toString(J: *mut js_State) {
    if js_iscoercible(J, 0 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"'this' is not an object\0" as *const u8 as *const libc::c_char,
        );
    }
    js_getproperty(
        J,
        0 as libc::c_int,
        b"join\0" as *const u8 as *const libc::c_char,
    );
    if js_iscallable(J, -(1 as libc::c_int)) == 0 {
        js_pop(J, 1 as libc::c_int);
        js_getglobal(J, b"Object\0" as *const u8 as *const libc::c_char);
        js_getproperty(
            J,
            -(1 as libc::c_int),
            b"prototype\0" as *const u8 as *const libc::c_char,
        );
        js_rot2pop1(J);
        js_getproperty(
            J,
            -(1 as libc::c_int),
            b"toString\0" as *const u8 as *const libc::c_char,
        );
        js_rot2pop1(J);
    }
    js_copy(J, 0 as libc::c_int);
    js_call(J, 0 as libc::c_int);
}
unsafe extern "C" fn Ap_indexOf(J: *mut js_State) {
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut from: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    from = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    if from < 0 as libc::c_int {
        from += len;
    }
    if from < 0 as libc::c_int {
        from = 0 as libc::c_int;
    }
    js_copy(J, 1 as libc::c_int);
    k = from;
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            if js_strictequal(J) != 0 {
                js_pushnumber(J, k as libc::c_double);
                return;
            }
            js_pop(J, 1 as libc::c_int);
        }
        k += 1;
        k;
    }
    js_pushnumber(J, -(1 as libc::c_int) as libc::c_double);
}
unsafe extern "C" fn Ap_lastIndexOf(J: *mut js_State) {
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut from: libc::c_int = 0;
    len = js_getlength(J, 0 as libc::c_int);
    from = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        len - 1 as libc::c_int
    };
    if from > len - 1 as libc::c_int {
        from = len - 1 as libc::c_int;
    }
    if from < 0 as libc::c_int {
        from += len;
    }
    js_copy(J, 1 as libc::c_int);
    k = from;
    while k >= 0 as libc::c_int {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            if js_strictequal(J) != 0 {
                js_pushnumber(J, k as libc::c_double);
                return;
            }
            js_pop(J, 1 as libc::c_int);
        }
        k -= 1;
        k;
    }
    js_pushnumber(J, -(1 as libc::c_int) as libc::c_double);
}
unsafe extern "C" fn Ap_every(J: *mut js_State) {
    let hasthis: libc::c_int = (js_gettop(J) >= 3 as libc::c_int) as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if js_iscallable(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0 as libc::c_int);
    k = 0 as libc::c_int;
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_copy(J, 1 as libc::c_int);
            if hasthis != 0 {
                js_copy(J, 2 as libc::c_int);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as libc::c_int));
            js_pushnumber(J, k as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 3 as libc::c_int);
            if js_toboolean(J, -(1 as libc::c_int)) == 0 {
                return;
            }
            js_pop(J, 2 as libc::c_int);
        }
        k += 1;
        k;
    }
    js_pushboolean(J, 1 as libc::c_int);
}
unsafe extern "C" fn Ap_some(J: *mut js_State) {
    let hasthis: libc::c_int = (js_gettop(J) >= 3 as libc::c_int) as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if js_iscallable(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0 as libc::c_int);
    k = 0 as libc::c_int;
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_copy(J, 1 as libc::c_int);
            if hasthis != 0 {
                js_copy(J, 2 as libc::c_int);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as libc::c_int));
            js_pushnumber(J, k as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 3 as libc::c_int);
            if js_toboolean(J, -(1 as libc::c_int)) != 0 {
                return;
            }
            js_pop(J, 2 as libc::c_int);
        }
        k += 1;
        k;
    }
    js_pushboolean(J, 0 as libc::c_int);
}
unsafe extern "C" fn Ap_forEach(J: *mut js_State) {
    let hasthis: libc::c_int = (js_gettop(J) >= 3 as libc::c_int) as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if js_iscallable(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0 as libc::c_int);
    k = 0 as libc::c_int;
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_copy(J, 1 as libc::c_int);
            if hasthis != 0 {
                js_copy(J, 2 as libc::c_int);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as libc::c_int));
            js_pushnumber(J, k as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 3 as libc::c_int);
            js_pop(J, 2 as libc::c_int);
        }
        k += 1;
        k;
    }
    js_pushundefined(J);
}
unsafe extern "C" fn Ap_map(J: *mut js_State) {
    let hasthis: libc::c_int = (js_gettop(J) >= 3 as libc::c_int) as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if js_iscallable(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    js_newarray(J);
    len = js_getlength(J, 0 as libc::c_int);
    k = 0 as libc::c_int;
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_copy(J, 1 as libc::c_int);
            if hasthis != 0 {
                js_copy(J, 2 as libc::c_int);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as libc::c_int));
            js_pushnumber(J, k as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 3 as libc::c_int);
            js_setindex(J, -(3 as libc::c_int), k);
            js_pop(J, 1 as libc::c_int);
        }
        k += 1;
        k;
    }
    js_setlength(J, -(1 as libc::c_int), len);
}
unsafe extern "C" fn Ap_filter(J: *mut js_State) {
    let hasthis: libc::c_int = (js_gettop(J) >= 3 as libc::c_int) as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut to: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if js_iscallable(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    js_newarray(J);
    to = 0 as libc::c_int;
    len = js_getlength(J, 0 as libc::c_int);
    k = 0 as libc::c_int;
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_copy(J, 1 as libc::c_int);
            if hasthis != 0 {
                js_copy(J, 2 as libc::c_int);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as libc::c_int));
            js_pushnumber(J, k as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 3 as libc::c_int);
            if js_toboolean(J, -(1 as libc::c_int)) != 0 {
                js_pop(J, 1 as libc::c_int);
                let fresh2 = to;
                to += 1;
                js_setindex(J, -(2 as libc::c_int), fresh2);
            } else {
                js_pop(J, 2 as libc::c_int);
            }
        }
        k += 1;
        k;
    }
}
unsafe extern "C" fn Ap_reduce(J: *mut js_State) {
    let hasinitial: libc::c_int = (js_gettop(J) >= 3 as libc::c_int) as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if js_iscallable(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0 as libc::c_int);
    k = 0 as libc::c_int;
    if len == 0 as libc::c_int && hasinitial == 0 {
        js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
    }
    if hasinitial != 0 {
        js_copy(J, 2 as libc::c_int);
    } else {
        while k < len {
            let fresh3 = k;
            k += 1;
            if js_hasindex(J, 0 as libc::c_int, fresh3) != 0 {
                break;
            }
        }
        if k == len {
            js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
        }
    }
    while k < len {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_copy(J, 1 as libc::c_int);
            js_pushundefined(J);
            js_rot(J, 4 as libc::c_int);
            js_rot(J, 4 as libc::c_int);
            js_pushnumber(J, k as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 4 as libc::c_int);
        }
        k += 1;
        k;
    }
}
unsafe extern "C" fn Ap_reduceRight(J: *mut js_State) {
    let hasinitial: libc::c_int = (js_gettop(J) >= 3 as libc::c_int) as libc::c_int;
    let mut k: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if js_iscallable(J, 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0 as libc::c_int);
    k = len - 1 as libc::c_int;
    if len == 0 as libc::c_int && hasinitial == 0 {
        js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
    }
    if hasinitial != 0 {
        js_copy(J, 2 as libc::c_int);
    } else {
        while k >= 0 as libc::c_int {
            let fresh4 = k;
            k -= 1;
            if js_hasindex(J, 0 as libc::c_int, fresh4) != 0 {
                break;
            }
        }
        if k < 0 as libc::c_int {
            js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
        }
    }
    while k >= 0 as libc::c_int {
        if js_hasindex(J, 0 as libc::c_int, k) != 0 {
            js_copy(J, 1 as libc::c_int);
            js_pushundefined(J);
            js_rot(J, 4 as libc::c_int);
            js_rot(J, 4 as libc::c_int);
            js_pushnumber(J, k as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 4 as libc::c_int);
        }
        k -= 1;
        k;
    }
}
unsafe extern "C" fn A_isArray(J: *mut js_State) {
    if js_isobject(J, 1 as libc::c_int) != 0 {
        let T: *mut js_Object = js_toobject(J, 1 as libc::c_int);
        js_pushboolean(
            J,
            ((*T).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint)
                as libc::c_int,
        );
    } else {
        js_pushboolean(J, 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initarray(J: *mut js_State) {
    js_pushobject(J, (*J).Array_prototype);
    jsB_propf(
        J,
        b"Array.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Ap_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.concat\0" as *const u8 as *const libc::c_char,
        Some(Ap_concat as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.join\0" as *const u8 as *const libc::c_char,
        Some(Ap_join as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.pop\0" as *const u8 as *const libc::c_char,
        Some(Ap_pop as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.push\0" as *const u8 as *const libc::c_char,
        Some(Ap_push as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.reverse\0" as *const u8 as *const libc::c_char,
        Some(Ap_reverse as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.shift\0" as *const u8 as *const libc::c_char,
        Some(Ap_shift as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.slice\0" as *const u8 as *const libc::c_char,
        Some(Ap_slice as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.sort\0" as *const u8 as *const libc::c_char,
        Some(Ap_sort as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.splice\0" as *const u8 as *const libc::c_char,
        Some(Ap_splice as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.unshift\0" as *const u8 as *const libc::c_char,
        Some(Ap_unshift as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.indexOf\0" as *const u8 as *const libc::c_char,
        Some(Ap_indexOf as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.lastIndexOf\0" as *const u8 as *const libc::c_char,
        Some(Ap_lastIndexOf as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.every\0" as *const u8 as *const libc::c_char,
        Some(Ap_every as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.some\0" as *const u8 as *const libc::c_char,
        Some(Ap_some as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.forEach\0" as *const u8 as *const libc::c_char,
        Some(Ap_forEach as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.map\0" as *const u8 as *const libc::c_char,
        Some(Ap_map as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.filter\0" as *const u8 as *const libc::c_char,
        Some(Ap_filter as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.reduce\0" as *const u8 as *const libc::c_char,
        Some(Ap_reduce as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.prototype.reduceRight\0" as *const u8 as *const libc::c_char,
        Some(Ap_reduceRight as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_new_Array as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_new_Array as unsafe extern "C" fn(*mut js_State) -> ()),
        b"Array\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Array.isArray\0" as *const u8 as *const libc::c_char,
        Some(A_isArray as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"Array\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn jsB_new_Boolean(J: *mut js_State) {
    js_newboolean(J, js_toboolean(J, 1 as libc::c_int));
}
unsafe extern "C" fn jsB_Boolean(J: *mut js_State) {
    js_pushboolean(J, js_toboolean(J, 1 as libc::c_int));
}
unsafe extern "C" fn Bp_toString(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    if (*self_0).type_0 as libc::c_uint != JS_CBOOLEAN as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a boolean\0" as *const u8 as *const libc::c_char);
    }
    js_pushliteral(
        J,
        if (*self_0).u.boolean != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn Bp_valueOf(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    if (*self_0).type_0 as libc::c_uint != JS_CBOOLEAN as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a boolean\0" as *const u8 as *const libc::c_char);
    }
    js_pushboolean(J, (*self_0).u.boolean);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initboolean(J: *mut js_State) {
    (*(*J).Boolean_prototype).u.boolean = 0 as libc::c_int;
    js_pushobject(J, (*J).Boolean_prototype);
    jsB_propf(
        J,
        b"Boolean.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Bp_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Boolean.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Bp_valueOf as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_Boolean as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_new_Boolean as unsafe extern "C" fn(*mut js_State) -> ()),
        b"Boolean\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"Boolean\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn jsB_globalf(
    J: *mut js_State,
    name: *const libc::c_char,
    cfun: js_CFunction,
    n: libc::c_int,
) {
    js_newcfunction(J, cfun, name, n);
    js_defglobal(J, name, JS_DONTENUM as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_propf(
    J: *mut js_State,
    name: *const libc::c_char,
    cfun: js_CFunction,
    n: libc::c_int,
) {
    let mut pname: *const libc::c_char = strrchr(name, '.' as i32);
    pname = if !pname.is_null() {
        pname.offset(1 as libc::c_int as isize)
    } else {
        name
    };
    js_newcfunction(J, cfun, name, n);
    js_defproperty(J, -(2 as libc::c_int), pname, JS_DONTENUM as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_propn(
    J: *mut js_State,
    name: *const libc::c_char,
    number: libc::c_double,
) {
    js_pushnumber(J, number);
    js_defproperty(
        J,
        -(2 as libc::c_int),
        name,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsB_props(
    J: *mut js_State,
    name: *const libc::c_char,
    string: *const libc::c_char,
) {
    js_pushliteral(J, string);
    js_defproperty(J, -(2 as libc::c_int), name, JS_DONTENUM as libc::c_int);
}
unsafe extern "C" fn jsB_parseInt(J: *mut js_State) {
    let mut s: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    let mut radix: libc::c_int = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        0 as libc::c_int
    };
    let mut sign: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut n: libc::c_double = 0.;
    let mut e: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    while jsY_iswhite(*s as libc::c_int) != 0 || jsY_isnewline(*s as libc::c_int) != 0 {
        s = s.offset(1);
        s;
    }
    if *s as libc::c_int == '-' as i32 {
        s = s.offset(1);
        s;
        sign = -(1 as libc::c_int) as libc::c_double;
    } else if *s as libc::c_int == '+' as i32 {
        s = s.offset(1);
        s;
    }
    if radix == 0 as libc::c_int {
        radix = 10 as libc::c_int;
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
            && (*s.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
                || *s.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
        {
            s = s.offset(2 as libc::c_int as isize);
            radix = 16 as libc::c_int;
        }
    } else if radix < 2 as libc::c_int || radix > 36 as libc::c_int {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
        return;
    }
    n = js_strtol(s, &mut e, radix);
    if s == e as *const libc::c_char {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, n * sign);
    };
}
unsafe extern "C" fn jsB_parseFloat(J: *mut js_State) {
    let mut s: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    let mut e: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut n: libc::c_double = 0.;
    while jsY_iswhite(*s as libc::c_int) != 0 || jsY_isnewline(*s as libc::c_int) != 0 {
        s = s.offset(1);
        s;
    }
    if strncmp(
        s,
        b"Infinity\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        js_pushnumber(J, ::core::f32::INFINITY as libc::c_double);
    } else if strncmp(
        s,
        b"+Infinity\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        js_pushnumber(J, ::core::f32::INFINITY as libc::c_double);
    } else if strncmp(
        s,
        b"-Infinity\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        js_pushnumber(J, -::core::f32::INFINITY as libc::c_double);
    } else {
        n = js_stringtofloat(s, &mut e);
        if e == s as *mut libc::c_char {
            js_pushnumber(J, ::core::f32::NAN as libc::c_double);
        } else {
            js_pushnumber(J, n);
        }
    };
}
unsafe extern "C" fn jsB_isNaN(J: *mut js_State) {
    let n: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    js_pushboolean(J, n.is_nan() as i32);
}
unsafe extern "C" fn jsB_isFinite(J: *mut js_State) {
    let n: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    js_pushboolean(J, n.is_finite() as i32);
}
unsafe extern "C" fn Encode(
    J: *mut js_State,
    str_: *const libc::c_char,
    unescaped: *const libc::c_char,
) {
    let mut str: *const libc::c_char = str_;
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    static mut HEX: *const libc::c_char = b"0123456789ABCDEF\0" as *const u8 as *const libc::c_char;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    while *str != 0 {
        let fresh5 =
            ::core::ptr::read_volatile::<*const libc::c_char>(&str as *const *const libc::c_char);
        ::core::ptr::write_volatile(
            &mut str as *mut *const libc::c_char,
            (::core::ptr::read_volatile::<*const libc::c_char>(&str as *const *const libc::c_char))
                .offset(1),
        );
        let c: libc::c_int = *fresh5 as libc::c_uchar as libc::c_int;
        if !(strchr(unescaped, c)).is_null() {
            js_putc(J, &mut sb, c);
        } else {
            js_putc(J, &mut sb, '%' as i32);
            js_putc(
                J,
                &mut sb,
                *HEX.offset((c >> 4 as libc::c_int & 0xf as libc::c_int) as isize) as libc::c_int,
            );
            js_putc(
                J,
                &mut sb,
                *HEX.offset((c & 0xf as libc::c_int) as isize) as libc::c_int,
            );
        }
    }
    js_putc(J, &mut sb, 0 as libc::c_int);
    js_pushstring(
        J,
        if !sb.is_null() {
            ((*sb).s).as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
unsafe extern "C" fn Decode(
    J: *mut js_State,
    str_: *const libc::c_char,
    reserved: *const libc::c_char,
) {
    let mut str: *const libc::c_char = str_;
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    while *str != 0 {
        let fresh6 =
            ::core::ptr::read_volatile::<*const libc::c_char>(&str as *const *const libc::c_char);
        ::core::ptr::write_volatile(
            &mut str as *mut *const libc::c_char,
            (::core::ptr::read_volatile::<*const libc::c_char>(&str as *const *const libc::c_char))
                .offset(1),
        );
        let mut c: libc::c_int = *fresh6 as libc::c_uchar as libc::c_int;
        if c != '%' as i32 {
            js_putc(J, &mut sb, c);
        } else {
            if *str.offset(0 as libc::c_int as isize) == 0
                || *str.offset(1 as libc::c_int as isize) == 0
            {
                js_urierror(
                    J,
                    b"truncated escape sequence\0" as *const u8 as *const libc::c_char,
                );
            }
            let fresh7 = ::core::ptr::read_volatile::<*const libc::c_char>(
                &str as *const *const libc::c_char,
            );
            ::core::ptr::write_volatile(
                &mut str as *mut *const libc::c_char,
                (::core::ptr::read_volatile::<*const libc::c_char>(
                    &str as *const *const libc::c_char,
                ))
                .offset(1),
            );
            a = *fresh7 as libc::c_int;
            let fresh8 = ::core::ptr::read_volatile::<*const libc::c_char>(
                &str as *const *const libc::c_char,
            );
            ::core::ptr::write_volatile(
                &mut str as *mut *const libc::c_char,
                (::core::ptr::read_volatile::<*const libc::c_char>(
                    &str as *const *const libc::c_char,
                ))
                .offset(1),
            );
            b = *fresh8 as libc::c_int;
            if jsY_ishex(a) == 0 || jsY_ishex(b) == 0 {
                js_urierror(
                    J,
                    b"invalid escape sequence\0" as *const u8 as *const libc::c_char,
                );
            }
            c = jsY_tohex(a) << 4 as libc::c_int | jsY_tohex(b);
            if (strchr(reserved, c)).is_null() {
                js_putc(J, &mut sb, c);
            } else {
                js_putc(J, &mut sb, '%' as i32);
                js_putc(J, &mut sb, a);
                js_putc(J, &mut sb, b);
            }
        }
    }
    js_putc(J, &mut sb, 0 as libc::c_int);
    js_pushstring(
        J,
        if !sb.is_null() {
            ((*sb).s).as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
unsafe extern "C" fn jsB_decodeURI(J: *mut js_State) {
    Decode(
        J,
        js_tostring(J, 1 as libc::c_int),
        b";/?:@&=+$,#\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn jsB_decodeURIComponent(J: *mut js_State) {
    Decode(
        J,
        js_tostring(J, 1 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn jsB_encodeURI(J: *mut js_State) {
    Encode(
        J,
        js_tostring(J, 1 as libc::c_int),
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_.!~*'();/?:@&=+$,#\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn jsB_encodeURIComponent(J: *mut js_State) {
    Encode(
        J,
        js_tostring(J, 1 as libc::c_int),
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_.!~*'()\0" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsB_init(J: *mut js_State) {
    (*J).Object_prototype = jsV_newobject(J, JS_COBJECT, std::ptr::null_mut::<js_Object>());
    (*J).Array_prototype = jsV_newobject(J, JS_CARRAY, (*J).Object_prototype);
    (*J).Function_prototype = jsV_newobject(J, JS_CCFUNCTION, (*J).Object_prototype);
    (*J).Boolean_prototype = jsV_newobject(J, JS_CBOOLEAN, (*J).Object_prototype);
    (*J).Number_prototype = jsV_newobject(J, JS_CNUMBER, (*J).Object_prototype);
    (*J).String_prototype = jsV_newobject(J, JS_CSTRING, (*J).Object_prototype);
    (*J).Date_prototype = jsV_newobject(J, JS_CDATE, (*J).Object_prototype);
    (*J).RegExp_prototype = jsV_newobject(J, JS_CREGEXP, (*J).Object_prototype);
    (*(*J).RegExp_prototype).u.r.prog = js_regcompx(
        (*J).alloc,
        (*J).actx,
        b"(?:)\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
        std::ptr::null_mut::<*const libc::c_char>(),
    ) as *mut libc::c_void;
    (*(*J).RegExp_prototype).u.r.source =
        js_strdup(J, b"(?:)\0" as *const u8 as *const libc::c_char);
    (*J).Error_prototype = jsV_newobject(J, JS_CERROR, (*J).Object_prototype);
    (*J).EvalError_prototype = jsV_newobject(J, JS_CERROR, (*J).Error_prototype);
    (*J).RangeError_prototype = jsV_newobject(J, JS_CERROR, (*J).Error_prototype);
    (*J).ReferenceError_prototype = jsV_newobject(J, JS_CERROR, (*J).Error_prototype);
    (*J).SyntaxError_prototype = jsV_newobject(J, JS_CERROR, (*J).Error_prototype);
    (*J).TypeError_prototype = jsV_newobject(J, JS_CERROR, (*J).Error_prototype);
    (*J).URIError_prototype = jsV_newobject(J, JS_CERROR, (*J).Error_prototype);
    jsB_initobject(J);
    jsB_initarray(J);
    jsB_initfunction(J);
    jsB_initboolean(J);
    jsB_initnumber(J);
    jsB_initstring(J);
    jsB_initregexp(J);
    jsB_initdate(J);
    jsB_initerror(J);
    jsB_initmath(J);
    jsB_initjson(J);
    js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    js_defglobal(
        J,
        b"NaN\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    js_pushnumber(J, ::core::f32::INFINITY as libc::c_double);
    js_defglobal(
        J,
        b"Infinity\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    js_pushundefined(J);
    js_defglobal(
        J,
        b"undefined\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    jsB_globalf(
        J,
        b"parseInt\0" as *const u8 as *const libc::c_char,
        Some(jsB_parseInt as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_globalf(
        J,
        b"parseFloat\0" as *const u8 as *const libc::c_char,
        Some(jsB_parseFloat as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_globalf(
        J,
        b"isNaN\0" as *const u8 as *const libc::c_char,
        Some(jsB_isNaN as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_globalf(
        J,
        b"isFinite\0" as *const u8 as *const libc::c_char,
        Some(jsB_isFinite as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_globalf(
        J,
        b"decodeURI\0" as *const u8 as *const libc::c_char,
        Some(jsB_decodeURI as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_globalf(
        J,
        b"decodeURIComponent\0" as *const u8 as *const libc::c_char,
        Some(jsB_decodeURIComponent as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_globalf(
        J,
        b"encodeURI\0" as *const u8 as *const libc::c_char,
        Some(jsB_encodeURI as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_globalf(
        J,
        b"encodeURIComponent\0" as *const u8 as *const libc::c_char,
        Some(jsB_encodeURIComponent as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsC_error(
    J: *mut js_State,
    node: *mut js_Ast,
    fmt: *const libc::c_char,
    args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut msgbuf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        msgbuf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    snprintf(
        buf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"%s:%d: \0" as *const u8 as *const libc::c_char,
        (*J).filename,
        (*node).line,
    );
    strcat(buf.as_mut_ptr(), msgbuf.as_mut_ptr());
    js_newsyntaxerror(J, buf.as_mut_ptr());
    js_throw(J);
}
static mut futurewords: [*const libc::c_char; 7] = [
    b"class\0" as *const u8 as *const libc::c_char,
    b"const\0" as *const u8 as *const libc::c_char,
    b"enum\0" as *const u8 as *const libc::c_char,
    b"export\0" as *const u8 as *const libc::c_char,
    b"extends\0" as *const u8 as *const libc::c_char,
    b"import\0" as *const u8 as *const libc::c_char,
    b"super\0" as *const u8 as *const libc::c_char,
];
static mut strictfuturewords: [*const libc::c_char; 9] = [
    b"implements\0" as *const u8 as *const libc::c_char,
    b"interface\0" as *const u8 as *const libc::c_char,
    b"let\0" as *const u8 as *const libc::c_char,
    b"package\0" as *const u8 as *const libc::c_char,
    b"private\0" as *const u8 as *const libc::c_char,
    b"protected\0" as *const u8 as *const libc::c_char,
    b"public\0" as *const u8 as *const libc::c_char,
    b"static\0" as *const u8 as *const libc::c_char,
    b"yield\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn checkfutureword(J: *mut js_State, F: *mut js_Function, exp_0: *mut js_Ast) {
    if jsY_findword(
        (*exp_0).string,
        futurewords.as_mut_ptr(),
        (::core::mem::size_of::<[*const libc::c_char; 7]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    ) >= 0 as libc::c_int
    {
        jsC_error(
            J,
            exp_0,
            b"'%s' is a future reserved word\0" as *const u8 as *const libc::c_char,
            (*exp_0).string,
        );
    }
    if (*F).strict != 0
        && jsY_findword(
            (*exp_0).string,
            strictfuturewords.as_mut_ptr(),
            (::core::mem::size_of::<[*const libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                as libc::c_int,
        ) >= 0 as libc::c_int
    {
        jsC_error(
            J,
            exp_0,
            b"'%s' is a strict mode future reserved word\0" as *const u8 as *const libc::c_char,
            (*exp_0).string,
        );
    }
}
unsafe extern "C" fn newfun(
    J: *mut js_State,
    line: libc::c_int,
    name: *mut js_Ast,
    params: *mut js_Ast,
    body: *mut js_Ast,
    script_0: libc::c_int,
    default_strict: libc::c_int,
    is_fun_exp: libc::c_int,
) -> *mut js_Function {
    let F: *mut js_Function = js_malloc(
        J,
        ::core::mem::size_of::<js_Function>() as libc::c_ulong as libc::c_int,
    ) as *mut js_Function;
    memset(
        F as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<js_Function>() as libc::c_ulong,
    );
    (*F).gcmark = 0 as libc::c_int;
    (*F).gcnext = (*J).gcfun;
    (*J).gcfun = F;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    (*F).filename = js_intern(J, (*J).filename);
    (*F).line = line;
    (*F).script = script_0;
    (*F).strict = default_strict;
    (*F).name = if !name.is_null() {
        (*name).string
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    cfunbody(J, F, name, params, body, is_fun_exp);
    F
}
unsafe extern "C" fn emitraw(J: *mut js_State, F: *mut js_Function, value: libc::c_int) {
    if value != value as js_Instruction as libc::c_int {
        js_syntaxerror(
            J,
            b"integer overflow in instruction coding\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*F).codelen >= (*F).codecap {
        (*F).codecap = if (*F).codecap != 0 {
            (*F).codecap * 2 as libc::c_int
        } else {
            64 as libc::c_int
        };
        (*F).code = js_realloc(
            J,
            (*F).code as *mut libc::c_void,
            ((*F).codecap as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut js_Instruction;
    }
    let fresh9 = (*F).codelen;
    (*F).codelen += 1;
    *((*F).code).offset(fresh9 as isize) = value as js_Instruction;
}
unsafe extern "C" fn emit(J: *mut js_State, F: *mut js_Function, value: libc::c_int) {
    emitraw(J, F, (*F).lastline);
    emitraw(J, F, value);
}
unsafe extern "C" fn emitarg(J: *mut js_State, F: *mut js_Function, value: libc::c_int) {
    emitraw(J, F, value);
}
unsafe extern "C" fn emitline(J: *mut js_State, F: *mut js_Function, node: *mut js_Ast) {
    (*F).lastline = (*node).line;
}
unsafe extern "C" fn addfunction(
    J: *mut js_State,
    F: *mut js_Function,
    value: *mut js_Function,
) -> libc::c_int {
    if (*F).funlen >= (*F).funcap {
        (*F).funcap = if (*F).funcap != 0 {
            (*F).funcap * 2 as libc::c_int
        } else {
            16 as libc::c_int
        };
        (*F).funtab = js_realloc(
            J,
            (*F).funtab as *mut libc::c_void,
            ((*F).funcap as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*mut js_Function>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut *mut js_Function;
    }
    let fresh10 = &mut (*((*F).funtab).offset((*F).funlen as isize));
    *fresh10 = value;
    let fresh11 = (*F).funlen;
    (*F).funlen += 1;
    fresh11
}
unsafe extern "C" fn addlocal(
    J: *mut js_State,
    F: *mut js_Function,
    ident: *mut js_Ast,
    reuse: libc::c_int,
) -> libc::c_int {
    let name: *const libc::c_char = (*ident).string;
    if (*F).strict != 0 {
        if strcmp(name, b"arguments\0" as *const u8 as *const libc::c_char) == 0 {
            jsC_error(
                J,
                ident,
                b"redefining 'arguments' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if strcmp(name, b"eval\0" as *const u8 as *const libc::c_char) == 0 {
            jsC_error(
                J,
                ident,
                b"redefining 'eval' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if strcmp(name, b"eval\0" as *const u8 as *const libc::c_char) == 0 {
        js_evalerror(
            J,
            b"%s:%d: invalid use of 'eval'\0" as *const u8 as *const libc::c_char,
            (*J).filename,
            (*ident).line,
        );
    }
    if reuse != 0 || (*F).strict != 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*F).varlen {
            if strcmp(*((*F).vartab).offset(i as isize), name) == 0 {
                if reuse != 0 {
                    return i + 1 as libc::c_int;
                }
                if (*F).strict != 0 {
                    jsC_error(
                        J,
                        ident,
                        b"duplicate formal parameter '%s'\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
            }
            i += 1;
            i;
        }
    }
    if (*F).varlen >= (*F).varcap {
        (*F).varcap = if (*F).varcap != 0 {
            (*F).varcap * 2 as libc::c_int
        } else {
            16 as libc::c_int
        };
        (*F).vartab = js_realloc(
            J,
            (*F).vartab as *mut libc::c_void,
            ((*F).varcap as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut *const libc::c_char;
    }
    let fresh12 = &mut (*((*F).vartab).offset((*F).varlen as isize));
    *fresh12 = name;
    (*F).varlen += 1;
    (*F).varlen
}
unsafe extern "C" fn findlocal(
    J: *mut js_State,
    F: *mut js_Function,
    name: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = (*F).varlen;
    while i > 0 as libc::c_int {
        if strcmp(*((*F).vartab).offset((i - 1 as libc::c_int) as isize), name) == 0 {
            return i;
        }
        i -= 1;
        i;
    }
    -(1 as libc::c_int)
}
unsafe extern "C" fn emitfunction(J: *mut js_State, F: *mut js_Function, fun: *mut js_Function) {
    (*F).lightweight = 0 as libc::c_int;
    emit(J, F, OP_CLOSURE as libc::c_int);
    emitarg(J, F, addfunction(J, F, fun));
}
unsafe extern "C" fn emitnumber(J: *mut js_State, F: *mut js_Function, mut num: libc::c_double) {
    if num == 0 as libc::c_int as libc::c_double {
        emit(J, F, OP_INTEGER as libc::c_int);
        emitarg(J, F, 32768 as libc::c_int);
        if num.is_sign_negative() as libc::c_int != 0 {
            emit(J, F, OP_NEG as libc::c_int);
        }
    } else if num >= (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        && num <= 32767 as libc::c_int as libc::c_double
        && num == num as libc::c_int as libc::c_double
    {
        emit(J, F, OP_INTEGER as libc::c_int);
        emitarg(
            J,
            F,
            (num + 32768 as libc::c_int as libc::c_double) as libc::c_int,
        );
    } else {
        let mut x: [js_Instruction; 4] = [0; 4];
        let mut i: size_t = 0;
        emit(J, F, OP_NUMBER as libc::c_int);
        memcpy(
            x.as_mut_ptr() as *mut libc::c_void,
            &mut num as *mut libc::c_double as *const libc::c_void,
            ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
        );
        i = 0 as libc::c_int as size_t;
        while i
            < (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
        {
            emitarg(J, F, x[i as usize] as libc::c_int);
            i = i.wrapping_add(1);
            i;
        }
    };
}
unsafe extern "C" fn emitstring(
    J: *mut js_State,
    F: *mut js_Function,
    opcode: libc::c_int,
    mut str: *const libc::c_char,
) {
    let mut x: [js_Instruction; 4] = [0; 4];
    let mut i: size_t = 0;
    emit(J, F, opcode);
    memcpy(
        x.as_mut_ptr() as *mut libc::c_void,
        &mut str as *mut *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    );
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
    {
        emitarg(J, F, x[i as usize] as libc::c_int);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn emitlocal(
    J: *mut js_State,
    F: *mut js_Function,
    oploc: libc::c_int,
    opvar: libc::c_int,
    ident: *mut js_Ast,
) {
    let is_arguments: libc::c_int = (strcmp(
        (*ident).string,
        b"arguments\0" as *const u8 as *const libc::c_char,
    ) == 0) as libc::c_int;
    let is_eval: libc::c_int = (strcmp(
        (*ident).string,
        b"eval\0" as *const u8 as *const libc::c_char,
    ) == 0) as libc::c_int;
    let mut i: libc::c_int = 0;
    if is_arguments != 0 {
        (*F).lightweight = 0 as libc::c_int;
        (*F).arguments = 1 as libc::c_int;
    }
    checkfutureword(J, F, ident);
    if (*F).strict != 0 && oploc == OP_SETLOCAL as libc::c_int {
        if is_arguments != 0 {
            jsC_error(
                J,
                ident,
                b"'arguments' is read-only in strict mode\0" as *const u8 as *const libc::c_char,
            );
        }
        if is_eval != 0 {
            jsC_error(
                J,
                ident,
                b"'eval' is read-only in strict mode\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if is_eval != 0 {
        js_evalerror(
            J,
            b"%s:%d: invalid use of 'eval'\0" as *const u8 as *const libc::c_char,
            (*J).filename,
            (*ident).line,
        );
    }
    i = findlocal(J, F, (*ident).string);
    if i < 0 as libc::c_int {
        emitstring(J, F, opvar, (*ident).string);
    } else {
        emit(J, F, oploc);
        emitarg(J, F, i);
    };
}
unsafe extern "C" fn here(J: *mut js_State, F: *mut js_Function) -> libc::c_int {
    (*F).codelen
}
unsafe extern "C" fn emitjump(
    J: *mut js_State,
    F: *mut js_Function,
    opcode: libc::c_int,
) -> libc::c_int {
    let mut inst: libc::c_int = 0;
    emit(J, F, opcode);
    inst = (*F).codelen;
    emitarg(J, F, 0 as libc::c_int);
    inst
}
unsafe extern "C" fn emitjumpto(
    J: *mut js_State,
    F: *mut js_Function,
    opcode: libc::c_int,
    dest: libc::c_int,
) {
    emit(J, F, opcode);
    if dest != dest as js_Instruction as libc::c_int {
        js_syntaxerror(
            J,
            b"jump address integer overflow\0" as *const u8 as *const libc::c_char,
        );
    }
    emitarg(J, F, dest);
}
unsafe extern "C" fn labelto(
    J: *mut js_State,
    F: *mut js_Function,
    inst: libc::c_int,
    addr: libc::c_int,
) {
    if addr != addr as js_Instruction as libc::c_int {
        js_syntaxerror(
            J,
            b"jump address integer overflow\0" as *const u8 as *const libc::c_char,
        );
    }
    *((*F).code).offset(inst as isize) = addr as js_Instruction;
}
unsafe extern "C" fn label(J: *mut js_State, F: *mut js_Function, inst: libc::c_int) {
    labelto(J, F, inst, (*F).codelen);
}
unsafe extern "C" fn ctypeof(J: *mut js_State, F: *mut js_Function, exp_0: *mut js_Ast) {
    if (*(*exp_0).a).type_0 as libc::c_uint == EXP_IDENTIFIER as libc::c_int as libc::c_uint {
        emitline(J, F, (*exp_0).a);
        emitlocal(
            J,
            F,
            OP_GETLOCAL as libc::c_int,
            OP_HASVAR as libc::c_int,
            (*exp_0).a,
        );
    } else {
        jsC_cexp(J, F, (*exp_0).a);
    }
    emitline(J, F, exp_0);
    emit(J, F, OP_TYPEOF as libc::c_int);
}
unsafe extern "C" fn cunary(
    J: *mut js_State,
    F: *mut js_Function,
    exp_0: *mut js_Ast,
    opcode: libc::c_int,
) {
    jsC_cexp(J, F, (*exp_0).a);
    emitline(J, F, exp_0);
    emit(J, F, opcode);
}
unsafe extern "C" fn cbinary(
    J: *mut js_State,
    F: *mut js_Function,
    exp_0: *mut js_Ast,
    opcode: libc::c_int,
) {
    jsC_cexp(J, F, (*exp_0).a);
    jsC_cexp(J, F, (*exp_0).b);
    emitline(J, F, exp_0);
    emit(J, F, opcode);
}
unsafe extern "C" fn carray(J: *mut js_State, F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        emitline(J, F, (*list).a);
        if (*(*list).a).type_0 as libc::c_uint == EXP_ELISION as libc::c_int as libc::c_uint {
            emit(J, F, OP_SKIPARRAY as libc::c_int);
        } else {
            jsC_cexp(J, F, (*list).a);
            emit(J, F, OP_INITARRAY as libc::c_int);
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn checkdup(
    J: *mut js_State,
    F: *mut js_Function,
    mut list: *mut js_Ast,
    end: *mut js_Ast,
) {
    let mut nbuf: [libc::c_char; 32] = [0; 32];
    let mut sbuf: [libc::c_char; 32] = [0; 32];
    let mut needle: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut straw: *const libc::c_char = std::ptr::null::<libc::c_char>();
    if (*(*end).a).type_0 as libc::c_uint == EXP_NUMBER as libc::c_int as libc::c_uint {
        needle = jsV_numbertostring(J, nbuf.as_mut_ptr(), (*(*end).a).number);
    } else {
        needle = (*(*end).a).string;
    }
    while (*list).a != end {
        if (*(*list).a).type_0 as libc::c_uint == (*end).type_0 as libc::c_uint {
            let prop: *mut js_Ast = (*(*list).a).a;
            if (*prop).type_0 as libc::c_uint == EXP_NUMBER as libc::c_int as libc::c_uint {
                straw = jsV_numbertostring(J, sbuf.as_mut_ptr(), (*prop).number);
            } else {
                straw = (*prop).string;
            }
            if strcmp(needle, straw) == 0 {
                jsC_error(
                    J,
                    list,
                    b"duplicate property '%s' in object literal\0" as *const u8
                        as *const libc::c_char,
                    needle,
                );
            }
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cobject(J: *mut js_State, F: *mut js_Function, mut list: *mut js_Ast) {
    let head: *mut js_Ast = list;
    while !list.is_null() {
        let kv: *mut js_Ast = (*list).a;
        let prop: *mut js_Ast = (*kv).a;
        if (*prop).type_0 as libc::c_uint == AST_IDENTIFIER as libc::c_int as libc::c_uint
            || (*prop).type_0 as libc::c_uint == EXP_STRING as libc::c_int as libc::c_uint
        {
            emitline(J, F, prop);
            emitstring(J, F, OP_STRING as libc::c_int, (*prop).string);
        } else if (*prop).type_0 as libc::c_uint == EXP_NUMBER as libc::c_int as libc::c_uint {
            emitline(J, F, prop);
            emitnumber(J, F, (*prop).number);
        } else {
            jsC_error(
                J,
                prop,
                b"invalid property name in object initializer\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*F).strict != 0 {
            checkdup(J, F, head, kv);
        }
        match (*kv).type_0 as libc::c_uint {
            14 => {
                jsC_cexp(J, F, (*kv).b);
                emitline(J, F, kv);
                emit(J, F, OP_INITPROP as libc::c_int);
            }
            15 => {
                emitfunction(
                    J,
                    F,
                    newfun(
                        J,
                        (*prop).line,
                        std::ptr::null_mut::<js_Ast>(),
                        std::ptr::null_mut::<js_Ast>(),
                        (*kv).c,
                        0 as libc::c_int,
                        (*F).strict,
                        1 as libc::c_int,
                    ),
                );
                emitline(J, F, kv);
                emit(J, F, OP_INITGETTER as libc::c_int);
            }
            16 => {
                emitfunction(
                    J,
                    F,
                    newfun(
                        J,
                        (*prop).line,
                        std::ptr::null_mut::<js_Ast>(),
                        (*kv).b,
                        (*kv).c,
                        0 as libc::c_int,
                        (*F).strict,
                        1 as libc::c_int,
                    ),
                );
                emitline(J, F, kv);
                emit(J, F, OP_INITSETTER as libc::c_int);
            }
            _ => {}
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cargs(
    J: *mut js_State,
    F: *mut js_Function,
    mut list: *mut js_Ast,
) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    while !list.is_null() {
        jsC_cexp(J, F, (*list).a);
        list = (*list).b;
        n += 1;
        n;
    }
    n
}
unsafe extern "C" fn cassign(J: *mut js_State, F: *mut js_Function, exp_0: *mut js_Ast) {
    let lhs: *mut js_Ast = (*exp_0).a;
    let rhs: *mut js_Ast = (*exp_0).b;
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            jsC_cexp(J, F, rhs);
            emitline(J, F, exp_0);
            emitlocal(
                J,
                F,
                OP_SETLOCAL as libc::c_int,
                OP_SETVAR as libc::c_int,
                lhs,
            );
        }
        18 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, (*lhs).b);
            jsC_cexp(J, F, rhs);
            emitline(J, F, exp_0);
            emit(J, F, OP_SETPROP as libc::c_int);
        }
        19 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, rhs);
            emitline(J, F, exp_0);
            emitstring(J, F, OP_SETPROP_S as libc::c_int, (*(*lhs).b).string);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignforin(J: *mut js_State, F: *mut js_Function, stm: *mut js_Ast) {
    let lhs: *mut js_Ast = (*stm).a;
    if (*stm).type_0 as libc::c_uint == STM_FOR_IN_VAR as libc::c_int as libc::c_uint {
        if !((*lhs).b).is_null() {
            jsC_error(
                J,
                (*lhs).b,
                b"more than one loop variable in for-in statement\0" as *const u8
                    as *const libc::c_char,
            );
        }
        emitline(J, F, (*lhs).a);
        emitlocal(
            J,
            F,
            OP_SETLOCAL as libc::c_int,
            OP_SETVAR as libc::c_int,
            (*(*lhs).a).a,
        );
        emit(J, F, OP_POP as libc::c_int);
        return;
    }
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            emitline(J, F, lhs);
            emitlocal(
                J,
                F,
                OP_SETLOCAL as libc::c_int,
                OP_SETVAR as libc::c_int,
                lhs,
            );
            emit(J, F, OP_POP as libc::c_int);
        }
        18 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, (*lhs).b);
            emitline(J, F, lhs);
            emit(J, F, OP_ROT3 as libc::c_int);
            emit(J, F, OP_SETPROP as libc::c_int);
            emit(J, F, OP_POP as libc::c_int);
        }
        19 => {
            jsC_cexp(J, F, (*lhs).a);
            emitline(J, F, lhs);
            emit(J, F, OP_ROT2 as libc::c_int);
            emitstring(J, F, OP_SETPROP_S as libc::c_int, (*(*lhs).b).string);
            emit(J, F, OP_POP as libc::c_int);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in for-in loop assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignop1(J: *mut js_State, F: *mut js_Function, lhs: *mut js_Ast) {
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            emitline(J, F, lhs);
            emitlocal(
                J,
                F,
                OP_GETLOCAL as libc::c_int,
                OP_GETVAR as libc::c_int,
                lhs,
            );
        }
        18 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, (*lhs).b);
            emitline(J, F, lhs);
            emit(J, F, OP_DUP2 as libc::c_int);
            emit(J, F, OP_GETPROP as libc::c_int);
        }
        19 => {
            jsC_cexp(J, F, (*lhs).a);
            emitline(J, F, lhs);
            emit(J, F, OP_DUP as libc::c_int);
            emitstring(J, F, OP_GETPROP_S as libc::c_int, (*(*lhs).b).string);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignop2(
    J: *mut js_State,
    F: *mut js_Function,
    lhs: *mut js_Ast,
    postfix_0: libc::c_int,
) {
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            emitline(J, F, lhs);
            if postfix_0 != 0 {
                emit(J, F, OP_ROT2 as libc::c_int);
            }
            emitlocal(
                J,
                F,
                OP_SETLOCAL as libc::c_int,
                OP_SETVAR as libc::c_int,
                lhs,
            );
        }
        18 => {
            emitline(J, F, lhs);
            if postfix_0 != 0 {
                emit(J, F, OP_ROT4 as libc::c_int);
            }
            emit(J, F, OP_SETPROP as libc::c_int);
        }
        19 => {
            emitline(J, F, lhs);
            if postfix_0 != 0 {
                emit(J, F, OP_ROT3 as libc::c_int);
            }
            emitstring(J, F, OP_SETPROP_S as libc::c_int, (*(*lhs).b).string);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignop(
    J: *mut js_State,
    F: *mut js_Function,
    exp_0: *mut js_Ast,
    opcode: libc::c_int,
) {
    let lhs: *mut js_Ast = (*exp_0).a;
    let rhs: *mut js_Ast = (*exp_0).b;
    cassignop1(J, F, lhs);
    jsC_cexp(J, F, rhs);
    emitline(J, F, exp_0);
    emit(J, F, opcode);
    cassignop2(J, F, lhs, 0 as libc::c_int);
}
unsafe extern "C" fn cdelete(J: *mut js_State, F: *mut js_Function, exp_0: *mut js_Ast) {
    let arg: *mut js_Ast = (*exp_0).a;
    match (*arg).type_0 as libc::c_uint {
        3 => {
            if (*F).strict != 0 {
                jsC_error(
                    J,
                    exp_0,
                    b"delete on an unqualified name is not allowed in strict mode\0" as *const u8
                        as *const libc::c_char,
                );
            }
            emitline(J, F, exp_0);
            emitlocal(
                J,
                F,
                OP_DELLOCAL as libc::c_int,
                OP_DELVAR as libc::c_int,
                arg,
            );
        }
        18 => {
            jsC_cexp(J, F, (*arg).a);
            jsC_cexp(J, F, (*arg).b);
            emitline(J, F, exp_0);
            emit(J, F, OP_DELPROP as libc::c_int);
        }
        19 => {
            jsC_cexp(J, F, (*arg).a);
            emitline(J, F, exp_0);
            emitstring(J, F, OP_DELPROP_S as libc::c_int, (*(*arg).b).string);
        }
        _ => {
            jsC_error(
                J,
                exp_0,
                b"invalid l-value in delete expression\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn ceval(
    J: *mut js_State,
    F: *mut js_Function,
    fun: *mut js_Ast,
    args: *mut js_Ast,
) {
    let mut n: libc::c_int = cargs(J, F, args);
    (*F).lightweight = 0 as libc::c_int;
    (*F).arguments = 1 as libc::c_int;
    if n == 0 as libc::c_int {
        emit(J, F, OP_UNDEF as libc::c_int);
    } else {
        loop {
            let fresh13 = n;
            n -= 1;
            if fresh13 <= 1 as libc::c_int {
                break;
            }
            emit(J, F, OP_POP as libc::c_int);
        }
    }
    emit(J, F, OP_EVAL as libc::c_int);
}
unsafe extern "C" fn ccall(
    J: *mut js_State,
    F: *mut js_Function,
    fun: *mut js_Ast,
    args: *mut js_Ast,
) {
    let mut n: libc::c_int = 0;
    let current_block_14: u64;
    match (*fun).type_0 as libc::c_uint {
        18 => {
            jsC_cexp(J, F, (*fun).a);
            emit(J, F, OP_DUP as libc::c_int);
            jsC_cexp(J, F, (*fun).b);
            emit(J, F, OP_GETPROP as libc::c_int);
            emit(J, F, OP_ROT2 as libc::c_int);
            current_block_14 = 11050875288958768710;
        }
        19 => {
            jsC_cexp(J, F, (*fun).a);
            emit(J, F, OP_DUP as libc::c_int);
            emitstring(J, F, OP_GETPROP_S as libc::c_int, (*(*fun).b).string);
            emit(J, F, OP_ROT2 as libc::c_int);
            current_block_14 = 11050875288958768710;
        }
        3 => {
            if strcmp((*fun).string, b"eval\0" as *const u8 as *const libc::c_char) == 0 {
                ceval(J, F, fun, args);
                return;
            }
            current_block_14 = 5035918384899282284;
        }
        _ => {
            current_block_14 = 5035918384899282284;
        }
    }
    if current_block_14 == 5035918384899282284 {
        jsC_cexp(J, F, fun);
        emit(J, F, OP_UNDEF as libc::c_int);
    }
    n = cargs(J, F, args);
    emit(J, F, OP_CALL as libc::c_int);
    emitarg(J, F, n);
}
unsafe extern "C" fn jsC_cexp(J: *mut js_State, F: *mut js_Function, exp_0: *mut js_Ast) {
    let mut then: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    match (*exp_0).type_0 as libc::c_uint {
        5 => {
            emitline(J, F, exp_0);
            emitstring(J, F, OP_STRING as libc::c_int, (*exp_0).string);
        }
        4 => {
            emitline(J, F, exp_0);
            emitnumber(J, F, (*exp_0).number);
        }
        7 => {}
        8 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_NULL as libc::c_int);
        }
        9 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_TRUE as libc::c_int);
        }
        10 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_FALSE as libc::c_int);
        }
        11 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_THIS as libc::c_int);
        }
        6 => {
            emitline(J, F, exp_0);
            emitstring(J, F, OP_NEWREGEXP as libc::c_int, (*exp_0).string);
            emitarg(J, F, (*exp_0).number as libc::c_int);
        }
        13 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_NEWOBJECT as libc::c_int);
            cobject(J, F, (*exp_0).a);
        }
        12 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_NEWARRAY as libc::c_int);
            carray(J, F, (*exp_0).a);
        }
        17 => {
            emitline(J, F, exp_0);
            emitfunction(
                J,
                F,
                newfun(
                    J,
                    (*exp_0).line,
                    (*exp_0).a,
                    (*exp_0).b,
                    (*exp_0).c,
                    0 as libc::c_int,
                    (*F).strict,
                    1 as libc::c_int,
                ),
            );
        }
        3 => {
            emitline(J, F, exp_0);
            emitlocal(
                J,
                F,
                OP_GETLOCAL as libc::c_int,
                OP_GETVAR as libc::c_int,
                exp_0,
            );
        }
        18 => {
            jsC_cexp(J, F, (*exp_0).a);
            jsC_cexp(J, F, (*exp_0).b);
            emitline(J, F, exp_0);
            emit(J, F, OP_GETPROP as libc::c_int);
        }
        19 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emitstring(J, F, OP_GETPROP_S as libc::c_int, (*(*exp_0).b).string);
        }
        20 => {
            ccall(J, F, (*exp_0).a, (*exp_0).b);
        }
        21 => {
            jsC_cexp(J, F, (*exp_0).a);
            n = cargs(J, F, (*exp_0).b);
            emitline(J, F, exp_0);
            emit(J, F, OP_NEW as libc::c_int);
            emitarg(J, F, n);
        }
        24 => {
            cdelete(J, F, exp_0);
        }
        27 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_INC as libc::c_int);
            cassignop2(J, F, (*exp_0).a, 0 as libc::c_int);
        }
        28 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_DEC as libc::c_int);
            cassignop2(J, F, (*exp_0).a, 0 as libc::c_int);
        }
        22 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POSTINC as libc::c_int);
            cassignop2(J, F, (*exp_0).a, 1 as libc::c_int);
            emit(J, F, OP_POP as libc::c_int);
        }
        23 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POSTDEC as libc::c_int);
            cassignop2(J, F, (*exp_0).a, 1 as libc::c_int);
            emit(J, F, OP_POP as libc::c_int);
        }
        25 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POP as libc::c_int);
            emit(J, F, OP_UNDEF as libc::c_int);
        }
        26 => {
            ctypeof(J, F, exp_0);
        }
        29 => {
            cunary(J, F, exp_0, OP_POS as libc::c_int);
        }
        30 => {
            cunary(J, F, exp_0, OP_NEG as libc::c_int);
        }
        31 => {
            cunary(J, F, exp_0, OP_BITNOT as libc::c_int);
        }
        32 => {
            cunary(J, F, exp_0, OP_LOGNOT as libc::c_int);
        }
        53 => {
            cbinary(J, F, exp_0, OP_BITOR as libc::c_int);
        }
        52 => {
            cbinary(J, F, exp_0, OP_BITXOR as libc::c_int);
        }
        51 => {
            cbinary(J, F, exp_0, OP_BITAND as libc::c_int);
        }
        50 => {
            cbinary(J, F, exp_0, OP_EQ as libc::c_int);
        }
        49 => {
            cbinary(J, F, exp_0, OP_NE as libc::c_int);
        }
        48 => {
            cbinary(J, F, exp_0, OP_STRICTEQ as libc::c_int);
        }
        47 => {
            cbinary(J, F, exp_0, OP_STRICTNE as libc::c_int);
        }
        46 => {
            cbinary(J, F, exp_0, OP_LT as libc::c_int);
        }
        45 => {
            cbinary(J, F, exp_0, OP_GT as libc::c_int);
        }
        44 => {
            cbinary(J, F, exp_0, OP_LE as libc::c_int);
        }
        43 => {
            cbinary(J, F, exp_0, OP_GE as libc::c_int);
        }
        42 => {
            cbinary(J, F, exp_0, OP_INSTANCEOF as libc::c_int);
        }
        41 => {
            cbinary(J, F, exp_0, OP_IN as libc::c_int);
        }
        40 => {
            cbinary(J, F, exp_0, OP_SHL as libc::c_int);
        }
        39 => {
            cbinary(J, F, exp_0, OP_SHR as libc::c_int);
        }
        38 => {
            cbinary(J, F, exp_0, OP_USHR as libc::c_int);
        }
        37 => {
            cbinary(J, F, exp_0, OP_ADD as libc::c_int);
        }
        36 => {
            cbinary(J, F, exp_0, OP_SUB as libc::c_int);
        }
        35 => {
            cbinary(J, F, exp_0, OP_MUL as libc::c_int);
        }
        34 => {
            cbinary(J, F, exp_0, OP_DIV as libc::c_int);
        }
        33 => {
            cbinary(J, F, exp_0, OP_MOD as libc::c_int);
        }
        57 => {
            cassign(J, F, exp_0);
        }
        58 => {
            cassignop(J, F, exp_0, OP_MUL as libc::c_int);
        }
        59 => {
            cassignop(J, F, exp_0, OP_DIV as libc::c_int);
        }
        60 => {
            cassignop(J, F, exp_0, OP_MOD as libc::c_int);
        }
        61 => {
            cassignop(J, F, exp_0, OP_ADD as libc::c_int);
        }
        62 => {
            cassignop(J, F, exp_0, OP_SUB as libc::c_int);
        }
        63 => {
            cassignop(J, F, exp_0, OP_SHL as libc::c_int);
        }
        64 => {
            cassignop(J, F, exp_0, OP_SHR as libc::c_int);
        }
        65 => {
            cassignop(J, F, exp_0, OP_USHR as libc::c_int);
        }
        66 => {
            cassignop(J, F, exp_0, OP_BITAND as libc::c_int);
        }
        67 => {
            cassignop(J, F, exp_0, OP_BITXOR as libc::c_int);
        }
        68 => {
            cassignop(J, F, exp_0, OP_BITOR as libc::c_int);
        }
        69 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POP as libc::c_int);
            jsC_cexp(J, F, (*exp_0).b);
        }
        55 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_DUP as libc::c_int);
            end = emitjump(J, F, OP_JTRUE as libc::c_int);
            emit(J, F, OP_POP as libc::c_int);
            jsC_cexp(J, F, (*exp_0).b);
            label(J, F, end);
        }
        54 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_DUP as libc::c_int);
            end = emitjump(J, F, OP_JFALSE as libc::c_int);
            emit(J, F, OP_POP as libc::c_int);
            jsC_cexp(J, F, (*exp_0).b);
            label(J, F, end);
        }
        56 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            then = emitjump(J, F, OP_JTRUE as libc::c_int);
            jsC_cexp(J, F, (*exp_0).c);
            end = emitjump(J, F, OP_JUMP as libc::c_int);
            label(J, F, then);
            jsC_cexp(J, F, (*exp_0).b);
            label(J, F, end);
        }
        _ => {
            jsC_error(
                J,
                exp_0,
                b"unknown expression type\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn addjump(
    J: *mut js_State,
    F: *mut js_Function,
    type_0: js_AstType,
    target: *mut js_Ast,
    inst: libc::c_int,
) {
    let jump: *mut js_JumpList = js_malloc(
        J,
        ::core::mem::size_of::<js_JumpList>() as libc::c_ulong as libc::c_int,
    ) as *mut js_JumpList;
    (*jump).type_0 = type_0;
    (*jump).inst = inst;
    (*jump).next = (*target).jumps;
    (*target).jumps = jump;
}
unsafe extern "C" fn labeljumps(
    J: *mut js_State,
    F: *mut js_Function,
    stm: *mut js_Ast,
    baddr: libc::c_int,
    caddr: libc::c_int,
) {
    let mut jump: *mut js_JumpList = (*stm).jumps;
    while !jump.is_null() {
        let next_0: *mut js_JumpList = (*jump).next;
        if (*jump).type_0 as libc::c_uint == STM_BREAK as libc::c_int as libc::c_uint {
            labelto(J, F, (*jump).inst, baddr);
        }
        if (*jump).type_0 as libc::c_uint == STM_CONTINUE as libc::c_int as libc::c_uint {
            labelto(J, F, (*jump).inst, caddr);
        }
        js_free(J, jump as *mut libc::c_void);
        jump = next_0;
    }
    (*stm).jumps = std::ptr::null_mut::<js_JumpList>();
}
unsafe extern "C" fn isloop(T: js_AstType) -> libc::c_int {
    (T as libc::c_uint == STM_DO as libc::c_int as libc::c_uint
        || T as libc::c_uint == STM_WHILE as libc::c_int as libc::c_uint
        || T as libc::c_uint == STM_FOR as libc::c_int as libc::c_uint
        || T as libc::c_uint == STM_FOR_VAR as libc::c_int as libc::c_uint
        || T as libc::c_uint == STM_FOR_IN as libc::c_int as libc::c_uint
        || T as libc::c_uint == STM_FOR_IN_VAR as libc::c_int as libc::c_uint) as libc::c_int
}
unsafe extern "C" fn isfun(T: js_AstType) -> libc::c_int {
    (T as libc::c_uint == AST_FUNDEC as libc::c_int as libc::c_uint
        || T as libc::c_uint == EXP_FUN as libc::c_int as libc::c_uint
        || T as libc::c_uint == EXP_PROP_GET as libc::c_int as libc::c_uint
        || T as libc::c_uint == EXP_PROP_SET as libc::c_int as libc::c_uint) as libc::c_int
}
unsafe extern "C" fn matchlabel(
    mut node: *mut js_Ast,
    label_0: *const libc::c_char,
) -> libc::c_int {
    while !node.is_null()
        && (*node).type_0 as libc::c_uint == STM_LABEL as libc::c_int as libc::c_uint
    {
        if strcmp((*(*node).a).string, label_0) == 0 {
            return 1 as libc::c_int;
        }
        node = (*node).parent;
    }
    0 as libc::c_int
}
unsafe extern "C" fn breaktarget(
    J: *mut js_State,
    F: *mut js_Function,
    mut node: *mut js_Ast,
    label_0: *const libc::c_char,
) -> *mut js_Ast {
    while !node.is_null() {
        if isfun((*node).type_0) != 0 {
            break;
        }
        if label_0.is_null() {
            if isloop((*node).type_0) != 0
                || (*node).type_0 as libc::c_uint == STM_SWITCH as libc::c_int as libc::c_uint
            {
                return node;
            }
        } else if matchlabel((*node).parent, label_0) != 0 {
            return node;
        }
        node = (*node).parent;
    }
    std::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn continuetarget(
    J: *mut js_State,
    F: *mut js_Function,
    mut node: *mut js_Ast,
    label_0: *const libc::c_char,
) -> *mut js_Ast {
    while !node.is_null() {
        if isfun((*node).type_0) != 0 {
            break;
        }
        if isloop((*node).type_0) != 0 {
            if label_0.is_null() {
                return node;
            } else if matchlabel((*node).parent, label_0) != 0 {
                return node;
            }
        }
        node = (*node).parent;
    }
    std::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn returntarget(
    J: *mut js_State,
    F: *mut js_Function,
    mut node: *mut js_Ast,
) -> *mut js_Ast {
    while !node.is_null() {
        if isfun((*node).type_0) != 0 {
            return node;
        }
        node = (*node).parent;
    }
    std::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn cexit(
    J: *mut js_State,
    F: *mut js_Function,
    T: js_AstType,
    mut node: *mut js_Ast,
    target: *mut js_Ast,
) {
    let mut prev: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    loop {
        prev = node;
        node = (*node).parent;
        match (*node).type_0 as libc::c_uint {
            84 => {
                emitline(J, F, node);
                emit(J, F, OP_ENDWITH as libc::c_int);
            }
            79 | 80 => {
                emitline(J, F, node);
                if (*F).script != 0 {
                    if T as libc::c_uint == STM_RETURN as libc::c_int as libc::c_uint
                        || T as libc::c_uint == STM_BREAK as libc::c_int as libc::c_uint
                        || T as libc::c_uint == STM_CONTINUE as libc::c_int as libc::c_uint
                            && target != node
                    {
                        emit(J, F, OP_ROT2 as libc::c_int);
                        emit(J, F, OP_POP as libc::c_int);
                    }
                    if T as libc::c_uint == STM_CONTINUE as libc::c_int as libc::c_uint {
                        emit(J, F, OP_ROT2 as libc::c_int);
                    }
                } else {
                    if T as libc::c_uint == STM_RETURN as libc::c_int as libc::c_uint {
                        emit(J, F, OP_ROT2 as libc::c_int);
                        emit(J, F, OP_POP as libc::c_int);
                    }
                    if T as libc::c_uint == STM_BREAK as libc::c_int as libc::c_uint
                        || T as libc::c_uint == STM_CONTINUE as libc::c_int as libc::c_uint
                            && target != node
                    {
                        emit(J, F, OP_POP as libc::c_int);
                    }
                }
            }
            87 => {
                emitline(J, F, node);
                if prev == (*node).a {
                    emit(J, F, OP_ENDTRY as libc::c_int);
                    if !((*node).d).is_null() {
                        cstm(J, F, (*node).d);
                    }
                }
                if prev == (*node).c {
                    if !((*node).d).is_null() {
                        emit(J, F, OP_ENDCATCH as libc::c_int);
                        emit(J, F, OP_ENDTRY as libc::c_int);
                        cstm(J, F, (*node).d);
                    } else {
                        emit(J, F, OP_ENDCATCH as libc::c_int);
                    }
                }
            }
            _ => {}
        }
        if node == target {
            break;
        }
    }
}
unsafe extern "C" fn ctryfinally(
    J: *mut js_State,
    F: *mut js_Function,
    trystm: *mut js_Ast,
    finallystm: *mut js_Ast,
) {
    let mut L1: libc::c_int = 0;
    L1 = emitjump(J, F, OP_TRY as libc::c_int);
    cstm(J, F, finallystm);
    emit(J, F, OP_THROW as libc::c_int);
    label(J, F, L1);
    cstm(J, F, trystm);
    emit(J, F, OP_ENDTRY as libc::c_int);
    cstm(J, F, finallystm);
}
unsafe extern "C" fn ctrycatch(
    J: *mut js_State,
    F: *mut js_Function,
    trystm: *mut js_Ast,
    catchvar: *mut js_Ast,
    catchstm: *mut js_Ast,
) {
    let mut L1: libc::c_int = 0;
    let mut L2: libc::c_int = 0;
    L1 = emitjump(J, F, OP_TRY as libc::c_int);
    checkfutureword(J, F, catchvar);
    if (*F).strict != 0 {
        if strcmp(
            (*catchvar).string,
            b"arguments\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'arguments' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if strcmp(
            (*catchvar).string,
            b"eval\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'eval' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    emitline(J, F, catchvar);
    emitstring(J, F, OP_CATCH as libc::c_int, (*catchvar).string);
    cstm(J, F, catchstm);
    emit(J, F, OP_ENDCATCH as libc::c_int);
    L2 = emitjump(J, F, OP_JUMP as libc::c_int);
    label(J, F, L1);
    cstm(J, F, trystm);
    emit(J, F, OP_ENDTRY as libc::c_int);
    label(J, F, L2);
}
unsafe extern "C" fn ctrycatchfinally(
    J: *mut js_State,
    F: *mut js_Function,
    trystm: *mut js_Ast,
    catchvar: *mut js_Ast,
    catchstm: *mut js_Ast,
    finallystm: *mut js_Ast,
) {
    let mut L1: libc::c_int = 0;
    let mut L2: libc::c_int = 0;
    let mut L3: libc::c_int = 0;
    L1 = emitjump(J, F, OP_TRY as libc::c_int);
    L2 = emitjump(J, F, OP_TRY as libc::c_int);
    cstm(J, F, finallystm);
    emit(J, F, OP_THROW as libc::c_int);
    label(J, F, L2);
    if (*F).strict != 0 {
        checkfutureword(J, F, catchvar);
        if strcmp(
            (*catchvar).string,
            b"arguments\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'arguments' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if strcmp(
            (*catchvar).string,
            b"eval\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'eval' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    emitline(J, F, catchvar);
    emitstring(J, F, OP_CATCH as libc::c_int, (*catchvar).string);
    cstm(J, F, catchstm);
    emit(J, F, OP_ENDCATCH as libc::c_int);
    emit(J, F, OP_ENDTRY as libc::c_int);
    L3 = emitjump(J, F, OP_JUMP as libc::c_int);
    label(J, F, L1);
    cstm(J, F, trystm);
    emit(J, F, OP_ENDTRY as libc::c_int);
    label(J, F, L3);
    cstm(J, F, finallystm);
}
unsafe extern "C" fn cswitch(
    J: *mut js_State,
    F: *mut js_Function,
    ref_0: *mut js_Ast,
    head: *mut js_Ast,
) {
    let mut node: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut clause: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut def: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut end: libc::c_int = 0;
    jsC_cexp(J, F, ref_0);
    node = head;
    while !node.is_null() {
        clause = (*node).a;
        if (*clause).type_0 as libc::c_uint == STM_DEFAULT as libc::c_int as libc::c_uint {
            if !def.is_null() {
                jsC_error(
                    J,
                    clause,
                    b"more than one default label in switch\0" as *const u8 as *const libc::c_char,
                );
            }
            def = clause;
        } else {
            jsC_cexp(J, F, (*clause).a);
            emitline(J, F, clause);
            (*clause).casejump = emitjump(J, F, OP_JCASE as libc::c_int);
        }
        node = (*node).b;
    }
    emit(J, F, OP_POP as libc::c_int);
    if !def.is_null() {
        emitline(J, F, def);
        (*def).casejump = emitjump(J, F, OP_JUMP as libc::c_int);
        end = 0 as libc::c_int;
    } else {
        end = emitjump(J, F, OP_JUMP as libc::c_int);
    }
    node = head;
    while !node.is_null() {
        clause = (*node).a;
        label(J, F, (*clause).casejump);
        if (*clause).type_0 as libc::c_uint == STM_DEFAULT as libc::c_int as libc::c_uint {
            cstmlist(J, F, (*clause).a);
        } else {
            cstmlist(J, F, (*clause).b);
        }
        node = (*node).b;
    }
    if end != 0 {
        label(J, F, end);
    }
}
unsafe extern "C" fn cvarinit(J: *mut js_State, F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        let var: *mut js_Ast = (*list).a;
        if !((*var).b).is_null() {
            jsC_cexp(J, F, (*var).b);
            emitline(J, F, var);
            emitlocal(
                J,
                F,
                OP_SETLOCAL as libc::c_int,
                OP_SETVAR as libc::c_int,
                (*var).a,
            );
            emit(J, F, OP_POP as libc::c_int);
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cstm(J: *mut js_State, F: *mut js_Function, mut stm: *mut js_Ast) {
    let mut target: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut loop_0: libc::c_int = 0;
    let mut cont: libc::c_int = 0;
    let mut then: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    emitline(J, F, stm);
    match (*stm).type_0 as libc::c_uint {
        1 => {}
        71 => {
            cstmlist(J, F, (*stm).a);
        }
        72 => {
            if (*F).script != 0 {
                emitline(J, F, stm);
                emit(J, F, OP_POP as libc::c_int);
                emit(J, F, OP_UNDEF as libc::c_int);
            }
        }
        73 => {
            cvarinit(J, F, (*stm).a);
        }
        74 => {
            if !((*stm).c).is_null() {
                jsC_cexp(J, F, (*stm).a);
                emitline(J, F, stm);
                then = emitjump(J, F, OP_JTRUE as libc::c_int);
                cstm(J, F, (*stm).c);
                emitline(J, F, stm);
                end = emitjump(J, F, OP_JUMP as libc::c_int);
                label(J, F, then);
                cstm(J, F, (*stm).b);
                label(J, F, end);
            } else {
                jsC_cexp(J, F, (*stm).a);
                emitline(J, F, stm);
                end = emitjump(J, F, OP_JFALSE as libc::c_int);
                cstm(J, F, (*stm).b);
                label(J, F, end);
            }
        }
        75 => {
            loop_0 = here(J, F);
            cstm(J, F, (*stm).a);
            cont = here(J, F);
            jsC_cexp(J, F, (*stm).b);
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JTRUE as libc::c_int, loop_0);
            labeljumps(J, F, stm, here(J, F), cont);
        }
        76 => {
            loop_0 = here(J, F);
            jsC_cexp(J, F, (*stm).a);
            emitline(J, F, stm);
            end = emitjump(J, F, OP_JFALSE as libc::c_int);
            cstm(J, F, (*stm).b);
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JUMP as libc::c_int, loop_0);
            label(J, F, end);
            labeljumps(J, F, stm, here(J, F), loop_0);
        }
        77 | 78 => {
            if (*stm).type_0 as libc::c_uint == STM_FOR_VAR as libc::c_int as libc::c_uint {
                cvarinit(J, F, (*stm).a);
            } else if !((*stm).a).is_null() {
                jsC_cexp(J, F, (*stm).a);
                emit(J, F, OP_POP as libc::c_int);
            }
            loop_0 = here(J, F);
            if !((*stm).b).is_null() {
                jsC_cexp(J, F, (*stm).b);
                emitline(J, F, stm);
                end = emitjump(J, F, OP_JFALSE as libc::c_int);
            } else {
                end = 0 as libc::c_int;
            }
            cstm(J, F, (*stm).d);
            cont = here(J, F);
            if !((*stm).c).is_null() {
                jsC_cexp(J, F, (*stm).c);
                emit(J, F, OP_POP as libc::c_int);
            }
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JUMP as libc::c_int, loop_0);
            if end != 0 {
                label(J, F, end);
            }
            labeljumps(J, F, stm, here(J, F), cont);
        }
        79 | 80 => {
            jsC_cexp(J, F, (*stm).b);
            emitline(J, F, stm);
            emit(J, F, OP_ITERATOR as libc::c_int);
            loop_0 = here(J, F);
            emitline(J, F, stm);
            emit(J, F, OP_NEXTITER as libc::c_int);
            end = emitjump(J, F, OP_JFALSE as libc::c_int);
            cassignforin(J, F, stm);
            if (*F).script != 0 {
                emit(J, F, OP_ROT2 as libc::c_int);
                cstm(J, F, (*stm).c);
                emit(J, F, OP_ROT2 as libc::c_int);
            } else {
                cstm(J, F, (*stm).c);
            }
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JUMP as libc::c_int, loop_0);
            label(J, F, end);
            labeljumps(J, F, stm, here(J, F), loop_0);
        }
        85 => {
            cswitch(J, F, (*stm).a, (*stm).b);
            labeljumps(J, F, stm, here(J, F), 0 as libc::c_int);
        }
        89 => {
            cstm(J, F, (*stm).b);
            while (*stm).type_0 as libc::c_uint == STM_LABEL as libc::c_int as libc::c_uint {
                stm = (*stm).b;
            }
            if isloop((*stm).type_0) == 0
                && (*stm).type_0 as libc::c_uint != STM_SWITCH as libc::c_int as libc::c_uint
            {
                labeljumps(J, F, stm, here(J, F), 0 as libc::c_int);
            }
        }
        82 => {
            if !((*stm).a).is_null() {
                checkfutureword(J, F, (*stm).a);
                target = breaktarget(J, F, (*stm).parent, (*(*stm).a).string);
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"break label '%s' not found\0" as *const u8 as *const libc::c_char,
                        (*(*stm).a).string,
                    );
                }
            } else {
                target = breaktarget(J, F, (*stm).parent, std::ptr::null::<libc::c_char>());
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"unlabelled break must be inside loop or switch\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            cexit(J, F, STM_BREAK, stm, target);
            emitline(J, F, stm);
            addjump(
                J,
                F,
                STM_BREAK,
                target,
                emitjump(J, F, OP_JUMP as libc::c_int),
            );
        }
        81 => {
            if !((*stm).a).is_null() {
                checkfutureword(J, F, (*stm).a);
                target = continuetarget(J, F, (*stm).parent, (*(*stm).a).string);
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"continue label '%s' not found\0" as *const u8 as *const libc::c_char,
                        (*(*stm).a).string,
                    );
                }
            } else {
                target = continuetarget(J, F, (*stm).parent, std::ptr::null::<libc::c_char>());
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"continue must be inside loop\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            cexit(J, F, STM_CONTINUE, stm, target);
            emitline(J, F, stm);
            addjump(
                J,
                F,
                STM_CONTINUE,
                target,
                emitjump(J, F, OP_JUMP as libc::c_int),
            );
        }
        83 => {
            if !((*stm).a).is_null() {
                jsC_cexp(J, F, (*stm).a);
            } else {
                emit(J, F, OP_UNDEF as libc::c_int);
            }
            target = returntarget(J, F, (*stm).parent);
            if target.is_null() {
                jsC_error(
                    J,
                    stm,
                    b"return not in function\0" as *const u8 as *const libc::c_char,
                );
            }
            cexit(J, F, STM_RETURN, stm, target);
            emitline(J, F, stm);
            emit(J, F, OP_RETURN as libc::c_int);
        }
        86 => {
            jsC_cexp(J, F, (*stm).a);
            emitline(J, F, stm);
            emit(J, F, OP_THROW as libc::c_int);
        }
        84 => {
            (*F).lightweight = 0 as libc::c_int;
            if (*F).strict != 0 {
                jsC_error(
                    J,
                    (*stm).a,
                    b"'with' statements are not allowed in strict mode\0" as *const u8
                        as *const libc::c_char,
                );
            }
            jsC_cexp(J, F, (*stm).a);
            emitline(J, F, stm);
            emit(J, F, OP_WITH as libc::c_int);
            cstm(J, F, (*stm).b);
            emitline(J, F, stm);
            emit(J, F, OP_ENDWITH as libc::c_int);
        }
        87 => {
            emitline(J, F, stm);
            if !((*stm).b).is_null() && !((*stm).c).is_null() {
                (*F).lightweight = 0 as libc::c_int;
                if !((*stm).d).is_null() {
                    ctrycatchfinally(J, F, (*stm).a, (*stm).b, (*stm).c, (*stm).d);
                } else {
                    ctrycatch(J, F, (*stm).a, (*stm).b, (*stm).c);
                }
            } else {
                ctryfinally(J, F, (*stm).a, (*stm).d);
            }
        }
        88 => {
            emitline(J, F, stm);
            emit(J, F, OP_DEBUGGER as libc::c_int);
        }
        _ => {
            if (*F).script != 0 {
                emitline(J, F, stm);
                emit(J, F, OP_POP as libc::c_int);
                jsC_cexp(J, F, stm);
            } else {
                jsC_cexp(J, F, stm);
                emitline(J, F, stm);
                emit(J, F, OP_POP as libc::c_int);
            }
        }
    };
}
unsafe extern "C" fn cstmlist(J: *mut js_State, F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        cstm(J, F, (*list).a);
        list = (*list).b;
    }
}
unsafe extern "C" fn listlength(mut list: *mut js_Ast) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    while !list.is_null() {
        n += 1;
        n;
        list = (*list).b;
    }
    n
}
unsafe extern "C" fn cparams(
    J: *mut js_State,
    F: *mut js_Function,
    mut list: *mut js_Ast,
    fname: *mut js_Ast,
) {
    (*F).numparams = listlength(list);
    while !list.is_null() {
        checkfutureword(J, F, (*list).a);
        addlocal(J, F, (*list).a, 0 as libc::c_int);
        list = (*list).b;
    }
}
unsafe extern "C" fn cvardecs(J: *mut js_State, F: *mut js_Function, mut node: *mut js_Ast) {
    if (*node).type_0 as libc::c_uint == AST_LIST as libc::c_int as libc::c_uint {
        while !node.is_null() {
            cvardecs(J, F, (*node).a);
            node = (*node).b;
        }
        return;
    }
    if isfun((*node).type_0) != 0 {
        return;
    }
    if (*node).type_0 as libc::c_uint == EXP_VAR as libc::c_int as libc::c_uint {
        checkfutureword(J, F, (*node).a);
        addlocal(J, F, (*node).a, 1 as libc::c_int);
    }
    if !((*node).a).is_null() {
        cvardecs(J, F, (*node).a);
    }
    if !((*node).b).is_null() {
        cvardecs(J, F, (*node).b);
    }
    if !((*node).c).is_null() {
        cvardecs(J, F, (*node).c);
    }
    if !((*node).d).is_null() {
        cvardecs(J, F, (*node).d);
    }
}
unsafe extern "C" fn cfundecs(J: *mut js_State, F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        let stm: *mut js_Ast = (*list).a;
        if (*stm).type_0 as libc::c_uint == AST_FUNDEC as libc::c_int as libc::c_uint {
            emitline(J, F, stm);
            emitfunction(
                J,
                F,
                newfun(
                    J,
                    (*stm).line,
                    (*stm).a,
                    (*stm).b,
                    (*stm).c,
                    0 as libc::c_int,
                    (*F).strict,
                    0 as libc::c_int,
                ),
            );
            emitline(J, F, stm);
            emit(J, F, OP_SETLOCAL as libc::c_int);
            emitarg(J, F, addlocal(J, F, (*stm).a, 1 as libc::c_int));
            emit(J, F, OP_POP as libc::c_int);
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cfunbody(
    J: *mut js_State,
    F: *mut js_Function,
    name: *mut js_Ast,
    params: *mut js_Ast,
    body: *mut js_Ast,
    is_fun_exp: libc::c_int,
) {
    (*F).lightweight = 1 as libc::c_int;
    (*F).arguments = 0 as libc::c_int;
    if (*F).script != 0 {
        (*F).lightweight = 0 as libc::c_int;
    }
    if !body.is_null()
        && (*body).type_0 as libc::c_uint == AST_LIST as libc::c_int as libc::c_uint
        && !((*body).a).is_null()
        && (*(*body).a).type_0 as libc::c_uint == EXP_STRING as libc::c_int as libc::c_uint
        && strcmp(
            (*(*body).a).string,
            b"use strict\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        (*F).strict = 1 as libc::c_int;
    }
    (*F).lastline = (*F).line;
    cparams(J, F, params, name);
    if !body.is_null() {
        cvardecs(J, F, body);
        cfundecs(J, F, body);
    }
    if !name.is_null() {
        checkfutureword(J, F, name);
        if is_fun_exp != 0 && findlocal(J, F, (*name).string) < 0 as libc::c_int {
            emit(J, F, OP_CURRENT as libc::c_int);
            emit(J, F, OP_SETLOCAL as libc::c_int);
            emitarg(J, F, addlocal(J, F, name, 1 as libc::c_int));
            emit(J, F, OP_POP as libc::c_int);
        }
    }
    if (*F).script != 0 {
        emit(J, F, OP_UNDEF as libc::c_int);
        cstmlist(J, F, body);
        emit(J, F, OP_RETURN as libc::c_int);
    } else {
        cstmlist(J, F, body);
        emit(J, F, OP_UNDEF as libc::c_int);
        emit(J, F, OP_RETURN as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsC_compilefunction(
    J: *mut js_State,
    prog: *mut js_Ast,
) -> *mut js_Function {
    newfun(
        J,
        (*prog).line,
        (*prog).a,
        (*prog).b,
        (*prog).c,
        0 as libc::c_int,
        (*J).default_strict,
        1 as libc::c_int,
    )
}
#[no_mangle]
pub unsafe extern "C" fn jsC_compilescript(
    J: *mut js_State,
    prog: *mut js_Ast,
    default_strict: libc::c_int,
) -> *mut js_Function {
    newfun(
        J,
        if !prog.is_null() {
            (*prog).line
        } else {
            0 as libc::c_int
        },
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        prog,
        1 as libc::c_int,
        default_strict,
        0 as libc::c_int,
    )
}
unsafe extern "C" fn Now() -> libc::c_double {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut tv, std::ptr::null_mut::<libc::c_void>());
    floor(tv.tv_sec as libc::c_double * 1000.0f64 + tv.tv_usec as libc::c_double / 1000.0f64)
}
unsafe extern "C" fn LocalTZA() -> libc::c_double {
    static mut once: libc::c_int = 1 as libc::c_int;
    static mut tza: libc::c_double = 0 as libc::c_int as libc::c_double;
    if once != 0 {
        let mut now: time_t = time(std::ptr::null_mut::<time_t>());
        let utc: time_t = mktime(gmtime(&mut now));
        let loc: time_t = mktime(localtime(&mut now));
        tza = ((loc - utc) * 1000 as libc::c_int as libc::c_long) as libc::c_double;
        once = 0 as libc::c_int;
    }
    tza
}
unsafe extern "C" fn DaylightSavingTA(t: libc::c_double) -> libc::c_double {
    0 as libc::c_int as libc::c_double
}
unsafe extern "C" fn pmod(mut x: libc::c_double, y: libc::c_double) -> libc::c_double {
    x = fmod(x, y);
    if x < 0 as libc::c_int as libc::c_double {
        x += y;
    }
    x
}
unsafe extern "C" fn Day(t: libc::c_double) -> libc::c_int {
    floor(t / (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64)) as libc::c_int
}
unsafe extern "C" fn TimeWithinDay(t: libc::c_double) -> libc::c_double {
    pmod(t, 24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64)
}
unsafe extern "C" fn DaysInYear(y: libc::c_int) -> libc::c_int {
    if y % 4 as libc::c_int == 0 as libc::c_int
        && (y % 100 as libc::c_int != 0 || y % 400 as libc::c_int == 0 as libc::c_int)
    {
        366 as libc::c_int
    } else {
        365 as libc::c_int
    }
}
unsafe extern "C" fn DayFromYear(y: libc::c_int) -> libc::c_int {
    ((365 as libc::c_int * (y - 1970 as libc::c_int)) as libc::c_double
        + floor((y - 1969 as libc::c_int) as libc::c_double / 4.0f64)
        - floor((y - 1901 as libc::c_int) as libc::c_double / 100.0f64)
        + floor((y - 1601 as libc::c_int) as libc::c_double / 400.0f64)) as libc::c_int
}
unsafe extern "C" fn TimeFromYear(y: libc::c_int) -> libc::c_double {
    DayFromYear(y) as libc::c_double * (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64)
}
unsafe extern "C" fn YearFromTime(t: libc::c_double) -> libc::c_int {
    let mut y: libc::c_int = (floor(t / (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64 * 365.2425f64))
        + 1970 as libc::c_int as libc::c_double) as libc::c_int;
    let t2: libc::c_double = TimeFromYear(y);
    if t2 > t {
        y -= 1;
        y;
    } else if t2 + 24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64 * DaysInYear(y) as libc::c_double <= t {
        y += 1;
        y;
    }
    y
}
unsafe extern "C" fn InLeapYear(t: libc::c_double) -> libc::c_int {
    (DaysInYear(YearFromTime(t)) == 366 as libc::c_int) as libc::c_int
}
unsafe extern "C" fn DayWithinYear(t: libc::c_double) -> libc::c_int {
    Day(t) - DayFromYear(YearFromTime(t))
}
unsafe extern "C" fn MonthFromTime(t: libc::c_double) -> libc::c_int {
    let day: libc::c_int = DayWithinYear(t);
    let leap: libc::c_int = InLeapYear(t);
    if day < 31 as libc::c_int {
        return 0 as libc::c_int;
    }
    if day < 59 as libc::c_int + leap {
        return 1 as libc::c_int;
    }
    if day < 90 as libc::c_int + leap {
        return 2 as libc::c_int;
    }
    if day < 120 as libc::c_int + leap {
        return 3 as libc::c_int;
    }
    if day < 151 as libc::c_int + leap {
        return 4 as libc::c_int;
    }
    if day < 181 as libc::c_int + leap {
        return 5 as libc::c_int;
    }
    if day < 212 as libc::c_int + leap {
        return 6 as libc::c_int;
    }
    if day < 243 as libc::c_int + leap {
        return 7 as libc::c_int;
    }
    if day < 273 as libc::c_int + leap {
        return 8 as libc::c_int;
    }
    if day < 304 as libc::c_int + leap {
        return 9 as libc::c_int;
    }
    if day < 334 as libc::c_int + leap {
        return 10 as libc::c_int;
    }
    11 as libc::c_int
}
unsafe extern "C" fn DateFromTime(t: libc::c_double) -> libc::c_int {
    let day: libc::c_int = DayWithinYear(t);
    let leap: libc::c_int = InLeapYear(t);
    match MonthFromTime(t) {
        0 => day + 1 as libc::c_int,
        1 => day - 30 as libc::c_int,
        2 => day - 58 as libc::c_int - leap,
        3 => day - 89 as libc::c_int - leap,
        4 => day - 119 as libc::c_int - leap,
        5 => day - 150 as libc::c_int - leap,
        6 => day - 180 as libc::c_int - leap,
        7 => day - 211 as libc::c_int - leap,
        8 => day - 242 as libc::c_int - leap,
        9 => day - 272 as libc::c_int - leap,
        10 => day - 303 as libc::c_int - leap,
        _ => day - 333 as libc::c_int - leap,
    }
}
unsafe extern "C" fn WeekDay(t: libc::c_double) -> libc::c_int {
    pmod(
        (Day(t) + 4 as libc::c_int) as libc::c_double,
        7 as libc::c_int as libc::c_double,
    ) as libc::c_int
}
unsafe extern "C" fn LocalTime(utc: libc::c_double) -> libc::c_double {
    utc + LocalTZA() + DaylightSavingTA(utc)
}
unsafe extern "C" fn UTC(loc: libc::c_double) -> libc::c_double {
    loc - LocalTZA() - DaylightSavingTA(loc - LocalTZA())
}
unsafe extern "C" fn HourFromTime(t: libc::c_double) -> libc::c_int {
    pmod(floor(t / (60.0f64 * 60.0f64 * 1000.0f64)), 24.0f64) as libc::c_int
}
unsafe extern "C" fn MinFromTime(t: libc::c_double) -> libc::c_int {
    pmod(floor(t / (60.0f64 * 1000.0f64)), 60.0f64) as libc::c_int
}
unsafe extern "C" fn SecFromTime(t: libc::c_double) -> libc::c_int {
    pmod(floor(t / 1000.0f64), 60.0f64) as libc::c_int
}
unsafe extern "C" fn msFromTime(t: libc::c_double) -> libc::c_int {
    pmod(t, 1000.0f64) as libc::c_int
}
unsafe extern "C" fn MakeTime(
    hour: libc::c_double,
    min: libc::c_double,
    sec: libc::c_double,
    ms: libc::c_double,
) -> libc::c_double {
    ((hour * 60.0f64 + min) * 60.0f64 + sec) * 1000.0f64 + ms
}
unsafe extern "C" fn MakeDay(
    mut y: libc::c_double,
    mut m: libc::c_double,
    date: libc::c_double,
) -> libc::c_double {
    static mut firstDayOfMonth: [[libc::c_double; 12]; 2] = [
        [
            0 as libc::c_int as libc::c_double,
            31 as libc::c_int as libc::c_double,
            59 as libc::c_int as libc::c_double,
            90 as libc::c_int as libc::c_double,
            120 as libc::c_int as libc::c_double,
            151 as libc::c_int as libc::c_double,
            181 as libc::c_int as libc::c_double,
            212 as libc::c_int as libc::c_double,
            243 as libc::c_int as libc::c_double,
            273 as libc::c_int as libc::c_double,
            304 as libc::c_int as libc::c_double,
            334 as libc::c_int as libc::c_double,
        ],
        [
            0 as libc::c_int as libc::c_double,
            31 as libc::c_int as libc::c_double,
            60 as libc::c_int as libc::c_double,
            91 as libc::c_int as libc::c_double,
            121 as libc::c_int as libc::c_double,
            152 as libc::c_int as libc::c_double,
            182 as libc::c_int as libc::c_double,
            213 as libc::c_int as libc::c_double,
            244 as libc::c_int as libc::c_double,
            274 as libc::c_int as libc::c_double,
            305 as libc::c_int as libc::c_double,
            335 as libc::c_int as libc::c_double,
        ],
    ];
    let mut yd: libc::c_double = 0.;
    let mut md: libc::c_double = 0.;
    let mut im: libc::c_int = 0;
    y += floor(m / 12 as libc::c_int as libc::c_double);
    m = pmod(m, 12 as libc::c_int as libc::c_double);
    im = m as libc::c_int;
    if im < 0 as libc::c_int || im >= 12 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    yd = floor(TimeFromYear(y as libc::c_int) / (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64));
    md = firstDayOfMonth
        [(DaysInYear(y as libc::c_int) == 366 as libc::c_int) as libc::c_int as usize][im as usize];
    yd + md + date - 1 as libc::c_int as libc::c_double
}
unsafe extern "C" fn MakeDate(day: libc::c_double, time_0: libc::c_double) -> libc::c_double {
    day * (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64) + time_0
}
unsafe extern "C" fn TimeClip(t: libc::c_double) -> libc::c_double {
    if t.is_finite() as i32 == 0 {
        return ::core::f32::NAN as libc::c_double;
    }
    if fabs(t) > 8.64e15f64 {
        return ::core::f32::NAN as libc::c_double;
    }
    if t < 0 as libc::c_int as libc::c_double {
        -floor(-t)
    } else {
        floor(t)
    }
}
unsafe extern "C" fn toint(
    sp: *mut *const libc::c_char,
    mut w: libc::c_int,
    v: *mut libc::c_int,
) -> libc::c_int {
    let mut s: *const libc::c_char = *sp;
    *v = 0 as libc::c_int;
    loop {
        let fresh14 = w;
        w -= 1;
        if fresh14 == 0 {
            break;
        }
        if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
            return 0 as libc::c_int;
        }
        let fresh15 = s;
        s = s.offset(1);
        *v = *v * 10 as libc::c_int + (*fresh15 as libc::c_int - '0' as i32);
    }
    *sp = s;
    1 as libc::c_int
}
unsafe extern "C" fn parseDateTime(mut s: *const libc::c_char) -> libc::c_double {
    let mut y: libc::c_int = 1970 as libc::c_int;
    let mut m: libc::c_int = 1 as libc::c_int;
    let mut d: libc::c_int = 1 as libc::c_int;
    let mut H: libc::c_int = 0 as libc::c_int;
    let mut M: libc::c_int = 0 as libc::c_int;
    let mut S: libc::c_int = 0 as libc::c_int;
    let mut ms: libc::c_int = 0 as libc::c_int;
    let mut tza: libc::c_int = 0 as libc::c_int;
    let mut t: libc::c_double = 0.;
    if toint(&mut s, 4 as libc::c_int, &mut y) == 0 {
        return ::core::f32::NAN as libc::c_double;
    }
    if *s as libc::c_int == '-' as i32 {
        s = s.offset(1 as libc::c_int as isize);
        if toint(&mut s, 2 as libc::c_int, &mut m) == 0 {
            return ::core::f32::NAN as libc::c_double;
        }
        if *s as libc::c_int == '-' as i32 {
            s = s.offset(1 as libc::c_int as isize);
            if toint(&mut s, 2 as libc::c_int, &mut d) == 0 {
                return ::core::f32::NAN as libc::c_double;
            }
        }
    }
    if *s as libc::c_int == 'T' as i32 {
        s = s.offset(1 as libc::c_int as isize);
        if toint(&mut s, 2 as libc::c_int, &mut H) == 0 {
            return ::core::f32::NAN as libc::c_double;
        }
        if *s as libc::c_int != ':' as i32 {
            return ::core::f32::NAN as libc::c_double;
        }
        s = s.offset(1 as libc::c_int as isize);
        if toint(&mut s, 2 as libc::c_int, &mut M) == 0 {
            return ::core::f32::NAN as libc::c_double;
        }
        if *s as libc::c_int == ':' as i32 {
            s = s.offset(1 as libc::c_int as isize);
            if toint(&mut s, 2 as libc::c_int, &mut S) == 0 {
                return ::core::f32::NAN as libc::c_double;
            }
            if *s as libc::c_int == '.' as i32 {
                s = s.offset(1 as libc::c_int as isize);
                if toint(&mut s, 3 as libc::c_int, &mut ms) == 0 {
                    return ::core::f32::NAN as libc::c_double;
                }
            }
        }
        if *s as libc::c_int == 'Z' as i32 {
            s = s.offset(1 as libc::c_int as isize);
            tza = 0 as libc::c_int;
        } else if *s as libc::c_int == '+' as i32 || *s as libc::c_int == '-' as i32 {
            let mut tzh: libc::c_int = 0 as libc::c_int;
            let mut tzm: libc::c_int = 0 as libc::c_int;
            let tzs: libc::c_int = if *s as libc::c_int == '+' as i32 {
                1 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
            s = s.offset(1 as libc::c_int as isize);
            if toint(&mut s, 2 as libc::c_int, &mut tzh) == 0 {
                return ::core::f32::NAN as libc::c_double;
            }
            if *s as libc::c_int == ':' as i32 {
                s = s.offset(1 as libc::c_int as isize);
                if toint(&mut s, 2 as libc::c_int, &mut tzm) == 0 {
                    return ::core::f32::NAN as libc::c_double;
                }
            }
            if tzh > 23 as libc::c_int || tzm > 59 as libc::c_int {
                return ::core::f32::NAN as libc::c_double;
            }
            tza = (tzs as libc::c_double
                * (tzh as libc::c_double * (60.0f64 * 60.0f64 * 1000.0f64)
                    + tzm as libc::c_double * (60.0f64 * 1000.0f64)))
                as libc::c_int;
        } else {
            tza = LocalTZA() as libc::c_int;
        }
    }
    if *s != 0 {
        return ::core::f32::NAN as libc::c_double;
    }
    if m < 1 as libc::c_int || m > 12 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    if d < 1 as libc::c_int || d > 31 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    if H < 0 as libc::c_int || H > 24 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    if M < 0 as libc::c_int || M > 59 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    if S < 0 as libc::c_int || S > 59 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    if ms < 0 as libc::c_int || ms > 999 as libc::c_int {
        return ::core::f32::NAN as libc::c_double;
    }
    if H == 24 as libc::c_int
        && (M != 0 as libc::c_int || S != 0 as libc::c_int || ms != 0 as libc::c_int)
    {
        return ::core::f32::NAN as libc::c_double;
    }
    t = MakeDate(
        MakeDay(
            y as libc::c_double,
            (m - 1 as libc::c_int) as libc::c_double,
            d as libc::c_double,
        ),
        MakeTime(
            H as libc::c_double,
            M as libc::c_double,
            S as libc::c_double,
            ms as libc::c_double,
        ),
    );
    t - tza as libc::c_double
}
unsafe extern "C" fn fmtdate(buf: *mut libc::c_char, t: libc::c_double) -> *mut libc::c_char {
    let y: libc::c_int = YearFromTime(t);
    let m: libc::c_int = MonthFromTime(t);
    let d: libc::c_int = DateFromTime(t);
    if t.is_finite() as i32 == 0 {
        return b"Invalid Date\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sprintf(
        buf,
        b"%04d-%02d-%02d\0" as *const u8 as *const libc::c_char,
        y,
        m + 1 as libc::c_int,
        d,
    );
    buf
}
unsafe extern "C" fn fmttime(
    buf: *mut libc::c_char,
    t: libc::c_double,
    tza: libc::c_double,
) -> *mut libc::c_char {
    let H: libc::c_int = HourFromTime(t);
    let M: libc::c_int = MinFromTime(t);
    let S: libc::c_int = SecFromTime(t);
    let ms: libc::c_int = msFromTime(t);
    let tzh: libc::c_int = HourFromTime(fabs(tza));
    let tzm: libc::c_int = MinFromTime(fabs(tza));
    if t.is_finite() as i32 == 0 {
        return b"Invalid Date\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if tza == 0 as libc::c_int as libc::c_double {
        sprintf(
            buf,
            b"%02d:%02d:%02d.%03dZ\0" as *const u8 as *const libc::c_char,
            H,
            M,
            S,
            ms,
        );
    } else if tza < 0 as libc::c_int as libc::c_double {
        sprintf(
            buf,
            b"%02d:%02d:%02d.%03d-%02d:%02d\0" as *const u8 as *const libc::c_char,
            H,
            M,
            S,
            ms,
            tzh,
            tzm,
        );
    } else {
        sprintf(
            buf,
            b"%02d:%02d:%02d.%03d+%02d:%02d\0" as *const u8 as *const libc::c_char,
            H,
            M,
            S,
            ms,
            tzh,
            tzm,
        );
    }
    buf
}
unsafe extern "C" fn fmtdatetime(
    buf: *mut libc::c_char,
    t: libc::c_double,
    tza: libc::c_double,
) -> *mut libc::c_char {
    let mut dbuf: [libc::c_char; 20] = [0; 20];
    let mut tbuf: [libc::c_char; 20] = [0; 20];
    if t.is_finite() as i32 == 0 {
        return b"Invalid Date\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    fmtdate(dbuf.as_mut_ptr(), t);
    fmttime(tbuf.as_mut_ptr(), t, tza);
    sprintf(
        buf,
        b"%sT%s\0" as *const u8 as *const libc::c_char,
        dbuf.as_mut_ptr(),
        tbuf.as_mut_ptr(),
    );
    buf
}
unsafe extern "C" fn js_todate(J: *mut js_State, idx: libc::c_int) -> libc::c_double {
    let self_0: *mut js_Object = js_toobject(J, idx);
    if (*self_0).type_0 as libc::c_uint != JS_CDATE as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a date\0" as *const u8 as *const libc::c_char);
    }
    (*self_0).u.number
}
unsafe extern "C" fn js_setdate(J: *mut js_State, idx: libc::c_int, t: libc::c_double) {
    let self_0: *mut js_Object = js_toobject(J, idx);
    if (*self_0).type_0 as libc::c_uint != JS_CDATE as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a date\0" as *const u8 as *const libc::c_char);
    }
    (*self_0).u.number = TimeClip(t);
    js_pushnumber(J, (*self_0).u.number);
}
unsafe extern "C" fn D_parse(J: *mut js_State) {
    let t: libc::c_double = parseDateTime(js_tostring(J, 1 as libc::c_int));
    js_pushnumber(J, t);
}
unsafe extern "C" fn D_UTC(J: *mut js_State) {
    let mut y: libc::c_double = 0.;
    let mut m: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut H: libc::c_double = 0.;
    let mut M: libc::c_double = 0.;
    let mut S: libc::c_double = 0.;
    let mut ms: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    y = js_tonumber(J, 1 as libc::c_int);
    if y < 100 as libc::c_int as libc::c_double {
        y += 1900 as libc::c_int as libc::c_double;
    }
    m = js_tonumber(J, 2 as libc::c_int);
    d = if js_isdefined(J, 3 as libc::c_int) != 0 {
        js_tonumber(J, 3 as libc::c_int)
    } else {
        1 as libc::c_int as libc::c_double
    };
    H = if js_isdefined(J, 4 as libc::c_int) != 0 {
        js_tonumber(J, 4 as libc::c_int)
    } else {
        0 as libc::c_int as libc::c_double
    };
    M = if js_isdefined(J, 5 as libc::c_int) != 0 {
        js_tonumber(J, 5 as libc::c_int)
    } else {
        0 as libc::c_int as libc::c_double
    };
    S = if js_isdefined(J, 6 as libc::c_int) != 0 {
        js_tonumber(J, 6 as libc::c_int)
    } else {
        0 as libc::c_int as libc::c_double
    };
    ms = if js_isdefined(J, 7 as libc::c_int) != 0 {
        js_tonumber(J, 7 as libc::c_int)
    } else {
        0 as libc::c_int as libc::c_double
    };
    t = MakeDate(MakeDay(y, m, d), MakeTime(H, M, S, ms));
    t = TimeClip(t);
    js_pushnumber(J, t);
}
unsafe extern "C" fn D_now(J: *mut js_State) {
    js_pushnumber(J, Now());
}
unsafe extern "C" fn jsB_Date(J: *mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    js_pushstring(
        J,
        fmtdatetime(buf.as_mut_ptr(), LocalTime(Now()), LocalTZA()),
    );
}
unsafe extern "C" fn jsB_new_Date(J: *mut js_State) {
    let top: libc::c_int = js_gettop(J);
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut t: libc::c_double = 0.;
    if top == 1 as libc::c_int {
        t = Now();
    } else if top == 2 as libc::c_int {
        js_toprimitive(J, 1 as libc::c_int, JS_HNONE as libc::c_int);
        if js_isstring(J, 1 as libc::c_int) != 0 {
            t = parseDateTime(js_tostring(J, 1 as libc::c_int));
        } else {
            t = TimeClip(js_tonumber(J, 1 as libc::c_int));
        }
    } else {
        let mut y: libc::c_double = 0.;
        let mut m: libc::c_double = 0.;
        let mut d: libc::c_double = 0.;
        let mut H: libc::c_double = 0.;
        let mut M: libc::c_double = 0.;
        let mut S: libc::c_double = 0.;
        let mut ms: libc::c_double = 0.;
        y = js_tonumber(J, 1 as libc::c_int);
        if y < 100 as libc::c_int as libc::c_double {
            y += 1900 as libc::c_int as libc::c_double;
        }
        m = js_tonumber(J, 2 as libc::c_int);
        d = if js_isdefined(J, 3 as libc::c_int) != 0 {
            js_tonumber(J, 3 as libc::c_int)
        } else {
            1 as libc::c_int as libc::c_double
        };
        H = if js_isdefined(J, 4 as libc::c_int) != 0 {
            js_tonumber(J, 4 as libc::c_int)
        } else {
            0 as libc::c_int as libc::c_double
        };
        M = if js_isdefined(J, 5 as libc::c_int) != 0 {
            js_tonumber(J, 5 as libc::c_int)
        } else {
            0 as libc::c_int as libc::c_double
        };
        S = if js_isdefined(J, 6 as libc::c_int) != 0 {
            js_tonumber(J, 6 as libc::c_int)
        } else {
            0 as libc::c_int as libc::c_double
        };
        ms = if js_isdefined(J, 7 as libc::c_int) != 0 {
            js_tonumber(J, 7 as libc::c_int)
        } else {
            0 as libc::c_int as libc::c_double
        };
        t = MakeDate(MakeDay(y, m, d), MakeTime(H, M, S, ms));
        t = TimeClip(UTC(t));
    }
    obj = jsV_newobject(J, JS_CDATE, (*J).Date_prototype);
    (*obj).u.number = t;
    js_pushobject(J, obj);
}
unsafe extern "C" fn Dp_valueOf(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    js_pushnumber(J, t);
}
unsafe extern "C" fn Dp_toString(J: *mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    js_pushstring(J, fmtdatetime(buf.as_mut_ptr(), LocalTime(t), LocalTZA()));
}
unsafe extern "C" fn Dp_toDateString(J: *mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    js_pushstring(J, fmtdate(buf.as_mut_ptr(), LocalTime(t)));
}
unsafe extern "C" fn Dp_toTimeString(J: *mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    js_pushstring(J, fmttime(buf.as_mut_ptr(), LocalTime(t), LocalTZA()));
}
unsafe extern "C" fn Dp_toUTCString(J: *mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    js_pushstring(
        J,
        fmtdatetime(buf.as_mut_ptr(), t, 0 as libc::c_int as libc::c_double),
    );
}
unsafe extern "C" fn Dp_toISOString(J: *mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_finite() as i32 == 0 {
        js_rangeerror(J, b"invalid date\0" as *const u8 as *const libc::c_char);
    }
    js_pushstring(
        J,
        fmtdatetime(buf.as_mut_ptr(), t, 0 as libc::c_int as libc::c_double),
    );
}
unsafe extern "C" fn Dp_getFullYear(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, YearFromTime(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getMonth(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, MonthFromTime(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getDate(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, DateFromTime(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getDay(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, WeekDay(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getHours(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, HourFromTime(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getMinutes(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, MinFromTime(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getSeconds(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, SecFromTime(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getMilliseconds(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, msFromTime(LocalTime(t)) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCFullYear(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, YearFromTime(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCMonth(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, MonthFromTime(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCDate(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, DateFromTime(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCDay(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, WeekDay(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCHours(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, HourFromTime(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCMinutes(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, MinFromTime(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCSeconds(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, SecFromTime(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getUTCMilliseconds(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, msFromTime(t) as libc::c_double);
    };
}
unsafe extern "C" fn Dp_getTimezoneOffset(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, (t - LocalTime(t)) / (60.0f64 * 1000.0f64));
    };
}
unsafe extern "C" fn Dp_setTime(J: *mut js_State) {
    js_setdate(J, 0 as libc::c_int, js_tonumber(J, 1 as libc::c_int));
}
unsafe extern "C" fn Dp_setMilliseconds(J: *mut js_State) {
    let t: libc::c_double = LocalTime(js_todate(J, 0 as libc::c_int));
    let h: libc::c_double = HourFromTime(t) as libc::c_double;
    let m: libc::c_double = MinFromTime(t) as libc::c_double;
    let s: libc::c_double = SecFromTime(t) as libc::c_double;
    let ms: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    js_setdate(
        J,
        0 as libc::c_int,
        UTC(MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms))),
    );
}
unsafe extern "C" fn Dp_setSeconds(J: *mut js_State) {
    let t: libc::c_double = LocalTime(js_todate(J, 0 as libc::c_int));
    let h: libc::c_double = HourFromTime(t) as libc::c_double;
    let m: libc::c_double = MinFromTime(t) as libc::c_double;
    let s: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let ms: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        msFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        UTC(MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms))),
    );
}
unsafe extern "C" fn Dp_setMinutes(J: *mut js_State) {
    let t: libc::c_double = LocalTime(js_todate(J, 0 as libc::c_int));
    let h: libc::c_double = HourFromTime(t) as libc::c_double;
    let m: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let s: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        SecFromTime(t) as libc::c_double
    };
    let ms: libc::c_double = if js_isdefined(J, 3 as libc::c_int) != 0 {
        js_tonumber(J, 3 as libc::c_int)
    } else {
        msFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        UTC(MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms))),
    );
}
unsafe extern "C" fn Dp_setHours(J: *mut js_State) {
    let t: libc::c_double = LocalTime(js_todate(J, 0 as libc::c_int));
    let h: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let m: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        MinFromTime(t) as libc::c_double
    };
    let s: libc::c_double = if js_isdefined(J, 3 as libc::c_int) != 0 {
        js_tonumber(J, 3 as libc::c_int)
    } else {
        SecFromTime(t) as libc::c_double
    };
    let ms: libc::c_double = if js_isdefined(J, 4 as libc::c_int) != 0 {
        js_tonumber(J, 4 as libc::c_int)
    } else {
        msFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        UTC(MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms))),
    );
}
unsafe extern "C" fn Dp_setDate(J: *mut js_State) {
    let t: libc::c_double = LocalTime(js_todate(J, 0 as libc::c_int));
    let y: libc::c_double = YearFromTime(t) as libc::c_double;
    let m: libc::c_double = MonthFromTime(t) as libc::c_double;
    let d: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    js_setdate(
        J,
        0 as libc::c_int,
        UTC(MakeDate(MakeDay(y, m, d), TimeWithinDay(t))),
    );
}
unsafe extern "C" fn Dp_setMonth(J: *mut js_State) {
    let t: libc::c_double = LocalTime(js_todate(J, 0 as libc::c_int));
    let y: libc::c_double = YearFromTime(t) as libc::c_double;
    let m: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let d: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        DateFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        UTC(MakeDate(MakeDay(y, m, d), TimeWithinDay(t))),
    );
}
unsafe extern "C" fn Dp_setFullYear(J: *mut js_State) {
    let t: libc::c_double = LocalTime(js_todate(J, 0 as libc::c_int));
    let y: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let m: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        MonthFromTime(t) as libc::c_double
    };
    let d: libc::c_double = if js_isdefined(J, 3 as libc::c_int) != 0 {
        js_tonumber(J, 3 as libc::c_int)
    } else {
        DateFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        UTC(MakeDate(MakeDay(y, m, d), TimeWithinDay(t))),
    );
}
unsafe extern "C" fn Dp_setUTCMilliseconds(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    let h: libc::c_double = HourFromTime(t) as libc::c_double;
    let m: libc::c_double = MinFromTime(t) as libc::c_double;
    let s: libc::c_double = SecFromTime(t) as libc::c_double;
    let ms: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    js_setdate(
        J,
        0 as libc::c_int,
        MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms)),
    );
}
unsafe extern "C" fn Dp_setUTCSeconds(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    let h: libc::c_double = HourFromTime(t) as libc::c_double;
    let m: libc::c_double = MinFromTime(t) as libc::c_double;
    let s: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let ms: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        msFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms)),
    );
}
unsafe extern "C" fn Dp_setUTCMinutes(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    let h: libc::c_double = HourFromTime(t) as libc::c_double;
    let m: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let s: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        SecFromTime(t) as libc::c_double
    };
    let ms: libc::c_double = if js_isdefined(J, 3 as libc::c_int) != 0 {
        js_tonumber(J, 3 as libc::c_int)
    } else {
        msFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms)),
    );
}
unsafe extern "C" fn Dp_setUTCHours(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    let h: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let m: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        HourFromTime(t) as libc::c_double
    };
    let s: libc::c_double = if js_isdefined(J, 3 as libc::c_int) != 0 {
        js_tonumber(J, 3 as libc::c_int)
    } else {
        SecFromTime(t) as libc::c_double
    };
    let ms: libc::c_double = if js_isdefined(J, 4 as libc::c_int) != 0 {
        js_tonumber(J, 4 as libc::c_int)
    } else {
        msFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        MakeDate(Day(t) as libc::c_double, MakeTime(h, m, s, ms)),
    );
}
unsafe extern "C" fn Dp_setUTCDate(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    let y: libc::c_double = YearFromTime(t) as libc::c_double;
    let m: libc::c_double = MonthFromTime(t) as libc::c_double;
    let d: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    js_setdate(
        J,
        0 as libc::c_int,
        MakeDate(MakeDay(y, m, d), TimeWithinDay(t)),
    );
}
unsafe extern "C" fn Dp_setUTCMonth(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    let y: libc::c_double = YearFromTime(t) as libc::c_double;
    let m: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let d: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        DateFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        MakeDate(MakeDay(y, m, d), TimeWithinDay(t)),
    );
}
unsafe extern "C" fn Dp_setUTCFullYear(J: *mut js_State) {
    let t: libc::c_double = js_todate(J, 0 as libc::c_int);
    let y: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let m: libc::c_double = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tonumber(J, 2 as libc::c_int)
    } else {
        MonthFromTime(t) as libc::c_double
    };
    let d: libc::c_double = if js_isdefined(J, 3 as libc::c_int) != 0 {
        js_tonumber(J, 3 as libc::c_int)
    } else {
        DateFromTime(t) as libc::c_double
    };
    js_setdate(
        J,
        0 as libc::c_int,
        MakeDate(MakeDay(y, m, d), TimeWithinDay(t)),
    );
}
unsafe extern "C" fn Dp_toJSON(J: *mut js_State) {
    js_copy(J, 0 as libc::c_int);
    js_toprimitive(J, -(1 as libc::c_int), JS_HNUMBER as libc::c_int);
    if js_isnumber(J, -(1 as libc::c_int)) != 0
        && (js_tonumber(J, -(1 as libc::c_int))).is_finite() as i32 == 0
    {
        js_pushnull(J);
        return;
    }
    js_pop(J, 1 as libc::c_int);
    js_getproperty(
        J,
        0 as libc::c_int,
        b"toISOString\0" as *const u8 as *const libc::c_char,
    );
    if js_iscallable(J, -(1 as libc::c_int)) == 0 {
        js_typeerror(
            J,
            b"this.toISOString is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    js_copy(J, 0 as libc::c_int);
    js_call(J, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initdate(J: *mut js_State) {
    (*(*J).Date_prototype).u.number = 0 as libc::c_int as libc::c_double;
    js_pushobject(J, (*J).Date_prototype);
    jsB_propf(
        J,
        b"Date.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Dp_valueOf as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toDateString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toDateString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toTimeString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toTimeString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toLocaleString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toLocaleDateString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toDateString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toLocaleTimeString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toTimeString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toUTCString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toUTCString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getTime\0" as *const u8 as *const libc::c_char,
        Some(Dp_valueOf as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_getFullYear as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCFullYear as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_getMonth as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCMonth as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_getDate as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCDate as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getDay\0" as *const u8 as *const libc::c_char,
        Some(Dp_getDay as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCDay\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCDay as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_getHours as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCHours as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_getMinutes as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCMinutes as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getSeconds as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCSeconds as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getMilliseconds as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCMilliseconds as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.getTimezoneOffset\0" as *const u8 as *const libc::c_char,
        Some(Dp_getTimezoneOffset as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setTime\0" as *const u8 as *const libc::c_char,
        Some(Dp_setTime as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setMilliseconds as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCMilliseconds as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setSeconds as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCSeconds as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_setMinutes as unsafe extern "C" fn(*mut js_State) -> ()),
        3 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCMinutes as unsafe extern "C" fn(*mut js_State) -> ()),
        3 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_setHours as unsafe extern "C" fn(*mut js_State) -> ()),
        4 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCHours as unsafe extern "C" fn(*mut js_State) -> ()),
        4 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_setDate as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCDate as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_setMonth as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCMonth as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_setFullYear as unsafe extern "C" fn(*mut js_State) -> ()),
        3 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCFullYear as unsafe extern "C" fn(*mut js_State) -> ()),
        3 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toISOString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toISOString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.prototype.toJSON\0" as *const u8 as *const libc::c_char,
        Some(Dp_toJSON as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_Date as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_new_Date as unsafe extern "C" fn(*mut js_State) -> ()),
        b"Date\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.parse\0" as *const u8 as *const libc::c_char,
        Some(D_parse as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.UTC\0" as *const u8 as *const libc::c_char,
        Some(D_UTC as unsafe extern "C" fn(*mut js_State) -> ()),
        7 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Date.now\0" as *const u8 as *const libc::c_char,
        Some(D_now as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    js_defglobal(
        J,
        b"Date\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_fmtexp(mut p: *mut libc::c_char, mut e: libc::c_int) {
    let mut se: [libc::c_char; 9] = [0; 9];
    let mut i: libc::c_int = 0;
    let fresh16 = p;
    p = p.offset(1);
    *fresh16 = 'e' as i32 as libc::c_char;
    if e < 0 as libc::c_int {
        let fresh17 = p;
        p = p.offset(1);
        *fresh17 = '-' as i32 as libc::c_char;
        e = -e;
    } else {
        let fresh18 = p;
        p = p.offset(1);
        *fresh18 = '+' as i32 as libc::c_char;
    }
    i = 0 as libc::c_int;
    while e != 0 {
        let fresh19 = i;
        i += 1;
        se[fresh19 as usize] = (e % 10 as libc::c_int + '0' as i32) as libc::c_char;
        e /= 10 as libc::c_int;
    }
    while i < 1 as libc::c_int {
        let fresh20 = i;
        i += 1;
        se[fresh20 as usize] = '0' as i32 as libc::c_char;
    }
    while i > 0 as libc::c_int {
        i -= 1;
        let fresh21 = p;
        p = p.offset(1);
        *fresh21 = se[i as usize];
    }
    let fresh22 = p;
    p = p.offset(1);
    *fresh22 = '\0' as i32 as libc::c_char;
}
static mut powers_ten: [uint64_t; 687] = [
    0xbf29dcaba82fdeae as libc::c_ulong,
    0xeef453d6923bd65a as libc::c_ulong,
    0x9558b4661b6565f8 as libc::c_ulong,
    0xbaaee17fa23ebf76 as libc::c_ulong,
    0xe95a99df8ace6f54 as libc::c_ulong,
    0x91d8a02bb6c10594 as libc::c_ulong,
    0xb64ec836a47146fa as libc::c_ulong,
    0xe3e27a444d8d98b8 as libc::c_ulong,
    0x8e6d8c6ab0787f73 as libc::c_ulong,
    0xb208ef855c969f50 as libc::c_ulong,
    0xde8b2b66b3bc4724 as libc::c_ulong,
    0x8b16fb203055ac76 as libc::c_ulong,
    0xaddcb9e83c6b1794 as libc::c_ulong,
    0xd953e8624b85dd79 as libc::c_ulong,
    0x87d4713d6f33aa6c as libc::c_ulong,
    0xa9c98d8ccb009506 as libc::c_ulong,
    0xd43bf0effdc0ba48 as libc::c_ulong,
    0x84a57695fe98746d as libc::c_ulong,
    0xa5ced43b7e3e9188 as libc::c_ulong,
    0xcf42894a5dce35ea as libc::c_ulong,
    0x818995ce7aa0e1b2 as libc::c_ulong,
    0xa1ebfb4219491a1f as libc::c_ulong,
    0xca66fa129f9b60a7 as libc::c_ulong,
    0xfd00b897478238d1 as libc::c_ulong,
    0x9e20735e8cb16382 as libc::c_ulong,
    0xc5a890362fddbc63 as libc::c_ulong,
    0xf712b443bbd52b7c as libc::c_ulong,
    0x9a6bb0aa55653b2d as libc::c_ulong,
    0xc1069cd4eabe89f9 as libc::c_ulong,
    0xf148440a256e2c77 as libc::c_ulong,
    0x96cd2a865764dbca as libc::c_ulong,
    0xbc807527ed3e12bd as libc::c_ulong,
    0xeba09271e88d976c as libc::c_ulong,
    0x93445b8731587ea3 as libc::c_ulong,
    0xb8157268fdae9e4c as libc::c_ulong,
    0xe61acf033d1a45df as libc::c_ulong,
    0x8fd0c16206306bac as libc::c_ulong,
    0xb3c4f1ba87bc8697 as libc::c_ulong,
    0xe0b62e2929aba83c as libc::c_ulong,
    0x8c71dcd9ba0b4926 as libc::c_ulong,
    0xaf8e5410288e1b6f as libc::c_ulong,
    0xdb71e91432b1a24b as libc::c_ulong,
    0x892731ac9faf056f as libc::c_ulong,
    0xab70fe17c79ac6ca as libc::c_ulong,
    0xd64d3d9db981787d as libc::c_ulong,
    0x85f0468293f0eb4e as libc::c_ulong,
    0xa76c582338ed2622 as libc::c_ulong,
    0xd1476e2c07286faa as libc::c_ulong,
    0x82cca4db847945ca as libc::c_ulong,
    0xa37fce126597973d as libc::c_ulong,
    0xcc5fc196fefd7d0c as libc::c_ulong,
    0xff77b1fcbebcdc4f as libc::c_ulong,
    0x9faacf3df73609b1 as libc::c_ulong,
    0xc795830d75038c1e as libc::c_ulong,
    0xf97ae3d0d2446f25 as libc::c_ulong,
    0x9becce62836ac577 as libc::c_ulong,
    0xc2e801fb244576d5 as libc::c_ulong,
    0xf3a20279ed56d48a as libc::c_ulong,
    0x9845418c345644d7 as libc::c_ulong,
    0xbe5691ef416bd60c as libc::c_ulong,
    0xedec366b11c6cb8f as libc::c_ulong,
    0x94b3a202eb1c3f39 as libc::c_ulong,
    0xb9e08a83a5e34f08 as libc::c_ulong,
    0xe858ad248f5c22ca as libc::c_ulong,
    0x91376c36d99995be as libc::c_ulong,
    0xb58547448ffffb2e as libc::c_ulong,
    0xe2e69915b3fff9f9 as libc::c_ulong,
    0x8dd01fad907ffc3c as libc::c_ulong,
    0xb1442798f49ffb4b as libc::c_ulong,
    0xdd95317f31c7fa1d as libc::c_ulong,
    0x8a7d3eef7f1cfc52 as libc::c_ulong,
    0xad1c8eab5ee43b67 as libc::c_ulong,
    0xd863b256369d4a41 as libc::c_ulong,
    0x873e4f75e2224e68 as libc::c_ulong,
    0xa90de3535aaae202 as libc::c_ulong,
    0xd3515c2831559a83 as libc::c_ulong,
    0x8412d9991ed58092 as libc::c_ulong,
    0xa5178fff668ae0b6 as libc::c_ulong,
    0xce5d73ff402d98e4 as libc::c_ulong,
    0x80fa687f881c7f8e as libc::c_ulong,
    0xa139029f6a239f72 as libc::c_ulong,
    0xc987434744ac874f as libc::c_ulong,
    0xfbe9141915d7a922 as libc::c_ulong,
    0x9d71ac8fada6c9b5 as libc::c_ulong,
    0xc4ce17b399107c23 as libc::c_ulong,
    0xf6019da07f549b2b as libc::c_ulong,
    0x99c102844f94e0fb as libc::c_ulong,
    0xc0314325637a193a as libc::c_ulong,
    0xf03d93eebc589f88 as libc::c_ulong,
    0x96267c7535b763b5 as libc::c_ulong,
    0xbbb01b9283253ca3 as libc::c_ulong,
    0xea9c227723ee8bcb as libc::c_ulong,
    0x92a1958a7675175f as libc::c_ulong,
    0xb749faed14125d37 as libc::c_ulong,
    0xe51c79a85916f485 as libc::c_ulong,
    0x8f31cc0937ae58d3 as libc::c_ulong,
    0xb2fe3f0b8599ef08 as libc::c_ulong,
    0xdfbdcece67006ac9 as libc::c_ulong,
    0x8bd6a141006042be as libc::c_ulong,
    0xaecc49914078536d as libc::c_ulong,
    0xda7f5bf590966849 as libc::c_ulong,
    0x888f99797a5e012d as libc::c_ulong,
    0xaab37fd7d8f58179 as libc::c_ulong,
    0xd5605fcdcf32e1d7 as libc::c_ulong,
    0x855c3be0a17fcd26 as libc::c_ulong,
    0xa6b34ad8c9dfc070 as libc::c_ulong,
    0xd0601d8efc57b08c as libc::c_ulong,
    0x823c12795db6ce57 as libc::c_ulong,
    0xa2cb1717b52481ed as libc::c_ulong,
    0xcb7ddcdda26da269 as libc::c_ulong,
    0xfe5d54150b090b03 as libc::c_ulong,
    0x9efa548d26e5a6e2 as libc::c_ulong,
    0xc6b8e9b0709f109a as libc::c_ulong,
    0xf867241c8cc6d4c1 as libc::c_ulong,
    0x9b407691d7fc44f8 as libc::c_ulong,
    0xc21094364dfb5637 as libc::c_ulong,
    0xf294b943e17a2bc4 as libc::c_ulong,
    0x979cf3ca6cec5b5b as libc::c_ulong,
    0xbd8430bd08277231 as libc::c_ulong,
    0xece53cec4a314ebe as libc::c_ulong,
    0x940f4613ae5ed137 as libc::c_ulong,
    0xb913179899f68584 as libc::c_ulong,
    0xe757dd7ec07426e5 as libc::c_ulong,
    0x9096ea6f3848984f as libc::c_ulong,
    0xb4bca50b065abe63 as libc::c_ulong,
    0xe1ebce4dc7f16dfc as libc::c_ulong,
    0x8d3360f09cf6e4bd as libc::c_ulong,
    0xb080392cc4349ded as libc::c_ulong,
    0xdca04777f541c568 as libc::c_ulong,
    0x89e42caaf9491b61 as libc::c_ulong,
    0xac5d37d5b79b6239 as libc::c_ulong,
    0xd77485cb25823ac7 as libc::c_ulong,
    0x86a8d39ef77164bd as libc::c_ulong,
    0xa8530886b54dbdec as libc::c_ulong,
    0xd267caa862a12d67 as libc::c_ulong,
    0x8380dea93da4bc60 as libc::c_ulong,
    0xa46116538d0deb78 as libc::c_ulong,
    0xcd795be870516656 as libc::c_ulong,
    0x806bd9714632dff6 as libc::c_ulong,
    0xa086cfcd97bf97f4 as libc::c_ulong,
    0xc8a883c0fdaf7df0 as libc::c_ulong,
    0xfad2a4b13d1b5d6c as libc::c_ulong,
    0x9cc3a6eec6311a64 as libc::c_ulong,
    0xc3f490aa77bd60fd as libc::c_ulong,
    0xf4f1b4d515acb93c as libc::c_ulong,
    0x991711052d8bf3c5 as libc::c_ulong,
    0xbf5cd54678eef0b7 as libc::c_ulong,
    0xef340a98172aace5 as libc::c_ulong,
    0x9580869f0e7aac0f as libc::c_ulong,
    0xbae0a846d2195713 as libc::c_ulong,
    0xe998d258869facd7 as libc::c_ulong,
    0x91ff83775423cc06 as libc::c_ulong,
    0xb67f6455292cbf08 as libc::c_ulong,
    0xe41f3d6a7377eeca as libc::c_ulong,
    0x8e938662882af53e as libc::c_ulong,
    0xb23867fb2a35b28e as libc::c_ulong,
    0xdec681f9f4c31f31 as libc::c_ulong,
    0x8b3c113c38f9f37f as libc::c_ulong,
    0xae0b158b4738705f as libc::c_ulong,
    0xd98ddaee19068c76 as libc::c_ulong,
    0x87f8a8d4cfa417ca as libc::c_ulong,
    0xa9f6d30a038d1dbc as libc::c_ulong,
    0xd47487cc8470652b as libc::c_ulong,
    0x84c8d4dfd2c63f3b as libc::c_ulong,
    0xa5fb0a17c777cf0a as libc::c_ulong,
    0xcf79cc9db955c2cc as libc::c_ulong,
    0x81ac1fe293d599c0 as libc::c_ulong,
    0xa21727db38cb0030 as libc::c_ulong,
    0xca9cf1d206fdc03c as libc::c_ulong,
    0xfd442e4688bd304b as libc::c_ulong,
    0x9e4a9cec15763e2f as libc::c_ulong,
    0xc5dd44271ad3cdba as libc::c_ulong,
    0xf7549530e188c129 as libc::c_ulong,
    0x9a94dd3e8cf578ba as libc::c_ulong,
    0xc13a148e3032d6e8 as libc::c_ulong,
    0xf18899b1bc3f8ca2 as libc::c_ulong,
    0x96f5600f15a7b7e5 as libc::c_ulong,
    0xbcb2b812db11a5de as libc::c_ulong,
    0xebdf661791d60f56 as libc::c_ulong,
    0x936b9fcebb25c996 as libc::c_ulong,
    0xb84687c269ef3bfb as libc::c_ulong,
    0xe65829b3046b0afa as libc::c_ulong,
    0x8ff71a0fe2c2e6dc as libc::c_ulong,
    0xb3f4e093db73a093 as libc::c_ulong,
    0xe0f218b8d25088b8 as libc::c_ulong,
    0x8c974f7383725573 as libc::c_ulong,
    0xafbd2350644eead0 as libc::c_ulong,
    0xdbac6c247d62a584 as libc::c_ulong,
    0x894bc396ce5da772 as libc::c_ulong,
    0xab9eb47c81f5114f as libc::c_ulong,
    0xd686619ba27255a3 as libc::c_ulong,
    0x8613fd0145877586 as libc::c_ulong,
    0xa798fc4196e952e7 as libc::c_ulong,
    0xd17f3b51fca3a7a1 as libc::c_ulong,
    0x82ef85133de648c5 as libc::c_ulong,
    0xa3ab66580d5fdaf6 as libc::c_ulong,
    0xcc963fee10b7d1b3 as libc::c_ulong,
    0xffbbcfe994e5c620 as libc::c_ulong,
    0x9fd561f1fd0f9bd4 as libc::c_ulong,
    0xc7caba6e7c5382c9 as libc::c_ulong,
    0xf9bd690a1b68637b as libc::c_ulong,
    0x9c1661a651213e2d as libc::c_ulong,
    0xc31bfa0fe5698db8 as libc::c_ulong,
    0xf3e2f893dec3f126 as libc::c_ulong,
    0x986ddb5c6b3a76b8 as libc::c_ulong,
    0xbe89523386091466 as libc::c_ulong,
    0xee2ba6c0678b597f as libc::c_ulong,
    0x94db483840b717f0 as libc::c_ulong,
    0xba121a4650e4ddec as libc::c_ulong,
    0xe896a0d7e51e1566 as libc::c_ulong,
    0x915e2486ef32cd60 as libc::c_ulong,
    0xb5b5ada8aaff80b8 as libc::c_ulong,
    0xe3231912d5bf60e6 as libc::c_ulong,
    0x8df5efabc5979c90 as libc::c_ulong,
    0xb1736b96b6fd83b4 as libc::c_ulong,
    0xddd0467c64bce4a1 as libc::c_ulong,
    0x8aa22c0dbef60ee4 as libc::c_ulong,
    0xad4ab7112eb3929e as libc::c_ulong,
    0xd89d64d57a607745 as libc::c_ulong,
    0x87625f056c7c4a8b as libc::c_ulong,
    0xa93af6c6c79b5d2e as libc::c_ulong,
    0xd389b47879823479 as libc::c_ulong,
    0x843610cb4bf160cc as libc::c_ulong,
    0xa54394fe1eedb8ff as libc::c_ulong,
    0xce947a3da6a9273e as libc::c_ulong,
    0x811ccc668829b887 as libc::c_ulong,
    0xa163ff802a3426a9 as libc::c_ulong,
    0xc9bcff6034c13053 as libc::c_ulong,
    0xfc2c3f3841f17c68 as libc::c_ulong,
    0x9d9ba7832936edc1 as libc::c_ulong,
    0xc5029163f384a931 as libc::c_ulong,
    0xf64335bcf065d37d as libc::c_ulong,
    0x99ea0196163fa42e as libc::c_ulong,
    0xc06481fb9bcf8d3a as libc::c_ulong,
    0xf07da27a82c37088 as libc::c_ulong,
    0x964e858c91ba2655 as libc::c_ulong,
    0xbbe226efb628afeb as libc::c_ulong,
    0xeadab0aba3b2dbe5 as libc::c_ulong,
    0x92c8ae6b464fc96f as libc::c_ulong,
    0xb77ada0617e3bbcb as libc::c_ulong,
    0xe55990879ddcaabe as libc::c_ulong,
    0x8f57fa54c2a9eab7 as libc::c_ulong,
    0xb32df8e9f3546564 as libc::c_ulong,
    0xdff9772470297ebd as libc::c_ulong,
    0x8bfbea76c619ef36 as libc::c_ulong,
    0xaefae51477a06b04 as libc::c_ulong,
    0xdab99e59958885c5 as libc::c_ulong,
    0x88b402f7fd75539b as libc::c_ulong,
    0xaae103b5fcd2a882 as libc::c_ulong,
    0xd59944a37c0752a2 as libc::c_ulong,
    0x857fcae62d8493a5 as libc::c_ulong,
    0xa6dfbd9fb8e5b88f as libc::c_ulong,
    0xd097ad07a71f26b2 as libc::c_ulong,
    0x825ecc24c8737830 as libc::c_ulong,
    0xa2f67f2dfa90563b as libc::c_ulong,
    0xcbb41ef979346bca as libc::c_ulong,
    0xfea126b7d78186bd as libc::c_ulong,
    0x9f24b832e6b0f436 as libc::c_ulong,
    0xc6ede63fa05d3144 as libc::c_ulong,
    0xf8a95fcf88747d94 as libc::c_ulong,
    0x9b69dbe1b548ce7d as libc::c_ulong,
    0xc24452da229b021c as libc::c_ulong,
    0xf2d56790ab41c2a3 as libc::c_ulong,
    0x97c560ba6b0919a6 as libc::c_ulong,
    0xbdb6b8e905cb600f as libc::c_ulong,
    0xed246723473e3813 as libc::c_ulong,
    0x9436c0760c86e30c as libc::c_ulong,
    0xb94470938fa89bcf as libc::c_ulong,
    0xe7958cb87392c2c3 as libc::c_ulong,
    0x90bd77f3483bb9ba as libc::c_ulong,
    0xb4ecd5f01a4aa828 as libc::c_ulong,
    0xe2280b6c20dd5232 as libc::c_ulong,
    0x8d590723948a535f as libc::c_ulong,
    0xb0af48ec79ace837 as libc::c_ulong,
    0xdcdb1b2798182245 as libc::c_ulong,
    0x8a08f0f8bf0f156b as libc::c_ulong,
    0xac8b2d36eed2dac6 as libc::c_ulong,
    0xd7adf884aa879177 as libc::c_ulong,
    0x86ccbb52ea94baeb as libc::c_ulong,
    0xa87fea27a539e9a5 as libc::c_ulong,
    0xd29fe4b18e88640f as libc::c_ulong,
    0x83a3eeeef9153e89 as libc::c_ulong,
    0xa48ceaaab75a8e2b as libc::c_ulong,
    0xcdb02555653131b6 as libc::c_ulong,
    0x808e17555f3ebf12 as libc::c_ulong,
    0xa0b19d2ab70e6ed6 as libc::c_ulong,
    0xc8de047564d20a8c as libc::c_ulong,
    0xfb158592be068d2f as libc::c_ulong,
    0x9ced737bb6c4183d as libc::c_ulong,
    0xc428d05aa4751e4d as libc::c_ulong,
    0xf53304714d9265e0 as libc::c_ulong,
    0x993fe2c6d07b7fac as libc::c_ulong,
    0xbf8fdb78849a5f97 as libc::c_ulong,
    0xef73d256a5c0f77d as libc::c_ulong,
    0x95a8637627989aae as libc::c_ulong,
    0xbb127c53b17ec159 as libc::c_ulong,
    0xe9d71b689dde71b0 as libc::c_ulong,
    0x9226712162ab070e as libc::c_ulong,
    0xb6b00d69bb55c8d1 as libc::c_ulong,
    0xe45c10c42a2b3b06 as libc::c_ulong,
    0x8eb98a7a9a5b04e3 as libc::c_ulong,
    0xb267ed1940f1c61c as libc::c_ulong,
    0xdf01e85f912e37a3 as libc::c_ulong,
    0x8b61313bbabce2c6 as libc::c_ulong,
    0xae397d8aa96c1b78 as libc::c_ulong,
    0xd9c7dced53c72256 as libc::c_ulong,
    0x881cea14545c7575 as libc::c_ulong,
    0xaa242499697392d3 as libc::c_ulong,
    0xd4ad2dbfc3d07788 as libc::c_ulong,
    0x84ec3c97da624ab5 as libc::c_ulong,
    0xa6274bbdd0fadd62 as libc::c_ulong,
    0xcfb11ead453994ba as libc::c_ulong,
    0x81ceb32c4b43fcf5 as libc::c_ulong,
    0xa2425ff75e14fc32 as libc::c_ulong,
    0xcad2f7f5359a3b3e as libc::c_ulong,
    0xfd87b5f28300ca0e as libc::c_ulong,
    0x9e74d1b791e07e48 as libc::c_ulong,
    0xc612062576589ddb as libc::c_ulong,
    0xf79687aed3eec551 as libc::c_ulong,
    0x9abe14cd44753b53 as libc::c_ulong,
    0xc16d9a0095928a27 as libc::c_ulong,
    0xf1c90080baf72cb1 as libc::c_ulong,
    0x971da05074da7bef as libc::c_ulong,
    0xbce5086492111aeb as libc::c_ulong,
    0xec1e4a7db69561a5 as libc::c_ulong,
    0x9392ee8e921d5d07 as libc::c_ulong,
    0xb877aa3236a4b449 as libc::c_ulong,
    0xe69594bec44de15b as libc::c_ulong,
    0x901d7cf73ab0acd9 as libc::c_ulong,
    0xb424dc35095cd80f as libc::c_ulong,
    0xe12e13424bb40e13 as libc::c_ulong,
    0x8cbccc096f5088cc as libc::c_ulong,
    0xafebff0bcb24aaff as libc::c_ulong,
    0xdbe6fecebdedd5bf as libc::c_ulong,
    0x89705f4136b4a597 as libc::c_ulong,
    0xabcc77118461cefd as libc::c_ulong,
    0xd6bf94d5e57a42bc as libc::c_ulong,
    0x8637bd05af6c69b6 as libc::c_ulong,
    0xa7c5ac471b478423 as libc::c_ulong,
    0xd1b71758e219652c as libc::c_ulong,
    0x83126e978d4fdf3b as libc::c_ulong,
    0xa3d70a3d70a3d70a as libc::c_ulong,
    0xcccccccccccccccd as libc::c_ulong,
    0x8000000000000000 as libc::c_ulong,
    0xa000000000000000 as libc::c_ulong,
    0xc800000000000000 as libc::c_ulong,
    0xfa00000000000000 as libc::c_ulong,
    0x9c40000000000000 as libc::c_ulong,
    0xc350000000000000 as libc::c_ulong,
    0xf424000000000000 as libc::c_ulong,
    0x9896800000000000 as libc::c_ulong,
    0xbebc200000000000 as libc::c_ulong,
    0xee6b280000000000 as libc::c_ulong,
    0x9502f90000000000 as libc::c_ulong,
    0xba43b74000000000 as libc::c_ulong,
    0xe8d4a51000000000 as libc::c_ulong,
    0x9184e72a00000000 as libc::c_ulong,
    0xb5e620f480000000 as libc::c_ulong,
    0xe35fa931a0000000 as libc::c_ulong,
    0x8e1bc9bf04000000 as libc::c_ulong,
    0xb1a2bc2ec5000000 as libc::c_ulong,
    0xde0b6b3a76400000 as libc::c_ulong,
    0x8ac7230489e80000 as libc::c_ulong,
    0xad78ebc5ac620000 as libc::c_ulong,
    0xd8d726b7177a8000 as libc::c_ulong,
    0x878678326eac9000 as libc::c_ulong,
    0xa968163f0a57b400 as libc::c_ulong,
    0xd3c21bcecceda100 as libc::c_ulong,
    0x84595161401484a0 as libc::c_ulong,
    0xa56fa5b99019a5c8 as libc::c_ulong,
    0xcecb8f27f4200f3a as libc::c_ulong,
    0x813f3978f8940984 as libc::c_ulong,
    0xa18f07d736b90be5 as libc::c_ulong,
    0xc9f2c9cd04674edf as libc::c_ulong,
    0xfc6f7c4045812296 as libc::c_ulong,
    0x9dc5ada82b70b59e as libc::c_ulong,
    0xc5371912364ce305 as libc::c_ulong,
    0xf684df56c3e01bc7 as libc::c_ulong,
    0x9a130b963a6c115c as libc::c_ulong,
    0xc097ce7bc90715b3 as libc::c_ulong,
    0xf0bdc21abb48db20 as libc::c_ulong,
    0x96769950b50d88f4 as libc::c_ulong,
    0xbc143fa4e250eb31 as libc::c_ulong,
    0xeb194f8e1ae525fd as libc::c_ulong,
    0x92efd1b8d0cf37be as libc::c_ulong,
    0xb7abc627050305ae as libc::c_ulong,
    0xe596b7b0c643c719 as libc::c_ulong,
    0x8f7e32ce7bea5c70 as libc::c_ulong,
    0xb35dbf821ae4f38c as libc::c_ulong,
    0xe0352f62a19e306f as libc::c_ulong,
    0x8c213d9da502de45 as libc::c_ulong,
    0xaf298d050e4395d7 as libc::c_ulong,
    0xdaf3f04651d47b4c as libc::c_ulong,
    0x88d8762bf324cd10 as libc::c_ulong,
    0xab0e93b6efee0054 as libc::c_ulong,
    0xd5d238a4abe98068 as libc::c_ulong,
    0x85a36366eb71f041 as libc::c_ulong,
    0xa70c3c40a64e6c52 as libc::c_ulong,
    0xd0cf4b50cfe20766 as libc::c_ulong,
    0x82818f1281ed44a0 as libc::c_ulong,
    0xa321f2d7226895c8 as libc::c_ulong,
    0xcbea6f8ceb02bb3a as libc::c_ulong,
    0xfee50b7025c36a08 as libc::c_ulong,
    0x9f4f2726179a2245 as libc::c_ulong,
    0xc722f0ef9d80aad6 as libc::c_ulong,
    0xf8ebad2b84e0d58c as libc::c_ulong,
    0x9b934c3b330c8577 as libc::c_ulong,
    0xc2781f49ffcfa6d5 as libc::c_ulong,
    0xf316271c7fc3908b as libc::c_ulong,
    0x97edd871cfda3a57 as libc::c_ulong,
    0xbde94e8e43d0c8ec as libc::c_ulong,
    0xed63a231d4c4fb27 as libc::c_ulong,
    0x945e455f24fb1cf9 as libc::c_ulong,
    0xb975d6b6ee39e437 as libc::c_ulong,
    0xe7d34c64a9c85d44 as libc::c_ulong,
    0x90e40fbeea1d3a4b as libc::c_ulong,
    0xb51d13aea4a488dd as libc::c_ulong,
    0xe264589a4dcdab15 as libc::c_ulong,
    0x8d7eb76070a08aed as libc::c_ulong,
    0xb0de65388cc8ada8 as libc::c_ulong,
    0xdd15fe86affad912 as libc::c_ulong,
    0x8a2dbf142dfcc7ab as libc::c_ulong,
    0xacb92ed9397bf996 as libc::c_ulong,
    0xd7e77a8f87daf7fc as libc::c_ulong,
    0x86f0ac99b4e8dafd as libc::c_ulong,
    0xa8acd7c0222311bd as libc::c_ulong,
    0xd2d80db02aabd62c as libc::c_ulong,
    0x83c7088e1aab65db as libc::c_ulong,
    0xa4b8cab1a1563f52 as libc::c_ulong,
    0xcde6fd5e09abcf27 as libc::c_ulong,
    0x80b05e5ac60b6178 as libc::c_ulong,
    0xa0dc75f1778e39d6 as libc::c_ulong,
    0xc913936dd571c84c as libc::c_ulong,
    0xfb5878494ace3a5f as libc::c_ulong,
    0x9d174b2dcec0e47b as libc::c_ulong,
    0xc45d1df942711d9a as libc::c_ulong,
    0xf5746577930d6501 as libc::c_ulong,
    0x9968bf6abbe85f20 as libc::c_ulong,
    0xbfc2ef456ae276e9 as libc::c_ulong,
    0xefb3ab16c59b14a3 as libc::c_ulong,
    0x95d04aee3b80ece6 as libc::c_ulong,
    0xbb445da9ca61281f as libc::c_ulong,
    0xea1575143cf97227 as libc::c_ulong,
    0x924d692ca61be758 as libc::c_ulong,
    0xb6e0c377cfa2e12e as libc::c_ulong,
    0xe498f455c38b997a as libc::c_ulong,
    0x8edf98b59a373fec as libc::c_ulong,
    0xb2977ee300c50fe7 as libc::c_ulong,
    0xdf3d5e9bc0f653e1 as libc::c_ulong,
    0x8b865b215899f46d as libc::c_ulong,
    0xae67f1e9aec07188 as libc::c_ulong,
    0xda01ee641a708dea as libc::c_ulong,
    0x884134fe908658b2 as libc::c_ulong,
    0xaa51823e34a7eedf as libc::c_ulong,
    0xd4e5e2cdc1d1ea96 as libc::c_ulong,
    0x850fadc09923329e as libc::c_ulong,
    0xa6539930bf6bff46 as libc::c_ulong,
    0xcfe87f7cef46ff17 as libc::c_ulong,
    0x81f14fae158c5f6e as libc::c_ulong,
    0xa26da3999aef774a as libc::c_ulong,
    0xcb090c8001ab551c as libc::c_ulong,
    0xfdcb4fa002162a63 as libc::c_ulong,
    0x9e9f11c4014dda7e as libc::c_ulong,
    0xc646d63501a1511e as libc::c_ulong,
    0xf7d88bc24209a565 as libc::c_ulong,
    0x9ae757596946075f as libc::c_ulong,
    0xc1a12d2fc3978937 as libc::c_ulong,
    0xf209787bb47d6b85 as libc::c_ulong,
    0x9745eb4d50ce6333 as libc::c_ulong,
    0xbd176620a501fc00 as libc::c_ulong,
    0xec5d3fa8ce427b00 as libc::c_ulong,
    0x93ba47c980e98ce0 as libc::c_ulong,
    0xb8a8d9bbe123f018 as libc::c_ulong,
    0xe6d3102ad96cec1e as libc::c_ulong,
    0x9043ea1ac7e41393 as libc::c_ulong,
    0xb454e4a179dd1877 as libc::c_ulong,
    0xe16a1dc9d8545e95 as libc::c_ulong,
    0x8ce2529e2734bb1d as libc::c_ulong,
    0xb01ae745b101e9e4 as libc::c_ulong,
    0xdc21a1171d42645d as libc::c_ulong,
    0x899504ae72497eba as libc::c_ulong,
    0xabfa45da0edbde69 as libc::c_ulong,
    0xd6f8d7509292d603 as libc::c_ulong,
    0x865b86925b9bc5c2 as libc::c_ulong,
    0xa7f26836f282b733 as libc::c_ulong,
    0xd1ef0244af2364ff as libc::c_ulong,
    0x8335616aed761f1f as libc::c_ulong,
    0xa402b9c5a8d3a6e7 as libc::c_ulong,
    0xcd036837130890a1 as libc::c_ulong,
    0x802221226be55a65 as libc::c_ulong,
    0xa02aa96b06deb0fe as libc::c_ulong,
    0xc83553c5c8965d3d as libc::c_ulong,
    0xfa42a8b73abbf48d as libc::c_ulong,
    0x9c69a97284b578d8 as libc::c_ulong,
    0xc38413cf25e2d70e as libc::c_ulong,
    0xf46518c2ef5b8cd1 as libc::c_ulong,
    0x98bf2f79d5993803 as libc::c_ulong,
    0xbeeefb584aff8604 as libc::c_ulong,
    0xeeaaba2e5dbf6785 as libc::c_ulong,
    0x952ab45cfa97a0b3 as libc::c_ulong,
    0xba756174393d88e0 as libc::c_ulong,
    0xe912b9d1478ceb17 as libc::c_ulong,
    0x91abb422ccb812ef as libc::c_ulong,
    0xb616a12b7fe617aa as libc::c_ulong,
    0xe39c49765fdf9d95 as libc::c_ulong,
    0x8e41ade9fbebc27d as libc::c_ulong,
    0xb1d219647ae6b31c as libc::c_ulong,
    0xde469fbd99a05fe3 as libc::c_ulong,
    0x8aec23d680043bee as libc::c_ulong,
    0xada72ccc20054aea as libc::c_ulong,
    0xd910f7ff28069da4 as libc::c_ulong,
    0x87aa9aff79042287 as libc::c_ulong,
    0xa99541bf57452b28 as libc::c_ulong,
    0xd3fa922f2d1675f2 as libc::c_ulong,
    0x847c9b5d7c2e09b7 as libc::c_ulong,
    0xa59bc234db398c25 as libc::c_ulong,
    0xcf02b2c21207ef2f as libc::c_ulong,
    0x8161afb94b44f57d as libc::c_ulong,
    0xa1ba1ba79e1632dc as libc::c_ulong,
    0xca28a291859bbf93 as libc::c_ulong,
    0xfcb2cb35e702af78 as libc::c_ulong,
    0x9defbf01b061adab as libc::c_ulong,
    0xc56baec21c7a1916 as libc::c_ulong,
    0xf6c69a72a3989f5c as libc::c_ulong,
    0x9a3c2087a63f6399 as libc::c_ulong,
    0xc0cb28a98fcf3c80 as libc::c_ulong,
    0xf0fdf2d3f3c30b9f as libc::c_ulong,
    0x969eb7c47859e744 as libc::c_ulong,
    0xbc4665b596706115 as libc::c_ulong,
    0xeb57ff22fc0c795a as libc::c_ulong,
    0x9316ff75dd87cbd8 as libc::c_ulong,
    0xb7dcbf5354e9bece as libc::c_ulong,
    0xe5d3ef282a242e82 as libc::c_ulong,
    0x8fa475791a569d11 as libc::c_ulong,
    0xb38d92d760ec4455 as libc::c_ulong,
    0xe070f78d3927556b as libc::c_ulong,
    0x8c469ab843b89563 as libc::c_ulong,
    0xaf58416654a6babb as libc::c_ulong,
    0xdb2e51bfe9d0696a as libc::c_ulong,
    0x88fcf317f22241e2 as libc::c_ulong,
    0xab3c2fddeeaad25b as libc::c_ulong,
    0xd60b3bd56a5586f2 as libc::c_ulong,
    0x85c7056562757457 as libc::c_ulong,
    0xa738c6bebb12d16d as libc::c_ulong,
    0xd106f86e69d785c8 as libc::c_ulong,
    0x82a45b450226b39d as libc::c_ulong,
    0xa34d721642b06084 as libc::c_ulong,
    0xcc20ce9bd35c78a5 as libc::c_ulong,
    0xff290242c83396ce as libc::c_ulong,
    0x9f79a169bd203e41 as libc::c_ulong,
    0xc75809c42c684dd1 as libc::c_ulong,
    0xf92e0c3537826146 as libc::c_ulong,
    0x9bbcc7a142b17ccc as libc::c_ulong,
    0xc2abf989935ddbfe as libc::c_ulong,
    0xf356f7ebf83552fe as libc::c_ulong,
    0x98165af37b2153df as libc::c_ulong,
    0xbe1bf1b059e9a8d6 as libc::c_ulong,
    0xeda2ee1c7064130c as libc::c_ulong,
    0x9485d4d1c63e8be8 as libc::c_ulong,
    0xb9a74a0637ce2ee1 as libc::c_ulong,
    0xe8111c87c5c1ba9a as libc::c_ulong,
    0x910ab1d4db9914a0 as libc::c_ulong,
    0xb54d5e4a127f59c8 as libc::c_ulong,
    0xe2a0b5dc971f303a as libc::c_ulong,
    0x8da471a9de737e24 as libc::c_ulong,
    0xb10d8e1456105dad as libc::c_ulong,
    0xdd50f1996b947519 as libc::c_ulong,
    0x8a5296ffe33cc930 as libc::c_ulong,
    0xace73cbfdc0bfb7b as libc::c_ulong,
    0xd8210befd30efa5a as libc::c_ulong,
    0x8714a775e3e95c78 as libc::c_ulong,
    0xa8d9d1535ce3b396 as libc::c_ulong,
    0xd31045a8341ca07c as libc::c_ulong,
    0x83ea2b892091e44e as libc::c_ulong,
    0xa4e4b66b68b65d61 as libc::c_ulong,
    0xce1de40642e3f4b9 as libc::c_ulong,
    0x80d2ae83e9ce78f4 as libc::c_ulong,
    0xa1075a24e4421731 as libc::c_ulong,
    0xc94930ae1d529cfd as libc::c_ulong,
    0xfb9b7cd9a4a7443c as libc::c_ulong,
    0x9d412e0806e88aa6 as libc::c_ulong,
    0xc491798a08a2ad4f as libc::c_ulong,
    0xf5b5d7ec8acb58a3 as libc::c_ulong,
    0x9991a6f3d6bf1766 as libc::c_ulong,
    0xbff610b0cc6edd3f as libc::c_ulong,
    0xeff394dcff8a948f as libc::c_ulong,
    0x95f83d0a1fb69cd9 as libc::c_ulong,
    0xbb764c4ca7a44410 as libc::c_ulong,
    0xea53df5fd18d5514 as libc::c_ulong,
    0x92746b9be2f8552c as libc::c_ulong,
    0xb7118682dbb66a77 as libc::c_ulong,
    0xe4d5e82392a40515 as libc::c_ulong,
    0x8f05b1163ba6832d as libc::c_ulong,
    0xb2c71d5bca9023f8 as libc::c_ulong,
    0xdf78e4b2bd342cf7 as libc::c_ulong,
    0x8bab8eefb6409c1a as libc::c_ulong,
    0xae9672aba3d0c321 as libc::c_ulong,
    0xda3c0f568cc4f3e9 as libc::c_ulong,
    0x8865899617fb1871 as libc::c_ulong,
    0xaa7eebfb9df9de8e as libc::c_ulong,
    0xd51ea6fa85785631 as libc::c_ulong,
    0x8533285c936b35df as libc::c_ulong,
    0xa67ff273b8460357 as libc::c_ulong,
    0xd01fef10a657842c as libc::c_ulong,
    0x8213f56a67f6b29c as libc::c_ulong,
    0xa298f2c501f45f43 as libc::c_ulong,
    0xcb3f2f7642717713 as libc::c_ulong,
    0xfe0efb53d30dd4d8 as libc::c_ulong,
    0x9ec95d1463e8a507 as libc::c_ulong,
    0xc67bb4597ce2ce49 as libc::c_ulong,
    0xf81aa16fdc1b81db as libc::c_ulong,
    0x9b10a4e5e9913129 as libc::c_ulong,
    0xc1d4ce1f63f57d73 as libc::c_ulong,
    0xf24a01a73cf2dcd0 as libc::c_ulong,
    0x976e41088617ca02 as libc::c_ulong,
    0xbd49d14aa79dbc82 as libc::c_ulong,
    0xec9c459d51852ba3 as libc::c_ulong,
    0x93e1ab8252f33b46 as libc::c_ulong,
    0xb8da1662e7b00a17 as libc::c_ulong,
    0xe7109bfba19c0c9d as libc::c_ulong,
    0x906a617d450187e2 as libc::c_ulong,
    0xb484f9dc9641e9db as libc::c_ulong,
    0xe1a63853bbd26451 as libc::c_ulong,
    0x8d07e33455637eb3 as libc::c_ulong,
    0xb049dc016abc5e60 as libc::c_ulong,
    0xdc5c5301c56b75f7 as libc::c_ulong,
    0x89b9b3e11b6329bb as libc::c_ulong,
    0xac2820d9623bf429 as libc::c_ulong,
    0xd732290fbacaf134 as libc::c_ulong,
    0x867f59a9d4bed6c0 as libc::c_ulong,
    0xa81f301449ee8c70 as libc::c_ulong,
    0xd226fc195c6a2f8c as libc::c_ulong,
    0x83585d8fd9c25db8 as libc::c_ulong,
    0xa42e74f3d032f526 as libc::c_ulong,
    0xcd3a1230c43fb26f as libc::c_ulong,
    0x80444b5e7aa7cf85 as libc::c_ulong,
    0xa0555e361951c367 as libc::c_ulong,
    0xc86ab5c39fa63441 as libc::c_ulong,
    0xfa856334878fc151 as libc::c_ulong,
    0x9c935e00d4b9d8d2 as libc::c_ulong,
    0xc3b8358109e84f07 as libc::c_ulong,
    0xf4a642e14c6262c9 as libc::c_ulong,
    0x98e7e9cccfbd7dbe as libc::c_ulong,
    0xbf21e44003acdd2d as libc::c_ulong,
    0xeeea5d5004981478 as libc::c_ulong,
    0x95527a5202df0ccb as libc::c_ulong,
    0xbaa718e68396cffe as libc::c_ulong,
    0xe950df20247c83fd as libc::c_ulong,
    0x91d28b7416cdd27e as libc::c_ulong,
    0xb6472e511c81471e as libc::c_ulong,
    0xe3d8f9e563a198e5 as libc::c_ulong,
    0x8e679c2f5e44ff8f as libc::c_ulong,
    0xb201833b35d63f73 as libc::c_ulong,
    0xde81e40a034bcf50 as libc::c_ulong,
    0x8b112e86420f6192 as libc::c_ulong,
    0xadd57a27d29339f6 as libc::c_ulong,
    0xd94ad8b1c7380874 as libc::c_ulong,
    0x87cec76f1c830549 as libc::c_ulong,
    0xa9c2794ae3a3c69b as libc::c_ulong,
    0xd433179d9c8cb841 as libc::c_ulong,
    0x849feec281d7f329 as libc::c_ulong,
    0xa5c7ea73224deff3 as libc::c_ulong,
    0xcf39e50feae16bf0 as libc::c_ulong,
    0x81842f29f2cce376 as libc::c_ulong,
    0xa1e53af46f801c53 as libc::c_ulong,
    0xca5e89b18b602368 as libc::c_ulong,
    0xfcf62c1dee382c42 as libc::c_ulong,
    0x9e19db92b4e31ba9 as libc::c_ulong,
    0xc5a05277621be294 as libc::c_ulong,
    0xf70867153aa2db39 as libc::c_ulong,
    0x9a65406d44a5c903 as libc::c_ulong,
    0xc0fe908895cf3b44 as libc::c_ulong,
    0xf13e34aabb430a15 as libc::c_ulong,
    0x96c6e0eab509e64d as libc::c_ulong,
    0xbc789925624c5fe1 as libc::c_ulong,
    0xeb96bf6ebadf77d9 as libc::c_ulong,
    0x933e37a534cbaae8 as libc::c_ulong,
    0xb80dc58e81fe95a1 as libc::c_ulong,
    0xe61136f2227e3b0a as libc::c_ulong,
    0x8fcac257558ee4e6 as libc::c_ulong,
    0xb3bd72ed2af29e20 as libc::c_ulong,
    0xe0accfa875af45a8 as libc::c_ulong,
    0x8c6c01c9498d8b89 as libc::c_ulong,
    0xaf87023b9bf0ee6b as libc::c_ulong,
    0xdb68c2ca82ed2a06 as libc::c_ulong,
    0x892179be91d43a44 as libc::c_ulong,
    0xab69d82e364948d4 as libc::c_ulong,
];
static mut powers_ten_e: [libc::c_int; 687] = [
    -(1203 as libc::c_int),
    -(1200 as libc::c_int),
    -(1196 as libc::c_int),
    -(1193 as libc::c_int),
    -(1190 as libc::c_int),
    -(1186 as libc::c_int),
    -(1183 as libc::c_int),
    -(1180 as libc::c_int),
    -(1176 as libc::c_int),
    -(1173 as libc::c_int),
    -(1170 as libc::c_int),
    -(1166 as libc::c_int),
    -(1163 as libc::c_int),
    -(1160 as libc::c_int),
    -(1156 as libc::c_int),
    -(1153 as libc::c_int),
    -(1150 as libc::c_int),
    -(1146 as libc::c_int),
    -(1143 as libc::c_int),
    -(1140 as libc::c_int),
    -(1136 as libc::c_int),
    -(1133 as libc::c_int),
    -(1130 as libc::c_int),
    -(1127 as libc::c_int),
    -(1123 as libc::c_int),
    -(1120 as libc::c_int),
    -(1117 as libc::c_int),
    -(1113 as libc::c_int),
    -(1110 as libc::c_int),
    -(1107 as libc::c_int),
    -(1103 as libc::c_int),
    -(1100 as libc::c_int),
    -(1097 as libc::c_int),
    -(1093 as libc::c_int),
    -(1090 as libc::c_int),
    -(1087 as libc::c_int),
    -(1083 as libc::c_int),
    -(1080 as libc::c_int),
    -(1077 as libc::c_int),
    -(1073 as libc::c_int),
    -(1070 as libc::c_int),
    -(1067 as libc::c_int),
    -(1063 as libc::c_int),
    -(1060 as libc::c_int),
    -(1057 as libc::c_int),
    -(1053 as libc::c_int),
    -(1050 as libc::c_int),
    -(1047 as libc::c_int),
    -(1043 as libc::c_int),
    -(1040 as libc::c_int),
    -(1037 as libc::c_int),
    -(1034 as libc::c_int),
    -(1030 as libc::c_int),
    -(1027 as libc::c_int),
    -(1024 as libc::c_int),
    -(1020 as libc::c_int),
    -(1017 as libc::c_int),
    -(1014 as libc::c_int),
    -(1010 as libc::c_int),
    -(1007 as libc::c_int),
    -(1004 as libc::c_int),
    -(1000 as libc::c_int),
    -(997 as libc::c_int),
    -(994 as libc::c_int),
    -(990 as libc::c_int),
    -(987 as libc::c_int),
    -(984 as libc::c_int),
    -(980 as libc::c_int),
    -(977 as libc::c_int),
    -(974 as libc::c_int),
    -(970 as libc::c_int),
    -(967 as libc::c_int),
    -(964 as libc::c_int),
    -(960 as libc::c_int),
    -(957 as libc::c_int),
    -(954 as libc::c_int),
    -(950 as libc::c_int),
    -(947 as libc::c_int),
    -(944 as libc::c_int),
    -(940 as libc::c_int),
    -(937 as libc::c_int),
    -(934 as libc::c_int),
    -(931 as libc::c_int),
    -(927 as libc::c_int),
    -(924 as libc::c_int),
    -(921 as libc::c_int),
    -(917 as libc::c_int),
    -(914 as libc::c_int),
    -(911 as libc::c_int),
    -(907 as libc::c_int),
    -(904 as libc::c_int),
    -(901 as libc::c_int),
    -(897 as libc::c_int),
    -(894 as libc::c_int),
    -(891 as libc::c_int),
    -(887 as libc::c_int),
    -(884 as libc::c_int),
    -(881 as libc::c_int),
    -(877 as libc::c_int),
    -(874 as libc::c_int),
    -(871 as libc::c_int),
    -(867 as libc::c_int),
    -(864 as libc::c_int),
    -(861 as libc::c_int),
    -(857 as libc::c_int),
    -(854 as libc::c_int),
    -(851 as libc::c_int),
    -(847 as libc::c_int),
    -(844 as libc::c_int),
    -(841 as libc::c_int),
    -(838 as libc::c_int),
    -(834 as libc::c_int),
    -(831 as libc::c_int),
    -(828 as libc::c_int),
    -(824 as libc::c_int),
    -(821 as libc::c_int),
    -(818 as libc::c_int),
    -(814 as libc::c_int),
    -(811 as libc::c_int),
    -(808 as libc::c_int),
    -(804 as libc::c_int),
    -(801 as libc::c_int),
    -(798 as libc::c_int),
    -(794 as libc::c_int),
    -(791 as libc::c_int),
    -(788 as libc::c_int),
    -(784 as libc::c_int),
    -(781 as libc::c_int),
    -(778 as libc::c_int),
    -(774 as libc::c_int),
    -(771 as libc::c_int),
    -(768 as libc::c_int),
    -(764 as libc::c_int),
    -(761 as libc::c_int),
    -(758 as libc::c_int),
    -(754 as libc::c_int),
    -(751 as libc::c_int),
    -(748 as libc::c_int),
    -(744 as libc::c_int),
    -(741 as libc::c_int),
    -(738 as libc::c_int),
    -(735 as libc::c_int),
    -(731 as libc::c_int),
    -(728 as libc::c_int),
    -(725 as libc::c_int),
    -(721 as libc::c_int),
    -(718 as libc::c_int),
    -(715 as libc::c_int),
    -(711 as libc::c_int),
    -(708 as libc::c_int),
    -(705 as libc::c_int),
    -(701 as libc::c_int),
    -(698 as libc::c_int),
    -(695 as libc::c_int),
    -(691 as libc::c_int),
    -(688 as libc::c_int),
    -(685 as libc::c_int),
    -(681 as libc::c_int),
    -(678 as libc::c_int),
    -(675 as libc::c_int),
    -(671 as libc::c_int),
    -(668 as libc::c_int),
    -(665 as libc::c_int),
    -(661 as libc::c_int),
    -(658 as libc::c_int),
    -(655 as libc::c_int),
    -(651 as libc::c_int),
    -(648 as libc::c_int),
    -(645 as libc::c_int),
    -(642 as libc::c_int),
    -(638 as libc::c_int),
    -(635 as libc::c_int),
    -(632 as libc::c_int),
    -(628 as libc::c_int),
    -(625 as libc::c_int),
    -(622 as libc::c_int),
    -(618 as libc::c_int),
    -(615 as libc::c_int),
    -(612 as libc::c_int),
    -(608 as libc::c_int),
    -(605 as libc::c_int),
    -(602 as libc::c_int),
    -(598 as libc::c_int),
    -(595 as libc::c_int),
    -(592 as libc::c_int),
    -(588 as libc::c_int),
    -(585 as libc::c_int),
    -(582 as libc::c_int),
    -(578 as libc::c_int),
    -(575 as libc::c_int),
    -(572 as libc::c_int),
    -(568 as libc::c_int),
    -(565 as libc::c_int),
    -(562 as libc::c_int),
    -(558 as libc::c_int),
    -(555 as libc::c_int),
    -(552 as libc::c_int),
    -(549 as libc::c_int),
    -(545 as libc::c_int),
    -(542 as libc::c_int),
    -(539 as libc::c_int),
    -(535 as libc::c_int),
    -(532 as libc::c_int),
    -(529 as libc::c_int),
    -(525 as libc::c_int),
    -(522 as libc::c_int),
    -(519 as libc::c_int),
    -(515 as libc::c_int),
    -(512 as libc::c_int),
    -(509 as libc::c_int),
    -(505 as libc::c_int),
    -(502 as libc::c_int),
    -(499 as libc::c_int),
    -(495 as libc::c_int),
    -(492 as libc::c_int),
    -(489 as libc::c_int),
    -(485 as libc::c_int),
    -(482 as libc::c_int),
    -(479 as libc::c_int),
    -(475 as libc::c_int),
    -(472 as libc::c_int),
    -(469 as libc::c_int),
    -(465 as libc::c_int),
    -(462 as libc::c_int),
    -(459 as libc::c_int),
    -(455 as libc::c_int),
    -(452 as libc::c_int),
    -(449 as libc::c_int),
    -(446 as libc::c_int),
    -(442 as libc::c_int),
    -(439 as libc::c_int),
    -(436 as libc::c_int),
    -(432 as libc::c_int),
    -(429 as libc::c_int),
    -(426 as libc::c_int),
    -(422 as libc::c_int),
    -(419 as libc::c_int),
    -(416 as libc::c_int),
    -(412 as libc::c_int),
    -(409 as libc::c_int),
    -(406 as libc::c_int),
    -(402 as libc::c_int),
    -(399 as libc::c_int),
    -(396 as libc::c_int),
    -(392 as libc::c_int),
    -(389 as libc::c_int),
    -(386 as libc::c_int),
    -(382 as libc::c_int),
    -(379 as libc::c_int),
    -(376 as libc::c_int),
    -(372 as libc::c_int),
    -(369 as libc::c_int),
    -(366 as libc::c_int),
    -(362 as libc::c_int),
    -(359 as libc::c_int),
    -(356 as libc::c_int),
    -(353 as libc::c_int),
    -(349 as libc::c_int),
    -(346 as libc::c_int),
    -(343 as libc::c_int),
    -(339 as libc::c_int),
    -(336 as libc::c_int),
    -(333 as libc::c_int),
    -(329 as libc::c_int),
    -(326 as libc::c_int),
    -(323 as libc::c_int),
    -(319 as libc::c_int),
    -(316 as libc::c_int),
    -(313 as libc::c_int),
    -(309 as libc::c_int),
    -(306 as libc::c_int),
    -(303 as libc::c_int),
    -(299 as libc::c_int),
    -(296 as libc::c_int),
    -(293 as libc::c_int),
    -(289 as libc::c_int),
    -(286 as libc::c_int),
    -(283 as libc::c_int),
    -(279 as libc::c_int),
    -(276 as libc::c_int),
    -(273 as libc::c_int),
    -(269 as libc::c_int),
    -(266 as libc::c_int),
    -(263 as libc::c_int),
    -(259 as libc::c_int),
    -(256 as libc::c_int),
    -(253 as libc::c_int),
    -(250 as libc::c_int),
    -(246 as libc::c_int),
    -(243 as libc::c_int),
    -(240 as libc::c_int),
    -(236 as libc::c_int),
    -(233 as libc::c_int),
    -(230 as libc::c_int),
    -(226 as libc::c_int),
    -(223 as libc::c_int),
    -(220 as libc::c_int),
    -(216 as libc::c_int),
    -(213 as libc::c_int),
    -(210 as libc::c_int),
    -(206 as libc::c_int),
    -(203 as libc::c_int),
    -(200 as libc::c_int),
    -(196 as libc::c_int),
    -(193 as libc::c_int),
    -(190 as libc::c_int),
    -(186 as libc::c_int),
    -(183 as libc::c_int),
    -(180 as libc::c_int),
    -(176 as libc::c_int),
    -(173 as libc::c_int),
    -(170 as libc::c_int),
    -(166 as libc::c_int),
    -(163 as libc::c_int),
    -(160 as libc::c_int),
    -(157 as libc::c_int),
    -(153 as libc::c_int),
    -(150 as libc::c_int),
    -(147 as libc::c_int),
    -(143 as libc::c_int),
    -(140 as libc::c_int),
    -(137 as libc::c_int),
    -(133 as libc::c_int),
    -(130 as libc::c_int),
    -(127 as libc::c_int),
    -(123 as libc::c_int),
    -(120 as libc::c_int),
    -(117 as libc::c_int),
    -(113 as libc::c_int),
    -(110 as libc::c_int),
    -(107 as libc::c_int),
    -(103 as libc::c_int),
    -(100 as libc::c_int),
    -(97 as libc::c_int),
    -(93 as libc::c_int),
    -(90 as libc::c_int),
    -(87 as libc::c_int),
    -(83 as libc::c_int),
    -(80 as libc::c_int),
    -(77 as libc::c_int),
    -(73 as libc::c_int),
    -(70 as libc::c_int),
    -(67 as libc::c_int),
    -(63 as libc::c_int),
    -(60 as libc::c_int),
    -(57 as libc::c_int),
    -(54 as libc::c_int),
    -(50 as libc::c_int),
    -(47 as libc::c_int),
    -(44 as libc::c_int),
    -(40 as libc::c_int),
    -(37 as libc::c_int),
    -(34 as libc::c_int),
    -(30 as libc::c_int),
    -(27 as libc::c_int),
    -(24 as libc::c_int),
    -(20 as libc::c_int),
    -(17 as libc::c_int),
    -(14 as libc::c_int),
    -(10 as libc::c_int),
    -(7 as libc::c_int),
    -(4 as libc::c_int),
    0 as libc::c_int,
    3 as libc::c_int,
    6 as libc::c_int,
    10 as libc::c_int,
    13 as libc::c_int,
    16 as libc::c_int,
    20 as libc::c_int,
    23 as libc::c_int,
    26 as libc::c_int,
    30 as libc::c_int,
    33 as libc::c_int,
    36 as libc::c_int,
    39 as libc::c_int,
    43 as libc::c_int,
    46 as libc::c_int,
    49 as libc::c_int,
    53 as libc::c_int,
    56 as libc::c_int,
    59 as libc::c_int,
    63 as libc::c_int,
    66 as libc::c_int,
    69 as libc::c_int,
    73 as libc::c_int,
    76 as libc::c_int,
    79 as libc::c_int,
    83 as libc::c_int,
    86 as libc::c_int,
    89 as libc::c_int,
    93 as libc::c_int,
    96 as libc::c_int,
    99 as libc::c_int,
    103 as libc::c_int,
    106 as libc::c_int,
    109 as libc::c_int,
    113 as libc::c_int,
    116 as libc::c_int,
    119 as libc::c_int,
    123 as libc::c_int,
    126 as libc::c_int,
    129 as libc::c_int,
    132 as libc::c_int,
    136 as libc::c_int,
    139 as libc::c_int,
    142 as libc::c_int,
    146 as libc::c_int,
    149 as libc::c_int,
    152 as libc::c_int,
    156 as libc::c_int,
    159 as libc::c_int,
    162 as libc::c_int,
    166 as libc::c_int,
    169 as libc::c_int,
    172 as libc::c_int,
    176 as libc::c_int,
    179 as libc::c_int,
    182 as libc::c_int,
    186 as libc::c_int,
    189 as libc::c_int,
    192 as libc::c_int,
    196 as libc::c_int,
    199 as libc::c_int,
    202 as libc::c_int,
    206 as libc::c_int,
    209 as libc::c_int,
    212 as libc::c_int,
    216 as libc::c_int,
    219 as libc::c_int,
    222 as libc::c_int,
    226 as libc::c_int,
    229 as libc::c_int,
    232 as libc::c_int,
    235 as libc::c_int,
    239 as libc::c_int,
    242 as libc::c_int,
    245 as libc::c_int,
    249 as libc::c_int,
    252 as libc::c_int,
    255 as libc::c_int,
    259 as libc::c_int,
    262 as libc::c_int,
    265 as libc::c_int,
    269 as libc::c_int,
    272 as libc::c_int,
    275 as libc::c_int,
    279 as libc::c_int,
    282 as libc::c_int,
    285 as libc::c_int,
    289 as libc::c_int,
    292 as libc::c_int,
    295 as libc::c_int,
    299 as libc::c_int,
    302 as libc::c_int,
    305 as libc::c_int,
    309 as libc::c_int,
    312 as libc::c_int,
    315 as libc::c_int,
    319 as libc::c_int,
    322 as libc::c_int,
    325 as libc::c_int,
    328 as libc::c_int,
    332 as libc::c_int,
    335 as libc::c_int,
    338 as libc::c_int,
    342 as libc::c_int,
    345 as libc::c_int,
    348 as libc::c_int,
    352 as libc::c_int,
    355 as libc::c_int,
    358 as libc::c_int,
    362 as libc::c_int,
    365 as libc::c_int,
    368 as libc::c_int,
    372 as libc::c_int,
    375 as libc::c_int,
    378 as libc::c_int,
    382 as libc::c_int,
    385 as libc::c_int,
    388 as libc::c_int,
    392 as libc::c_int,
    395 as libc::c_int,
    398 as libc::c_int,
    402 as libc::c_int,
    405 as libc::c_int,
    408 as libc::c_int,
    412 as libc::c_int,
    415 as libc::c_int,
    418 as libc::c_int,
    422 as libc::c_int,
    425 as libc::c_int,
    428 as libc::c_int,
    431 as libc::c_int,
    435 as libc::c_int,
    438 as libc::c_int,
    441 as libc::c_int,
    445 as libc::c_int,
    448 as libc::c_int,
    451 as libc::c_int,
    455 as libc::c_int,
    458 as libc::c_int,
    461 as libc::c_int,
    465 as libc::c_int,
    468 as libc::c_int,
    471 as libc::c_int,
    475 as libc::c_int,
    478 as libc::c_int,
    481 as libc::c_int,
    485 as libc::c_int,
    488 as libc::c_int,
    491 as libc::c_int,
    495 as libc::c_int,
    498 as libc::c_int,
    501 as libc::c_int,
    505 as libc::c_int,
    508 as libc::c_int,
    511 as libc::c_int,
    515 as libc::c_int,
    518 as libc::c_int,
    521 as libc::c_int,
    524 as libc::c_int,
    528 as libc::c_int,
    531 as libc::c_int,
    534 as libc::c_int,
    538 as libc::c_int,
    541 as libc::c_int,
    544 as libc::c_int,
    548 as libc::c_int,
    551 as libc::c_int,
    554 as libc::c_int,
    558 as libc::c_int,
    561 as libc::c_int,
    564 as libc::c_int,
    568 as libc::c_int,
    571 as libc::c_int,
    574 as libc::c_int,
    578 as libc::c_int,
    581 as libc::c_int,
    584 as libc::c_int,
    588 as libc::c_int,
    591 as libc::c_int,
    594 as libc::c_int,
    598 as libc::c_int,
    601 as libc::c_int,
    604 as libc::c_int,
    608 as libc::c_int,
    611 as libc::c_int,
    614 as libc::c_int,
    617 as libc::c_int,
    621 as libc::c_int,
    624 as libc::c_int,
    627 as libc::c_int,
    631 as libc::c_int,
    634 as libc::c_int,
    637 as libc::c_int,
    641 as libc::c_int,
    644 as libc::c_int,
    647 as libc::c_int,
    651 as libc::c_int,
    654 as libc::c_int,
    657 as libc::c_int,
    661 as libc::c_int,
    664 as libc::c_int,
    667 as libc::c_int,
    671 as libc::c_int,
    674 as libc::c_int,
    677 as libc::c_int,
    681 as libc::c_int,
    684 as libc::c_int,
    687 as libc::c_int,
    691 as libc::c_int,
    694 as libc::c_int,
    697 as libc::c_int,
    701 as libc::c_int,
    704 as libc::c_int,
    707 as libc::c_int,
    711 as libc::c_int,
    714 as libc::c_int,
    717 as libc::c_int,
    720 as libc::c_int,
    724 as libc::c_int,
    727 as libc::c_int,
    730 as libc::c_int,
    734 as libc::c_int,
    737 as libc::c_int,
    740 as libc::c_int,
    744 as libc::c_int,
    747 as libc::c_int,
    750 as libc::c_int,
    754 as libc::c_int,
    757 as libc::c_int,
    760 as libc::c_int,
    764 as libc::c_int,
    767 as libc::c_int,
    770 as libc::c_int,
    774 as libc::c_int,
    777 as libc::c_int,
    780 as libc::c_int,
    784 as libc::c_int,
    787 as libc::c_int,
    790 as libc::c_int,
    794 as libc::c_int,
    797 as libc::c_int,
    800 as libc::c_int,
    804 as libc::c_int,
    807 as libc::c_int,
    810 as libc::c_int,
    813 as libc::c_int,
    817 as libc::c_int,
    820 as libc::c_int,
    823 as libc::c_int,
    827 as libc::c_int,
    830 as libc::c_int,
    833 as libc::c_int,
    837 as libc::c_int,
    840 as libc::c_int,
    843 as libc::c_int,
    847 as libc::c_int,
    850 as libc::c_int,
    853 as libc::c_int,
    857 as libc::c_int,
    860 as libc::c_int,
    863 as libc::c_int,
    867 as libc::c_int,
    870 as libc::c_int,
    873 as libc::c_int,
    877 as libc::c_int,
    880 as libc::c_int,
    883 as libc::c_int,
    887 as libc::c_int,
    890 as libc::c_int,
    893 as libc::c_int,
    897 as libc::c_int,
    900 as libc::c_int,
    903 as libc::c_int,
    907 as libc::c_int,
    910 as libc::c_int,
    913 as libc::c_int,
    916 as libc::c_int,
    920 as libc::c_int,
    923 as libc::c_int,
    926 as libc::c_int,
    930 as libc::c_int,
    933 as libc::c_int,
    936 as libc::c_int,
    940 as libc::c_int,
    943 as libc::c_int,
    946 as libc::c_int,
    950 as libc::c_int,
    953 as libc::c_int,
    956 as libc::c_int,
    960 as libc::c_int,
    963 as libc::c_int,
    966 as libc::c_int,
    970 as libc::c_int,
    973 as libc::c_int,
    976 as libc::c_int,
    980 as libc::c_int,
    983 as libc::c_int,
    986 as libc::c_int,
    990 as libc::c_int,
    993 as libc::c_int,
    996 as libc::c_int,
    1000 as libc::c_int,
    1003 as libc::c_int,
    1006 as libc::c_int,
    1009 as libc::c_int,
    1013 as libc::c_int,
    1016 as libc::c_int,
    1019 as libc::c_int,
    1023 as libc::c_int,
    1026 as libc::c_int,
    1029 as libc::c_int,
    1033 as libc::c_int,
    1036 as libc::c_int,
    1039 as libc::c_int,
    1043 as libc::c_int,
    1046 as libc::c_int,
    1049 as libc::c_int,
    1053 as libc::c_int,
    1056 as libc::c_int,
    1059 as libc::c_int,
    1063 as libc::c_int,
    1066 as libc::c_int,
    1069 as libc::c_int,
    1073 as libc::c_int,
    1076 as libc::c_int,
];
unsafe extern "C" fn cached_power(k: libc::c_int) -> diy_fp_t {
    let mut res: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let index: libc::c_int = 343 as libc::c_int + k;
    res.f = powers_ten[index as usize];
    res.e = powers_ten_e[index as usize];
    res
}
unsafe extern "C" fn k_comp(e: libc::c_int, alpha: libc::c_int, gamma: libc::c_int) -> libc::c_int {
    ceil((alpha - e + 63 as libc::c_int) as libc::c_double * 0.30102999566398114f64) as libc::c_int
}
unsafe extern "C" fn minus(x: diy_fp_t, y: diy_fp_t) -> diy_fp_t {
    let mut r: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    if x.e == y.e {
    } else {
        __assert_fail(
            b"x.e == y.e\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            4977 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"diy_fp_t minus(diy_fp_t, diy_fp_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_11239: {
        if x.e == y.e {
        } else {
            __assert_fail(
                b"x.e == y.e\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                4977 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"diy_fp_t minus(diy_fp_t, diy_fp_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if x.f >= y.f {
    } else {
        __assert_fail(
            b"x.f >= y.f\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            4978 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                b"diy_fp_t minus(diy_fp_t, diy_fp_t)\0",
            ))
            .as_ptr(),
        );
    }
    'c_11188: {
        if x.f >= y.f {
        } else {
            __assert_fail(
                b"x.f >= y.f\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                4978 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 35], &[libc::c_char; 35]>(
                    b"diy_fp_t minus(diy_fp_t, diy_fp_t)\0",
                ))
                .as_ptr(),
            );
        }
    };
    r.f = (x.f).wrapping_sub(y.f);
    r.e = x.e;
    r
}
unsafe extern "C" fn multiply(x: diy_fp_t, y: diy_fp_t) -> diy_fp_t {
    let mut a: uint64_t = 0;
    let mut b: uint64_t = 0;
    let mut c: uint64_t = 0;
    let mut d: uint64_t = 0;
    let mut ac: uint64_t = 0;
    let mut bc: uint64_t = 0;
    let mut ad: uint64_t = 0;
    let mut bd: uint64_t = 0;
    let mut tmp: uint64_t = 0;
    let mut r: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let M32: uint64_t = 0xffffffff as libc::c_uint as uint64_t;
    a = x.f >> 32 as libc::c_int;
    b = x.f & M32;
    c = y.f >> 32 as libc::c_int;
    d = y.f & M32;
    ac = a.wrapping_mul(c);
    bc = b.wrapping_mul(c);
    ad = a.wrapping_mul(d);
    bd = b.wrapping_mul(d);
    tmp = (bd >> 32 as libc::c_int)
        .wrapping_add(ad & M32)
        .wrapping_add(bc & M32);
    tmp = (tmp as libc::c_ulong)
        .wrapping_add(((1 as libc::c_uint) << 31 as libc::c_int) as libc::c_ulong)
        as uint64_t as uint64_t;
    r.f = ac
        .wrapping_add(ad >> 32 as libc::c_int)
        .wrapping_add(bc >> 32 as libc::c_int)
        .wrapping_add(tmp >> 32 as libc::c_int);
    r.e = x.e + y.e + 64 as libc::c_int;
    r
}
unsafe extern "C" fn double_to_uint64(mut d: libc::c_double) -> uint64_t {
    let mut n: uint64_t = 0;
    memcpy(
        &mut n as *mut uint64_t as *mut libc::c_void,
        &mut d as *mut libc::c_double as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    );
    n
}
unsafe extern "C" fn double2diy_fp(d: libc::c_double) -> diy_fp_t {
    let d64: uint64_t = double_to_uint64(d);
    let biased_e: libc::c_int = ((d64 & 0x7ff0000000000000 as libc::c_long as libc::c_ulong)
        >> 52 as libc::c_int) as libc::c_int;
    let significand: uint64_t = d64 & 0xfffffffffffff as libc::c_long as libc::c_ulong;
    let mut res: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    if biased_e != 0 as libc::c_int {
        res.f = significand.wrapping_add(0x10000000000000 as libc::c_long as libc::c_ulong);
        res.e = biased_e - (0x3ff as libc::c_int + 52 as libc::c_int);
    } else {
        res.f = significand;
        res.e = -(0x3ff as libc::c_int + 52 as libc::c_int) + 1 as libc::c_int;
    }
    res
}
unsafe extern "C" fn normalize_boundary(in_0: diy_fp_t) -> diy_fp_t {
    let mut res: diy_fp_t = in_0;
    while res.f & ((0x10000000000000 as libc::c_long) << 1 as libc::c_int) as libc::c_ulong == 0 {
        res.f <<= 1 as libc::c_int;
        res.e -= 1;
        res.e;
    }
    res.f <<= 64 as libc::c_int - 52 as libc::c_int - 2 as libc::c_int;
    res.e -= 64 as libc::c_int - 52 as libc::c_int - 2 as libc::c_int;
    res
}
unsafe extern "C" fn normalized_boundaries(
    d: libc::c_double,
    out_m_minus: *mut diy_fp_t,
    out_m_plus: *mut diy_fp_t,
) {
    let v: diy_fp_t = double2diy_fp(d);
    let mut pl: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let mut mi: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let significand_is_zero: libc::c_int =
        (v.f == 0x10000000000000 as libc::c_long as libc::c_ulong) as libc::c_int;
    pl.f = (v.f << 1 as libc::c_int).wrapping_add(1 as libc::c_int as libc::c_ulong);
    pl.e = v.e - 1 as libc::c_int;
    pl = normalize_boundary(pl);
    if significand_is_zero != 0 {
        mi.f = (v.f << 2 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        mi.e = v.e - 2 as libc::c_int;
    } else {
        mi.f = (v.f << 1 as libc::c_int).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        mi.e = v.e - 1 as libc::c_int;
    }
    mi.f <<= mi.e - pl.e;
    mi.e = pl.e;
    *out_m_plus = pl;
    *out_m_minus = mi;
}
unsafe extern "C" fn digit_gen(
    Mp: diy_fp_t,
    mut delta: diy_fp_t,
    buffer: *mut libc::c_char,
    len: *mut libc::c_int,
    K: *mut libc::c_int,
) {
    let mut div: uint32_t = 0;
    let mut p1: uint32_t = 0;
    let mut p2: uint64_t = 0;
    let mut d: libc::c_int = 0;
    let mut kappa: libc::c_int = 0;
    let mut one: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    one.f = (1 as libc::c_int as uint64_t) << -Mp.e;
    one.e = Mp.e;
    p1 = (Mp.f >> -one.e) as uint32_t;
    p2 = Mp.f & (one.f).wrapping_sub(1 as libc::c_int as libc::c_ulong);
    *len = 0 as libc::c_int;
    kappa = 3 as libc::c_int;
    div = 100 as libc::c_int as uint32_t;
    while kappa > 0 as libc::c_int {
        d = p1.wrapping_div(div) as libc::c_int;
        if d != 0 || *len != 0 {
            let fresh23 = *len;
            *len += 1;
            *buffer.offset(fresh23 as isize) = ('0' as i32 + d) as libc::c_char;
        }
        p1 = (p1 as libc::c_uint).wrapping_rem(div) as uint32_t as uint32_t;
        kappa -= 1;
        kappa;
        div = (div as libc::c_uint).wrapping_div(10 as libc::c_int as libc::c_uint) as uint32_t
            as uint32_t;
        if ((p1 as uint64_t) << -one.e).wrapping_add(p2) <= delta.f {
            *K += kappa;
            return;
        }
    }
    loop {
        p2 = (p2 as libc::c_ulong).wrapping_mul(10 as libc::c_int as libc::c_ulong) as uint64_t
            as uint64_t;
        d = (p2 >> -one.e) as libc::c_int;
        if d != 0 || *len != 0 {
            let fresh24 = *len;
            *len += 1;
            *buffer.offset(fresh24 as isize) = ('0' as i32 + d) as libc::c_char;
        }
        p2 &= (one.f).wrapping_sub(1 as libc::c_int as libc::c_ulong);
        kappa -= 1;
        kappa;
        delta.f = (delta.f as libc::c_ulong).wrapping_mul(10 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        if p2 <= delta.f {
            break;
        }
    }
    *K += kappa;
}
#[no_mangle]
pub unsafe extern "C" fn js_grisu2(
    v: libc::c_double,
    buffer: *mut libc::c_char,
    K: *mut libc::c_int,
) -> libc::c_int {
    let mut length: libc::c_int = 0;
    let mut mk: libc::c_int = 0;
    let mut w_m: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let mut w_p: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let mut c_mk: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let mut Wp: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let mut Wm: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let mut delta: diy_fp_t = diy_fp_t { f: 0, e: 0 };
    let q: libc::c_int = 64 as libc::c_int;
    let alpha: libc::c_int = -(59 as libc::c_int);
    let gamma: libc::c_int = -(56 as libc::c_int);
    normalized_boundaries(v, &mut w_m, &mut w_p);
    mk = k_comp(w_p.e + q, alpha, gamma);
    c_mk = cached_power(mk);
    Wp = multiply(w_p, c_mk);
    Wm = multiply(w_m, c_mk);
    Wm.f = (Wm.f).wrapping_add(1);
    Wm.f;
    Wp.f = (Wp.f).wrapping_sub(1);
    Wp.f;
    delta = minus(Wp, Wm);
    *K = -mk;
    digit_gen(Wp, delta, buffer, &mut length, K);
    length
}
static mut maxExponent: libc::c_int = 511 as libc::c_int;
static mut powersOf10: [libc::c_double; 9] = [
    10.0f64, 100.0f64, 1.0e4f64, 1.0e8f64, 1.0e16f64, 1.0e32f64, 1.0e64f64, 1.0e128f64, 1.0e256f64,
];
#[no_mangle]
pub unsafe extern "C" fn js_strtod(
    string: *const libc::c_char,
    endPtr: *mut *mut libc::c_char,
) -> libc::c_double {
    let mut sign: libc::c_int = 0;
    let mut expSign: libc::c_int = 0 as libc::c_int;
    let mut fraction: libc::c_double = 0.;
    let mut dblExp: libc::c_double = 0.;
    let mut d: *mut libc::c_double = std::ptr::null_mut::<libc::c_double>();
    let mut p: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut c: libc::c_int = 0;
    let mut exp_0: libc::c_int = 0 as libc::c_int;
    let mut fracExp: libc::c_int = 0 as libc::c_int;
    let mut mantSize: libc::c_int = 0;
    let mut decPt: libc::c_int = 0;
    let mut pExp: *const libc::c_char = std::ptr::null::<libc::c_char>();
    p = string;
    while *p as libc::c_int == ' ' as i32
        || *p as libc::c_int == '\t' as i32
        || *p as libc::c_int == '\n' as i32
        || *p as libc::c_int == '\r' as i32
    {
        p = p.offset(1 as libc::c_int as isize);
    }
    if *p as libc::c_int == '-' as i32 {
        sign = 1 as libc::c_int;
        p = p.offset(1 as libc::c_int as isize);
    } else {
        if *p as libc::c_int == '+' as i32 {
            p = p.offset(1 as libc::c_int as isize);
        }
        sign = 0 as libc::c_int;
    }
    decPt = -(1 as libc::c_int);
    mantSize = 0 as libc::c_int;
    loop {
        c = *p as libc::c_int;
        if !(c >= '0' as i32 && c <= '9' as i32) {
            if c != '.' as i32 || decPt >= 0 as libc::c_int {
                break;
            }
            decPt = mantSize;
        }
        p = p.offset(1 as libc::c_int as isize);
        mantSize += 1 as libc::c_int;
    }
    pExp = p;
    p = p.offset(-(mantSize as isize));
    if decPt < 0 as libc::c_int {
        decPt = mantSize;
    } else {
        mantSize -= 1 as libc::c_int;
    }
    if mantSize > 18 as libc::c_int {
        fracExp = decPt - 18 as libc::c_int;
        mantSize = 18 as libc::c_int;
    } else {
        fracExp = decPt - mantSize;
    }
    if mantSize == 0 as libc::c_int {
        fraction = 0.0f64;
        p = string;
    } else {
        let mut frac1: libc::c_int = 0;
        let mut frac2: libc::c_int = 0;
        frac1 = 0 as libc::c_int;
        while mantSize > 9 as libc::c_int {
            c = *p as libc::c_int;
            p = p.offset(1 as libc::c_int as isize);
            if c == '.' as i32 {
                c = *p as libc::c_int;
                p = p.offset(1 as libc::c_int as isize);
            }
            frac1 = 10 as libc::c_int * frac1 + (c - '0' as i32);
            mantSize -= 1 as libc::c_int;
        }
        frac2 = 0 as libc::c_int;
        while mantSize > 0 as libc::c_int {
            c = *p as libc::c_int;
            p = p.offset(1 as libc::c_int as isize);
            if c == '.' as i32 {
                c = *p as libc::c_int;
                p = p.offset(1 as libc::c_int as isize);
            }
            frac2 = 10 as libc::c_int * frac2 + (c - '0' as i32);
            mantSize -= 1 as libc::c_int;
        }
        fraction = 1.0e9f64 * frac1 as libc::c_double + frac2 as libc::c_double;
        p = pExp;
        if *p as libc::c_int == 'E' as i32 || *p as libc::c_int == 'e' as i32 {
            p = p.offset(1 as libc::c_int as isize);
            if *p as libc::c_int == '-' as i32 {
                expSign = 1 as libc::c_int;
                p = p.offset(1 as libc::c_int as isize);
            } else {
                if *p as libc::c_int == '+' as i32 {
                    p = p.offset(1 as libc::c_int as isize);
                }
                expSign = 0 as libc::c_int;
            }
            while *p as libc::c_int >= '0' as i32
                && *p as libc::c_int <= '9' as i32
                && exp_0 < 2147483647 as libc::c_int / 100 as libc::c_int
            {
                exp_0 = exp_0 * 10 as libc::c_int + (*p as libc::c_int - '0' as i32);
                p = p.offset(1 as libc::c_int as isize);
            }
            while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
                p = p.offset(1 as libc::c_int as isize);
            }
        }
        if expSign != 0 {
            exp_0 = fracExp - exp_0;
        } else {
            exp_0 += fracExp;
        }
        if exp_0 < -maxExponent {
            exp_0 = maxExponent;
            expSign = 1 as libc::c_int;
            *__errno_location() = 34 as libc::c_int;
        } else if exp_0 > maxExponent {
            exp_0 = maxExponent;
            expSign = 0 as libc::c_int;
            *__errno_location() = 34 as libc::c_int;
        } else if exp_0 < 0 as libc::c_int {
            expSign = 1 as libc::c_int;
            exp_0 = -exp_0;
        } else {
            expSign = 0 as libc::c_int;
        }
        dblExp = 1.0f64;
        d = powersOf10.as_mut_ptr();
        while exp_0 != 0 as libc::c_int {
            if exp_0 & 0o1 as libc::c_int != 0 {
                dblExp *= *d;
            }
            exp_0 >>= 1 as libc::c_int;
            d = d.offset(1 as libc::c_int as isize);
        }
        if expSign != 0 {
            fraction /= dblExp;
        } else {
            fraction *= dblExp;
        }
    }
    if !endPtr.is_null() {
        *endPtr = p as *mut libc::c_char;
    }
    if sign != 0 {
        return -fraction;
    }
    fraction
}
unsafe extern "C" fn jsB_stacktrace(J: *mut js_State, skip: libc::c_int) -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut n: libc::c_int = (*J).tracetop - skip;
    if n <= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    while n > 0 as libc::c_int {
        let name: *const libc::c_char = (*J).trace[n as usize].name;
        let file: *const libc::c_char = (*J).trace[n as usize].file;
        let line: libc::c_int = (*J).trace[n as usize].line;
        if line > 0 as libc::c_int {
            if *name.offset(0 as libc::c_int as isize) != 0 {
                snprintf(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"\n\tat %s (%s:%d)\0" as *const u8 as *const libc::c_char,
                    name,
                    file,
                    line,
                );
            } else {
                snprintf(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    b"\n\tat %s:%d\0" as *const u8 as *const libc::c_char,
                    file,
                    line,
                );
            }
        } else {
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"\n\tat %s (%s)\0" as *const u8 as *const libc::c_char,
                name,
                file,
            );
        }
        js_pushstring(J, buf.as_mut_ptr());
        if n < (*J).tracetop - skip {
            js_concat(J);
        }
        n -= 1;
        n;
    }
    1 as libc::c_int
}
unsafe extern "C" fn Ep_toString(J: *mut js_State) {
    let mut name: *const libc::c_char = b"Error\0" as *const u8 as *const libc::c_char;
    let mut message: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    if js_isobject(J, -(1 as libc::c_int)) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    if js_hasproperty(
        J,
        0 as libc::c_int,
        b"name\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        name = js_tostring(J, -(1 as libc::c_int));
    }
    if js_hasproperty(
        J,
        0 as libc::c_int,
        b"message\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        message = js_tostring(J, -(1 as libc::c_int));
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        js_pushstring(J, message);
    } else if *message.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        js_pushstring(J, name);
    } else {
        js_pushstring(J, name);
        js_pushstring(J, b": \0" as *const u8 as *const libc::c_char);
        js_concat(J);
        js_pushstring(J, message);
        js_concat(J);
    };
}
unsafe extern "C" fn jsB_ErrorX(J: *mut js_State, prototype: *mut js_Object) -> libc::c_int {
    js_pushobject(J, jsV_newobject(J, JS_CERROR, prototype));
    if js_isdefined(J, 1 as libc::c_int) != 0 {
        js_pushstring(J, js_tostring(J, 1 as libc::c_int));
        js_defproperty(
            J,
            -(2 as libc::c_int),
            b"message\0" as *const u8 as *const libc::c_char,
            JS_DONTENUM as libc::c_int,
        );
    }
    if jsB_stacktrace(J, 1 as libc::c_int) != 0 {
        js_defproperty(
            J,
            -(2 as libc::c_int),
            b"stackTrace\0" as *const u8 as *const libc::c_char,
            JS_DONTENUM as libc::c_int,
        );
    }
    1 as libc::c_int
}
unsafe extern "C" fn js_newerrorx(
    J: *mut js_State,
    message: *const libc::c_char,
    prototype: *mut js_Object,
) {
    js_pushobject(J, jsV_newobject(J, JS_CERROR, prototype));
    js_pushstring(J, message);
    js_setproperty(
        J,
        -(2 as libc::c_int),
        b"message\0" as *const u8 as *const libc::c_char,
    );
    if jsB_stacktrace(J, 0 as libc::c_int) != 0 {
        js_setproperty(
            J,
            -(2 as libc::c_int),
            b"stackTrace\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_newerror(J: *mut js_State, s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).Error_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_error(J: *mut js_State, fmt: *const libc::c_char, args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).Error_prototype);
    js_throw(J);
}
unsafe extern "C" fn jsB_Error(J: *mut js_State) {
    jsB_ErrorX(J, (*J).Error_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_evalerror(J: *mut js_State, fmt: *const libc::c_char, args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).EvalError_prototype);
    js_throw(J);
}
unsafe extern "C" fn jsB_EvalError(J: *mut js_State) {
    jsB_ErrorX(J, (*J).EvalError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_newevalerror(J: *mut js_State, s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).EvalError_prototype);
}
unsafe extern "C" fn jsB_RangeError(J: *mut js_State) {
    jsB_ErrorX(J, (*J).RangeError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_rangeerror(J: *mut js_State, fmt: *const libc::c_char, args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).RangeError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newrangeerror(J: *mut js_State, s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).RangeError_prototype);
}
unsafe extern "C" fn jsB_ReferenceError(J: *mut js_State) {
    jsB_ErrorX(J, (*J).ReferenceError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_newreferenceerror(J: *mut js_State, s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).ReferenceError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_referenceerror(
    J: *mut js_State,
    fmt: *const libc::c_char,
    args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).ReferenceError_prototype);
    js_throw(J);
}
unsafe extern "C" fn jsB_SyntaxError(J: *mut js_State) {
    jsB_ErrorX(J, (*J).SyntaxError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_syntaxerror(
    J: *mut js_State,
    fmt: *const libc::c_char,
    args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).SyntaxError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newsyntaxerror(J: *mut js_State, s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).SyntaxError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_typeerror(J: *mut js_State, fmt: *const libc::c_char, args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).TypeError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newtypeerror(J: *mut js_State, s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).TypeError_prototype);
}
unsafe extern "C" fn jsB_TypeError(J: *mut js_State) {
    jsB_ErrorX(J, (*J).TypeError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_urierror(J: *mut js_State, fmt: *const libc::c_char, args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).URIError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newurierror(J: *mut js_State, s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).URIError_prototype);
}
unsafe extern "C" fn jsB_URIError(J: *mut js_State) {
    jsB_ErrorX(J, (*J).URIError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initerror(J: *mut js_State) {
    js_pushobject(J, (*J).Error_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"Error\0" as *const u8 as *const libc::c_char,
    );
    jsB_propf(
        J,
        b"Error.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Ep_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_Error as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_Error as unsafe extern "C" fn(*mut js_State) -> ()),
        b"Error\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"Error\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_pushobject(J, (*J).EvalError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"EvalError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_EvalError as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_EvalError as unsafe extern "C" fn(*mut js_State) -> ()),
        b"EvalError\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"EvalError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_pushobject(J, (*J).RangeError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"RangeError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_RangeError as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_RangeError as unsafe extern "C" fn(*mut js_State) -> ()),
        b"RangeError\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"RangeError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_pushobject(J, (*J).ReferenceError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"ReferenceError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_ReferenceError as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_ReferenceError as unsafe extern "C" fn(*mut js_State) -> ()),
        b"ReferenceError\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"ReferenceError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_pushobject(J, (*J).SyntaxError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"SyntaxError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_SyntaxError as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_SyntaxError as unsafe extern "C" fn(*mut js_State) -> ()),
        b"SyntaxError\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"SyntaxError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_pushobject(J, (*J).TypeError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"TypeError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_TypeError as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_TypeError as unsafe extern "C" fn(*mut js_State) -> ()),
        b"TypeError\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"TypeError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_pushobject(J, (*J).URIError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"URIError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_URIError as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_URIError as unsafe extern "C" fn(*mut js_State) -> ()),
        b"URIError\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"URIError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn jsB_Function(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut body: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut parse: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut fun: *mut js_Function = std::ptr::null_mut::<js_Function>();
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        jsP_freeparse(J);
        js_throw(J);
    }
    if top > 2 as libc::c_int {
        i = 1 as libc::c_int;
        while i < top - 1 as libc::c_int {
            if i > 1 as libc::c_int {
                js_putc(J, &mut sb, ',' as i32);
            }
            js_puts(J, &mut sb, js_tostring(J, i));
            i += 1;
            i;
        }
        js_putc(J, &mut sb, ')' as i32);
        js_putc(J, &mut sb, 0 as libc::c_int);
    }
    body = if js_isdefined(J, top - 1 as libc::c_int) != 0 {
        js_tostring(J, top - 1 as libc::c_int)
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    parse = jsP_parsefunction(
        J,
        b"[string]\0" as *const u8 as *const libc::c_char,
        if !sb.is_null() {
            ((*sb).s).as_mut_ptr() as *const libc::c_char
        } else {
            std::ptr::null_mut::<libc::c_char>()
        },
        body,
    );
    fun = jsC_compilefunction(J, parse);
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
    jsP_freeparse(J);
    js_newfunction(J, fun, (*J).GE);
}
unsafe extern "C" fn jsB_Function_prototype(J: *mut js_State) {
    js_pushundefined(J);
}
unsafe extern "C" fn Fp_toString(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut i: libc::c_int = 0;
    if js_iscallable(J, 0 as libc::c_int) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    if (*self_0).type_0 as libc::c_uint == JS_CFUNCTION as libc::c_int as libc::c_uint
        || (*self_0).type_0 as libc::c_uint == JS_CSCRIPT as libc::c_int as libc::c_uint
    {
        let F: *mut js_Function = (*self_0).u.f.function;
        if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
            js_free(J, sb as *mut libc::c_void);
            js_throw(J);
        }
        js_puts(
            J,
            &mut sb,
            b"function \0" as *const u8 as *const libc::c_char,
        );
        js_puts(J, &mut sb, (*F).name);
        js_putc(J, &mut sb, '(' as i32);
        i = 0 as libc::c_int;
        while i < (*F).numparams {
            if i > 0 as libc::c_int {
                js_putc(J, &mut sb, ',' as i32);
            }
            js_puts(J, &mut sb, *((*F).vartab).offset(i as isize));
            i += 1;
            i;
        }
        js_puts(
            J,
            &mut sb,
            b") { [byte code] }\0" as *const u8 as *const libc::c_char,
        );
        js_putc(J, &mut sb, 0 as libc::c_int);
        js_pushstring(J, ((*sb).s).as_mut_ptr() as *const libc::c_char);
        js_endtry(J);
        js_free(J, sb as *mut libc::c_void);
    } else if (*self_0).type_0 as libc::c_uint == JS_CCFUNCTION as libc::c_int as libc::c_uint {
        if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
            js_free(J, sb as *mut libc::c_void);
            js_throw(J);
        }
        js_puts(
            J,
            &mut sb,
            b"function \0" as *const u8 as *const libc::c_char,
        );
        js_puts(J, &mut sb, (*self_0).u.c.name);
        js_puts(
            J,
            &mut sb,
            b"() { [native code] }\0" as *const u8 as *const libc::c_char,
        );
        js_putc(J, &mut sb, 0 as libc::c_int);
        js_pushstring(J, ((*sb).s).as_mut_ptr() as *const libc::c_char);
        js_endtry(J);
        js_free(J, sb as *mut libc::c_void);
    } else {
        js_pushliteral(J, b"function () { }\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn Fp_apply(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if js_iscallable(J, 0 as libc::c_int) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    js_copy(J, 0 as libc::c_int);
    js_copy(J, 1 as libc::c_int);
    if js_isnull(J, 2 as libc::c_int) != 0 || js_isundefined(J, 2 as libc::c_int) != 0 {
        n = 0 as libc::c_int;
    } else {
        n = js_getlength(J, 2 as libc::c_int);
        if n < 0 as libc::c_int {
            n = 0 as libc::c_int;
        }
        i = 0 as libc::c_int;
        while i < n {
            js_getindex(J, 2 as libc::c_int, i);
            i += 1;
            i;
        }
    }
    js_call(J, n);
}
unsafe extern "C" fn Fp_call(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    if js_iscallable(J, 0 as libc::c_int) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    i = 0 as libc::c_int;
    while i < top {
        js_copy(J, i);
        i += 1;
        i;
    }
    js_call(J, top - 2 as libc::c_int);
}
unsafe extern "C" fn callbound(J: *mut js_State) {
    let top: libc::c_int = js_gettop(J);
    let mut i: libc::c_int = 0;
    let mut fun: libc::c_int = 0;
    let mut args: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    fun = js_gettop(J);
    js_currentfunction(J);
    js_getproperty(
        J,
        fun,
        b"__TargetFunction__\0" as *const u8 as *const libc::c_char,
    );
    js_getproperty(
        J,
        fun,
        b"__BoundThis__\0" as *const u8 as *const libc::c_char,
    );
    args = js_gettop(J);
    js_getproperty(
        J,
        fun,
        b"__BoundArguments__\0" as *const u8 as *const libc::c_char,
    );
    n = js_getlength(J, args);
    if n < 0 as libc::c_int {
        n = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        js_getindex(J, args, i);
        i += 1;
        i;
    }
    js_remove(J, args);
    i = 1 as libc::c_int;
    while i < top {
        js_copy(J, i);
        i += 1;
        i;
    }
    js_call(J, n + top - 1 as libc::c_int);
}
unsafe extern "C" fn constructbound(J: *mut js_State) {
    let top: libc::c_int = js_gettop(J);
    let mut i: libc::c_int = 0;
    let mut fun: libc::c_int = 0;
    let mut args: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    fun = js_gettop(J);
    js_currentfunction(J);
    js_getproperty(
        J,
        fun,
        b"__TargetFunction__\0" as *const u8 as *const libc::c_char,
    );
    args = js_gettop(J);
    js_getproperty(
        J,
        fun,
        b"__BoundArguments__\0" as *const u8 as *const libc::c_char,
    );
    n = js_getlength(J, args);
    if n < 0 as libc::c_int {
        n = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < n {
        js_getindex(J, args, i);
        i += 1;
        i;
    }
    js_remove(J, args);
    i = 1 as libc::c_int;
    while i < top {
        js_copy(J, i);
        i += 1;
        i;
    }
    js_construct(J, n + top - 1 as libc::c_int);
}
unsafe extern "C" fn Fp_bind(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    let mut n: libc::c_int = 0;
    if js_iscallable(J, 0 as libc::c_int) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    n = js_getlength(J, 0 as libc::c_int);
    if n > top - 2 as libc::c_int {
        n -= top - 2 as libc::c_int;
    } else {
        n = 0 as libc::c_int;
    }
    js_getproperty(
        J,
        0 as libc::c_int,
        b"prototype\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(callbound as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(constructbound as unsafe extern "C" fn(*mut js_State) -> ()),
        b"[bind]\0" as *const u8 as *const libc::c_char,
        n,
    );
    js_copy(J, 0 as libc::c_int);
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"__TargetFunction__\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    js_copy(J, 1 as libc::c_int);
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"__BoundThis__\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    js_newarray(J);
    i = 2 as libc::c_int;
    while i < top {
        js_copy(J, i);
        js_setindex(J, -(2 as libc::c_int), i - 2 as libc::c_int);
        i += 1;
        i;
    }
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"__BoundArguments__\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initfunction(J: *mut js_State) {
    (*(*J).Function_prototype).u.c.name =
        b"Function.prototype\0" as *const u8 as *const libc::c_char;
    (*(*J).Function_prototype).u.c.function =
        Some(jsB_Function_prototype as unsafe extern "C" fn(*mut js_State) -> ());
    (*(*J).Function_prototype).u.c.constructor = None;
    (*(*J).Function_prototype).u.c.length = 0 as libc::c_int;
    js_pushobject(J, (*J).Function_prototype);
    jsB_propf(
        J,
        b"Function.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Fp_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Function.prototype.apply\0" as *const u8 as *const libc::c_char,
        Some(Fp_apply as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Function.prototype.call\0" as *const u8 as *const libc::c_char,
        Some(Fp_call as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Function.prototype.bind\0" as *const u8 as *const libc::c_char,
        Some(Fp_bind as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_Function as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_Function as unsafe extern "C" fn(*mut js_State) -> ()),
        b"Function\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"Function\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn jsG_freeenvironment(J: *mut js_State, env: *mut js_Environment) {
    js_free(J, env as *mut libc::c_void);
}
unsafe extern "C" fn jsG_freefunction(J: *mut js_State, fun: *mut js_Function) {
    js_free(J, (*fun).funtab as *mut libc::c_void);
    js_free(J, (*fun).vartab as *mut libc::c_void);
    js_free(J, (*fun).code as *mut libc::c_void);
    js_free(J, fun as *mut libc::c_void);
}
unsafe extern "C" fn jsG_freeproperty(J: *mut js_State, node: *mut js_Property) {
    if (*(*node).left).level != 0 {
        jsG_freeproperty(J, (*node).left);
    }
    if (*(*node).right).level != 0 {
        jsG_freeproperty(J, (*node).right);
    }
    js_free(J, node as *mut libc::c_void);
}
unsafe extern "C" fn jsG_freeiterator(J: *mut js_State, mut node: *mut js_Iterator) {
    while !node.is_null() {
        let next_0: *mut js_Iterator = (*node).next;
        js_free(J, node as *mut libc::c_void);
        node = next_0;
    }
}
unsafe extern "C" fn jsG_freeobject(J: *mut js_State, obj: *mut js_Object) {
    if (*(*obj).properties).level != 0 {
        jsG_freeproperty(J, (*obj).properties);
    }
    if (*obj).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint {
        js_free(J, (*obj).u.r.source as *mut libc::c_void);
        js_regfreex((*J).alloc, (*J).actx, (*obj).u.r.prog as *mut Reprog);
    }
    if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint
        && (*obj).u.s.string != ((*obj).u.s.shrstr).as_mut_ptr()
    {
        js_free(J, (*obj).u.s.string as *mut libc::c_void);
    }
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        js_free(J, (*obj).u.a.array as *mut libc::c_void);
    }
    if (*obj).type_0 as libc::c_uint == JS_CITERATOR as libc::c_int as libc::c_uint {
        jsG_freeiterator(J, (*obj).u.iter.head);
    }
    if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as libc::c_int as libc::c_uint
        && ((*obj).u.user.finalize).is_some()
    {
        ((*obj).u.user.finalize).expect("non-null function pointer")(J, (*obj).u.user.data);
    }
    if (*obj).type_0 as libc::c_uint == JS_CCFUNCTION as libc::c_int as libc::c_uint
        && ((*obj).u.c.finalize).is_some()
    {
        ((*obj).u.c.finalize).expect("non-null function pointer")(J, (*obj).u.c.data);
    }
    js_free(J, obj as *mut libc::c_void);
}
unsafe extern "C" fn jsG_markobject(J: *mut js_State, mark: libc::c_int, obj: *mut js_Object) {
    (*obj).gcmark = mark;
    (*obj).gcroot = (*J).gcroot;
    (*J).gcroot = obj;
}
unsafe extern "C" fn jsG_markfunction(J: *mut js_State, mark: libc::c_int, fun: *mut js_Function) {
    let mut i: libc::c_int = 0;
    (*fun).gcmark = mark;
    i = 0 as libc::c_int;
    while i < (*fun).funlen {
        if (**((*fun).funtab).offset(i as isize)).gcmark != mark {
            jsG_markfunction(J, mark, *((*fun).funtab).offset(i as isize));
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn jsG_markenvironment(
    J: *mut js_State,
    mark: libc::c_int,
    mut env: *mut js_Environment,
) {
    loop {
        (*env).gcmark = mark;
        if (*(*env).variables).gcmark != mark {
            jsG_markobject(J, mark, (*env).variables);
        }
        env = (*env).outer;
        if !(!env.is_null() && (*env).gcmark != mark) {
            break;
        }
    }
}
unsafe extern "C" fn jsG_markproperty(J: *mut js_State, mark: libc::c_int, node: *mut js_Property) {
    if (*(*node).left).level != 0 {
        jsG_markproperty(J, mark, (*node).left);
    }
    if (*(*node).right).level != 0 {
        jsG_markproperty(J, mark, (*node).right);
    }
    if (*node).value.t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
        && (*(*node).value.u.memstr).gcmark as libc::c_int != mark
    {
        (*(*node).value.u.memstr).gcmark = mark as libc::c_char;
    }
    if (*node).value.t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && (*(*node).value.u.object).gcmark != mark
    {
        jsG_markobject(J, mark, (*node).value.u.object);
    }
    if !((*node).getter).is_null() && (*(*node).getter).gcmark != mark {
        jsG_markobject(J, mark, (*node).getter);
    }
    if !((*node).setter).is_null() && (*(*node).setter).gcmark != mark {
        jsG_markobject(J, mark, (*node).setter);
    }
}
unsafe extern "C" fn jsG_scanobject(J: *mut js_State, mark: libc::c_int, obj: *mut js_Object) {
    if (*(*obj).properties).level != 0 {
        jsG_markproperty(J, mark, (*obj).properties);
    }
    if !((*obj).prototype).is_null() && (*(*obj).prototype).gcmark != mark {
        jsG_markobject(J, mark, (*obj).prototype);
    }
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < (*obj).u.a.flat_length {
            let v: *mut js_Value = &mut *((*obj).u.a.array).offset(i as isize) as *mut js_Value;
            if (*v).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
                && (*(*v).u.memstr).gcmark as libc::c_int != mark
            {
                (*(*v).u.memstr).gcmark = mark as libc::c_char;
            }
            if (*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
                && (*(*v).u.object).gcmark != mark
            {
                jsG_markobject(J, mark, (*v).u.object);
            }
            i += 1;
            i;
        }
    }
    if (*obj).type_0 as libc::c_uint == JS_CITERATOR as libc::c_int as libc::c_uint
        && (*(*obj).u.iter.target).gcmark != mark
    {
        jsG_markobject(J, mark, (*obj).u.iter.target);
    }
    if (*obj).type_0 as libc::c_uint == JS_CFUNCTION as libc::c_int as libc::c_uint
        || (*obj).type_0 as libc::c_uint == JS_CSCRIPT as libc::c_int as libc::c_uint
    {
        if !((*obj).u.f.scope).is_null() && (*(*obj).u.f.scope).gcmark != mark {
            jsG_markenvironment(J, mark, (*obj).u.f.scope);
        }
        if !((*obj).u.f.function).is_null() && (*(*obj).u.f.function).gcmark != mark {
            jsG_markfunction(J, mark, (*obj).u.f.function);
        }
    }
}
unsafe extern "C" fn jsG_markstack(J: *mut js_State, mark: libc::c_int) {
    let mut v: *mut js_Value = (*J).stack;
    let mut n: libc::c_int = (*J).top;
    loop {
        let fresh25 = n;
        n -= 1;
        if fresh25 == 0 {
            break;
        }
        if (*v).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
            && (*(*v).u.memstr).gcmark as libc::c_int != mark
        {
            (*(*v).u.memstr).gcmark = mark as libc::c_char;
        }
        if (*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
            && (*(*v).u.object).gcmark != mark
        {
            jsG_markobject(J, mark, (*v).u.object);
        }
        v = v.offset(1);
        v;
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_gc(J: *mut js_State, report: libc::c_int) {
    let mut fun: *mut js_Function = std::ptr::null_mut::<js_Function>();
    let mut nextfun: *mut js_Function = std::ptr::null_mut::<js_Function>();
    let mut prevnextfun: *mut *mut js_Function = std::ptr::null_mut::<*mut js_Function>();
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut nextobj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut prevnextobj: *mut *mut js_Object = std::ptr::null_mut::<*mut js_Object>();
    let mut str: *mut js_String = std::ptr::null_mut::<js_String>();
    let mut nextstr: *mut js_String = std::ptr::null_mut::<js_String>();
    let mut prevnextstr: *mut *mut js_String = std::ptr::null_mut::<*mut js_String>();
    let mut env: *mut js_Environment = std::ptr::null_mut::<js_Environment>();
    let mut nextenv: *mut js_Environment = std::ptr::null_mut::<js_Environment>();
    let mut prevnextenv: *mut *mut js_Environment = std::ptr::null_mut::<*mut js_Environment>();
    let mut nenv: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut nfun: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut nobj: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut nstr: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut nprop: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut genv: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut gfun: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut gobj: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut gstr: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut gprop: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut mark: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    (*J).gcmark = if (*J).gcmark == 1 as libc::c_int {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    };
    mark = (*J).gcmark;
    jsG_markobject(J, mark, (*J).Object_prototype);
    jsG_markobject(J, mark, (*J).Array_prototype);
    jsG_markobject(J, mark, (*J).Function_prototype);
    jsG_markobject(J, mark, (*J).Boolean_prototype);
    jsG_markobject(J, mark, (*J).Number_prototype);
    jsG_markobject(J, mark, (*J).String_prototype);
    jsG_markobject(J, mark, (*J).RegExp_prototype);
    jsG_markobject(J, mark, (*J).Date_prototype);
    jsG_markobject(J, mark, (*J).Error_prototype);
    jsG_markobject(J, mark, (*J).EvalError_prototype);
    jsG_markobject(J, mark, (*J).RangeError_prototype);
    jsG_markobject(J, mark, (*J).ReferenceError_prototype);
    jsG_markobject(J, mark, (*J).SyntaxError_prototype);
    jsG_markobject(J, mark, (*J).TypeError_prototype);
    jsG_markobject(J, mark, (*J).URIError_prototype);
    jsG_markobject(J, mark, (*J).R);
    jsG_markobject(J, mark, (*J).G);
    jsG_markstack(J, mark);
    jsG_markenvironment(J, mark, (*J).E);
    jsG_markenvironment(J, mark, (*J).GE);
    i = 0 as libc::c_int;
    while i < (*J).envtop {
        jsG_markenvironment(J, mark, (*J).envstack[i as usize]);
        i += 1;
        i;
    }
    loop {
        obj = (*J).gcroot;
        if obj.is_null() {
            break;
        }
        (*J).gcroot = (*obj).gcroot;
        (*obj).gcroot = std::ptr::null_mut::<js_Object>();
        jsG_scanobject(J, mark, obj);
    }
    prevnextenv = &mut (*J).gcenv;
    env = (*J).gcenv;
    while !env.is_null() {
        nextenv = (*env).gcnext;
        if (*env).gcmark != mark {
            *prevnextenv = nextenv;
            jsG_freeenvironment(J, env);
            genv = genv.wrapping_add(1);
            genv;
        } else {
            prevnextenv = &mut (*env).gcnext;
        }
        nenv = nenv.wrapping_add(1);
        nenv;
        env = nextenv;
    }
    prevnextfun = &mut (*J).gcfun;
    fun = (*J).gcfun;
    while !fun.is_null() {
        nextfun = (*fun).gcnext;
        if (*fun).gcmark != mark {
            *prevnextfun = nextfun;
            jsG_freefunction(J, fun);
            gfun = gfun.wrapping_add(1);
            gfun;
        } else {
            prevnextfun = &mut (*fun).gcnext;
        }
        nfun = nfun.wrapping_add(1);
        nfun;
        fun = nextfun;
    }
    prevnextobj = &mut (*J).gcobj;
    obj = (*J).gcobj;
    while !obj.is_null() {
        nprop = nprop.wrapping_add((*obj).count as libc::c_uint);
        nextobj = (*obj).gcnext;
        if (*obj).gcmark != mark {
            gprop = gprop.wrapping_add((*obj).count as libc::c_uint);
            *prevnextobj = nextobj;
            jsG_freeobject(J, obj);
            gobj = gobj.wrapping_add(1);
            gobj;
        } else {
            prevnextobj = &mut (*obj).gcnext;
        }
        nobj = nobj.wrapping_add(1);
        nobj;
        obj = nextobj;
    }
    prevnextstr = &mut (*J).gcstr;
    str = (*J).gcstr;
    while !str.is_null() {
        nextstr = (*str).gcnext;
        if (*str).gcmark as libc::c_int != mark {
            *prevnextstr = nextstr;
            js_free(J, str as *mut libc::c_void);
            gstr = gstr.wrapping_add(1);
            gstr;
        } else {
            prevnextstr = &mut (*str).gcnext;
        }
        nstr = nstr.wrapping_add(1);
        nstr;
        str = nextstr;
    }
    let ntot: libc::c_uint = nenv
        .wrapping_add(nfun)
        .wrapping_add(nobj)
        .wrapping_add(nstr)
        .wrapping_add(nprop);
    let gtot: libc::c_uint = genv
        .wrapping_add(gfun)
        .wrapping_add(gobj)
        .wrapping_add(gstr)
        .wrapping_add(gprop);
    let remaining: libc::c_uint = ntot.wrapping_sub(gtot);
    (*J).gccounter = remaining;
    (*J).gcthresh = (remaining as libc::c_double * 5.0f64) as libc::c_uint;
    if report != 0 {
        let mut buf: [libc::c_char; 256] = [0; 256];
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"garbage collected (%d%%): %d/%d envs, %d/%d funs, %d/%d objs, %d/%d props, %d/%d strs\0"
                as *const u8 as *const libc::c_char,
            (100 as libc::c_int as libc::c_uint).wrapping_mul(gtot).wrapping_div(ntot),
            genv,
            nenv,
            gfun,
            nfun,
            gobj,
            nobj,
            gprop,
            nprop,
            gstr,
            nstr,
        );
        js_report(J, buf.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_freestate(J: *mut js_State) {
    let mut fun: *mut js_Function = std::ptr::null_mut::<js_Function>();
    let mut nextfun: *mut js_Function = std::ptr::null_mut::<js_Function>();
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut nextobj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut env: *mut js_Environment = std::ptr::null_mut::<js_Environment>();
    let mut nextenv: *mut js_Environment = std::ptr::null_mut::<js_Environment>();
    let mut str: *mut js_String = std::ptr::null_mut::<js_String>();
    let mut nextstr: *mut js_String = std::ptr::null_mut::<js_String>();
    if J.is_null() {
        return;
    }
    env = (*J).gcenv;
    while !env.is_null() {
        nextenv = (*env).gcnext;
        jsG_freeenvironment(J, env);
        env = nextenv;
    }
    fun = (*J).gcfun;
    while !fun.is_null() {
        nextfun = (*fun).gcnext;
        jsG_freefunction(J, fun);
        fun = nextfun;
    }
    obj = (*J).gcobj;
    while !obj.is_null() {
        nextobj = (*obj).gcnext;
        jsG_freeobject(J, obj);
        obj = nextobj;
    }
    str = (*J).gcstr;
    while !str.is_null() {
        nextstr = (*str).gcnext;
        js_free(J, str as *mut libc::c_void);
        str = nextstr;
    }
    jsS_freestrings(J);
    js_free(J, (*J).lexbuf.text as *mut libc::c_void);
    ((*J).alloc).expect("non-null function pointer")(
        (*J).actx,
        (*J).stack as *mut libc::c_void,
        0 as libc::c_int,
    );
    ((*J).alloc).expect("non-null function pointer")(
        (*J).actx,
        J as *mut libc::c_void,
        0 as libc::c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn js_putc(J: *mut js_State, sbp: *mut *mut js_Buffer, c: libc::c_int) {
    let mut sb: *mut js_Buffer = *sbp;
    if sb.is_null() {
        sb = Box::into_raw(Box::new(js_Buffer {
            s: Vec::with_capacity(64),
        }));
        *sbp = sb;
    }
    (*sb).s.push(c);
}

#[no_mangle]
pub unsafe extern "C" fn js_puts(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    mut s: *const libc::c_char,
) {
    while *s != 0 {
        let fresh27 = s;
        s = s.offset(1);
        js_putc(J, sb, *fresh27 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_putm(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    mut s: *const libc::c_char,
    e: *const libc::c_char,
) {
    while s < e {
        let fresh28 = s;
        s = s.offset(1);
        js_putc(J, sb, *fresh28 as libc::c_int);
    }
}
static mut jsS_sentinel: js_StringNode = unsafe {
    {
        js_StringNode {
            left: &jsS_sentinel as *const js_StringNode as *mut js_StringNode,
            right: &jsS_sentinel as *const js_StringNode as *mut js_StringNode,
            level: 0 as libc::c_int,
            string: *::core::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0"),
        }
    }
};
unsafe extern "C" fn jsS_newstringnode(
    J: *mut js_State,
    string: *const libc::c_char,
    result: *mut *const libc::c_char,
) -> *mut js_StringNode {
    let n: size_t = strlen(string);
    if n > ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong {
        js_rangeerror(
            J,
            b"invalid string length\0" as *const u8 as *const libc::c_char,
        );
    }
    let node: *mut js_StringNode = js_malloc(
        J,
        (20 as libc::c_ulong as libc::c_int as libc::c_ulong)
            .wrapping_add(n)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut js_StringNode;
    (*node).right = &mut jsS_sentinel;
    (*node).left = (*node).right;
    (*node).level = 1 as libc::c_int;
    memcpy(
        ((*node).string).as_mut_ptr() as *mut libc::c_void,
        string as *const libc::c_void,
        n.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    *result = ((*node).string).as_mut_ptr();
    node
}
unsafe extern "C" fn jsS_skew(mut node: *mut js_StringNode) -> *mut js_StringNode {
    if (*(*node).left).level == (*node).level {
        let temp: *mut js_StringNode = node;
        node = (*node).left;
        (*temp).left = (*node).right;
        (*node).right = temp;
    }
    node
}
unsafe extern "C" fn jsS_split(mut node: *mut js_StringNode) -> *mut js_StringNode {
    if (*(*(*node).right).right).level == (*node).level {
        let temp: *mut js_StringNode = node;
        node = (*node).right;
        (*temp).right = (*node).left;
        (*node).left = temp;
        (*node).level += 1;
        (*node).level;
    }
    node
}
unsafe extern "C" fn jsS_insert(
    J: *mut js_State,
    mut node: *mut js_StringNode,
    string: *const libc::c_char,
    result: *mut *const libc::c_char,
) -> *mut js_StringNode {
    if node != &mut jsS_sentinel as *mut js_StringNode {
        let c: libc::c_int = strcmp(string, ((*node).string).as_mut_ptr());
        if c < 0 as libc::c_int {
            (*node).left = jsS_insert(J, (*node).left, string, result);
        } else if c > 0 as libc::c_int {
            (*node).right = jsS_insert(J, (*node).right, string, result);
        } else {
            *result = ((*node).string).as_mut_ptr();
            return node;
        }
        node = jsS_skew(node);
        node = jsS_split(node);
        return node;
    }
    jsS_newstringnode(J, string, result)
}
unsafe extern "C" fn dumpstringnode(node: *mut js_StringNode, level: libc::c_int) {
    let mut i: libc::c_int = 0;
    if (*node).left != &mut jsS_sentinel as *mut js_StringNode {
        dumpstringnode((*node).left, level + 1 as libc::c_int);
    }
    printf(b"%d: \0" as *const u8 as *const libc::c_char, (*node).level);
    i = 0 as libc::c_int;
    while i < level {
        putchar('\t' as i32);
        i += 1;
        i;
    }
    printf(
        b"'%s'\n\0" as *const u8 as *const libc::c_char,
        ((*node).string).as_mut_ptr(),
    );
    if (*node).right != &mut jsS_sentinel as *mut js_StringNode {
        dumpstringnode((*node).right, level + 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsS_dumpstrings(J: *mut js_State) {
    let root: *mut js_StringNode = (*J).strings;
    printf(b"interned strings {\n\0" as *const u8 as *const libc::c_char);
    if !root.is_null() && root != &mut jsS_sentinel as *mut js_StringNode {
        dumpstringnode(root, 1 as libc::c_int);
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn jsS_freestringnode(J: *mut js_State, node: *mut js_StringNode) {
    if (*node).left != &mut jsS_sentinel as *mut js_StringNode {
        jsS_freestringnode(J, (*node).left);
    }
    if (*node).right != &mut jsS_sentinel as *mut js_StringNode {
        jsS_freestringnode(J, (*node).right);
    }
    js_free(J, node as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn jsS_freestrings(J: *mut js_State) {
    if !((*J).strings).is_null() && (*J).strings != &mut jsS_sentinel as *mut js_StringNode {
        jsS_freestringnode(J, (*J).strings);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_intern(
    J: *mut js_State,
    s: *const libc::c_char,
) -> *const libc::c_char {
    let mut result: *const libc::c_char = std::ptr::null::<libc::c_char>();
    if ((*J).strings).is_null() {
        (*J).strings = &mut jsS_sentinel;
    }
    (*J).strings = jsS_insert(J, (*J).strings, s, &mut result);
    result
}
unsafe extern "C" fn jsY_error(J: *mut js_State, fmt: *const libc::c_char, args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut msgbuf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        msgbuf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    snprintf(
        buf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"%s:%d: \0" as *const u8 as *const libc::c_char,
        (*J).filename,
        (*J).lexline,
    );
    strcat(buf.as_mut_ptr(), msgbuf.as_mut_ptr());
    js_newsyntaxerror(J, buf.as_mut_ptr());
    js_throw(J);
}
static mut tokenstring: [*const libc::c_char; 313] = [
    b"(end-of-file)\0" as *const u8 as *const libc::c_char,
    b"'\\x01'\0" as *const u8 as *const libc::c_char,
    b"'\\x02'\0" as *const u8 as *const libc::c_char,
    b"'\\x03'\0" as *const u8 as *const libc::c_char,
    b"'\\x04'\0" as *const u8 as *const libc::c_char,
    b"'\\x05'\0" as *const u8 as *const libc::c_char,
    b"'\\x06'\0" as *const u8 as *const libc::c_char,
    b"'\\x07'\0" as *const u8 as *const libc::c_char,
    b"'\\x08'\0" as *const u8 as *const libc::c_char,
    b"'\\x09'\0" as *const u8 as *const libc::c_char,
    b"'\\x0A'\0" as *const u8 as *const libc::c_char,
    b"'\\x0B'\0" as *const u8 as *const libc::c_char,
    b"'\\x0C'\0" as *const u8 as *const libc::c_char,
    b"'\\x0D'\0" as *const u8 as *const libc::c_char,
    b"'\\x0E'\0" as *const u8 as *const libc::c_char,
    b"'\\x0F'\0" as *const u8 as *const libc::c_char,
    b"'\\x10'\0" as *const u8 as *const libc::c_char,
    b"'\\x11'\0" as *const u8 as *const libc::c_char,
    b"'\\x12'\0" as *const u8 as *const libc::c_char,
    b"'\\x13'\0" as *const u8 as *const libc::c_char,
    b"'\\x14'\0" as *const u8 as *const libc::c_char,
    b"'\\x15'\0" as *const u8 as *const libc::c_char,
    b"'\\x16'\0" as *const u8 as *const libc::c_char,
    b"'\\x17'\0" as *const u8 as *const libc::c_char,
    b"'\\x18'\0" as *const u8 as *const libc::c_char,
    b"'\\x19'\0" as *const u8 as *const libc::c_char,
    b"'\\x1A'\0" as *const u8 as *const libc::c_char,
    b"'\\x1B'\0" as *const u8 as *const libc::c_char,
    b"'\\x1C'\0" as *const u8 as *const libc::c_char,
    b"'\\x1D'\0" as *const u8 as *const libc::c_char,
    b"'\\x1E'\0" as *const u8 as *const libc::c_char,
    b"'\\x1F'\0" as *const u8 as *const libc::c_char,
    b"' '\0" as *const u8 as *const libc::c_char,
    b"'!'\0" as *const u8 as *const libc::c_char,
    b"'\"'\0" as *const u8 as *const libc::c_char,
    b"'#'\0" as *const u8 as *const libc::c_char,
    b"'$'\0" as *const u8 as *const libc::c_char,
    b"'%'\0" as *const u8 as *const libc::c_char,
    b"'&'\0" as *const u8 as *const libc::c_char,
    b"'\\''\0" as *const u8 as *const libc::c_char,
    b"'('\0" as *const u8 as *const libc::c_char,
    b"')'\0" as *const u8 as *const libc::c_char,
    b"'*'\0" as *const u8 as *const libc::c_char,
    b"'+'\0" as *const u8 as *const libc::c_char,
    b"','\0" as *const u8 as *const libc::c_char,
    b"'-'\0" as *const u8 as *const libc::c_char,
    b"'.'\0" as *const u8 as *const libc::c_char,
    b"'/'\0" as *const u8 as *const libc::c_char,
    b"'0'\0" as *const u8 as *const libc::c_char,
    b"'1'\0" as *const u8 as *const libc::c_char,
    b"'2'\0" as *const u8 as *const libc::c_char,
    b"'3'\0" as *const u8 as *const libc::c_char,
    b"'4'\0" as *const u8 as *const libc::c_char,
    b"'5'\0" as *const u8 as *const libc::c_char,
    b"'6'\0" as *const u8 as *const libc::c_char,
    b"'7'\0" as *const u8 as *const libc::c_char,
    b"'8'\0" as *const u8 as *const libc::c_char,
    b"'9'\0" as *const u8 as *const libc::c_char,
    b"':'\0" as *const u8 as *const libc::c_char,
    b"';'\0" as *const u8 as *const libc::c_char,
    b"'<'\0" as *const u8 as *const libc::c_char,
    b"'='\0" as *const u8 as *const libc::c_char,
    b"'>'\0" as *const u8 as *const libc::c_char,
    b"'?'\0" as *const u8 as *const libc::c_char,
    b"'@'\0" as *const u8 as *const libc::c_char,
    b"'A'\0" as *const u8 as *const libc::c_char,
    b"'B'\0" as *const u8 as *const libc::c_char,
    b"'C'\0" as *const u8 as *const libc::c_char,
    b"'D'\0" as *const u8 as *const libc::c_char,
    b"'E'\0" as *const u8 as *const libc::c_char,
    b"'F'\0" as *const u8 as *const libc::c_char,
    b"'G'\0" as *const u8 as *const libc::c_char,
    b"'H'\0" as *const u8 as *const libc::c_char,
    b"'I'\0" as *const u8 as *const libc::c_char,
    b"'J'\0" as *const u8 as *const libc::c_char,
    b"'K'\0" as *const u8 as *const libc::c_char,
    b"'L'\0" as *const u8 as *const libc::c_char,
    b"'M'\0" as *const u8 as *const libc::c_char,
    b"'N'\0" as *const u8 as *const libc::c_char,
    b"'O'\0" as *const u8 as *const libc::c_char,
    b"'P'\0" as *const u8 as *const libc::c_char,
    b"'Q'\0" as *const u8 as *const libc::c_char,
    b"'R'\0" as *const u8 as *const libc::c_char,
    b"'S'\0" as *const u8 as *const libc::c_char,
    b"'T'\0" as *const u8 as *const libc::c_char,
    b"'U'\0" as *const u8 as *const libc::c_char,
    b"'V'\0" as *const u8 as *const libc::c_char,
    b"'W'\0" as *const u8 as *const libc::c_char,
    b"'X'\0" as *const u8 as *const libc::c_char,
    b"'Y'\0" as *const u8 as *const libc::c_char,
    b"'Z'\0" as *const u8 as *const libc::c_char,
    b"'['\0" as *const u8 as *const libc::c_char,
    b"''\0" as *const u8 as *const libc::c_char,
    b"']'\0" as *const u8 as *const libc::c_char,
    b"'^'\0" as *const u8 as *const libc::c_char,
    b"'_'\0" as *const u8 as *const libc::c_char,
    b"'`'\0" as *const u8 as *const libc::c_char,
    b"'a'\0" as *const u8 as *const libc::c_char,
    b"'b'\0" as *const u8 as *const libc::c_char,
    b"'c'\0" as *const u8 as *const libc::c_char,
    b"'d'\0" as *const u8 as *const libc::c_char,
    b"'e'\0" as *const u8 as *const libc::c_char,
    b"'f'\0" as *const u8 as *const libc::c_char,
    b"'g'\0" as *const u8 as *const libc::c_char,
    b"'h'\0" as *const u8 as *const libc::c_char,
    b"'i'\0" as *const u8 as *const libc::c_char,
    b"'j'\0" as *const u8 as *const libc::c_char,
    b"'k'\0" as *const u8 as *const libc::c_char,
    b"'l'\0" as *const u8 as *const libc::c_char,
    b"'m'\0" as *const u8 as *const libc::c_char,
    b"'n'\0" as *const u8 as *const libc::c_char,
    b"'o'\0" as *const u8 as *const libc::c_char,
    b"'p'\0" as *const u8 as *const libc::c_char,
    b"'q'\0" as *const u8 as *const libc::c_char,
    b"'r'\0" as *const u8 as *const libc::c_char,
    b"'s'\0" as *const u8 as *const libc::c_char,
    b"'t'\0" as *const u8 as *const libc::c_char,
    b"'u'\0" as *const u8 as *const libc::c_char,
    b"'v'\0" as *const u8 as *const libc::c_char,
    b"'w'\0" as *const u8 as *const libc::c_char,
    b"'x'\0" as *const u8 as *const libc::c_char,
    b"'y'\0" as *const u8 as *const libc::c_char,
    b"'z'\0" as *const u8 as *const libc::c_char,
    b"'{'\0" as *const u8 as *const libc::c_char,
    b"'|'\0" as *const u8 as *const libc::c_char,
    b"'}'\0" as *const u8 as *const libc::c_char,
    b"'~'\0" as *const u8 as *const libc::c_char,
    b"'\\x7F'\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    0 as *const libc::c_char,
    b"(identifier)\0" as *const u8 as *const libc::c_char,
    b"(number)\0" as *const u8 as *const libc::c_char,
    b"(string)\0" as *const u8 as *const libc::c_char,
    b"(regexp)\0" as *const u8 as *const libc::c_char,
    b"'<='\0" as *const u8 as *const libc::c_char,
    b"'>='\0" as *const u8 as *const libc::c_char,
    b"'=='\0" as *const u8 as *const libc::c_char,
    b"'!='\0" as *const u8 as *const libc::c_char,
    b"'==='\0" as *const u8 as *const libc::c_char,
    b"'!=='\0" as *const u8 as *const libc::c_char,
    b"'<<'\0" as *const u8 as *const libc::c_char,
    b"'>>'\0" as *const u8 as *const libc::c_char,
    b"'>>>'\0" as *const u8 as *const libc::c_char,
    b"'&&'\0" as *const u8 as *const libc::c_char,
    b"'||'\0" as *const u8 as *const libc::c_char,
    b"'+='\0" as *const u8 as *const libc::c_char,
    b"'-='\0" as *const u8 as *const libc::c_char,
    b"'*='\0" as *const u8 as *const libc::c_char,
    b"'/='\0" as *const u8 as *const libc::c_char,
    b"'%='\0" as *const u8 as *const libc::c_char,
    b"'<<='\0" as *const u8 as *const libc::c_char,
    b"'>>='\0" as *const u8 as *const libc::c_char,
    b"'>>>='\0" as *const u8 as *const libc::c_char,
    b"'&='\0" as *const u8 as *const libc::c_char,
    b"'|='\0" as *const u8 as *const libc::c_char,
    b"'^='\0" as *const u8 as *const libc::c_char,
    b"'++'\0" as *const u8 as *const libc::c_char,
    b"'--'\0" as *const u8 as *const libc::c_char,
    b"'break'\0" as *const u8 as *const libc::c_char,
    b"'case'\0" as *const u8 as *const libc::c_char,
    b"'catch'\0" as *const u8 as *const libc::c_char,
    b"'continue'\0" as *const u8 as *const libc::c_char,
    b"'debugger'\0" as *const u8 as *const libc::c_char,
    b"'default'\0" as *const u8 as *const libc::c_char,
    b"'delete'\0" as *const u8 as *const libc::c_char,
    b"'do'\0" as *const u8 as *const libc::c_char,
    b"'else'\0" as *const u8 as *const libc::c_char,
    b"'false'\0" as *const u8 as *const libc::c_char,
    b"'finally'\0" as *const u8 as *const libc::c_char,
    b"'for'\0" as *const u8 as *const libc::c_char,
    b"'function'\0" as *const u8 as *const libc::c_char,
    b"'if'\0" as *const u8 as *const libc::c_char,
    b"'in'\0" as *const u8 as *const libc::c_char,
    b"'instanceof'\0" as *const u8 as *const libc::c_char,
    b"'new'\0" as *const u8 as *const libc::c_char,
    b"'null'\0" as *const u8 as *const libc::c_char,
    b"'return'\0" as *const u8 as *const libc::c_char,
    b"'switch'\0" as *const u8 as *const libc::c_char,
    b"'this'\0" as *const u8 as *const libc::c_char,
    b"'throw'\0" as *const u8 as *const libc::c_char,
    b"'true'\0" as *const u8 as *const libc::c_char,
    b"'try'\0" as *const u8 as *const libc::c_char,
    b"'typeof'\0" as *const u8 as *const libc::c_char,
    b"'var'\0" as *const u8 as *const libc::c_char,
    b"'void'\0" as *const u8 as *const libc::c_char,
    b"'while'\0" as *const u8 as *const libc::c_char,
    b"'with'\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn jsY_tokenstring(token: libc::c_int) -> *const libc::c_char {
    if token >= 0 as libc::c_int
        && token
            < (::core::mem::size_of::<[*const libc::c_char; 313]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                as libc::c_int
        && !(tokenstring[token as usize]).is_null()
    {
        return tokenstring[token as usize];
    }
    b"<unknown>\0" as *const u8 as *const libc::c_char
}
static mut keywords: [*const libc::c_char; 29] = [
    b"break\0" as *const u8 as *const libc::c_char,
    b"case\0" as *const u8 as *const libc::c_char,
    b"catch\0" as *const u8 as *const libc::c_char,
    b"continue\0" as *const u8 as *const libc::c_char,
    b"debugger\0" as *const u8 as *const libc::c_char,
    b"default\0" as *const u8 as *const libc::c_char,
    b"delete\0" as *const u8 as *const libc::c_char,
    b"do\0" as *const u8 as *const libc::c_char,
    b"else\0" as *const u8 as *const libc::c_char,
    b"false\0" as *const u8 as *const libc::c_char,
    b"finally\0" as *const u8 as *const libc::c_char,
    b"for\0" as *const u8 as *const libc::c_char,
    b"function\0" as *const u8 as *const libc::c_char,
    b"if\0" as *const u8 as *const libc::c_char,
    b"in\0" as *const u8 as *const libc::c_char,
    b"instanceof\0" as *const u8 as *const libc::c_char,
    b"new\0" as *const u8 as *const libc::c_char,
    b"null\0" as *const u8 as *const libc::c_char,
    b"return\0" as *const u8 as *const libc::c_char,
    b"switch\0" as *const u8 as *const libc::c_char,
    b"this\0" as *const u8 as *const libc::c_char,
    b"throw\0" as *const u8 as *const libc::c_char,
    b"true\0" as *const u8 as *const libc::c_char,
    b"try\0" as *const u8 as *const libc::c_char,
    b"typeof\0" as *const u8 as *const libc::c_char,
    b"var\0" as *const u8 as *const libc::c_char,
    b"void\0" as *const u8 as *const libc::c_char,
    b"while\0" as *const u8 as *const libc::c_char,
    b"with\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn jsY_findword(
    s: *const libc::c_char,
    list: *mut *const libc::c_char,
    num: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = num - 1 as libc::c_int;
    while l <= r {
        let m: libc::c_int = (l + r) >> 1 as libc::c_int;
        let c: libc::c_int = strcmp(s, *list.offset(m as isize));
        if c < 0 as libc::c_int {
            r = m - 1 as libc::c_int;
        } else if c > 0 as libc::c_int {
            l = m + 1 as libc::c_int;
        } else {
            return m;
        }
    }
    -(1 as libc::c_int)
}
unsafe extern "C" fn jsY_findkeyword(J: *mut js_State, s: *const libc::c_char) -> libc::c_int {
    let i: libc::c_int = jsY_findword(
        s,
        keywords.as_mut_ptr(),
        (::core::mem::size_of::<[*const libc::c_char; 29]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    if i >= 0 as libc::c_int {
        (*J).text = keywords[i as usize];
        return TK_BREAK as libc::c_int + i;
    }
    (*J).text = js_intern(J, s);
    TK_IDENTIFIER as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsY_iswhite(c: libc::c_int) -> libc::c_int {
    (c == 0x9 as libc::c_int
        || c == 0xb as libc::c_int
        || c == 0xc as libc::c_int
        || c == 0x20 as libc::c_int
        || c == 0xa0 as libc::c_int
        || c == 0xfeff as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsY_isnewline(c: libc::c_int) -> libc::c_int {
    (c == 0xa as libc::c_int
        || c == 0xd as libc::c_int
        || c == 0x2028 as libc::c_int
        || c == 0x2029 as libc::c_int) as libc::c_int
}
unsafe extern "C" fn jsY_isidentifierstart(c: libc::c_int) -> libc::c_int {
    (c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c == '$' as i32
        || c == '_' as i32
        || jsU_isalpharune(c) != 0) as libc::c_int
}
unsafe extern "C" fn jsY_isidentifierpart(c: libc::c_int) -> libc::c_int {
    (c >= '0' as i32 && c <= '9' as i32
        || (c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32)
        || c == '$' as i32
        || c == '_' as i32
        || jsU_isalpharune(c) != 0) as libc::c_int
}
unsafe extern "C" fn jsY_isdec(c: libc::c_int) -> libc::c_int {
    (c >= '0' as i32 && c <= '9' as i32) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsY_ishex(c: libc::c_int) -> libc::c_int {
    (c >= '0' as i32 && c <= '9' as i32
        || (c >= 'a' as i32 && c <= 'f' as i32 || c >= 'A' as i32 && c <= 'F' as i32))
        as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsY_tohex(c: libc::c_int) -> libc::c_int {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 0xa as libc::c_int;
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 0xa as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn jsY_next(J: *mut js_State) {
    let mut c: Rune = 0;
    if *(*J).source as libc::c_int == 0 as libc::c_int {
        (*J).lexchar = -(1 as libc::c_int);
        return;
    }
    (*J).source = ((*J).source).offset(jsU_chartorune(&mut c, (*J).source) as isize);
    if c == '\r' as i32 && *(*J).source as libc::c_int == '\n' as i32 {
        (*J).source = ((*J).source).offset(1);
        (*J).source;
    }
    if jsY_isnewline(c) != 0 {
        (*J).line += 1;
        (*J).line;
        c = '\n' as i32;
    }
    (*J).lexchar = c;
}
unsafe extern "C" fn jsY_unescape(J: *mut js_State) {
    if if (*J).lexchar == '\\' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if if (*J).lexchar == 'u' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            let mut x: libc::c_int = 0 as libc::c_int;
            if jsY_ishex((*J).lexchar) != 0 {
                x |= jsY_tohex((*J).lexchar) << 12 as libc::c_int;
                jsY_next(J);
                if jsY_ishex((*J).lexchar) != 0 {
                    x |= jsY_tohex((*J).lexchar) << 8 as libc::c_int;
                    jsY_next(J);
                    if jsY_ishex((*J).lexchar) != 0 {
                        x |= jsY_tohex((*J).lexchar) << 4 as libc::c_int;
                        jsY_next(J);
                        if jsY_ishex((*J).lexchar) != 0 {
                            x |= jsY_tohex((*J).lexchar);
                            (*J).lexchar = x;
                            return;
                        }
                    }
                }
            }
        }
        jsY_error(
            J,
            b"unexpected escape sequence\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn textinit(J: *mut js_State) {
    if ((*J).lexbuf.text).is_null() {
        (*J).lexbuf.cap = 4096 as libc::c_int;
        (*J).lexbuf.text = js_malloc(J, (*J).lexbuf.cap) as *mut libc::c_char;
    }
    (*J).lexbuf.len = 0 as libc::c_int;
}
unsafe extern "C" fn textpush(J: *mut js_State, mut c: Rune) {
    let mut n: libc::c_int = 0;
    if c == -(1 as libc::c_int) {
        n = 1 as libc::c_int;
    } else {
        n = jsU_runelen(c);
    }
    if (*J).lexbuf.len + n > (*J).lexbuf.cap {
        (*J).lexbuf.cap *= 2 as libc::c_int;
        (*J).lexbuf.text = js_realloc(J, (*J).lexbuf.text as *mut libc::c_void, (*J).lexbuf.cap)
            as *mut libc::c_char;
    }
    if c == -(1 as libc::c_int) {
        let fresh29 = (*J).lexbuf.len;
        (*J).lexbuf.len += 1;
        *((*J).lexbuf.text).offset(fresh29 as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        (*J).lexbuf.len +=
            jsU_runetochar(((*J).lexbuf.text).offset((*J).lexbuf.len as isize), &mut c);
    };
}
unsafe extern "C" fn textend(J: *mut js_State) -> *mut libc::c_char {
    textpush(J, -(1 as libc::c_int));
    (*J).lexbuf.text
}
unsafe extern "C" fn lexlinecomment(J: *mut js_State) {
    while (*J).lexchar != -(1 as libc::c_int) && (*J).lexchar != '\n' as i32 {
        jsY_next(J);
    }
}
unsafe extern "C" fn lexcomment(J: *mut js_State) -> libc::c_int {
    while (*J).lexchar != -(1 as libc::c_int) {
        if if (*J).lexchar == '*' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            while (*J).lexchar == '*' as i32 {
                jsY_next(J);
            }
            if if (*J).lexchar == '/' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                return 0 as libc::c_int;
            }
        } else {
            jsY_next(J);
        }
    }
    -(1 as libc::c_int)
}
unsafe extern "C" fn lexhex(J: *mut js_State) -> libc::c_double {
    let mut n: libc::c_double = 0 as libc::c_int as libc::c_double;
    if jsY_ishex((*J).lexchar) == 0 {
        jsY_error(
            J,
            b"malformed hexadecimal number\0" as *const u8 as *const libc::c_char,
        );
    }
    while jsY_ishex((*J).lexchar) != 0 {
        n = n * 16 as libc::c_int as libc::c_double + jsY_tohex((*J).lexchar) as libc::c_double;
        jsY_next(J);
    }
    n
}
unsafe extern "C" fn lexnumber(J: *mut js_State) -> libc::c_int {
    let s: *const libc::c_char = ((*J).source).offset(-(1 as libc::c_int as isize));
    if if (*J).lexchar == '0' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if (if (*J).lexchar == 'x' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
            || (if (*J).lexchar == 'X' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
        {
            (*J).number = lexhex(J);
            return TK_NUMBER as libc::c_int;
        }
        if jsY_isdec((*J).lexchar) != 0 {
            jsY_error(
                J,
                b"number with leading zero\0" as *const u8 as *const libc::c_char,
            );
        }
        if if (*J).lexchar == '.' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            while jsY_isdec((*J).lexchar) != 0 {
                jsY_next(J);
            }
        }
    } else if if (*J).lexchar == '.' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if jsY_isdec((*J).lexchar) == 0 {
            return '.' as i32;
        }
        while jsY_isdec((*J).lexchar) != 0 {
            jsY_next(J);
        }
    } else {
        while jsY_isdec((*J).lexchar) != 0 {
            jsY_next(J);
        }
        if if (*J).lexchar == '.' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            while jsY_isdec((*J).lexchar) != 0 {
                jsY_next(J);
            }
        }
    }
    if (if (*J).lexchar == 'e' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) != 0
        || (if (*J).lexchar == 'E' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
    {
        if (*J).lexchar == '-' as i32 || (*J).lexchar == '+' as i32 {
            jsY_next(J);
        }
        if jsY_isdec((*J).lexchar) != 0 {
            while jsY_isdec((*J).lexchar) != 0 {
                jsY_next(J);
            }
        } else {
            jsY_error(J, b"missing exponent\0" as *const u8 as *const libc::c_char);
        }
    }
    if jsY_isidentifierstart((*J).lexchar) != 0 {
        jsY_error(
            J,
            b"number with letter suffix\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).number = js_strtod(s, std::ptr::null_mut::<*mut libc::c_char>());
    TK_NUMBER as libc::c_int
}
unsafe extern "C" fn lexescape(J: *mut js_State) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    if if (*J).lexchar == '\n' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return 0 as libc::c_int;
    }
    match (*J).lexchar {
        -1 => {
            jsY_error(
                J,
                b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
            );
        }
        117 => {
            jsY_next(J);
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar) << 12 as libc::c_int;
                jsY_next(J);
            }
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar) << 8 as libc::c_int;
                jsY_next(J);
            }
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar) << 4 as libc::c_int;
                jsY_next(J);
            }
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar);
                jsY_next(J);
            }
            textpush(J, x);
        }
        120 => {
            jsY_next(J);
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar) << 4 as libc::c_int;
                jsY_next(J);
            }
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar);
                jsY_next(J);
            }
            textpush(J, x);
        }
        48 => {
            textpush(J, 0 as libc::c_int);
            jsY_next(J);
        }
        92 => {
            textpush(J, '\\' as i32);
            jsY_next(J);
        }
        39 => {
            textpush(J, '\'' as i32);
            jsY_next(J);
        }
        34 => {
            textpush(J, '"' as i32);
            jsY_next(J);
        }
        98 => {
            textpush(J, '\u{8}' as i32);
            jsY_next(J);
        }
        102 => {
            textpush(J, '\u{c}' as i32);
            jsY_next(J);
        }
        110 => {
            textpush(J, '\n' as i32);
            jsY_next(J);
        }
        114 => {
            textpush(J, '\r' as i32);
            jsY_next(J);
        }
        116 => {
            textpush(J, '\t' as i32);
            jsY_next(J);
        }
        118 => {
            textpush(J, '\u{b}' as i32);
            jsY_next(J);
        }
        _ => {
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    0 as libc::c_int
}
unsafe extern "C" fn lexstring(J: *mut js_State) -> libc::c_int {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let q: libc::c_int = (*J).lexchar;
    jsY_next(J);
    textinit(J);
    while (*J).lexchar != q {
        if (*J).lexchar == -(1 as libc::c_int) || (*J).lexchar == '\n' as i32 {
            jsY_error(
                J,
                b"string not terminated\0" as *const u8 as *const libc::c_char,
            );
        }
        if if (*J).lexchar == '\\' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            if lexescape(J) != 0 {
                jsY_error(
                    J,
                    b"malformed escape sequence\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    if if (*J).lexchar == q {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsY_error(J, b"expected '%c'\0" as *const u8 as *const libc::c_char, q);
    }
    s = textend(J);
    (*J).text = js_intern(J, s);
    TK_STRING as libc::c_int
}
unsafe extern "C" fn isregexpcontext(last: libc::c_int) -> libc::c_int {
    match last {
        93 | 41 | 125 | 256 | 257 | 258 | 293 | 301 | 304 | 306 => 0 as libc::c_int,
        _ => 1 as libc::c_int,
    }
}
unsafe extern "C" fn lexregexp(J: *mut js_State) -> libc::c_int {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut g: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut inclass: libc::c_int = 0 as libc::c_int;
    textinit(J);
    while (*J).lexchar != '/' as i32 || inclass != 0 {
        if (*J).lexchar == -(1 as libc::c_int) || (*J).lexchar == '\n' as i32 {
            jsY_error(
                J,
                b"regular expression not terminated\0" as *const u8 as *const libc::c_char,
            );
        } else if if (*J).lexchar == '\\' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            if if (*J).lexchar == '/' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                textpush(J, '/' as i32);
            } else {
                textpush(J, '\\' as i32);
                if (*J).lexchar == -(1 as libc::c_int) || (*J).lexchar == '\n' as i32 {
                    jsY_error(
                        J,
                        b"regular expression not terminated\0" as *const u8 as *const libc::c_char,
                    );
                }
                textpush(J, (*J).lexchar);
                jsY_next(J);
            }
        } else {
            if (*J).lexchar == '[' as i32 && inclass == 0 {
                inclass = 1 as libc::c_int;
            }
            if (*J).lexchar == ']' as i32 && inclass != 0 {
                inclass = 0 as libc::c_int;
            }
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    if if (*J).lexchar == '/' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsY_error(
            J,
            b"expected '%c'\0" as *const u8 as *const libc::c_char,
            '/' as i32,
        );
    }
    s = textend(J);
    m = 0 as libc::c_int;
    i = m;
    g = i;
    while jsY_isidentifierpart((*J).lexchar) != 0 {
        if if (*J).lexchar == 'g' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            g += 1;
            g;
        } else if if (*J).lexchar == 'i' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            i += 1;
            i;
        } else if if (*J).lexchar == 'm' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            m += 1;
            m;
        } else {
            jsY_error(
                J,
                b"illegal flag in regular expression: %c\0" as *const u8 as *const libc::c_char,
                (*J).lexchar,
            );
        }
    }
    if g > 1 as libc::c_int || i > 1 as libc::c_int || m > 1 as libc::c_int {
        jsY_error(
            J,
            b"duplicated flag in regular expression\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).text = js_intern(J, s);
    (*J).number = 0 as libc::c_int as libc::c_double;
    if g != 0 {
        (*J).number += JS_REGEXP_G as libc::c_int as libc::c_double;
    }
    if i != 0 {
        (*J).number += JS_REGEXP_I as libc::c_int as libc::c_double;
    }
    if m != 0 {
        (*J).number += JS_REGEXP_M as libc::c_int as libc::c_double;
    }
    TK_REGEXP as libc::c_int
}
unsafe extern "C" fn isnlthcontext(last: libc::c_int) -> libc::c_int {
    match last {
        284 | 287 | 302 | 305 => 1 as libc::c_int,
        _ => 0 as libc::c_int,
    }
}
unsafe extern "C" fn jsY_lexx(J: *mut js_State) -> libc::c_int {
    (*J).newline = 0 as libc::c_int;
    loop {
        (*J).lexline = (*J).line;
        while jsY_iswhite((*J).lexchar) != 0 {
            jsY_next(J);
        }
        if if (*J).lexchar == '\n' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            (*J).newline = 1 as libc::c_int;
            if isnlthcontext((*J).lasttoken) != 0 {
                return ';' as i32;
            }
        } else if if (*J).lexchar == '/' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            if if (*J).lexchar == '/' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                lexlinecomment(J);
            } else if if (*J).lexchar == '*' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                if lexcomment(J) != 0 {
                    jsY_error(
                        J,
                        b"multi-line comment not terminated\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if isregexpcontext((*J).lasttoken) != 0 {
                return lexregexp(J);
            } else if if (*J).lexchar == '=' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                return TK_DIV_ASS as libc::c_int;
            } else {
                return '/' as i32;
            }
        } else {
            if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
                return lexnumber(J);
            }
            match (*J).lexchar {
                40 => {
                    jsY_next(J);
                    return '(' as i32;
                }
                41 => {
                    jsY_next(J);
                    return ')' as i32;
                }
                44 => {
                    jsY_next(J);
                    return ',' as i32;
                }
                58 => {
                    jsY_next(J);
                    return ':' as i32;
                }
                59 => {
                    jsY_next(J);
                    return ';' as i32;
                }
                63 => {
                    jsY_next(J);
                    return '?' as i32;
                }
                91 => {
                    jsY_next(J);
                    return '[' as i32;
                }
                93 => {
                    jsY_next(J);
                    return ']' as i32;
                }
                123 => {
                    jsY_next(J);
                    return '{' as i32;
                }
                125 => {
                    jsY_next(J);
                    return '}' as i32;
                }
                126 => {
                    jsY_next(J);
                    return '~' as i32;
                }
                39 | 34 => return lexstring(J),
                46 => return lexnumber(J),
                60 => {
                    jsY_next(J);
                    if if (*J).lexchar == '<' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } != 0
                        {
                            return TK_SHL_ASS as libc::c_int;
                        }
                        return TK_SHL as libc::c_int;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_LE as libc::c_int;
                    }
                    return '<' as i32;
                }
                62 => {
                    jsY_next(J);
                    if if (*J).lexchar == '>' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        if if (*J).lexchar == '>' as i32 {
                            jsY_next(J);
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } != 0
                        {
                            if if (*J).lexchar == '=' as i32 {
                                jsY_next(J);
                                1 as libc::c_int
                            } else {
                                0 as libc::c_int
                            } != 0
                            {
                                return TK_USHR_ASS as libc::c_int;
                            }
                            return TK_USHR as libc::c_int;
                        }
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } != 0
                        {
                            return TK_SHR_ASS as libc::c_int;
                        }
                        return TK_SHR as libc::c_int;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_GE as libc::c_int;
                    }
                    return '>' as i32;
                }
                61 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } != 0
                        {
                            return TK_STRICTEQ as libc::c_int;
                        }
                        return TK_EQ as libc::c_int;
                    }
                    return '=' as i32;
                }
                33 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1 as libc::c_int
                        } else {
                            0 as libc::c_int
                        } != 0
                        {
                            return TK_STRICTNE as libc::c_int;
                        }
                        return TK_NE as libc::c_int;
                    }
                    return '!' as i32;
                }
                43 => {
                    jsY_next(J);
                    if if (*J).lexchar == '+' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_INC as libc::c_int;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_ADD_ASS as libc::c_int;
                    }
                    return '+' as i32;
                }
                45 => {
                    jsY_next(J);
                    if if (*J).lexchar == '-' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_DEC as libc::c_int;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_SUB_ASS as libc::c_int;
                    }
                    return '-' as i32;
                }
                42 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_MUL_ASS as libc::c_int;
                    }
                    return '*' as i32;
                }
                37 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_MOD_ASS as libc::c_int;
                    }
                    return '%' as i32;
                }
                38 => {
                    jsY_next(J);
                    if if (*J).lexchar == '&' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_AND as libc::c_int;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_AND_ASS as libc::c_int;
                    }
                    return '&' as i32;
                }
                124 => {
                    jsY_next(J);
                    if if (*J).lexchar == '|' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_OR as libc::c_int;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_OR_ASS as libc::c_int;
                    }
                    return '|' as i32;
                }
                94 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {
                        return TK_XOR_ASS as libc::c_int;
                    }
                    return '^' as i32;
                }
                -1 => return 0 as libc::c_int,
                _ => {}
            }
            jsY_unescape(J);
            if jsY_isidentifierstart((*J).lexchar) != 0 {
                textinit(J);
                textpush(J, (*J).lexchar);
                jsY_next(J);
                jsY_unescape(J);
                while jsY_isidentifierpart((*J).lexchar) != 0 {
                    textpush(J, (*J).lexchar);
                    jsY_next(J);
                    jsY_unescape(J);
                }
                textend(J);
                return jsY_findkeyword(J, (*J).lexbuf.text);
            }
            if (*J).lexchar >= 0x20 as libc::c_int && (*J).lexchar <= 0x7e as libc::c_int {
                jsY_error(
                    J,
                    b"unexpected character: '%c'\0" as *const u8 as *const libc::c_char,
                    (*J).lexchar,
                );
            }
            jsY_error(
                J,
                b"unexpected character: \\u%04X\0" as *const u8 as *const libc::c_char,
                (*J).lexchar,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsY_initlex(
    J: *mut js_State,
    filename: *const libc::c_char,
    source: *const libc::c_char,
) {
    (*J).filename = filename;
    (*J).source = source;
    (*J).line = 1 as libc::c_int;
    (*J).lasttoken = 0 as libc::c_int;
    jsY_next(J);
}
#[no_mangle]
pub unsafe extern "C" fn jsY_lex(J: *mut js_State) -> libc::c_int {
    (*J).lasttoken = jsY_lexx(J);
    (*J).lasttoken
}
unsafe extern "C" fn lexjsonnumber(J: *mut js_State) -> libc::c_int {
    let s: *const libc::c_char = ((*J).source).offset(-(1 as libc::c_int as isize));
    if (*J).lexchar == '-' as i32 {
        jsY_next(J);
    }
    if (*J).lexchar == '0' as i32 {
        jsY_next(J);
    } else if (*J).lexchar >= '1' as i32 && (*J).lexchar <= '9' as i32 {
        while (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
            jsY_next(J);
        }
    } else {
        jsY_error(
            J,
            b"unexpected non-digit\0" as *const u8 as *const libc::c_char,
        );
    }
    if if (*J).lexchar == '.' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
            while (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
                jsY_next(J);
            }
        } else {
            jsY_error(
                J,
                b"missing digits after decimal point\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (if (*J).lexchar == 'e' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) != 0
        || (if (*J).lexchar == 'E' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
    {
        if (*J).lexchar == '-' as i32 || (*J).lexchar == '+' as i32 {
            jsY_next(J);
        }
        if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
            while (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
                jsY_next(J);
            }
        } else {
            jsY_error(
                J,
                b"missing digits after exponent indicator\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*J).number = js_strtod(s, std::ptr::null_mut::<*mut libc::c_char>());
    TK_NUMBER as libc::c_int
}
unsafe extern "C" fn lexjsonescape(J: *mut js_State) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    match (*J).lexchar {
        117 => {
            jsY_next(J);
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar) << 12 as libc::c_int;
                jsY_next(J);
            }
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar) << 8 as libc::c_int;
                jsY_next(J);
            }
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar) << 4 as libc::c_int;
                jsY_next(J);
            }
            if jsY_ishex((*J).lexchar) == 0 {
                return 1 as libc::c_int;
            } else {
                x |= jsY_tohex((*J).lexchar);
                jsY_next(J);
            }
            textpush(J, x);
        }
        34 => {
            textpush(J, '"' as i32);
            jsY_next(J);
        }
        92 => {
            textpush(J, '\\' as i32);
            jsY_next(J);
        }
        47 => {
            textpush(J, '/' as i32);
            jsY_next(J);
        }
        98 => {
            textpush(J, '\u{8}' as i32);
            jsY_next(J);
        }
        102 => {
            textpush(J, '\u{c}' as i32);
            jsY_next(J);
        }
        110 => {
            textpush(J, '\n' as i32);
            jsY_next(J);
        }
        114 => {
            textpush(J, '\r' as i32);
            jsY_next(J);
        }
        116 => {
            textpush(J, '\t' as i32);
            jsY_next(J);
        }
        _ => {
            jsY_error(
                J,
                b"invalid escape sequence\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    0 as libc::c_int
}
unsafe extern "C" fn lexjsonstring(J: *mut js_State) -> libc::c_int {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    textinit(J);
    while (*J).lexchar != '"' as i32 {
        if (*J).lexchar == -(1 as libc::c_int) {
            jsY_error(
                J,
                b"unterminated string\0" as *const u8 as *const libc::c_char,
            );
        } else if (*J).lexchar < 32 as libc::c_int {
            jsY_error(
                J,
                b"invalid control character in string\0" as *const u8 as *const libc::c_char,
            );
        } else if if (*J).lexchar == '\\' as i32 {
            jsY_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            lexjsonescape(J);
        } else {
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    if if (*J).lexchar == '"' as i32 {
        jsY_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsY_error(
            J,
            b"expected '%c'\0" as *const u8 as *const libc::c_char,
            '"' as i32,
        );
    }
    s = textend(J);
    (*J).text = js_intern(J, s);
    TK_STRING as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsY_lexjson(J: *mut js_State) -> libc::c_int {
    (*J).lexline = (*J).line;
    while jsY_iswhite((*J).lexchar) != 0 || (*J).lexchar == '\n' as i32 {
        jsY_next(J);
    }
    if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 || (*J).lexchar == '-' as i32 {
        return lexjsonnumber(J);
    }
    match (*J).lexchar {
        44 => {
            jsY_next(J);
            return ',' as i32;
        }
        58 => {
            jsY_next(J);
            return ':' as i32;
        }
        91 => {
            jsY_next(J);
            return '[' as i32;
        }
        93 => {
            jsY_next(J);
            return ']' as i32;
        }
        123 => {
            jsY_next(J);
            return '{' as i32;
        }
        125 => {
            jsY_next(J);
            return '}' as i32;
        }
        34 => {
            jsY_next(J);
            return lexjsonstring(J);
        }
        102 => {
            jsY_next(J);
            if if (*J).lexchar == 'a' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'a' as i32,
                );
            }
            if if (*J).lexchar == 'l' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'l' as i32,
                );
            }
            if if (*J).lexchar == 's' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    's' as i32,
                );
            }
            if if (*J).lexchar == 'e' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'e' as i32,
                );
            }
            return TK_FALSE as libc::c_int;
        }
        110 => {
            jsY_next(J);
            if if (*J).lexchar == 'u' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'u' as i32,
                );
            }
            if if (*J).lexchar == 'l' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'l' as i32,
                );
            }
            if if (*J).lexchar == 'l' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'l' as i32,
                );
            }
            return TK_NULL as libc::c_int;
        }
        116 => {
            jsY_next(J);
            if if (*J).lexchar == 'r' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'r' as i32,
                );
            }
            if if (*J).lexchar == 'u' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'u' as i32,
                );
            }
            if if (*J).lexchar == 'e' as i32 {
                jsY_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'e' as i32,
                );
            }
            return TK_TRUE as libc::c_int;
        }
        -1 => return 0 as libc::c_int,
        _ => {}
    }
    if (*J).lexchar >= 0x20 as libc::c_int && (*J).lexchar <= 0x7e as libc::c_int {
        jsY_error(
            J,
            b"unexpected character: '%c'\0" as *const u8 as *const libc::c_char,
            (*J).lexchar,
        );
    }
    jsY_error(
        J,
        b"unexpected character: \\u%04X\0" as *const u8 as *const libc::c_char,
        (*J).lexchar,
    );
}
unsafe extern "C" fn jsM_round(x: libc::c_double) -> libc::c_double {
    if x.is_nan() as i32 != 0 {
        return x;
    }
    if if x.is_infinite() {
        if x.is_sign_positive() {
            1
        } else {
            -1
        }
    } else {
        0
    } != 0
    {
        return x;
    }
    if x == 0 as libc::c_int as libc::c_double {
        return x;
    }
    if x > 0 as libc::c_int as libc::c_double && x < 0.5f64 {
        return 0 as libc::c_int as libc::c_double;
    }
    if x < 0 as libc::c_int as libc::c_double && x >= -0.5f64 {
        return -(0 as libc::c_int) as libc::c_double;
    }
    floor(x + 0.5f64)
}
unsafe extern "C" fn Math_abs(J: *mut js_State) {
    js_pushnumber(J, fabs(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_acos(J: *mut js_State) {
    js_pushnumber(J, acos(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_asin(J: *mut js_State) {
    js_pushnumber(J, asin(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_atan(J: *mut js_State) {
    js_pushnumber(J, atan(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_atan2(J: *mut js_State) {
    let y: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let x: libc::c_double = js_tonumber(J, 2 as libc::c_int);
    js_pushnumber(J, atan2(y, x));
}
unsafe extern "C" fn Math_ceil(J: *mut js_State) {
    js_pushnumber(J, ceil(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_cos(J: *mut js_State) {
    js_pushnumber(J, cos(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_exp(J: *mut js_State) {
    js_pushnumber(J, exp(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_floor(J: *mut js_State) {
    js_pushnumber(J, floor(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_log(J: *mut js_State) {
    js_pushnumber(J, log(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_pow(J: *mut js_State) {
    let x: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    let y: libc::c_double = js_tonumber(J, 2 as libc::c_int);
    if y.is_finite() as i32 == 0 && fabs(x) == 1 as libc::c_int as libc::c_double {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    } else {
        js_pushnumber(J, pow(x, y));
    };
}
unsafe extern "C" fn Math_random(J: *mut js_State) {
    (*J).seed = ((*J).seed as uint64_t)
        .wrapping_mul(48271 as libc::c_int as libc::c_ulong)
        .wrapping_rem(0x7fffffff as libc::c_int as libc::c_ulong) as libc::c_uint;
    js_pushnumber(
        J,
        (*J).seed as libc::c_double / 0x7fffffff as libc::c_int as libc::c_double,
    );
}
unsafe extern "C" fn Math_init_random(J: *mut js_State) {
    (*J).seed =
        (time(std::ptr::null_mut::<time_t>()) + 123 as libc::c_int as libc::c_long) as libc::c_uint;
    (*J).seed ^= (*J).seed << 13 as libc::c_int;
    (*J).seed ^= (*J).seed >> 17 as libc::c_int;
    (*J).seed ^= (*J).seed << 5 as libc::c_int;
    (*J).seed = ((*J).seed).wrapping_rem(0x7fffffff as libc::c_int as libc::c_uint);
}
unsafe extern "C" fn Math_round(J: *mut js_State) {
    let x: libc::c_double = js_tonumber(J, 1 as libc::c_int);
    js_pushnumber(J, jsM_round(x));
}
unsafe extern "C" fn Math_sin(J: *mut js_State) {
    js_pushnumber(J, sin(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_sqrt(J: *mut js_State) {
    js_pushnumber(J, sqrt(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_tan(J: *mut js_State) {
    js_pushnumber(J, tan(js_tonumber(J, 1 as libc::c_int)));
}
unsafe extern "C" fn Math_max(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let n: libc::c_int = js_gettop(J);
    let mut x: libc::c_double = -::core::f32::INFINITY as libc::c_double;
    i = 1 as libc::c_int;
    while i < n {
        let y: libc::c_double = js_tonumber(J, i);
        if y.is_nan() as i32 != 0 {
            x = y;
            break;
        } else {
            if x.is_sign_negative() as libc::c_int == y.is_sign_negative() as libc::c_int {
                x = if x > y { x } else { y };
            } else if x.is_sign_negative() as libc::c_int != 0 {
                x = y;
            }
            i += 1;
            i;
        }
    }
    js_pushnumber(J, x);
}
unsafe extern "C" fn Math_min(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let n: libc::c_int = js_gettop(J);
    let mut x: libc::c_double = ::core::f32::INFINITY as libc::c_double;
    i = 1 as libc::c_int;
    while i < n {
        let y: libc::c_double = js_tonumber(J, i);
        if y.is_nan() as i32 != 0 {
            x = y;
            break;
        } else {
            if x.is_sign_negative() as libc::c_int == y.is_sign_negative() as libc::c_int {
                x = if x < y { x } else { y };
            } else if y.is_sign_negative() as libc::c_int != 0 {
                x = y;
            }
            i += 1;
            i;
        }
    }
    js_pushnumber(J, x);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initmath(J: *mut js_State) {
    Math_init_random(J);
    js_pushobject(J, jsV_newobject(J, JS_CMATH, (*J).Object_prototype));
    jsB_propn(
        J,
        b"E\0" as *const u8 as *const libc::c_char,
        2.718_281_828_459_045_f64,
    );
    jsB_propn(
        J,
        b"LN10\0" as *const u8 as *const libc::c_char,
        2.302585092994046f64,
    );
    jsB_propn(
        J,
        b"LN2\0" as *const u8 as *const libc::c_char,
        0.6931471805599453f64,
    );
    jsB_propn(
        J,
        b"LOG2E\0" as *const u8 as *const libc::c_char,
        1.4426950408889634f64,
    );
    jsB_propn(
        J,
        b"LOG10E\0" as *const u8 as *const libc::c_char,
        0.4342944819032518f64,
    );
    jsB_propn(
        J,
        b"PI\0" as *const u8 as *const libc::c_char,
        3.141_592_653_589_793_f64,
    );
    jsB_propn(
        J,
        b"SQRT1_2\0" as *const u8 as *const libc::c_char,
        0.7071067811865476f64,
    );
    jsB_propn(
        J,
        b"SQRT2\0" as *const u8 as *const libc::c_char,
        1.4142135623730951f64,
    );
    jsB_propf(
        J,
        b"Math.abs\0" as *const u8 as *const libc::c_char,
        Some(Math_abs as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.acos\0" as *const u8 as *const libc::c_char,
        Some(Math_acos as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.asin\0" as *const u8 as *const libc::c_char,
        Some(Math_asin as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.atan\0" as *const u8 as *const libc::c_char,
        Some(Math_atan as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.atan2\0" as *const u8 as *const libc::c_char,
        Some(Math_atan2 as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.ceil\0" as *const u8 as *const libc::c_char,
        Some(Math_ceil as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.cos\0" as *const u8 as *const libc::c_char,
        Some(Math_cos as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.exp\0" as *const u8 as *const libc::c_char,
        Some(Math_exp as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.floor\0" as *const u8 as *const libc::c_char,
        Some(Math_floor as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.log\0" as *const u8 as *const libc::c_char,
        Some(Math_log as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.max\0" as *const u8 as *const libc::c_char,
        Some(Math_max as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.min\0" as *const u8 as *const libc::c_char,
        Some(Math_min as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.pow\0" as *const u8 as *const libc::c_char,
        Some(Math_pow as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.random\0" as *const u8 as *const libc::c_char,
        Some(Math_random as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.round\0" as *const u8 as *const libc::c_char,
        Some(Math_round as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.sin\0" as *const u8 as *const libc::c_char,
        Some(Math_sin as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.sqrt\0" as *const u8 as *const libc::c_char,
        Some(Math_sqrt as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Math.tan\0" as *const u8 as *const libc::c_char,
        Some(Math_tan as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"Math\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn jsB_new_Number(J: *mut js_State) {
    js_newnumber(
        J,
        if js_gettop(J) > 1 as libc::c_int {
            js_tonumber(J, 1 as libc::c_int)
        } else {
            0 as libc::c_int as libc::c_double
        },
    );
}
unsafe extern "C" fn jsB_Number(J: *mut js_State) {
    js_pushnumber(
        J,
        if js_gettop(J) > 1 as libc::c_int {
            js_tonumber(J, 1 as libc::c_int)
        } else {
            0 as libc::c_int as libc::c_double
        },
    );
}
unsafe extern "C" fn Np_valueOf(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    js_pushnumber(J, (*self_0).u.number);
}
unsafe extern "C" fn Np_toString(J: *mut js_State) {
    let mut buf: [libc::c_char; 100] = [0; 100];
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    let radix: libc::c_int = if js_isundefined(J, 1 as libc::c_int) != 0 {
        10 as libc::c_int
    } else {
        js_tointeger(J, 1 as libc::c_int)
    };
    let mut x: libc::c_double = 0 as libc::c_int as libc::c_double;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    x = (*self_0).u.number;
    if radix == 10 as libc::c_int {
        js_pushstring(J, jsV_numbertostring(J, buf.as_mut_ptr(), x));
        return;
    }
    if radix < 2 as libc::c_int || radix > 36 as libc::c_int {
        js_rangeerror(J, b"invalid radix\0" as *const u8 as *const libc::c_char);
    }
    static mut digits: [libc::c_char; 37] = unsafe {
        *::core::mem::transmute::<&[u8; 37], &[libc::c_char; 37]>(
            b"0123456789abcdefghijklmnopqrstuvwxyz\0",
        )
    };
    let mut number: libc::c_double = x;
    let sign: libc::c_int = (x < 0 as libc::c_int as libc::c_double) as libc::c_int;
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut u: uint64_t = 0;
    let limit: uint64_t = (1 as libc::c_int as uint64_t) << 52 as libc::c_int;
    let mut ndigits: libc::c_int = 0;
    let mut exp_0: libc::c_int = 0;
    let mut point: libc::c_int = 0;
    if number == 0 as libc::c_int as libc::c_double {
        js_pushstring(J, b"0\0" as *const u8 as *const libc::c_char);
        return;
    }
    if number.is_nan() as i32 != 0 {
        js_pushstring(J, b"NaN\0" as *const u8 as *const libc::c_char);
        return;
    }
    if if number.is_infinite() {
        if number.is_sign_positive() {
            1
        } else {
            -1
        }
    } else {
        0
    } != 0
    {
        js_pushstring(
            J,
            if sign != 0 {
                b"-Infinity\0" as *const u8 as *const libc::c_char
            } else {
                b"Infinity\0" as *const u8 as *const libc::c_char
            },
        );
        return;
    }
    if sign != 0 {
        number = -number;
    }
    exp_0 = 0 as libc::c_int;
    while number * pow(radix as libc::c_double, exp_0 as libc::c_double) > limit as libc::c_double {
        exp_0 -= 1;
        exp_0;
    }
    while number
        * pow(
            radix as libc::c_double,
            (exp_0 + 1 as libc::c_int) as libc::c_double,
        )
        < limit as libc::c_double
    {
        exp_0 += 1;
        exp_0;
    }
    u = (number * pow(radix as libc::c_double, exp_0 as libc::c_double) + 0.5f64) as uint64_t;
    while u > 0 as libc::c_int as libc::c_ulong
        && u.wrapping_rem(radix as libc::c_ulong) == 0 as libc::c_int as libc::c_ulong
    {
        u = (u as libc::c_ulong).wrapping_div(radix as libc::c_ulong) as uint64_t as uint64_t;
        exp_0 -= 1;
        exp_0;
    }
    ndigits = 0 as libc::c_int;
    while u > 0 as libc::c_int as libc::c_ulong {
        let fresh30 = ndigits;
        ndigits += 1;
        buf[fresh30 as usize] = digits[u.wrapping_rem(radix as libc::c_ulong) as usize];
        u = (u as libc::c_ulong).wrapping_div(radix as libc::c_ulong) as uint64_t as uint64_t;
    }
    point = ndigits - exp_0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    if sign != 0 {
        js_putc(J, &mut sb, '-' as i32);
    }
    if point <= 0 as libc::c_int {
        js_putc(J, &mut sb, '0' as i32);
        js_putc(J, &mut sb, '.' as i32);
        loop {
            let fresh31 = point;
            point += 1;
            if fresh31 >= 0 as libc::c_int {
                break;
            }
            js_putc(J, &mut sb, '0' as i32);
        }
        loop {
            let fresh32 = ndigits;
            ndigits -= 1;
            if fresh32 <= 0 as libc::c_int {
                break;
            }
            js_putc(J, &mut sb, buf[ndigits as usize] as libc::c_int);
        }
    } else {
        loop {
            let fresh33 = ndigits;
            ndigits -= 1;
            if fresh33 <= 0 as libc::c_int {
                break;
            }
            js_putc(J, &mut sb, buf[ndigits as usize] as libc::c_int);
            point -= 1;
            if point == 0 as libc::c_int && ndigits > 0 as libc::c_int {
                js_putc(J, &mut sb, '.' as i32);
            }
        }
        loop {
            let fresh34 = point;
            point -= 1;
            if fresh34 <= 0 as libc::c_int {
                break;
            }
            js_putc(J, &mut sb, '0' as i32);
        }
    }
    js_putc(J, &mut sb, 0 as libc::c_int);
    js_pushstring(J, ((*sb).s).as_mut_ptr() as *const libc::c_char);
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
unsafe extern "C" fn numtostr(
    J: *mut js_State,
    fmt: *const libc::c_char,
    w: libc::c_int,
    n: libc::c_double,
) {
    let mut buf: [libc::c_char; 50] = [0; 50];
    let mut e: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    sprintf(buf.as_mut_ptr(), fmt, w, n);
    e = strchr(buf.as_mut_ptr(), 'e' as i32);
    if !e.is_null() {
        let exp_0: libc::c_int = atoi(e.offset(1 as libc::c_int as isize));
        sprintf(e, b"e%+d\0" as *const u8 as *const libc::c_char, exp_0);
    }
    js_pushstring(J, buf.as_mut_ptr());
}
unsafe extern "C" fn Np_toFixed(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    let width: libc::c_int = js_tointeger(J, 1 as libc::c_int);
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut x: libc::c_double = 0.;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    if width < 0 as libc::c_int {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    if width > 20 as libc::c_int {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    x = (*self_0).u.number;
    if x.is_nan() as i32 != 0
        || if x.is_infinite() {
            if x.is_sign_positive() {
                1
            } else {
                -1
            }
        } else {
            0
        } != 0
        || x <= -1e21f64
        || x >= 1e21f64
    {
        js_pushstring(J, jsV_numbertostring(J, buf.as_mut_ptr(), x));
    } else {
        numtostr(J, b"%.*f\0" as *const u8 as *const libc::c_char, width, x);
    };
}
unsafe extern "C" fn Np_toExponential(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    let width: libc::c_int = js_tointeger(J, 1 as libc::c_int);
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut x: libc::c_double = 0.;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    if width < 0 as libc::c_int {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    if width > 20 as libc::c_int {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    x = (*self_0).u.number;
    if x.is_nan() as i32 != 0
        || if x.is_infinite() {
            if x.is_sign_positive() {
                1
            } else {
                -1
            }
        } else {
            0
        } != 0
    {
        js_pushstring(J, jsV_numbertostring(J, buf.as_mut_ptr(), x));
    } else {
        numtostr(J, b"%.*e\0" as *const u8 as *const libc::c_char, width, x);
    };
}
unsafe extern "C" fn Np_toPrecision(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    let width: libc::c_int = js_tointeger(J, 1 as libc::c_int);
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut x: libc::c_double = 0.;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    if width < 1 as libc::c_int {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    if width > 21 as libc::c_int {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    x = (*self_0).u.number;
    if x.is_nan() as i32 != 0
        || if x.is_infinite() {
            if x.is_sign_positive() {
                1
            } else {
                -1
            }
        } else {
            0
        } != 0
    {
        js_pushstring(J, jsV_numbertostring(J, buf.as_mut_ptr(), x));
    } else {
        numtostr(J, b"%.*g\0" as *const u8 as *const libc::c_char, width, x);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initnumber(J: *mut js_State) {
    (*(*J).Number_prototype).u.number = 0 as libc::c_int as libc::c_double;
    js_pushobject(J, (*J).Number_prototype);
    jsB_propf(
        J,
        b"Number.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Np_valueOf as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Number.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Np_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Number.prototype.toLocaleString\0" as *const u8 as *const libc::c_char,
        Some(Np_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Number.prototype.toFixed\0" as *const u8 as *const libc::c_char,
        Some(Np_toFixed as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Number.prototype.toExponential\0" as *const u8 as *const libc::c_char,
        Some(Np_toExponential as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Number.prototype.toPrecision\0" as *const u8 as *const libc::c_char,
        Some(Np_toPrecision as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_Number as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_new_Number as unsafe extern "C" fn(*mut js_State) -> ()),
        b"Number\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    jsB_propn(
        J,
        b"MAX_VALUE\0" as *const u8 as *const libc::c_char,
        1.797_693_134_862_315_7e308_f64,
    );
    jsB_propn(
        J,
        b"MIN_VALUE\0" as *const u8 as *const libc::c_char,
        5e-324f64,
    );
    jsB_propn(
        J,
        b"NaN\0" as *const u8 as *const libc::c_char,
        ::core::f32::NAN as libc::c_double,
    );
    jsB_propn(
        J,
        b"NEGATIVE_INFINITY\0" as *const u8 as *const libc::c_char,
        -::core::f32::INFINITY as libc::c_double,
    );
    jsB_propn(
        J,
        b"POSITIVE_INFINITY\0" as *const u8 as *const libc::c_char,
        ::core::f32::INFINITY as libc::c_double,
    );
    js_defglobal(
        J,
        b"Number\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn jsB_new_Object(J: *mut js_State) {
    if js_isundefined(J, 1 as libc::c_int) != 0 || js_isnull(J, 1 as libc::c_int) != 0 {
        js_newobject(J);
    } else {
        js_pushobject(J, js_toobject(J, 1 as libc::c_int));
    };
}
unsafe extern "C" fn jsB_Object(J: *mut js_State) {
    if js_isundefined(J, 1 as libc::c_int) != 0 || js_isnull(J, 1 as libc::c_int) != 0 {
        js_newobject(J);
    } else {
        js_pushobject(J, js_toobject(J, 1 as libc::c_int));
    };
}
unsafe extern "C" fn Op_toString(J: *mut js_State) {
    if js_isundefined(J, 0 as libc::c_int) != 0 {
        js_pushliteral(
            J,
            b"[object Undefined]\0" as *const u8 as *const libc::c_char,
        );
    } else if js_isnull(J, 0 as libc::c_int) != 0 {
        js_pushliteral(J, b"[object Null]\0" as *const u8 as *const libc::c_char);
    } else {
        let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
        match (*self_0).type_0 as libc::c_uint {
            0 => {
                js_pushliteral(J, b"[object Object]\0" as *const u8 as *const libc::c_char);
            }
            1 => {
                js_pushliteral(J, b"[object Array]\0" as *const u8 as *const libc::c_char);
            }
            2 => {
                js_pushliteral(
                    J,
                    b"[object Function]\0" as *const u8 as *const libc::c_char,
                );
            }
            3 => {
                js_pushliteral(
                    J,
                    b"[object Function]\0" as *const u8 as *const libc::c_char,
                );
            }
            4 => {
                js_pushliteral(
                    J,
                    b"[object Function]\0" as *const u8 as *const libc::c_char,
                );
            }
            5 => {
                js_pushliteral(J, b"[object Error]\0" as *const u8 as *const libc::c_char);
            }
            6 => {
                js_pushliteral(J, b"[object Boolean]\0" as *const u8 as *const libc::c_char);
            }
            7 => {
                js_pushliteral(J, b"[object Number]\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                js_pushliteral(J, b"[object String]\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                js_pushliteral(J, b"[object RegExp]\0" as *const u8 as *const libc::c_char);
            }
            10 => {
                js_pushliteral(J, b"[object Date]\0" as *const u8 as *const libc::c_char);
            }
            11 => {
                js_pushliteral(J, b"[object Math]\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                js_pushliteral(J, b"[object JSON]\0" as *const u8 as *const libc::c_char);
            }
            13 => {
                js_pushliteral(
                    J,
                    b"[object Arguments]\0" as *const u8 as *const libc::c_char,
                );
            }
            14 => {
                js_pushliteral(
                    J,
                    b"[object Iterator]\0" as *const u8 as *const libc::c_char,
                );
            }
            15 => {
                js_pushliteral(J, b"[object \0" as *const u8 as *const libc::c_char);
                js_pushliteral(J, (*self_0).u.user.tag);
                js_concat(J);
                js_pushliteral(J, b"]\0" as *const u8 as *const libc::c_char);
                js_concat(J);
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn Op_valueOf(J: *mut js_State) {
    js_copy(J, 0 as libc::c_int);
}
unsafe extern "C" fn Op_hasOwnProperty(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    let name: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    let mut ref_0: *mut js_Property = std::ptr::null_mut::<js_Property>();
    let mut k: libc::c_int = 0;
    if (*self_0).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint
        && js_isarrayindex(J, name, &mut k) != 0
        && k >= 0 as libc::c_int
        && k < (*self_0).u.s.length
    {
        js_pushboolean(J, 1 as libc::c_int);
        return;
    }
    if (*self_0).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*self_0).u.a.simple != 0
        && js_isarrayindex(J, name, &mut k) != 0
        && k >= 0 as libc::c_int
        && k < (*self_0).u.a.flat_length
    {
        js_pushboolean(J, 1 as libc::c_int);
        return;
    }
    ref_0 = jsV_getownproperty(J, self_0, name);
    js_pushboolean(
        J,
        (ref_0 != std::ptr::null_mut::<libc::c_void>() as *mut js_Property) as libc::c_int,
    );
}
unsafe extern "C" fn Op_isPrototypeOf(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    if js_isobject(J, 1 as libc::c_int) != 0 {
        let mut V: *mut js_Object = js_toobject(J, 1 as libc::c_int);
        loop {
            V = (*V).prototype;
            if V == self_0 {
                js_pushboolean(J, 1 as libc::c_int);
                return;
            }
            if V.is_null() {
                break;
            }
        }
    }
    js_pushboolean(J, 0 as libc::c_int);
}
unsafe extern "C" fn Op_propertyIsEnumerable(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    let name: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    let ref_0: *mut js_Property = jsV_getownproperty(J, self_0, name);
    js_pushboolean(
        J,
        (!ref_0.is_null() && (*ref_0).atts & JS_DONTENUM as libc::c_int == 0) as libc::c_int,
    );
}
unsafe extern "C" fn O_getPrototypeOf(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    if !((*obj).prototype).is_null() {
        js_pushobject(J, (*obj).prototype);
    } else {
        js_pushnull(J);
    };
}
unsafe extern "C" fn O_getOwnPropertyDescriptor(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut ref_0: *mut js_Property = std::ptr::null_mut::<js_Property>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    ref_0 = jsV_getproperty(J, obj, js_tostring(J, 2 as libc::c_int));
    if ref_0.is_null() {
        js_pushundefined(J);
    } else {
        js_newobject(J);
        if ((*ref_0).getter).is_null() && ((*ref_0).setter).is_null() {
            js_pushvalue(J, (*ref_0).value);
            js_defproperty(
                J,
                -(2 as libc::c_int),
                b"value\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            js_pushboolean(
                J,
                ((*ref_0).atts & JS_READONLY as libc::c_int == 0) as libc::c_int,
            );
            js_defproperty(
                J,
                -(2 as libc::c_int),
                b"writable\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            if !((*ref_0).getter).is_null() {
                js_pushobject(J, (*ref_0).getter);
            } else {
                js_pushundefined(J);
            }
            js_defproperty(
                J,
                -(2 as libc::c_int),
                b"get\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if !((*ref_0).setter).is_null() {
                js_pushobject(J, (*ref_0).setter);
            } else {
                js_pushundefined(J);
            }
            js_defproperty(
                J,
                -(2 as libc::c_int),
                b"set\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        js_pushboolean(
            J,
            ((*ref_0).atts & JS_DONTENUM as libc::c_int == 0) as libc::c_int,
        );
        js_defproperty(
            J,
            -(2 as libc::c_int),
            b"enumerable\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        js_pushboolean(
            J,
            ((*ref_0).atts & JS_DONTCONF as libc::c_int == 0) as libc::c_int,
        );
        js_defproperty(
            J,
            -(2 as libc::c_int),
            b"configurable\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    };
}
unsafe extern "C" fn O_getOwnPropertyNames_walk(
    J: *mut js_State,
    ref_0: *mut js_Property,
    mut i: libc::c_int,
) -> libc::c_int {
    if (*(*ref_0).left).level != 0 {
        i = O_getOwnPropertyNames_walk(J, (*ref_0).left, i);
    }
    js_pushstring(J, ((*ref_0).name).as_mut_ptr());
    let fresh35 = i;
    i += 1;
    js_setindex(J, -(2 as libc::c_int), fresh35);
    if (*(*ref_0).right).level != 0 {
        i = O_getOwnPropertyNames_walk(J, (*ref_0).right, i);
    }
    i
}
unsafe extern "C" fn O_getOwnPropertyNames(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut k: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    js_newarray(J);
    if (*(*obj).properties).level != 0 {
        i = O_getOwnPropertyNames_walk(J, (*obj).properties, 0 as libc::c_int);
    } else {
        i = 0 as libc::c_int;
    }
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint {
        js_pushliteral(J, b"length\0" as *const u8 as *const libc::c_char);
        let fresh36 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh36);
        if (*obj).u.a.simple != 0 {
            k = 0 as libc::c_int;
            while k < (*obj).u.a.flat_length {
                js_itoa(name.as_mut_ptr(), k);
                js_pushstring(J, name.as_mut_ptr());
                let fresh37 = i;
                i += 1;
                js_setindex(J, -(2 as libc::c_int), fresh37);
                k += 1;
                k;
            }
        }
    }
    if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint {
        js_pushliteral(J, b"length\0" as *const u8 as *const libc::c_char);
        let fresh38 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh38);
        k = 0 as libc::c_int;
        while k < (*obj).u.s.length {
            js_itoa(name.as_mut_ptr(), k);
            js_pushstring(J, name.as_mut_ptr());
            let fresh39 = i;
            i += 1;
            js_setindex(J, -(2 as libc::c_int), fresh39);
            k += 1;
            k;
        }
    }
    if (*obj).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint {
        js_pushliteral(J, b"source\0" as *const u8 as *const libc::c_char);
        let fresh40 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh40);
        js_pushliteral(J, b"global\0" as *const u8 as *const libc::c_char);
        let fresh41 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh41);
        js_pushliteral(J, b"ignoreCase\0" as *const u8 as *const libc::c_char);
        let fresh42 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh42);
        js_pushliteral(J, b"multiline\0" as *const u8 as *const libc::c_char);
        let fresh43 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh43);
        js_pushliteral(J, b"lastIndex\0" as *const u8 as *const libc::c_char);
        let fresh44 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh44);
    }
}
unsafe extern "C" fn ToPropertyDescriptor(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
    desc: *mut js_Object,
) {
    let mut haswritable: libc::c_int = 0 as libc::c_int;
    let mut hasvalue: libc::c_int = 0 as libc::c_int;
    let mut enumerable: libc::c_int = 0 as libc::c_int;
    let mut configurable: libc::c_int = 0 as libc::c_int;
    let mut writable: libc::c_int = 0 as libc::c_int;
    let mut atts: libc::c_int = 0 as libc::c_int;
    js_pushobject(J, obj);
    js_pushobject(J, desc);
    if js_hasproperty(
        J,
        -(1 as libc::c_int),
        b"writable\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        haswritable = 1 as libc::c_int;
        writable = js_toboolean(J, -(1 as libc::c_int));
        js_pop(J, 1 as libc::c_int);
    }
    if js_hasproperty(
        J,
        -(1 as libc::c_int),
        b"enumerable\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        enumerable = js_toboolean(J, -(1 as libc::c_int));
        js_pop(J, 1 as libc::c_int);
    }
    if js_hasproperty(
        J,
        -(1 as libc::c_int),
        b"configurable\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        configurable = js_toboolean(J, -(1 as libc::c_int));
        js_pop(J, 1 as libc::c_int);
    }
    if js_hasproperty(
        J,
        -(1 as libc::c_int),
        b"value\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        hasvalue = 1 as libc::c_int;
        js_defproperty(J, -(3 as libc::c_int), name, 0 as libc::c_int);
    }
    if writable == 0 {
        atts |= JS_READONLY as libc::c_int;
    }
    if enumerable == 0 {
        atts |= JS_DONTENUM as libc::c_int;
    }
    if configurable == 0 {
        atts |= JS_DONTCONF as libc::c_int;
    }
    if js_hasproperty(
        J,
        -(1 as libc::c_int),
        b"get\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        if haswritable != 0 || hasvalue != 0 {
            js_typeerror(
                J,
                b"value/writable and get/set attributes are exclusive\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        js_pushundefined(J);
    }
    if js_hasproperty(
        J,
        -(2 as libc::c_int),
        b"set\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        if haswritable != 0 || hasvalue != 0 {
            js_typeerror(
                J,
                b"value/writable and get/set attributes are exclusive\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        js_pushundefined(J);
    }
    js_defaccessor(J, -(4 as libc::c_int), name, atts);
    js_pop(J, 2 as libc::c_int);
}
unsafe extern "C" fn O_defineProperty(J: *mut js_State) {
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    if js_isobject(J, 3 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    ToPropertyDescriptor(
        J,
        js_toobject(J, 1 as libc::c_int),
        js_tostring(J, 2 as libc::c_int),
        js_toobject(J, 3 as libc::c_int),
    );
    js_copy(J, 1 as libc::c_int);
}
unsafe extern "C" fn O_defineProperties_walk(J: *mut js_State, ref_0: *mut js_Property) {
    if (*(*ref_0).left).level != 0 {
        O_defineProperties_walk(J, (*ref_0).left);
    }
    if (*ref_0).atts & JS_DONTENUM as libc::c_int == 0 {
        js_pushvalue(J, (*ref_0).value);
        ToPropertyDescriptor(
            J,
            js_toobject(J, 1 as libc::c_int),
            ((*ref_0).name).as_mut_ptr(),
            js_toobject(J, -(1 as libc::c_int)),
        );
        js_pop(J, 1 as libc::c_int);
    }
    if (*(*ref_0).right).level != 0 {
        O_defineProperties_walk(J, (*ref_0).right);
    }
}
unsafe extern "C" fn O_defineProperties(J: *mut js_State) {
    let mut props: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    if js_isobject(J, 2 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    props = js_toobject(J, 2 as libc::c_int);
    if (*(*props).properties).level != 0 {
        O_defineProperties_walk(J, (*props).properties);
    }
    js_copy(J, 1 as libc::c_int);
}
unsafe extern "C" fn O_create_walk(J: *mut js_State, obj: *mut js_Object, ref_0: *mut js_Property) {
    if (*(*ref_0).left).level != 0 {
        O_create_walk(J, obj, (*ref_0).left);
    }
    if (*ref_0).atts & JS_DONTENUM as libc::c_int == 0 {
        if (*ref_0).value.t.type_0 as libc::c_int != JS_TOBJECT as libc::c_int {
            js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
        }
        ToPropertyDescriptor(
            J,
            obj,
            ((*ref_0).name).as_mut_ptr(),
            (*ref_0).value.u.object,
        );
    }
    if (*(*ref_0).right).level != 0 {
        O_create_walk(J, obj, (*ref_0).right);
    }
}
unsafe extern "C" fn O_create(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut proto: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut props: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) != 0 {
        proto = js_toobject(J, 1 as libc::c_int);
    } else if js_isnull(J, 1 as libc::c_int) != 0 {
        proto = std::ptr::null_mut::<js_Object>();
    } else {
        js_typeerror(
            J,
            b"not an object or null\0" as *const u8 as *const libc::c_char,
        );
    }
    obj = jsV_newobject(J, JS_COBJECT, proto);
    js_pushobject(J, obj);
    if js_isdefined(J, 2 as libc::c_int) != 0 {
        if js_isobject(J, 2 as libc::c_int) == 0 {
            js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
        }
        props = js_toobject(J, 2 as libc::c_int);
        if (*(*props).properties).level != 0 {
            O_create_walk(J, obj, (*props).properties);
        }
    }
}
unsafe extern "C" fn O_keys_walk(
    J: *mut js_State,
    ref_0: *mut js_Property,
    mut i: libc::c_int,
) -> libc::c_int {
    if (*(*ref_0).left).level != 0 {
        i = O_keys_walk(J, (*ref_0).left, i);
    }
    if (*ref_0).atts & JS_DONTENUM as libc::c_int == 0 {
        js_pushstring(J, ((*ref_0).name).as_mut_ptr());
        let fresh45 = i;
        i += 1;
        js_setindex(J, -(2 as libc::c_int), fresh45);
    }
    if (*(*ref_0).right).level != 0 {
        i = O_keys_walk(J, (*ref_0).right, i);
    }
    i
}
unsafe extern "C" fn O_keys(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    js_newarray(J);
    if (*(*obj).properties).level != 0 {
        i = O_keys_walk(J, (*obj).properties, 0 as libc::c_int);
    } else {
        i = 0 as libc::c_int;
    }
    if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint {
        k = 0 as libc::c_int;
        while k < (*obj).u.s.length {
            js_itoa(name.as_mut_ptr(), k);
            js_pushstring(J, name.as_mut_ptr());
            let fresh46 = i;
            i += 1;
            js_setindex(J, -(2 as libc::c_int), fresh46);
            k += 1;
            k;
        }
    }
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        k = 0 as libc::c_int;
        while k < (*obj).u.a.flat_length {
            js_itoa(name.as_mut_ptr(), k);
            js_pushstring(J, name.as_mut_ptr());
            let fresh47 = i;
            i += 1;
            js_setindex(J, -(2 as libc::c_int), fresh47);
            k += 1;
            k;
        }
    }
}
unsafe extern "C" fn O_preventExtensions(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    jsR_unflattenarray(J, obj);
    (*obj).extensible = 0 as libc::c_int;
    js_copy(J, 1 as libc::c_int);
}
unsafe extern "C" fn O_isExtensible(J: *mut js_State) {
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    js_pushboolean(J, (*js_toobject(J, 1 as libc::c_int)).extensible);
}
unsafe extern "C" fn O_seal_walk(J: *mut js_State, ref_0: *mut js_Property) {
    if (*(*ref_0).left).level != 0 {
        O_seal_walk(J, (*ref_0).left);
    }
    (*ref_0).atts |= JS_DONTCONF as libc::c_int;
    if (*(*ref_0).right).level != 0 {
        O_seal_walk(J, (*ref_0).right);
    }
}
unsafe extern "C" fn O_seal(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    jsR_unflattenarray(J, obj);
    (*obj).extensible = 0 as libc::c_int;
    if (*(*obj).properties).level != 0 {
        O_seal_walk(J, (*obj).properties);
    }
    js_copy(J, 1 as libc::c_int);
}
unsafe extern "C" fn O_isSealed_walk(J: *mut js_State, ref_0: *mut js_Property) -> libc::c_int {
    if (*(*ref_0).left).level != 0 && O_isSealed_walk(J, (*ref_0).left) == 0 {
        return 0 as libc::c_int;
    }
    if (*ref_0).atts & JS_DONTCONF as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    if (*(*ref_0).right).level != 0 && O_isSealed_walk(J, (*ref_0).right) == 0 {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn O_isSealed(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    if (*obj).extensible != 0 {
        js_pushboolean(J, 0 as libc::c_int);
        return;
    }
    if (*(*obj).properties).level != 0 {
        js_pushboolean(J, O_isSealed_walk(J, (*obj).properties));
    } else {
        js_pushboolean(J, 1 as libc::c_int);
    };
}
unsafe extern "C" fn O_freeze_walk(J: *mut js_State, ref_0: *mut js_Property) {
    if (*(*ref_0).left).level != 0 {
        O_freeze_walk(J, (*ref_0).left);
    }
    (*ref_0).atts |= JS_READONLY as libc::c_int | JS_DONTCONF as libc::c_int;
    if (*(*ref_0).right).level != 0 {
        O_freeze_walk(J, (*ref_0).right);
    }
}
unsafe extern "C" fn O_freeze(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    jsR_unflattenarray(J, obj);
    (*obj).extensible = 0 as libc::c_int;
    if (*(*obj).properties).level != 0 {
        O_freeze_walk(J, (*obj).properties);
    }
    js_copy(J, 1 as libc::c_int);
}
unsafe extern "C" fn O_isFrozen_walk(J: *mut js_State, ref_0: *mut js_Property) -> libc::c_int {
    if (*(*ref_0).left).level != 0 && O_isFrozen_walk(J, (*ref_0).left) == 0 {
        return 0 as libc::c_int;
    }
    if (*ref_0).atts & JS_READONLY as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    if (*ref_0).atts & JS_DONTCONF as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    if (*(*ref_0).right).level != 0 && O_isFrozen_walk(J, (*ref_0).right) == 0 {
        return 0 as libc::c_int;
    }
    1 as libc::c_int
}
unsafe extern "C" fn O_isFrozen(J: *mut js_State) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1 as libc::c_int) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1 as libc::c_int);
    if (*(*obj).properties).level != 0 && O_isFrozen_walk(J, (*obj).properties) == 0 {
        js_pushboolean(J, 0 as libc::c_int);
        return;
    }
    js_pushboolean(J, ((*obj).extensible == 0) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initobject(J: *mut js_State) {
    js_pushobject(J, (*J).Object_prototype);
    jsB_propf(
        J,
        b"Object.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Op_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.prototype.toLocaleString\0" as *const u8 as *const libc::c_char,
        Some(Op_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Op_valueOf as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.prototype.hasOwnProperty\0" as *const u8 as *const libc::c_char,
        Some(Op_hasOwnProperty as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.prototype.isPrototypeOf\0" as *const u8 as *const libc::c_char,
        Some(Op_isPrototypeOf as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.prototype.propertyIsEnumerable\0" as *const u8 as *const libc::c_char,
        Some(Op_propertyIsEnumerable as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_Object as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_new_Object as unsafe extern "C" fn(*mut js_State) -> ()),
        b"Object\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.getPrototypeOf\0" as *const u8 as *const libc::c_char,
        Some(O_getPrototypeOf as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.getOwnPropertyDescriptor\0" as *const u8 as *const libc::c_char,
        Some(O_getOwnPropertyDescriptor as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.getOwnPropertyNames\0" as *const u8 as *const libc::c_char,
        Some(O_getOwnPropertyNames as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.create\0" as *const u8 as *const libc::c_char,
        Some(O_create as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.defineProperty\0" as *const u8 as *const libc::c_char,
        Some(O_defineProperty as unsafe extern "C" fn(*mut js_State) -> ()),
        3 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.defineProperties\0" as *const u8 as *const libc::c_char,
        Some(O_defineProperties as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.seal\0" as *const u8 as *const libc::c_char,
        Some(O_seal as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.freeze\0" as *const u8 as *const libc::c_char,
        Some(O_freeze as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.preventExtensions\0" as *const u8 as *const libc::c_char,
        Some(O_preventExtensions as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.isSealed\0" as *const u8 as *const libc::c_char,
        Some(O_isSealed as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.isFrozen\0" as *const u8 as *const libc::c_char,
        Some(O_isFrozen as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.isExtensible\0" as *const u8 as *const libc::c_char,
        Some(O_isExtensible as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"Object.keys\0" as *const u8 as *const libc::c_char,
        Some(O_keys as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"Object\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_isnumberobject(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint
            == JS_CNUMBER as libc::c_int as libc::c_uint) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isstringobject(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint
            == JS_CSTRING as libc::c_int as libc::c_uint) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isbooleanobject(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint
            == JS_CBOOLEAN as libc::c_int as libc::c_uint) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isdateobject(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint == JS_CDATE as libc::c_int as libc::c_uint)
        as libc::c_int
}
unsafe extern "C" fn jsonnext(J: *mut js_State) {
    (*J).lookahead = jsY_lexjson(J);
}
unsafe extern "C" fn jsonaccept(J: *mut js_State, t: libc::c_int) -> libc::c_int {
    if (*J).lookahead == t {
        jsonnext(J);
        return 1 as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn jsonexpect(J: *mut js_State, t: libc::c_int) {
    if jsonaccept(J, t) == 0 {
        js_syntaxerror(
            J,
            b"JSON: unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(t),
        );
    }
}
unsafe extern "C" fn jsonvalue(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let mut name: *const libc::c_char = std::ptr::null::<libc::c_char>();
    match (*J).lookahead {
        258 => {
            js_pushstring(J, (*J).text);
            jsonnext(J);
        }
        257 => {
            js_pushnumber(J, (*J).number);
            jsonnext(J);
        }
        123 => {
            js_newobject(J);
            jsonnext(J);
            if jsonaccept(J, '}' as i32) != 0 {
                return;
            }
            loop {
                if (*J).lookahead != TK_STRING as libc::c_int {
                    js_syntaxerror(
                        J,
                        b"JSON: unexpected token: %s (expected string)\0" as *const u8
                            as *const libc::c_char,
                        jsY_tokenstring((*J).lookahead),
                    );
                }
                name = (*J).text;
                jsonnext(J);
                jsonexpect(J, ':' as i32);
                jsonvalue(J);
                js_setproperty(J, -(2 as libc::c_int), name);
                if jsonaccept(J, ',' as i32) == 0 {
                    break;
                }
            }
            jsonexpect(J, '}' as i32);
        }
        91 => {
            js_newarray(J);
            jsonnext(J);
            i = 0 as libc::c_int;
            if jsonaccept(J, ']' as i32) != 0 {
                return;
            }
            loop {
                jsonvalue(J);
                let fresh48 = i;
                i += 1;
                js_setindex(J, -(2 as libc::c_int), fresh48);
                if jsonaccept(J, ',' as i32) == 0 {
                    break;
                }
            }
            jsonexpect(J, ']' as i32);
        }
        306 => {
            js_pushboolean(J, 1 as libc::c_int);
            jsonnext(J);
        }
        293 => {
            js_pushboolean(J, 0 as libc::c_int);
            jsonnext(J);
        }
        301 => {
            js_pushnull(J);
            jsonnext(J);
        }
        _ => {
            js_syntaxerror(
                J,
                b"JSON: unexpected token: %s\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
            );
        }
    };
}
unsafe extern "C" fn jsonrevive(J: *mut js_State, name: *const libc::c_char) {
    let mut key: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut buf: [libc::c_char; 32] = [0; 32];
    js_getproperty(J, -(1 as libc::c_int), name);
    if js_isobject(J, -(1 as libc::c_int)) != 0 {
        if js_isarray(J, -(1 as libc::c_int)) != 0 {
            let mut i: libc::c_int = 0 as libc::c_int;
            let n: libc::c_int = js_getlength(J, -(1 as libc::c_int));
            i = 0 as libc::c_int;
            while i < n {
                jsonrevive(J, js_itoa(buf.as_mut_ptr(), i));
                if js_isundefined(J, -(1 as libc::c_int)) != 0 {
                    js_pop(J, 1 as libc::c_int);
                    js_delproperty(J, -(1 as libc::c_int), buf.as_mut_ptr());
                } else {
                    js_setproperty(J, -(2 as libc::c_int), buf.as_mut_ptr());
                }
                i += 1;
                i;
            }
        } else {
            js_pushiterator(J, -(1 as libc::c_int), 1 as libc::c_int);
            loop {
                key = js_nextiterator(J, -(1 as libc::c_int));
                if key.is_null() {
                    break;
                }
                js_rot2(J);
                jsonrevive(J, key);
                if js_isundefined(J, -(1 as libc::c_int)) != 0 {
                    js_pop(J, 1 as libc::c_int);
                    js_delproperty(J, -(1 as libc::c_int), key);
                } else {
                    js_setproperty(J, -(2 as libc::c_int), key);
                }
                js_rot2(J);
            }
            js_pop(J, 1 as libc::c_int);
        }
    }
    js_copy(J, 2 as libc::c_int);
    js_copy(J, -(3 as libc::c_int));
    js_pushstring(J, name);
    js_copy(J, -(4 as libc::c_int));
    js_call(J, 2 as libc::c_int);
    js_rot2pop1(J);
}
unsafe extern "C" fn JSON_parse(J: *mut js_State) {
    let source: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    jsY_initlex(J, b"JSON\0" as *const u8 as *const libc::c_char, source);
    jsonnext(J);
    if js_iscallable(J, 2 as libc::c_int) != 0 {
        js_newobject(J);
        jsonvalue(J);
        js_defproperty(
            J,
            -(2 as libc::c_int),
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        jsonrevive(J, b"\0" as *const u8 as *const libc::c_char);
    } else {
        jsonvalue(J);
    };
}
unsafe extern "C" fn fmtnum(J: *mut js_State, sb: *mut *mut js_Buffer, n: libc::c_double) {
    if n.is_nan() as i32 != 0 {
        js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
    } else if if n.is_infinite() {
        if n.is_sign_positive() {
            1
        } else {
            -1
        }
    } else {
        0
    } != 0
    {
        js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
    } else if n == 0 as libc::c_int as libc::c_double {
        js_puts(J, sb, b"0\0" as *const u8 as *const libc::c_char);
    } else {
        let mut buf: [libc::c_char; 40] = [0; 40];
        js_puts(J, sb, jsV_numbertostring(J, buf.as_mut_ptr(), n));
    };
}
unsafe extern "C" fn fmtstr(J: *mut js_State, sb: *mut *mut js_Buffer, mut s: *const libc::c_char) {
    static mut HEX: *const libc::c_char = b"0123456789abcdef\0" as *const u8 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: Rune = 0;
    js_putc(J, sb, '"' as i32);
    while *s != 0 {
        n = jsU_chartorune(&mut c, s);
        match c {
            34 => {
                js_puts(J, sb, b"\\\"\0" as *const u8 as *const libc::c_char);
            }
            92 => {
                js_puts(J, sb, b"\\\\\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                js_puts(J, sb, b"\\b\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                js_puts(J, sb, b"\\f\0" as *const u8 as *const libc::c_char);
            }
            10 => {
                js_puts(J, sb, b"\\n\0" as *const u8 as *const libc::c_char);
            }
            13 => {
                js_puts(J, sb, b"\\r\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                js_puts(J, sb, b"\\t\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                if c < ' ' as i32 || c >= 0xd800 as libc::c_int && c <= 0xdfff as libc::c_int {
                    js_putc(J, sb, '\\' as i32);
                    js_putc(J, sb, 'u' as i32);
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 12 as libc::c_int & 15 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 8 as libc::c_int & 15 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 4 as libc::c_int & 15 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c & 15 as libc::c_int) as isize) as libc::c_int,
                    );
                } else if c < 128 as libc::c_int {
                    js_putc(J, sb, c);
                } else {
                    i = 0 as libc::c_int;
                    while i < n {
                        js_putc(J, sb, *s.offset(i as isize) as libc::c_int);
                        i += 1;
                        i;
                    }
                }
            }
        }
        s = s.offset(n as isize);
    }
    js_putc(J, sb, '"' as i32);
}
unsafe extern "C" fn fmtindent(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    gap: *const libc::c_char,
    mut level: libc::c_int,
) {
    js_putc(J, sb, '\n' as i32);
    loop {
        let fresh49 = level;
        level -= 1;
        if fresh49 == 0 {
            break;
        }
        js_puts(J, sb, gap);
    }
}
unsafe extern "C" fn filterprop(J: *mut js_State, key: *const libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    if js_isarray(J, 2 as libc::c_int) != 0 {
        found = 0 as libc::c_int;
        n = js_getlength(J, 2 as libc::c_int);
        i = 0 as libc::c_int;
        while i < n && found == 0 {
            js_getindex(J, 2 as libc::c_int, i);
            if js_isstring(J, -(1 as libc::c_int)) != 0
                || js_isnumber(J, -(1 as libc::c_int)) != 0
                || js_isstringobject(J, -(1 as libc::c_int)) != 0
                || js_isnumberobject(J, -(1 as libc::c_int)) != 0
            {
                found = (strcmp(key, js_tostring(J, -(1 as libc::c_int))) == 0) as libc::c_int;
            }
            js_pop(J, 1 as libc::c_int);
            i += 1;
            i;
        }
        return found;
    }
    1 as libc::c_int
}
unsafe extern "C" fn fmtobject(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    obj: *mut js_Object,
    gap: *const libc::c_char,
    level: libc::c_int,
) {
    let mut key: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut save: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = js_gettop(J) - 1 as libc::c_int;
    i = 4 as libc::c_int;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as libc::c_int)) {
            js_typeerror(
                J,
                b"cyclic object value\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    n = 0 as libc::c_int;
    js_putc(J, sb, '{' as i32);
    js_pushiterator(J, -(1 as libc::c_int), 1 as libc::c_int);
    loop {
        key = js_nextiterator(J, -(1 as libc::c_int));
        if key.is_null() {
            break;
        }
        if filterprop(J, key) != 0 {
            save = (**sb).s.len() as libc::c_int;
            if n != 0 {
                js_putc(J, sb, ',' as i32);
            }
            if !gap.is_null() {
                fmtindent(J, sb, gap, level + 1 as libc::c_int);
            }
            fmtstr(J, sb, key);
            js_putc(J, sb, ':' as i32);
            if !gap.is_null() {
                js_putc(J, sb, ' ' as i32);
            }
            js_rot2(J);
            if fmtvalue(J, sb, key, gap, level + 1 as libc::c_int) == 0 {
                (**sb).s.set_len(save as usize);
            } else {
                n += 1;
                n;
            }
            js_rot2(J);
        }
    }
    js_pop(J, 1 as libc::c_int);
    if !gap.is_null() && n != 0 {
        fmtindent(J, sb, gap, level);
    }
    js_putc(J, sb, '}' as i32);
}
unsafe extern "C" fn fmtarray(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    gap: *const libc::c_char,
    level: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut buf: [libc::c_char; 32] = [0; 32];
    n = js_gettop(J) - 1 as libc::c_int;
    i = 4 as libc::c_int;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as libc::c_int)) {
            js_typeerror(
                J,
                b"cyclic object value\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
        i;
    }
    js_putc(J, sb, '[' as i32);
    n = js_getlength(J, -(1 as libc::c_int));
    i = 0 as libc::c_int;
    while i < n {
        if i != 0 {
            js_putc(J, sb, ',' as i32);
        }
        if !gap.is_null() {
            fmtindent(J, sb, gap, level + 1 as libc::c_int);
        }
        if fmtvalue(
            J,
            sb,
            js_itoa(buf.as_mut_ptr(), i),
            gap,
            level + 1 as libc::c_int,
        ) == 0
        {
            js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
        i;
    }
    if !gap.is_null() && n != 0 {
        fmtindent(J, sb, gap, level);
    }
    js_putc(J, sb, ']' as i32);
}
unsafe extern "C" fn fmtvalue(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    key: *const libc::c_char,
    gap: *const libc::c_char,
    level: libc::c_int,
) -> libc::c_int {
    js_getproperty(J, -(1 as libc::c_int), key);
    if js_isobject(J, -(1 as libc::c_int)) != 0
        && js_hasproperty(
            J,
            -(1 as libc::c_int),
            b"toJSON\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        if js_iscallable(J, -(1 as libc::c_int)) != 0 {
            js_copy(J, -(2 as libc::c_int));
            js_pushstring(J, key);
            js_call(J, 1 as libc::c_int);
            js_rot2pop1(J);
        } else {
            js_pop(J, 1 as libc::c_int);
        }
    }
    if js_iscallable(J, 2 as libc::c_int) != 0 {
        js_copy(J, 2 as libc::c_int);
        js_copy(J, -(3 as libc::c_int));
        js_pushstring(J, key);
        js_copy(J, -(4 as libc::c_int));
        js_call(J, 2 as libc::c_int);
        js_rot2pop1(J);
    }
    if js_isobject(J, -(1 as libc::c_int)) != 0 && js_iscallable(J, -(1 as libc::c_int)) == 0 {
        let obj: *mut js_Object = js_toobject(J, -(1 as libc::c_int));
        match (*obj).type_0 as libc::c_uint {
            7 => {
                fmtnum(J, sb, (*obj).u.number);
            }
            8 => {
                fmtstr(J, sb, (*obj).u.s.string);
            }
            6 => {
                js_puts(
                    J,
                    sb,
                    if (*obj).u.boolean != 0 {
                        b"true\0" as *const u8 as *const libc::c_char
                    } else {
                        b"false\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            1 => {
                fmtarray(J, sb, gap, level);
            }
            _ => {
                fmtobject(J, sb, obj, gap, level);
            }
        }
    } else if js_isboolean(J, -(1 as libc::c_int)) != 0 {
        js_puts(
            J,
            sb,
            if js_toboolean(J, -(1 as libc::c_int)) != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    } else if js_isnumber(J, -(1 as libc::c_int)) != 0 {
        fmtnum(J, sb, js_tonumber(J, -(1 as libc::c_int)));
    } else if js_isstring(J, -(1 as libc::c_int)) != 0 {
        fmtstr(J, sb, js_tostring(J, -(1 as libc::c_int)));
    } else if js_isnull(J, -(1 as libc::c_int)) != 0 {
        js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
    } else {
        js_pop(J, 1 as libc::c_int);
        return 0 as libc::c_int;
    }
    js_pop(J, 1 as libc::c_int);
    1 as libc::c_int
}
unsafe extern "C" fn JSON_stringify(J: *mut js_State) {
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut buf: [libc::c_char; 12] = [0; 12];
    let mut gap: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut n: libc::c_int = 0;
    ::core::ptr::write_volatile(
        &mut gap as *mut *const libc::c_char,
        std::ptr::null::<libc::c_char>(),
    );
    if js_isnumber(J, 3 as libc::c_int) != 0 || js_isnumberobject(J, 3 as libc::c_int) != 0 {
        n = js_tointeger(J, 3 as libc::c_int);
        if n < 0 as libc::c_int {
            n = 0 as libc::c_int;
        }
        if n > 10 as libc::c_int {
            n = 10 as libc::c_int;
        }
        memset(
            buf.as_mut_ptr() as *mut libc::c_void,
            ' ' as i32,
            n as libc::c_ulong,
        );
        buf[n as usize] = 0 as libc::c_int as libc::c_char;
        if n > 0 as libc::c_int {
            ::core::ptr::write_volatile(&mut gap as *mut *const libc::c_char, buf.as_mut_ptr());
        }
    } else if js_isstring(J, 3 as libc::c_int) != 0 || js_isstringobject(J, 3 as libc::c_int) != 0 {
        s = js_tostring(J, 3 as libc::c_int);
        n = strlen(s) as libc::c_int;
        if n > 10 as libc::c_int {
            n = 10 as libc::c_int;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            n as libc::c_ulong,
        );
        buf[n as usize] = 0 as libc::c_int as libc::c_char;
        if n > 0 as libc::c_int {
            ::core::ptr::write_volatile(&mut gap as *mut *const libc::c_char, buf.as_mut_ptr());
        }
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    js_newobject(J);
    js_copy(J, 1 as libc::c_int);
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if fmtvalue(
        J,
        &mut sb,
        b"\0" as *const u8 as *const libc::c_char,
        gap,
        0 as libc::c_int,
    ) == 0
    {
        js_pushundefined(J);
    } else {
        js_putc(J, &mut sb, 0 as libc::c_int);
        js_pushstring(
            J,
            if !sb.is_null() {
                ((*sb).s).as_mut_ptr() as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        js_rot2pop1(J);
    }
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initjson(J: *mut js_State) {
    js_pushobject(J, jsV_newobject(J, JS_CJSON, (*J).Object_prototype));
    jsB_propf(
        J,
        b"JSON.parse\0" as *const u8 as *const libc::c_char,
        Some(JSON_parse as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"JSON.stringify\0" as *const u8 as *const libc::c_char,
        Some(JSON_stringify as unsafe extern "C" fn(*mut js_State) -> ()),
        3 as libc::c_int,
    );
    js_defglobal(
        J,
        b"JSON\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn jsP_error(J: *mut js_State, fmt: *const libc::c_char, args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut msgbuf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        msgbuf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    snprintf(
        buf.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"%s:%d: \0" as *const u8 as *const libc::c_char,
        (*J).filename,
        (*J).lexline,
    );
    strcat(buf.as_mut_ptr(), msgbuf.as_mut_ptr());
    js_newsyntaxerror(J, buf.as_mut_ptr());
    js_throw(J);
}
unsafe extern "C" fn jsP_warning(J: *mut js_State, fmt: *const libc::c_char, args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut msg: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        b"%s:%d: warning: %s\0" as *const u8 as *const libc::c_char,
        (*J).filename,
        (*J).lexline,
        msg.as_mut_ptr(),
    );
    js_report(J, buf.as_mut_ptr());
}
unsafe extern "C" fn jsP_newnode(
    J: *mut js_State,
    type_0: js_AstType,
    line: libc::c_int,
    a: *mut js_Ast,
    b: *mut js_Ast,
    c: *mut js_Ast,
    d: *mut js_Ast,
) -> *mut js_Ast {
    let node: *mut js_Ast = js_malloc(
        J,
        ::core::mem::size_of::<js_Ast>() as libc::c_ulong as libc::c_int,
    ) as *mut js_Ast;
    (*node).type_0 = type_0;
    (*node).line = line;
    (*node).a = a;
    (*node).b = b;
    (*node).c = c;
    (*node).d = d;
    (*node).number = 0 as libc::c_int as libc::c_double;
    (*node).string = std::ptr::null::<libc::c_char>();
    (*node).jumps = std::ptr::null_mut::<js_JumpList>();
    (*node).casejump = 0 as libc::c_int;
    (*node).parent = std::ptr::null_mut::<js_Ast>();
    if !a.is_null() {
        (*a).parent = node;
    }
    if !b.is_null() {
        (*b).parent = node;
    }
    if !c.is_null() {
        (*c).parent = node;
    }
    if !d.is_null() {
        (*d).parent = node;
    }
    (*node).gcnext = (*J).gcast;
    (*J).gcast = node;
    node
}
unsafe extern "C" fn jsP_list(head: *mut js_Ast) -> *mut js_Ast {
    let mut prev: *mut js_Ast = head;
    let mut node: *mut js_Ast = (*head).b;
    while !node.is_null() {
        (*node).parent = prev;
        prev = node;
        node = (*node).b;
    }
    head
}
unsafe extern "C" fn jsP_newstrnode(
    J: *mut js_State,
    type_0: js_AstType,
    s: *const libc::c_char,
) -> *mut js_Ast {
    let node: *mut js_Ast = jsP_newnode(
        J,
        type_0,
        (*J).lexline,
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    (*node).string = s;
    node
}
unsafe extern "C" fn jsP_newnumnode(
    J: *mut js_State,
    type_0: js_AstType,
    n: libc::c_double,
) -> *mut js_Ast {
    let node: *mut js_Ast = jsP_newnode(
        J,
        type_0,
        (*J).lexline,
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    (*node).number = n;
    node
}
unsafe extern "C" fn jsP_freejumps(J: *mut js_State, mut node: *mut js_JumpList) {
    while !node.is_null() {
        let next_0: *mut js_JumpList = (*node).next;
        js_free(J, node as *mut libc::c_void);
        node = next_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsP_freeparse(J: *mut js_State) {
    let mut node: *mut js_Ast = (*J).gcast;
    while !node.is_null() {
        let next_0: *mut js_Ast = (*node).gcnext;
        jsP_freejumps(J, (*node).jumps);
        js_free(J, node as *mut libc::c_void);
        node = next_0;
    }
    (*J).gcast = std::ptr::null_mut::<js_Ast>();
}
unsafe extern "C" fn jsP_next(J: *mut js_State) {
    (*J).lookahead = jsY_lex(J);
}
unsafe extern "C" fn semicolon(J: *mut js_State) {
    if (*J).lookahead == ';' as i32 {
        jsP_next(J);
        return;
    }
    if (*J).newline != 0 || (*J).lookahead == '}' as i32 || (*J).lookahead == 0 as libc::c_int {
        return;
    }
    jsP_error(
        J,
        b"unexpected token: %s (expected ';')\0" as *const u8 as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn identifier(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == TK_IDENTIFIER as libc::c_int {
        a = jsP_newstrnode(J, AST_IDENTIFIER, (*J).text);
        jsP_next(J);
        return a;
    }
    jsP_error(
        J,
        b"unexpected token: %s (expected identifier)\0" as *const u8 as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn identifieropt(J: *mut js_State) -> *mut js_Ast {
    if (*J).lookahead == TK_IDENTIFIER as libc::c_int {
        return identifier(J);
    }
    std::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn identifiername(J: *mut js_State) -> *mut js_Ast {
    if (*J).lookahead == TK_IDENTIFIER as libc::c_int || (*J).lookahead >= TK_BREAK as libc::c_int {
        let a: *mut js_Ast = jsP_newstrnode(J, AST_IDENTIFIER, (*J).text);
        jsP_next(J);
        return a;
    }
    jsP_error(
        J,
        b"unexpected token: %s (expected identifier or keyword)\0" as *const u8
            as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn arrayelement(J: *mut js_State) -> *mut js_Ast {
    let line: libc::c_int = (*J).lexline;
    if (*J).lookahead == ',' as i32 {
        return jsP_newnode(
            J,
            EXP_ELISION,
            line,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    assignment(J, 0 as libc::c_int)
}
unsafe extern "C" fn arrayliteral(J: *mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == ']' as i32 {
        return std::ptr::null_mut::<js_Ast>();
    }
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        arrayelement(J),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if (*J).lookahead != ']' as i32 {
            (*tail).b = jsP_newnode(
                J,
                AST_LIST,
                0 as libc::c_int,
                arrayelement(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
            tail = (*tail).b;
        }
    }
    jsP_list(head)
}
unsafe extern "C" fn propname(J: *mut js_State) -> *mut js_Ast {
    let mut name: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == TK_NUMBER as libc::c_int {
        name = jsP_newnumnode(J, EXP_NUMBER, (*J).number);
        jsP_next(J);
    } else if (*J).lookahead == TK_STRING as libc::c_int {
        name = jsP_newstrnode(J, EXP_STRING, (*J).text);
        jsP_next(J);
    } else {
        name = identifiername(J);
    }
    name
}
unsafe extern "C" fn propassign(J: *mut js_State) -> *mut js_Ast {
    let mut name: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut value: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut arg: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut body: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = (*J).lexline;
    name = propname(J);
    if (*J).lookahead != ':' as i32
        && (*name).type_0 as libc::c_uint == AST_IDENTIFIER as libc::c_int as libc::c_uint
    {
        if strcmp((*name).string, b"get\0" as *const u8 as *const libc::c_char) == 0 {
            name = propname(J);
            if if (*J).lookahead == '(' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring('(' as i32),
                );
            }
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(')' as i32),
                );
            }
            body = funbody(J);
            return jsP_newnode(
                J,
                EXP_PROP_GET,
                line,
                name,
                std::ptr::null_mut::<js_Ast>(),
                body,
                std::ptr::null_mut::<js_Ast>(),
            );
        }
        if strcmp((*name).string, b"set\0" as *const u8 as *const libc::c_char) == 0 {
            name = propname(J);
            if if (*J).lookahead == '(' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring('(' as i32),
                );
            }
            arg = identifier(J);
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(')' as i32),
                );
            }
            body = funbody(J);
            return jsP_newnode(
                J,
                EXP_PROP_SET,
                line,
                name,
                jsP_newnode(
                    J,
                    AST_LIST,
                    0 as libc::c_int,
                    arg,
                    std::ptr::null_mut::<js_Ast>(),
                    std::ptr::null_mut::<js_Ast>(),
                    std::ptr::null_mut::<js_Ast>(),
                ),
                body,
                std::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    if if (*J).lookahead == ':' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(':' as i32),
        );
    }
    value = assignment(J, 0 as libc::c_int);
    jsP_newnode(
        J,
        EXP_PROP_VAL,
        line,
        name,
        value,
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn objectliteral(J: *mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == '}' as i32 {
        return std::ptr::null_mut::<js_Ast>();
    }
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        propassign(J),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if (*J).lookahead == '}' as i32 {
            break;
        }
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            propassign(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn parameters(J: *mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == ')' as i32 {
        return std::ptr::null_mut::<js_Ast>();
    }
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        identifier(J),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            identifier(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn fundec(J: *mut js_State, line: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    a = identifier(J);
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('(' as i32),
        );
    }
    b = parameters(J);
    if if (*J).lookahead == ')' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(')' as i32),
        );
    }
    c = funbody(J);
    jsP_newnode(J, AST_FUNDEC, line, a, b, c, std::ptr::null_mut::<js_Ast>())
}
unsafe extern "C" fn funstm(J: *mut js_State, line: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    a = identifier(J);
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('(' as i32),
        );
    }
    b = parameters(J);
    if if (*J).lookahead == ')' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(')' as i32),
        );
    }
    c = funbody(J);
    jsP_newnode(
        J,
        STM_VAR,
        line,
        jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            jsP_newnode(
                J,
                EXP_VAR,
                line,
                a,
                jsP_newnode(J, EXP_FUN, line, a, b, c, std::ptr::null_mut::<js_Ast>()),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            ),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        ),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn funexp(J: *mut js_State, line: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    a = identifieropt(J);
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('(' as i32),
        );
    }
    b = parameters(J);
    if if (*J).lookahead == ')' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(')' as i32),
        );
    }
    c = funbody(J);
    jsP_newnode(J, EXP_FUN, line, a, b, c, std::ptr::null_mut::<js_Ast>())
}
unsafe extern "C" fn primary(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = (*J).lexline;
    if (*J).lookahead == TK_IDENTIFIER as libc::c_int {
        a = jsP_newstrnode(J, EXP_IDENTIFIER, (*J).text);
        jsP_next(J);
        return a;
    }
    if (*J).lookahead == TK_STRING as libc::c_int {
        a = jsP_newstrnode(J, EXP_STRING, (*J).text);
        jsP_next(J);
        return a;
    }
    if (*J).lookahead == TK_REGEXP as libc::c_int {
        a = jsP_newstrnode(J, EXP_REGEXP, (*J).text);
        (*a).number = (*J).number;
        jsP_next(J);
        return a;
    }
    if (*J).lookahead == TK_NUMBER as libc::c_int {
        a = jsP_newnumnode(J, EXP_NUMBER, (*J).number);
        jsP_next(J);
        return a;
    }
    if if (*J).lookahead == TK_THIS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_THIS,
            line,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_NULL as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_NULL,
            line,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_TRUE as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_TRUE,
            line,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_FALSE as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_FALSE,
            line,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == '{' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_OBJECT,
            line,
            objectliteral(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        if if (*J).lookahead == '}' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('}' as i32),
            );
        }
        return a;
    }
    if if (*J).lookahead == '[' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ARRAY,
            line,
            arrayliteral(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        if if (*J).lookahead == ']' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(']' as i32),
            );
        }
        return a;
    }
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(')' as i32),
            );
        }
        return a;
    }
    jsP_error(
        J,
        b"unexpected token in expression: %s\0" as *const u8 as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn arguments(J: *mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == ')' as i32 {
        return std::ptr::null_mut::<js_Ast>();
    }
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        assignment(J, 0 as libc::c_int),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            assignment(J, 0 as libc::c_int),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn newexp(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == TK_NEW as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = memberexp(J);
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            b = arguments(J);
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(')' as i32),
                );
            }
            return jsP_newnode(
                J,
                EXP_NEW,
                line,
                a,
                b,
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        }
        return jsP_newnode(
            J,
            EXP_NEW,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_FUNCTION as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return funexp(J, line);
    }
    primary(J)
}
unsafe extern "C" fn memberexp(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = newexp(J);
    let mut line: libc::c_int = 0;
    let SAVE: libc::c_int = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '.' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_MEMBER,
                line,
                a,
                identifiername(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == '[' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0)
            {
                break;
            }
            a = jsP_newnode(
                J,
                EXP_INDEX,
                line,
                a,
                expression(J, 0 as libc::c_int),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
            if if (*J).lookahead == ']' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(']' as i32),
                );
            }
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn callexp(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = newexp(J);
    let mut line: libc::c_int = 0;
    let SAVE: libc::c_int = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '.' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_MEMBER,
                line,
                a,
                identifiername(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == '[' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_INDEX,
                line,
                a,
                expression(J, 0 as libc::c_int),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
            if if (*J).lookahead == ']' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(']' as i32),
                );
            }
        } else {
            if (if (*J).lookahead == '(' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0)
            {
                break;
            }
            a = jsP_newnode(
                J,
                EXP_CALL,
                line,
                a,
                arguments(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(')' as i32),
                );
            }
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn postfix(J: *mut js_State) -> *mut js_Ast {
    let a: *mut js_Ast = callexp(J);
    let line: libc::c_int = (*J).lexline;
    if (*J).newline == 0
        && (if (*J).lookahead == TK_INC as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
    {
        return jsP_newnode(
            J,
            EXP_POSTINC,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    if (*J).newline == 0
        && (if (*J).lookahead == TK_DEC as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }) != 0
    {
        return jsP_newnode(
            J,
            EXP_POSTDEC,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    a
}
unsafe extern "C" fn unary(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = (*J).lexline;
    (*J).astdepth += 1;
    if (*J).astdepth > 400 as libc::c_int {
        jsP_error(
            J,
            b"too much recursion\0" as *const u8 as *const libc::c_char,
        );
    }
    if if (*J).lookahead == TK_DELETE as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_DELETE,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_VOID as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_VOID,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_TYPEOF as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_TYPEOF,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_INC as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_PREINC,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_DEC as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_PREDEC,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '+' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_POS,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '-' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_NEG,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '~' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_BITNOT,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '!' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_LOGNOT,
            line,
            unary(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else {
        a = postfix(J);
    }
    (*J).astdepth -= 1;
    (*J).astdepth;
    a
}
unsafe extern "C" fn multiplicative(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = unary(J);
    let mut line: libc::c_int = 0;
    let SAVE: libc::c_int = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '*' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_MUL,
                line,
                a,
                unary(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == '/' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_DIV,
                line,
                a,
                unary(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == '%' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0)
            {
                break;
            }
            a = jsP_newnode(
                J,
                EXP_MOD,
                line,
                a,
                unary(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn additive(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = multiplicative(J);
    let mut line: libc::c_int = 0;
    let SAVE: libc::c_int = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '+' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_ADD,
                line,
                a,
                multiplicative(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == '-' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0)
            {
                break;
            }
            a = jsP_newnode(
                J,
                EXP_SUB,
                line,
                a,
                multiplicative(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn shift(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = additive(J);
    let mut line: libc::c_int = 0;
    let SAVE: libc::c_int = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == TK_SHL as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_SHL,
                line,
                a,
                additive(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_SHR as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_SHR,
                line,
                a,
                additive(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == TK_USHR as libc::c_int {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0)
            {
                break;
            }
            a = jsP_newnode(
                J,
                EXP_USHR,
                line,
                a,
                additive(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn relational(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = shift(J);
    let mut line: libc::c_int = 0;
    let SAVE: libc::c_int = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '<' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_LT,
                line,
                a,
                shift(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == '>' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_GT,
                line,
                a,
                shift(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_LE as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_LE,
                line,
                a,
                shift(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_GE as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_GE,
                line,
                a,
                shift(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_INSTANCEOF as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_INSTANCEOF,
                line,
                a,
                shift(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if !(notin == 0
                && (if (*J).lookahead == TK_IN as libc::c_int {
                    jsP_next(J);
                    1 as libc::c_int
                } else {
                    0 as libc::c_int
                }) != 0)
            {
                break;
            }
            a = jsP_newnode(
                J,
                EXP_IN,
                line,
                a,
                shift(J),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn equality(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = relational(J, notin);
    let mut line: libc::c_int = 0;
    let SAVE: libc::c_int = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == TK_EQ as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_EQ,
                line,
                a,
                relational(J, notin),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_NE as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_NE,
                line,
                a,
                relational(J, notin),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_STRICTEQ as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            a = jsP_newnode(
                J,
                EXP_STRICTEQ,
                line,
                a,
                relational(J, notin),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == TK_STRICTNE as libc::c_int {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0)
            {
                break;
            }
            a = jsP_newnode(
                J,
                EXP_STRICTNE,
                line,
                a,
                relational(J, notin),
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn bitand(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = equality(J, notin);
    let SAVE: libc::c_int = (*J).astdepth;
    let mut line: libc::c_int = (*J).lexline;
    while if (*J).lookahead == '&' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        a = jsP_newnode(
            J,
            EXP_BITAND,
            line,
            a,
            equality(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn bitxor(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = bitand(J, notin);
    let SAVE: libc::c_int = (*J).astdepth;
    let mut line: libc::c_int = (*J).lexline;
    while if (*J).lookahead == '^' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        a = jsP_newnode(
            J,
            EXP_BITXOR,
            line,
            a,
            bitand(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn bitor(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = bitxor(J, notin);
    let SAVE: libc::c_int = (*J).astdepth;
    let mut line: libc::c_int = (*J).lexline;
    while if (*J).lookahead == '|' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        a = jsP_newnode(
            J,
            EXP_BITOR,
            line,
            a,
            bitxor(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn logand(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = bitor(J, notin);
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == TK_AND as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        a = jsP_newnode(
            J,
            EXP_LOGAND,
            line,
            a,
            logand(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        (*J).astdepth -= 1;
        (*J).astdepth;
    }
    a
}
unsafe extern "C" fn logor(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = logand(J, notin);
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == TK_OR as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        a = jsP_newnode(
            J,
            EXP_LOGOR,
            line,
            a,
            logor(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        (*J).astdepth -= 1;
        (*J).astdepth;
    }
    a
}
unsafe extern "C" fn conditional(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let a: *mut js_Ast = logor(J, notin);
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == '?' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
        let mut c: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        b = assignment(J, 0 as libc::c_int);
        if if (*J).lookahead == ':' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(':' as i32),
            );
        }
        c = assignment(J, notin);
        (*J).astdepth -= 1;
        (*J).astdepth;
        return jsP_newnode(J, EXP_COND, line, a, b, c, std::ptr::null_mut::<js_Ast>());
    }
    a
}
unsafe extern "C" fn assignment(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = conditional(J, notin);
    let line: libc::c_int = (*J).lexline;
    (*J).astdepth += 1;
    if (*J).astdepth > 400 as libc::c_int {
        jsP_error(
            J,
            b"too much recursion\0" as *const u8 as *const libc::c_char,
        );
    }
    if if (*J).lookahead == '=' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_MUL_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_MUL,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_DIV_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_DIV,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_MOD_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_MOD,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_ADD_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_ADD,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SUB_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_SUB,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SHL_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_SHL,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SHR_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_SHR,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_USHR_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_USHR,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_AND_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_BITAND,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_XOR_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_BITXOR,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_OR_ASS as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = jsP_newnode(
            J,
            EXP_ASS_BITOR,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    (*J).astdepth -= 1;
    (*J).astdepth;
    a
}
unsafe extern "C" fn expression(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = assignment(J, notin);
    let SAVE: libc::c_int = (*J).astdepth;
    let mut line: libc::c_int = (*J).lexline;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 as libc::c_int {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        a = jsP_newnode(
            J,
            EXP_COMMA,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn vardec(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let a: *mut js_Ast = identifier(J);
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == '=' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_VAR,
            line,
            a,
            assignment(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    jsP_newnode(
        J,
        EXP_VAR,
        line,
        a,
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn vardeclist(J: *mut js_State, notin: libc::c_int) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        vardec(J, notin),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            vardec(J, notin),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn statementlist(J: *mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == '}' as i32
        || (*J).lookahead == TK_CASE as libc::c_int
        || (*J).lookahead == TK_DEFAULT as libc::c_int
    {
        return std::ptr::null_mut::<js_Ast>();
    }
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        statement(J),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while (*J).lookahead != '}' as i32
        && (*J).lookahead != TK_CASE as libc::c_int
        && (*J).lookahead != TK_DEFAULT as libc::c_int
    {
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            statement(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn caseclause(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == TK_CASE as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ':' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(':' as i32),
            );
        }
        b = statementlist(J);
        return jsP_newnode(
            J,
            STM_CASE,
            line,
            a,
            b,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_DEFAULT as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if if (*J).lookahead == ':' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(':' as i32),
            );
        }
        a = statementlist(J);
        return jsP_newnode(
            J,
            STM_DEFAULT,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    }
    jsP_error(
        J,
        b"unexpected token in switch: %s (expected 'case' or 'default')\0" as *const u8
            as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn caselist(J: *mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == '}' as i32 {
        return std::ptr::null_mut::<js_Ast>();
    }
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        caseclause(J),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while (*J).lookahead != '}' as i32 {
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            caseclause(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn block(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == '{' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('{' as i32),
        );
    }
    a = statementlist(J);
    if if (*J).lookahead == '}' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('}' as i32),
        );
    }
    jsP_newnode(
        J,
        STM_BLOCK,
        line,
        a,
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn forexpression(J: *mut js_State, end: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead != end {
        a = expression(J, 0 as libc::c_int);
    }
    if if (*J).lookahead == end {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(end),
        );
    }
    a
}
unsafe extern "C" fn forstatement(J: *mut js_State, line: libc::c_int) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut d: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('(' as i32),
        );
    }
    if if (*J).lookahead == TK_VAR as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = vardeclist(J, 1 as libc::c_int);
        if if (*J).lookahead == ';' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            b = forexpression(J, ';' as i32);
            c = forexpression(J, ')' as i32);
            d = statement(J);
            return jsP_newnode(J, STM_FOR_VAR, line, a, b, c, d);
        }
        if if (*J).lookahead == TK_IN as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            b = expression(J, 0 as libc::c_int);
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(')' as i32),
                );
            }
            c = statement(J);
            return jsP_newnode(
                J,
                STM_FOR_IN_VAR,
                line,
                a,
                b,
                c,
                std::ptr::null_mut::<js_Ast>(),
            );
        }
        jsP_error(
            J,
            b"unexpected token in for-var-statement: %s\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
        );
    }
    if (*J).lookahead != ';' as i32 {
        a = expression(J, 1 as libc::c_int);
    } else {
        a = std::ptr::null_mut::<js_Ast>();
    }
    if if (*J).lookahead == ';' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        b = forexpression(J, ';' as i32);
        c = forexpression(J, ')' as i32);
        d = statement(J);
        return jsP_newnode(J, STM_FOR, line, a, b, c, d);
    }
    if if (*J).lookahead == TK_IN as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        b = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(')' as i32),
            );
        }
        c = statement(J);
        return jsP_newnode(J, STM_FOR_IN, line, a, b, c, std::ptr::null_mut::<js_Ast>());
    }
    jsP_error(
        J,
        b"unexpected token in for-statement: %s\0" as *const u8 as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn statement(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut d: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut stm: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = (*J).lexline;
    (*J).astdepth += 1;
    if (*J).astdepth > 400 as libc::c_int {
        jsP_error(
            J,
            b"too much recursion\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*J).lookahead == '{' as i32 {
        stm = block(J);
    } else if if (*J).lookahead == TK_VAR as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = vardeclist(J, 0 as libc::c_int);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_VAR,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == ';' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        stm = jsP_newnode(
            J,
            STM_EMPTY,
            line,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_IF as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(')' as i32),
            );
        }
        b = statement(J);
        if if (*J).lookahead == TK_ELSE as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            c = statement(J);
        } else {
            c = std::ptr::null_mut::<js_Ast>();
        }
        stm = jsP_newnode(J, STM_IF, line, a, b, c, std::ptr::null_mut::<js_Ast>());
    } else if if (*J).lookahead == TK_DO as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = statement(J);
        if if (*J).lookahead == TK_WHILE as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(TK_WHILE as libc::c_int),
            );
        }
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        b = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(')' as i32),
            );
        }
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_DO,
            line,
            a,
            b,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_WHILE as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(')' as i32),
            );
        }
        b = statement(J);
        stm = jsP_newnode(
            J,
            STM_WHILE,
            line,
            a,
            b,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_FOR as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        stm = forstatement(J, line);
    } else if if (*J).lookahead == TK_CONTINUE as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = identifieropt(J);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_CONTINUE,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_BREAK as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = identifieropt(J);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_BREAK,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_RETURN as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if (*J).lookahead != ';' as i32
            && (*J).lookahead != '}' as i32
            && (*J).lookahead != 0 as libc::c_int
        {
            a = expression(J, 0 as libc::c_int);
        } else {
            a = std::ptr::null_mut::<js_Ast>();
        }
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_RETURN,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_WITH as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(')' as i32),
            );
        }
        b = statement(J);
        stm = jsP_newnode(
            J,
            STM_WITH,
            line,
            a,
            b,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SWITCH as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0 as libc::c_int);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(')' as i32),
            );
        }
        if if (*J).lookahead == '{' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('{' as i32),
            );
        }
        b = caselist(J);
        if if (*J).lookahead == '}' as i32 {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('}' as i32),
            );
        }
        stm = jsP_newnode(
            J,
            STM_SWITCH,
            line,
            a,
            b,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_THROW as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = expression(J, 0 as libc::c_int);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_THROW,
            line,
            a,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_TRY as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        a = block(J);
        d = std::ptr::null_mut::<js_Ast>();
        c = d;
        b = c;
        if if (*J).lookahead == TK_CATCH as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            if if (*J).lookahead == '(' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring('(' as i32),
                );
            }
            b = identifier(J);
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            } == 0
            {
                jsP_error(
                    J,
                    b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                    jsY_tokenstring((*J).lookahead),
                    jsY_tokenstring(')' as i32),
                );
            }
            c = block(J);
        }
        if if (*J).lookahead == TK_FINALLY as libc::c_int {
            jsP_next(J);
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0
        {
            d = block(J);
        }
        if b.is_null() && d.is_null() {
            jsP_error(
                J,
                b"unexpected token in try: %s (expected 'catch' or 'finally')\0" as *const u8
                    as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
            );
        }
        stm = jsP_newnode(J, STM_TRY, line, a, b, c, d);
    } else if if (*J).lookahead == TK_DEBUGGER as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_DEBUGGER,
            line,
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_FUNCTION as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        jsP_warning(
            J,
            b"function statements are not standard\0" as *const u8 as *const libc::c_char,
        );
        stm = funstm(J, line);
    } else if (*J).lookahead == TK_IDENTIFIER as libc::c_int {
        a = expression(J, 0 as libc::c_int);
        if (*a).type_0 as libc::c_uint == EXP_IDENTIFIER as libc::c_int as libc::c_uint
            && (if (*J).lookahead == ':' as i32 {
                jsP_next(J);
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) != 0
        {
            (*a).type_0 = AST_IDENTIFIER;
            b = statement(J);
            stm = jsP_newnode(
                J,
                STM_LABEL,
                line,
                a,
                b,
                std::ptr::null_mut::<js_Ast>(),
                std::ptr::null_mut::<js_Ast>(),
            );
        } else {
            semicolon(J);
            stm = a;
        }
    } else {
        stm = expression(J, 0 as libc::c_int);
        semicolon(J);
    }
    (*J).astdepth -= 1;
    (*J).astdepth;
    stm
}
unsafe extern "C" fn scriptelement(J: *mut js_State) -> *mut js_Ast {
    let line: libc::c_int = (*J).lexline;
    if if (*J).lookahead == TK_FUNCTION as libc::c_int {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0
    {
        return fundec(J, line);
    }
    statement(J)
}
unsafe extern "C" fn script(J: *mut js_State, terminator: libc::c_int) -> *mut js_Ast {
    let mut head: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == terminator {
        return std::ptr::null_mut::<js_Ast>();
    }
    tail = jsP_newnode(
        J,
        AST_LIST,
        0 as libc::c_int,
        scriptelement(J),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
        std::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while (*J).lookahead != terminator {
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0 as libc::c_int,
            scriptelement(J),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
            std::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn funbody(J: *mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    if if (*J).lookahead == '{' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('{' as i32),
        );
    }
    a = script(J, '}' as i32);
    if if (*J).lookahead == '}' as i32 {
        jsP_next(J);
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('}' as i32),
        );
    }
    a
}
unsafe extern "C" fn toint32(mut d: libc::c_double) -> libc::c_int {
    let two32: libc::c_double = 4294967296.0f64;
    let two31: libc::c_double = 2147483648.0f64;
    if d.is_finite() as i32 == 0 || d == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int;
    }
    d = fmod(d, two32);
    d = if d >= 0 as libc::c_int as libc::c_double {
        floor(d)
    } else {
        ceil(d) + two32
    };
    if d >= two31 {
        (d - two32) as libc::c_int
    } else {
        d as libc::c_int
    }
}
unsafe extern "C" fn touint32(d: libc::c_double) -> libc::c_uint {
    toint32(d) as libc::c_uint
}
unsafe extern "C" fn jsP_setnumnode(node: *mut js_Ast, x: libc::c_double) -> libc::c_int {
    (*node).type_0 = EXP_NUMBER;
    (*node).number = x;
    (*node).d = std::ptr::null_mut::<js_Ast>();
    (*node).c = (*node).d;
    (*node).b = (*node).c;
    (*node).a = (*node).b;
    1 as libc::c_int
}
unsafe extern "C" fn jsP_foldconst(mut node: *mut js_Ast) -> libc::c_int {
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    if (*node).type_0 as libc::c_uint == AST_LIST as libc::c_int as libc::c_uint {
        while !node.is_null() {
            jsP_foldconst((*node).a);
            node = (*node).b;
        }
        return 0 as libc::c_int;
    }
    if (*node).type_0 as libc::c_uint == EXP_NUMBER as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    a = if !((*node).a).is_null() {
        jsP_foldconst((*node).a)
    } else {
        0 as libc::c_int
    };
    b = if !((*node).b).is_null() {
        jsP_foldconst((*node).b)
    } else {
        0 as libc::c_int
    };
    if !((*node).c).is_null() {
        jsP_foldconst((*node).c);
    }
    if !((*node).d).is_null() {
        jsP_foldconst((*node).d);
    }
    if a != 0 {
        x = (*(*node).a).number;
        match (*node).type_0 as libc::c_uint {
            30 => return jsP_setnumnode(node, -x),
            29 => return jsP_setnumnode(node, x),
            31 => return jsP_setnumnode(node, !toint32(x) as libc::c_double),
            _ => {}
        }
        if b != 0 {
            y = (*(*node).b).number;
            match (*node).type_0 as libc::c_uint {
                35 => return jsP_setnumnode(node, x * y),
                34 => return jsP_setnumnode(node, x / y),
                33 => return jsP_setnumnode(node, fmod(x, y)),
                37 => return jsP_setnumnode(node, x + y),
                36 => return jsP_setnumnode(node, x - y),
                40 => {
                    return jsP_setnumnode(
                        node,
                        (toint32(x) << (touint32(y) & 0x1f as libc::c_int as libc::c_uint))
                            as libc::c_double,
                    );
                }
                39 => {
                    return jsP_setnumnode(
                        node,
                        (toint32(x) >> (touint32(y) & 0x1f as libc::c_int as libc::c_uint))
                            as libc::c_double,
                    );
                }
                38 => {
                    return jsP_setnumnode(
                        node,
                        (touint32(x) >> (touint32(y) & 0x1f as libc::c_int as libc::c_uint))
                            as libc::c_double,
                    );
                }
                51 => {
                    return jsP_setnumnode(node, (toint32(x) & toint32(y)) as libc::c_double);
                }
                52 => {
                    return jsP_setnumnode(node, (toint32(x) ^ toint32(y)) as libc::c_double);
                }
                53 => {
                    return jsP_setnumnode(node, (toint32(x) | toint32(y)) as libc::c_double);
                }
                _ => {}
            }
        }
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsP_parse(
    J: *mut js_State,
    filename: *const libc::c_char,
    source: *const libc::c_char,
) -> *mut js_Ast {
    let mut p: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    jsY_initlex(J, filename, source);
    jsP_next(J);
    (*J).astdepth = 0 as libc::c_int;
    p = script(J, 0 as libc::c_int);
    if !p.is_null() {
        jsP_foldconst(p);
    }
    p
}
#[no_mangle]
pub unsafe extern "C" fn jsP_parsefunction(
    J: *mut js_State,
    filename: *const libc::c_char,
    params: *const libc::c_char,
    body: *const libc::c_char,
) -> *mut js_Ast {
    let mut p: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let line: libc::c_int = 0 as libc::c_int;
    if !params.is_null() {
        jsY_initlex(J, filename, params);
        jsP_next(J);
        (*J).astdepth = 0 as libc::c_int;
        p = parameters(J);
    }
    jsP_newnode(
        J,
        EXP_FUN,
        line,
        std::ptr::null_mut::<js_Ast>(),
        p,
        jsP_parse(J, filename, body),
        std::ptr::null_mut::<js_Ast>(),
    )
}
static mut sentinel: js_Property = unsafe {
    {
        js_Property {
            left: &sentinel as *const js_Property as *mut js_Property,
            right: &sentinel as *const js_Property as *mut js_Property,
            level: 0 as libc::c_int,
            atts: 0 as libc::c_int,
            value: js_Value {
                t: {
                    C2RustUnnamed_6 {
                        pad: [
                            0 as libc::c_int as libc::c_char,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                            0,
                        ],
                        type_0: JS_TUNDEFINED as libc::c_int as libc::c_char,
                    }
                },
            },
            getter: 0 as *const js_Object as *mut js_Object,
            setter: 0 as *const js_Object as *mut js_Object,
            name: *::core::mem::transmute::<&[u8; 1], &mut [libc::c_char; 1]>(b"\0"),
        }
    }
};
unsafe extern "C" fn newproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
) -> *mut js_Property {
    let n: libc::c_int =
        (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let node: *mut js_Property = js_malloc(
        J,
        (56 as libc::c_ulong).wrapping_add(n as libc::c_ulong) as libc::c_int,
    ) as *mut js_Property;
    (*node).right = &mut sentinel;
    (*node).left = (*node).right;
    (*node).level = 1 as libc::c_int;
    (*node).atts = 0 as libc::c_int;
    (*node).value.t.type_0 = JS_TUNDEFINED as libc::c_int as libc::c_char;
    (*node).value.u.number = 0 as libc::c_int as libc::c_double;
    (*node).getter = std::ptr::null_mut::<js_Object>();
    (*node).setter = std::ptr::null_mut::<js_Object>();
    memcpy(
        ((*node).name).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        n as libc::c_ulong,
    );
    (*obj).count += 1;
    (*obj).count;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    node
}
unsafe extern "C" fn lookup(
    mut node: *mut js_Property,
    name: *const libc::c_char,
) -> *mut js_Property {
    while node != &mut sentinel as *mut js_Property {
        let c: libc::c_int = strcmp(name, ((*node).name).as_mut_ptr());
        if c == 0 as libc::c_int {
            return node;
        } else if c < 0 as libc::c_int {
            node = (*node).left;
        } else {
            node = (*node).right;
        }
    }
    std::ptr::null_mut::<js_Property>()
}
unsafe extern "C" fn skew(mut node: *mut js_Property) -> *mut js_Property {
    if (*(*node).left).level == (*node).level {
        let temp: *mut js_Property = node;
        node = (*node).left;
        (*temp).left = (*node).right;
        (*node).right = temp;
    }
    node
}
unsafe extern "C" fn split(mut node: *mut js_Property) -> *mut js_Property {
    if (*(*(*node).right).right).level == (*node).level {
        let temp: *mut js_Property = node;
        node = (*node).right;
        (*temp).right = (*node).left;
        (*node).left = temp;
        (*node).level += 1;
        (*node).level;
    }
    node
}
unsafe extern "C" fn insert(
    J: *mut js_State,
    obj: *mut js_Object,
    mut node: *mut js_Property,
    name: *const libc::c_char,
    result: *mut *mut js_Property,
) -> *mut js_Property {
    if node != &mut sentinel as *mut js_Property {
        let c: libc::c_int = strcmp(name, ((*node).name).as_mut_ptr());
        if c < 0 as libc::c_int {
            (*node).left = insert(J, obj, (*node).left, name, result);
        } else if c > 0 as libc::c_int {
            (*node).right = insert(J, obj, (*node).right, name, result);
        } else {
            *result = node;
            return *result;
        }
        node = skew(node);
        node = split(node);
        return node;
    }
    *result = newproperty(J, obj, name);
    *result
}
unsafe extern "C" fn freeproperty(J: *mut js_State, obj: *mut js_Object, node: *mut js_Property) {
    js_free(J, node as *mut libc::c_void);
    (*obj).count -= 1;
    (*obj).count;
}
unsafe extern "C" fn unlinkproperty(
    mut node: *mut js_Property,
    name: *const libc::c_char,
    garbage: *mut *mut js_Property,
) -> *mut js_Property {
    let mut temp: *mut js_Property = std::ptr::null_mut::<js_Property>();
    let mut a: *mut js_Property = std::ptr::null_mut::<js_Property>();
    let mut b: *mut js_Property = std::ptr::null_mut::<js_Property>();
    if node != &mut sentinel as *mut js_Property {
        let c: libc::c_int = strcmp(name, ((*node).name).as_mut_ptr());
        if c < 0 as libc::c_int {
            (*node).left = unlinkproperty((*node).left, name, garbage);
        } else if c > 0 as libc::c_int {
            (*node).right = unlinkproperty((*node).right, name, garbage);
        } else {
            *garbage = node;
            if (*node).left == &mut sentinel as *mut js_Property
                && (*node).right == &mut sentinel as *mut js_Property
            {
                return &mut sentinel;
            } else if (*node).left == &mut sentinel as *mut js_Property {
                a = (*node).right;
                while (*a).left != &mut sentinel as *mut js_Property {
                    a = (*a).left;
                }
                b = unlinkproperty((*node).right, ((*a).name).as_mut_ptr(), &mut temp);
                (*temp).level = (*node).level;
                (*temp).left = (*node).left;
                (*temp).right = b;
                node = temp;
            } else {
                a = (*node).left;
                while (*a).right != &mut sentinel as *mut js_Property {
                    a = (*a).right;
                }
                b = unlinkproperty((*node).left, ((*a).name).as_mut_ptr(), &mut temp);
                (*temp).level = (*node).level;
                (*temp).left = b;
                (*temp).right = (*node).right;
                node = temp;
            }
        }
        if (*(*node).left).level < (*node).level - 1 as libc::c_int
            || (*(*node).right).level < (*node).level - 1 as libc::c_int
        {
            (*node).level -= 1;
            if (*(*node).right).level > (*node).level {
                (*(*node).right).level = (*node).level;
            }
            node = skew(node);
            (*node).right = skew((*node).right);
            (*(*node).right).right = skew((*(*node).right).right);
            node = split(node);
            (*node).right = split((*node).right);
        }
    }
    node
}
unsafe extern "C" fn deleteproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    mut tree: *mut js_Property,
    name: *const libc::c_char,
) -> *mut js_Property {
    let mut garbage: *mut js_Property = &mut sentinel;
    tree = unlinkproperty(tree, name, &mut garbage);
    if garbage != &mut sentinel as *mut js_Property {
        freeproperty(J, obj, garbage);
    }
    tree
}
#[no_mangle]
pub unsafe extern "C" fn jsV_newobject(
    J: *mut js_State,
    type_0: js_Class,
    prototype: *mut js_Object,
) -> *mut js_Object {
    let obj: *mut js_Object = js_malloc(
        J,
        ::core::mem::size_of::<js_Object>() as libc::c_ulong as libc::c_int,
    ) as *mut js_Object;
    memset(
        obj as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<js_Object>() as libc::c_ulong,
    );
    (*obj).gcmark = 0 as libc::c_int;
    (*obj).gcnext = (*J).gcobj;
    (*J).gcobj = obj;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    (*obj).type_0 = type_0;
    (*obj).properties = &mut sentinel;
    (*obj).prototype = prototype;
    (*obj).extensible = 1 as libc::c_int;
    obj
}
#[no_mangle]
pub unsafe extern "C" fn jsV_getownproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
) -> *mut js_Property {
    lookup((*obj).properties, name)
}
#[no_mangle]
pub unsafe extern "C" fn jsV_getpropertyx(
    J: *mut js_State,
    mut obj: *mut js_Object,
    name: *const libc::c_char,
    own: *mut libc::c_int,
) -> *mut js_Property {
    *own = 1 as libc::c_int;
    loop {
        let ref_0: *mut js_Property = lookup((*obj).properties, name);
        if !ref_0.is_null() {
            return ref_0;
        }
        obj = (*obj).prototype;
        *own = 0 as libc::c_int;
        if obj.is_null() {
            break;
        }
    }
    std::ptr::null_mut::<js_Property>()
}
#[no_mangle]
pub unsafe extern "C" fn jsV_getproperty(
    J: *mut js_State,
    mut obj: *mut js_Object,
    name: *const libc::c_char,
) -> *mut js_Property {
    loop {
        let ref_0: *mut js_Property = lookup((*obj).properties, name);
        if !ref_0.is_null() {
            return ref_0;
        }
        obj = (*obj).prototype;
        if obj.is_null() {
            break;
        }
    }
    std::ptr::null_mut::<js_Property>()
}
unsafe extern "C" fn jsV_getenumproperty(
    J: *mut js_State,
    mut obj: *mut js_Object,
    name: *const libc::c_char,
) -> *mut js_Property {
    loop {
        let ref_0: *mut js_Property = lookup((*obj).properties, name);
        if !ref_0.is_null() && (*ref_0).atts & JS_DONTENUM as libc::c_int == 0 {
            return ref_0;
        }
        obj = (*obj).prototype;
        if obj.is_null() {
            break;
        }
    }
    std::ptr::null_mut::<js_Property>()
}
#[no_mangle]
pub unsafe extern "C" fn jsV_setproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
) -> *mut js_Property {
    let mut result: *mut js_Property = std::ptr::null_mut::<js_Property>();
    if (*obj).extensible == 0 {
        result = lookup((*obj).properties, name);
        if (*J).strict != 0 && result.is_null() {
            js_typeerror(
                J,
                b"object is non-extensible\0" as *const u8 as *const libc::c_char,
            );
        }
        return result;
    }
    (*obj).properties = insert(J, obj, (*obj).properties, name, &mut result);
    result
}
#[no_mangle]
pub unsafe extern "C" fn jsV_delproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
) {
    (*obj).properties = deleteproperty(J, obj, (*obj).properties, name);
}
unsafe extern "C" fn itnewnode(
    J: *mut js_State,
    name: *const libc::c_char,
    next_0: *mut js_Iterator,
) -> *mut js_Iterator {
    let n: libc::c_int =
        (strlen(name)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let node: *mut js_Iterator = js_malloc(
        J,
        (8 as libc::c_ulong).wrapping_add(n as libc::c_ulong) as libc::c_int,
    ) as *mut js_Iterator;
    (*node).next = next_0;
    memcpy(
        ((*node).name).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        n as libc::c_ulong,
    );
    node
}
unsafe extern "C" fn itwalk(
    J: *mut js_State,
    mut iter: *mut js_Iterator,
    prop: *mut js_Property,
    seen: *mut js_Object,
) -> *mut js_Iterator {
    if (*prop).right != &mut sentinel as *mut js_Property {
        iter = itwalk(J, iter, (*prop).right, seen);
    }
    if (*prop).atts & JS_DONTENUM as libc::c_int == 0
        && (seen.is_null() || (jsV_getenumproperty(J, seen, ((*prop).name).as_mut_ptr())).is_null())
    {
        iter = itnewnode(J, ((*prop).name).as_mut_ptr(), iter);
    }
    if (*prop).left != &mut sentinel as *mut js_Property {
        iter = itwalk(J, iter, (*prop).left, seen);
    }
    iter
}
unsafe extern "C" fn itflatten(J: *mut js_State, obj: *mut js_Object) -> *mut js_Iterator {
    let mut iter: *mut js_Iterator = std::ptr::null_mut::<js_Iterator>();
    if !((*obj).prototype).is_null() {
        iter = itflatten(J, (*obj).prototype);
    }
    if (*obj).properties != &mut sentinel as *mut js_Property {
        iter = itwalk(J, iter, (*obj).properties, (*obj).prototype);
    }
    iter
}
#[no_mangle]
pub unsafe extern "C" fn jsV_newiterator(
    J: *mut js_State,
    obj: *mut js_Object,
    own: libc::c_int,
) -> *mut js_Object {
    let io: *mut js_Object = jsV_newobject(J, JS_CITERATOR, std::ptr::null_mut::<js_Object>());
    (*io).u.iter.target = obj;
    (*io).u.iter.i = 0 as libc::c_int;
    (*io).u.iter.n = 0 as libc::c_int;
    if own != 0 {
        (*io).u.iter.head = std::ptr::null_mut::<js_Iterator>();
        if (*obj).properties != &mut sentinel as *mut js_Property {
            (*io).u.iter.head = itwalk(
                J,
                (*io).u.iter.head,
                (*obj).properties,
                std::ptr::null_mut::<js_Object>(),
            );
        }
    } else {
        (*io).u.iter.head = itflatten(J, obj);
    }
    (*io).u.iter.current = (*io).u.iter.head;
    if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint {
        (*io).u.iter.n = (*obj).u.s.length;
    }
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        (*io).u.iter.n = (*obj).u.a.flat_length;
    }
    io
}
#[no_mangle]
pub unsafe extern "C" fn jsV_nextiterator(
    J: *mut js_State,
    io: *mut js_Object,
) -> *const libc::c_char {
    if (*io).type_0 as libc::c_uint != JS_CITERATOR as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not an iterator\0" as *const u8 as *const libc::c_char);
    }
    if (*io).u.iter.i < (*io).u.iter.n {
        js_itoa(((*J).scratch).as_mut_ptr(), (*io).u.iter.i);
        (*io).u.iter.i += 1;
        (*io).u.iter.i;
        return ((*J).scratch).as_mut_ptr();
    }
    while !((*io).u.iter.current).is_null() {
        let name: *const libc::c_char = ((*(*io).u.iter.current).name).as_mut_ptr();
        (*io).u.iter.current = (*(*io).u.iter.current).next;
        if !(jsV_getproperty(J, (*io).u.iter.target, name)).is_null() {
            return name;
        }
    }
    std::ptr::null::<libc::c_char>()
}
#[no_mangle]
pub unsafe extern "C" fn jsV_resizearray(
    J: *mut js_State,
    obj: *mut js_Object,
    newlen: libc::c_int,
) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut k: libc::c_int = 0;
    if (*obj).u.a.simple == 0 {
    } else {
        __assert_fail(
            b"!obj->u.a.simple\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            9836 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void jsV_resizearray(js_State *, js_Object *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_56796: {
        if (*obj).u.a.simple == 0 {
        } else {
            __assert_fail(
                b"!obj->u.a.simple\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                9836 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"void jsV_resizearray(js_State *, js_Object *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if newlen < (*obj).u.a.length {
        if (*obj).u.a.length > (*obj).count * 2 as libc::c_int {
            let it: *mut js_Object = jsV_newiterator(J, obj, 1 as libc::c_int);
            loop {
                s = jsV_nextiterator(J, it);
                if s.is_null() {
                    break;
                }
                k = jsV_numbertointeger(jsV_stringtonumber(J, s));
                if k >= newlen
                    && strcmp(
                        s,
                        jsV_numbertostring(J, buf.as_mut_ptr(), k as libc::c_double),
                    ) == 0
                {
                    jsV_delproperty(J, obj, s);
                }
            }
        } else {
            k = newlen;
            while k < (*obj).u.a.length {
                jsV_delproperty(J, obj, js_itoa(buf.as_mut_ptr(), k));
                k += 1;
                k;
            }
        }
    }
    (*obj).u.a.length = newlen;
}
unsafe extern "C" fn escaperegexp(
    J: *mut js_State,
    pattern: *const libc::c_char,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut n: libc::c_int = 0 as libc::c_int;
    s = pattern;
    while *s != 0 {
        if *s as libc::c_int == '/' as i32 {
            n += 1;
            n;
        }
        n += 1;
        n;
        s = s.offset(1);
        s;
    }
    p = js_malloc(J, n + 1 as libc::c_int) as *mut libc::c_char;
    copy = p;
    s = pattern;
    while *s != 0 {
        if *s as libc::c_int == '/' as i32 {
            let fresh50 = p;
            p = p.offset(1);
            *fresh50 = '\\' as i32 as libc::c_char;
        }
        let fresh51 = p;
        p = p.offset(1);
        *fresh51 = *s;
        s = s.offset(1);
        s;
    }
    *p = 0 as libc::c_int as libc::c_char;
    copy
}
unsafe extern "C" fn js_newregexpx(
    J: *mut js_State,
    pattern: *const libc::c_char,
    flags: libc::c_int,
    is_clone: libc::c_int,
) {
    let mut error: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut prog: *mut Reprog = std::ptr::null_mut::<Reprog>();
    let mut opts: libc::c_int = 0;
    obj = jsV_newobject(J, JS_CREGEXP, (*J).RegExp_prototype);
    opts = 0 as libc::c_int;
    if flags & JS_REGEXP_I as libc::c_int != 0 {
        opts |= REG_ICASE as libc::c_int;
    }
    if flags & JS_REGEXP_M as libc::c_int != 0 {
        opts |= REG_NEWLINE as libc::c_int;
    }
    prog = js_regcompx((*J).alloc, (*J).actx, pattern, opts, &mut error);
    if prog.is_null() {
        js_syntaxerror(
            J,
            b"regular expression: %s\0" as *const u8 as *const libc::c_char,
            error,
        );
    }
    (*obj).u.r.prog = prog as *mut libc::c_void;
    (*obj).u.r.source = if is_clone != 0 {
        js_strdup(J, pattern)
    } else {
        escaperegexp(J, pattern)
    };
    (*obj).u.r.flags = flags as libc::c_ushort;
    (*obj).u.r.last = 0 as libc::c_int as libc::c_ushort;
    js_pushobject(J, obj);
}
#[no_mangle]
pub unsafe extern "C" fn js_newregexp(
    J: *mut js_State,
    pattern: *const libc::c_char,
    flags: libc::c_int,
) {
    js_newregexpx(J, pattern, flags, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_RegExp_prototype_exec(
    J: *mut js_State,
    re: *mut js_Regexp,
    text: *const libc::c_char,
) {
    let mut haystack: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut result: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut opts: libc::c_int = 0;
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    haystack = text;
    opts = 0 as libc::c_int;
    if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
        if (*re).last as libc::c_ulong > strlen(haystack) {
            (*re).last = 0 as libc::c_int as libc::c_ushort;
            js_pushnull(J);
            return;
        }
        if (*re).last as libc::c_int > 0 as libc::c_int {
            haystack = text.offset((*re).last as libc::c_int as isize);
            opts |= REG_NOTBOL as libc::c_int;
        }
    }
    result = js_regexec((*re).prog as *mut Reprog, haystack, &mut m, opts);
    if result < 0 as libc::c_int {
        js_error(J, b"regexec failed\0" as *const u8 as *const libc::c_char);
    }
    if result == 0 as libc::c_int {
        js_newarray(J);
        js_pushstring(J, text);
        js_setproperty(
            J,
            -(2 as libc::c_int),
            b"input\0" as *const u8 as *const libc::c_char,
        );
        js_pushnumber(
            J,
            js_utfptrtoidx(text, m.sub[0 as libc::c_int as usize].sp) as libc::c_double,
        );
        js_setproperty(
            J,
            -(2 as libc::c_int),
            b"index\0" as *const u8 as *const libc::c_char,
        );
        i = 0 as libc::c_int;
        while i < m.nsub {
            js_pushlstring(
                J,
                m.sub[i as usize].sp,
                (m.sub[i as usize].ep).offset_from(m.sub[i as usize].sp) as libc::c_long
                    as libc::c_int,
            );
            js_setindex(J, -(2 as libc::c_int), i);
            i += 1;
            i;
        }
        if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
            (*re).last = (m.sub[0 as libc::c_int as usize].ep).offset_from(text) as libc::c_long
                as libc::c_ushort;
        }
        return;
    }
    if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
        (*re).last = 0 as libc::c_int as libc::c_ushort;
    }
    js_pushnull(J);
}
unsafe extern "C" fn Rp_test(J: *mut js_State) {
    let mut re: *mut js_Regexp = std::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut result: libc::c_int = 0;
    let mut opts: libc::c_int = 0;
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    re = js_toregexp(J, 0 as libc::c_int);
    text = js_tostring(J, 1 as libc::c_int);
    opts = 0 as libc::c_int;
    if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
        if (*re).last as libc::c_ulong > strlen(text) {
            (*re).last = 0 as libc::c_int as libc::c_ushort;
            js_pushboolean(J, 0 as libc::c_int);
            return;
        }
        if (*re).last as libc::c_int > 0 as libc::c_int {
            text = text.offset((*re).last as libc::c_int as isize);
            opts |= REG_NOTBOL as libc::c_int;
        }
    }
    result = js_regexec((*re).prog as *mut Reprog, text, &mut m, opts);
    if result < 0 as libc::c_int {
        js_error(J, b"regexec failed\0" as *const u8 as *const libc::c_char);
    }
    if result == 0 as libc::c_int {
        if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
            (*re).last = ((*re).last as libc::c_long
                + (m.sub[0 as libc::c_int as usize].ep).offset_from(text) as libc::c_long)
                as libc::c_ushort;
        }
        js_pushboolean(J, 1 as libc::c_int);
        return;
    }
    if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
        (*re).last = 0 as libc::c_int as libc::c_ushort;
    }
    js_pushboolean(J, 0 as libc::c_int);
}
unsafe extern "C" fn jsB_new_RegExp(J: *mut js_State) {
    let mut old: *mut js_Regexp = std::ptr::null_mut::<js_Regexp>();
    let mut pattern: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut flags: libc::c_int = 0;
    let mut is_clone: libc::c_int = 0 as libc::c_int;
    if js_isregexp(J, 1 as libc::c_int) != 0 {
        if js_isdefined(J, 2 as libc::c_int) != 0 {
            js_typeerror(
                J,
                b"cannot supply flags when creating one RegExp from another\0" as *const u8
                    as *const libc::c_char,
            );
        }
        old = js_toregexp(J, 1 as libc::c_int);
        pattern = (*old).source;
        flags = (*old).flags as libc::c_int;
        is_clone = 1 as libc::c_int;
    } else if js_isundefined(J, 1 as libc::c_int) != 0 {
        pattern = b"(?:)\0" as *const u8 as *const libc::c_char;
        flags = 0 as libc::c_int;
    } else {
        pattern = js_tostring(J, 1 as libc::c_int);
        flags = 0 as libc::c_int;
    }
    if strlen(pattern) == 0 as libc::c_int as libc::c_ulong {
        pattern = b"(?:)\0" as *const u8 as *const libc::c_char;
    }
    if js_isdefined(J, 2 as libc::c_int) != 0 {
        let mut s: *const libc::c_char = js_tostring(J, 2 as libc::c_int);
        let mut g: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut m: libc::c_int = 0 as libc::c_int;
        while *s != 0 {
            if *s as libc::c_int == 'g' as i32 {
                g += 1;
                g;
            } else if *s as libc::c_int == 'i' as i32 {
                i += 1;
                i;
            } else if *s as libc::c_int == 'm' as i32 {
                m += 1;
                m;
            } else {
                js_syntaxerror(
                    J,
                    b"invalid regular expression flag: '%c'\0" as *const u8 as *const libc::c_char,
                    *s as libc::c_int,
                );
            }
            s = s.offset(1);
            s;
        }
        if g > 1 as libc::c_int {
            js_syntaxerror(
                J,
                b"invalid regular expression flag: 'g'\0" as *const u8 as *const libc::c_char,
            );
        }
        if i > 1 as libc::c_int {
            js_syntaxerror(
                J,
                b"invalid regular expression flag: 'i'\0" as *const u8 as *const libc::c_char,
            );
        }
        if m > 1 as libc::c_int {
            js_syntaxerror(
                J,
                b"invalid regular expression flag: 'm'\0" as *const u8 as *const libc::c_char,
            );
        }
        if g != 0 {
            flags |= JS_REGEXP_G as libc::c_int;
        }
        if i != 0 {
            flags |= JS_REGEXP_I as libc::c_int;
        }
        if m != 0 {
            flags |= JS_REGEXP_M as libc::c_int;
        }
    }
    js_newregexpx(J, pattern, flags, is_clone);
}
unsafe extern "C" fn jsB_RegExp(J: *mut js_State) {
    if js_isregexp(J, 1 as libc::c_int) != 0 {
        return;
    }
    jsB_new_RegExp(J);
}
unsafe extern "C" fn Rp_toString(J: *mut js_State) {
    let mut re: *mut js_Regexp = std::ptr::null_mut::<js_Regexp>();
    let mut out: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    re = js_toregexp(J, 0 as libc::c_int);
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, out as *mut libc::c_void);
        js_throw(J);
    }
    ::core::ptr::write_volatile(
        &mut out as *mut *mut libc::c_char,
        js_malloc(
            J,
            (strlen((*re).source)).wrapping_add(6 as libc::c_int as libc::c_ulong) as libc::c_int,
        ) as *mut libc::c_char,
    );
    strcpy(out, b"/\0" as *const u8 as *const libc::c_char);
    strcat(out, (*re).source);
    strcat(out, b"/\0" as *const u8 as *const libc::c_char);
    if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
        strcat(out, b"g\0" as *const u8 as *const libc::c_char);
    }
    if (*re).flags as libc::c_int & JS_REGEXP_I as libc::c_int != 0 {
        strcat(out, b"i\0" as *const u8 as *const libc::c_char);
    }
    if (*re).flags as libc::c_int & JS_REGEXP_M as libc::c_int != 0 {
        strcat(out, b"m\0" as *const u8 as *const libc::c_char);
    }
    js_pop(J, 0 as libc::c_int);
    js_pushstring(J, out);
    js_endtry(J);
    js_free(J, out as *mut libc::c_void);
}
unsafe extern "C" fn Rp_exec(J: *mut js_State) {
    js_RegExp_prototype_exec(
        J,
        js_toregexp(J, 0 as libc::c_int),
        js_tostring(J, 1 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initregexp(J: *mut js_State) {
    js_pushobject(J, (*J).RegExp_prototype);
    jsB_propf(
        J,
        b"RegExp.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Rp_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"RegExp.prototype.test\0" as *const u8 as *const libc::c_char,
        Some(Rp_test as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"RegExp.prototype.exec\0" as *const u8 as *const libc::c_char,
        Some(Rp_exec as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_RegExp as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_new_RegExp as unsafe extern "C" fn(*mut js_State) -> ()),
        b"RegExp\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    );
    js_defglobal(
        J,
        b"RegExp\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
unsafe extern "C" fn reprnum(J: *mut js_State, sb: *mut *mut js_Buffer, n: libc::c_double) {
    let mut buf: [libc::c_char; 40] = [0; 40];
    if n == 0 as libc::c_int as libc::c_double && n.is_sign_negative() as libc::c_int != 0 {
        js_puts(J, sb, b"-0\0" as *const u8 as *const libc::c_char);
    } else {
        js_puts(J, sb, jsV_numbertostring(J, buf.as_mut_ptr(), n));
    };
}
unsafe extern "C" fn reprstr(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    mut s: *const libc::c_char,
) {
    static mut HEX: *const libc::c_char = b"0123456789ABCDEF\0" as *const u8 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut c: Rune = 0;
    js_putc(J, sb, '"' as i32);
    while *s != 0 {
        n = jsU_chartorune(&mut c, s);
        match c {
            34 => {
                js_puts(J, sb, b"\\\"\0" as *const u8 as *const libc::c_char);
            }
            92 => {
                js_puts(J, sb, b"\\\\\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                js_puts(J, sb, b"\\b\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                js_puts(J, sb, b"\\f\0" as *const u8 as *const libc::c_char);
            }
            10 => {
                js_puts(J, sb, b"\\n\0" as *const u8 as *const libc::c_char);
            }
            13 => {
                js_puts(J, sb, b"\\r\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                js_puts(J, sb, b"\\t\0" as *const u8 as *const libc::c_char);
            }
            _ => {
                if c < ' ' as i32 {
                    js_putc(J, sb, '\\' as i32);
                    js_putc(J, sb, 'x' as i32);
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 4 as libc::c_int & 15 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c & 15 as libc::c_int) as isize) as libc::c_int,
                    );
                } else if c < 128 as libc::c_int {
                    js_putc(J, sb, c);
                } else if c < 0x10000 as libc::c_int {
                    js_putc(J, sb, '\\' as i32);
                    js_putc(J, sb, 'u' as i32);
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 12 as libc::c_int & 15 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 8 as libc::c_int & 15 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 4 as libc::c_int & 15 as libc::c_int) as isize)
                            as libc::c_int,
                    );
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c & 15 as libc::c_int) as isize) as libc::c_int,
                    );
                } else {
                    i = 0 as libc::c_int;
                    while i < n {
                        js_putc(J, sb, *s.offset(i as isize) as libc::c_int);
                        i += 1;
                        i;
                    }
                }
            }
        }
        s = s.offset(n as isize);
    }
    js_putc(J, sb, '"' as i32);
}
unsafe extern "C" fn reprident(
    J: *mut js_State,
    sb: *mut *mut js_Buffer,
    name: *const libc::c_char,
) {
    let mut p: *const libc::c_char = name;
    if *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            p = p.offset(1);
            p;
        }
    } else if *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32
        || *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'Z' as i32
        || *p as libc::c_int == '_' as i32
    {
        while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32
            || (*p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32
                || *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'Z' as i32)
            || *p as libc::c_int == '_' as i32
        {
            p = p.offset(1);
            p;
        }
    }
    if p > name && *p as libc::c_int == 0 as libc::c_int {
        js_puts(J, sb, name);
    } else {
        reprstr(J, sb, name);
    };
}
unsafe extern "C" fn reprobject(J: *mut js_State, sb: *mut *mut js_Buffer) {
    let mut key: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = js_gettop(J) - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as libc::c_int)) {
            js_puts(J, sb, b"{}\0" as *const u8 as *const libc::c_char);
            return;
        }
        i += 1;
        i;
    }
    n = 0 as libc::c_int;
    js_putc(J, sb, '{' as i32);
    js_pushiterator(J, -(1 as libc::c_int), 1 as libc::c_int);
    loop {
        key = js_nextiterator(J, -(1 as libc::c_int));
        if key.is_null() {
            break;
        }
        let fresh52 = n;
        n += 1;
        if fresh52 > 0 as libc::c_int {
            js_puts(J, sb, b", \0" as *const u8 as *const libc::c_char);
        }
        reprident(J, sb, key);
        js_puts(J, sb, b": \0" as *const u8 as *const libc::c_char);
        js_getproperty(J, -(2 as libc::c_int), key);
        reprvalue(J, sb);
        js_pop(J, 1 as libc::c_int);
    }
    js_pop(J, 1 as libc::c_int);
    js_putc(J, sb, '}' as i32);
}
unsafe extern "C" fn reprarray(J: *mut js_State, sb: *mut *mut js_Buffer) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    n = js_gettop(J) - 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as libc::c_int)) {
            js_puts(J, sb, b"[]\0" as *const u8 as *const libc::c_char);
            return;
        }
        i += 1;
        i;
    }
    js_putc(J, sb, '[' as i32);
    n = js_getlength(J, -(1 as libc::c_int));
    i = 0 as libc::c_int;
    while i < n {
        if i > 0 as libc::c_int {
            js_puts(J, sb, b", \0" as *const u8 as *const libc::c_char);
        }
        if js_hasindex(J, -(1 as libc::c_int), i) != 0 {
            reprvalue(J, sb);
            js_pop(J, 1 as libc::c_int);
        }
        i += 1;
        i;
    }
    js_putc(J, sb, ']' as i32);
}
unsafe extern "C" fn reprfun(J: *mut js_State, sb: *mut *mut js_Buffer, fun: *mut js_Function) {
    let mut i: libc::c_int = 0;
    js_puts(J, sb, b"function \0" as *const u8 as *const libc::c_char);
    js_puts(J, sb, (*fun).name);
    js_putc(J, sb, '(' as i32);
    i = 0 as libc::c_int;
    while i < (*fun).numparams {
        if i > 0 as libc::c_int {
            js_puts(J, sb, b", \0" as *const u8 as *const libc::c_char);
        }
        js_puts(J, sb, *((*fun).vartab).offset(i as isize));
        i += 1;
        i;
    }
    js_puts(
        J,
        sb,
        b") { [byte code] }\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn reprvalue(J: *mut js_State, sb: *mut *mut js_Buffer) {
    if js_isundefined(J, -(1 as libc::c_int)) != 0 {
        js_puts(J, sb, b"undefined\0" as *const u8 as *const libc::c_char);
    } else if js_isnull(J, -(1 as libc::c_int)) != 0 {
        js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
    } else if js_isboolean(J, -(1 as libc::c_int)) != 0 {
        js_puts(
            J,
            sb,
            if js_toboolean(J, -(1 as libc::c_int)) != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    } else if js_isnumber(J, -(1 as libc::c_int)) != 0 {
        reprnum(J, sb, js_tonumber(J, -(1 as libc::c_int)));
    } else if js_isstring(J, -(1 as libc::c_int)) != 0 {
        reprstr(J, sb, js_tostring(J, -(1 as libc::c_int)));
    } else if js_isobject(J, -(1 as libc::c_int)) != 0 {
        let obj: *mut js_Object = js_toobject(J, -(1 as libc::c_int));
        match (*obj).type_0 as libc::c_uint {
            1 => {
                reprarray(J, sb);
            }
            2 | 3 => {
                reprfun(J, sb, (*obj).u.f.function);
            }
            4 => {
                js_puts(J, sb, b"function \0" as *const u8 as *const libc::c_char);
                js_puts(J, sb, (*obj).u.c.name);
                js_puts(
                    J,
                    sb,
                    b"() { [native code] }\0" as *const u8 as *const libc::c_char,
                );
            }
            6 => {
                js_puts(
                    J,
                    sb,
                    b"(new Boolean(\0" as *const u8 as *const libc::c_char,
                );
                js_puts(
                    J,
                    sb,
                    if (*obj).u.boolean != 0 {
                        b"true\0" as *const u8 as *const libc::c_char
                    } else {
                        b"false\0" as *const u8 as *const libc::c_char
                    },
                );
                js_puts(J, sb, b"))\0" as *const u8 as *const libc::c_char);
            }
            7 => {
                js_puts(J, sb, b"(new Number(\0" as *const u8 as *const libc::c_char);
                reprnum(J, sb, (*obj).u.number);
                js_puts(J, sb, b"))\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                js_puts(J, sb, b"(new String(\0" as *const u8 as *const libc::c_char);
                reprstr(J, sb, (*obj).u.s.string);
                js_puts(J, sb, b"))\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                js_putc(J, sb, '/' as i32);
                js_puts(J, sb, (*obj).u.r.source);
                js_putc(J, sb, '/' as i32);
                if (*obj).u.r.flags as libc::c_int & JS_REGEXP_G as libc::c_int != 0 {
                    js_putc(J, sb, 'g' as i32);
                }
                if (*obj).u.r.flags as libc::c_int & JS_REGEXP_I as libc::c_int != 0 {
                    js_putc(J, sb, 'i' as i32);
                }
                if (*obj).u.r.flags as libc::c_int & JS_REGEXP_M as libc::c_int != 0 {
                    js_putc(J, sb, 'm' as i32);
                }
            }
            10 => {
                let mut buf: [libc::c_char; 40] = [0; 40];
                js_puts(J, sb, b"(new Date(\0" as *const u8 as *const libc::c_char);
                js_puts(
                    J,
                    sb,
                    jsV_numbertostring(J, buf.as_mut_ptr(), (*obj).u.number),
                );
                js_puts(J, sb, b"))\0" as *const u8 as *const libc::c_char);
            }
            5 => {
                js_puts(J, sb, b"(new \0" as *const u8 as *const libc::c_char);
                js_getproperty(
                    J,
                    -(1 as libc::c_int),
                    b"name\0" as *const u8 as *const libc::c_char,
                );
                js_puts(J, sb, js_tostring(J, -(1 as libc::c_int)));
                js_pop(J, 1 as libc::c_int);
                js_putc(J, sb, '(' as i32);
                if js_hasproperty(
                    J,
                    -(1 as libc::c_int),
                    b"message\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    reprvalue(J, sb);
                    js_pop(J, 1 as libc::c_int);
                }
                js_puts(J, sb, b"))\0" as *const u8 as *const libc::c_char);
            }
            11 => {
                js_puts(J, sb, b"Math\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                js_puts(J, sb, b"JSON\0" as *const u8 as *const libc::c_char);
            }
            14 => {
                js_puts(J, sb, b"[iterator \0" as *const u8 as *const libc::c_char);
            }
            15 => {
                js_puts(J, sb, b"[userdata \0" as *const u8 as *const libc::c_char);
                js_puts(J, sb, (*obj).u.user.tag);
                js_putc(J, sb, ']' as i32);
            }
            _ => {
                reprobject(J, sb);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_repr(J: *mut js_State, idx: libc::c_int) {
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut savebot: libc::c_int = 0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    js_copy(J, idx);
    savebot = (*J).bot;
    (*J).bot = (*J).top - 1 as libc::c_int;
    reprvalue(J, &mut sb);
    (*J).bot = savebot;
    js_pop(J, 1 as libc::c_int);
    js_putc(J, &mut sb, 0 as libc::c_int);
    js_pushstring(
        J,
        if !sb.is_null() {
            ((*sb).s).as_mut_ptr() as *const libc::c_char
        } else {
            b"undefined\0" as *const u8 as *const libc::c_char
        },
    );
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn js_torepr(J: *mut js_State, idx: libc::c_int) -> *const libc::c_char {
    js_repr(J, idx);
    js_replace(
        J,
        if idx < 0 as libc::c_int {
            idx - 1 as libc::c_int
        } else {
            idx
        },
    );
    js_tostring(J, idx)
}
#[no_mangle]
pub unsafe extern "C" fn js_tryrepr(
    J: *mut js_State,
    idx: libc::c_int,
    error: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    s = js_torepr(J, idx);
    js_endtry(J);
    s
}
unsafe extern "C" fn js_trystackoverflow(J: *mut js_State) {
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as libc::c_int as libc::c_char;
    let fresh53 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh53 = b"exception stack overflow\0" as *const u8 as *const libc::c_char;
    (*J).top += 1;
    (*J).top;
    js_throw(J);
}
unsafe extern "C" fn js_stackoverflow(J: *mut js_State) {
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as libc::c_int as libc::c_char;
    let fresh54 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh54 = b"stack overflow\0" as *const u8 as *const libc::c_char;
    (*J).top += 1;
    (*J).top;
    js_throw(J);
}
unsafe extern "C" fn js_outofmemory(J: *mut js_State) {
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as libc::c_int as libc::c_char;
    let fresh55 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh55 = b"out of memory\0" as *const u8 as *const libc::c_char;
    (*J).top += 1;
    (*J).top;
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_malloc(J: *mut js_State, size: libc::c_int) -> *mut libc::c_void {
    let ptr: *mut libc::c_void = ((*J).alloc).expect("non-null function pointer")(
        (*J).actx,
        std::ptr::null_mut::<libc::c_void>(),
        size,
    );
    if ptr.is_null() {
        js_outofmemory(J);
    }
    ptr
}
#[no_mangle]
pub unsafe extern "C" fn js_realloc(
    J: *mut js_State,
    mut ptr: *mut libc::c_void,
    size: libc::c_int,
) -> *mut libc::c_void {
    ptr = ((*J).alloc).expect("non-null function pointer")((*J).actx, ptr, size);
    if ptr.is_null() {
        js_outofmemory(J);
    }
    ptr
}
#[no_mangle]
pub unsafe extern "C" fn js_strdup(J: *mut js_State, s: *const libc::c_char) -> *mut libc::c_char {
    let n: libc::c_int = (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    let p: *mut libc::c_char = js_malloc(J, n) as *mut libc::c_char;
    memcpy(
        p as *mut libc::c_void,
        s as *const libc::c_void,
        n as libc::c_ulong,
    );
    p
}
#[no_mangle]
pub unsafe extern "C" fn js_free(J: *mut js_State, ptr: *mut libc::c_void) {
    ((*J).alloc).expect("non-null function pointer")((*J).actx, ptr, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn jsV_newmemstring(
    J: *mut js_State,
    s: *const libc::c_char,
    n: libc::c_int,
) -> *mut js_String {
    let v: *mut js_String =
        js_malloc(J, 9 as libc::c_ulong as libc::c_int + n + 1 as libc::c_int) as *mut js_String;
    memcpy(
        ((*v).p).as_mut_ptr() as *mut libc::c_void,
        s as *const libc::c_void,
        n as libc::c_ulong,
    );
    *((*v).p).as_mut_ptr().offset(n as isize) = 0 as libc::c_int as libc::c_char;
    (*v).gcmark = 0 as libc::c_int as libc::c_char;
    (*v).gcnext = (*J).gcstr;
    (*J).gcstr = v;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    v
}
#[no_mangle]
pub unsafe extern "C" fn js_pushvalue(J: *mut js_State, v: js_Value) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushundefined(J: *mut js_State) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 =
        JS_TUNDEFINED as libc::c_int as libc::c_char;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushnull(J: *mut js_State) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TNULL as libc::c_int as libc::c_char;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushboolean(J: *mut js_State, v: libc::c_int) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TBOOLEAN as libc::c_int as libc::c_char;
    (*((*J).stack).offset((*J).top as isize)).u.boolean = (v != 0) as libc::c_int;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushnumber(J: *mut js_State, v: libc::c_double) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TNUMBER as libc::c_int as libc::c_char;
    (*((*J).stack).offset((*J).top as isize)).u.number = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushstring(J: *mut js_State, mut v: *const libc::c_char) {
    let mut n: size_t = strlen(v);
    if n > ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong {
        js_rangeerror(
            J,
            b"invalid string length\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    if n <= 15 as libc::c_ulong as libc::c_int as libc::c_ulong {
        let mut s: *mut libc::c_char =
            ((*((*J).stack).offset((*J).top as isize)).u.shrstr).as_mut_ptr();
        loop {
            let fresh56 = n;
            n = n.wrapping_sub(1);
            if fresh56 == 0 {
                break;
            }
            let fresh57 = v;
            v = v.offset(1);
            let fresh58 = s;
            s = s.offset(1);
            *fresh58 = *fresh57;
        }
        *s = 0 as libc::c_int as libc::c_char;
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TSHRSTR as libc::c_int as libc::c_char;
    } else {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TMEMSTR as libc::c_int as libc::c_char;
        let fresh59 = &mut (*((*J).stack).offset((*J).top as isize)).u.memstr;
        *fresh59 = jsV_newmemstring(J, v, n as libc::c_int);
    }
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushlstring(
    J: *mut js_State,
    mut v: *const libc::c_char,
    mut n: libc::c_int,
) {
    if n > (1 as libc::c_int) << 28 as libc::c_int {
        js_rangeerror(
            J,
            b"invalid string length\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    if n <= 15 as libc::c_ulong as libc::c_int {
        let mut s: *mut libc::c_char =
            ((*((*J).stack).offset((*J).top as isize)).u.shrstr).as_mut_ptr();
        loop {
            let fresh60 = n;
            n -= 1;
            if fresh60 == 0 {
                break;
            }
            let fresh61 = v;
            v = v.offset(1);
            let fresh62 = s;
            s = s.offset(1);
            *fresh62 = *fresh61;
        }
        *s = 0 as libc::c_int as libc::c_char;
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TSHRSTR as libc::c_int as libc::c_char;
    } else {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TMEMSTR as libc::c_int as libc::c_char;
        let fresh63 = &mut (*((*J).stack).offset((*J).top as isize)).u.memstr;
        *fresh63 = jsV_newmemstring(J, v, n);
    }
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushliteral(J: *mut js_State, v: *const libc::c_char) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as libc::c_int as libc::c_char;
    let fresh64 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh64 = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushobject(J: *mut js_State, v: *mut js_Object) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TOBJECT as libc::c_int as libc::c_char;
    let fresh65 = &mut (*((*J).stack).offset((*J).top as isize)).u.object;
    *fresh65 = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushglobal(J: *mut js_State) {
    js_pushobject(J, (*J).G);
}
#[no_mangle]
pub unsafe extern "C" fn js_currentfunction(J: *mut js_State) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    if (*J).bot > 0 as libc::c_int {
        *((*J).stack).offset((*J).top as isize) =
            *((*J).stack).offset(((*J).bot - 1 as libc::c_int) as isize);
    } else {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TUNDEFINED as libc::c_int as libc::c_char;
    }
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_currentfunctiondata(J: *mut js_State) -> *mut libc::c_void {
    if (*J).bot > 0 as libc::c_int {
        return (*(*((*J).stack).offset(((*J).bot - 1 as libc::c_int) as isize))
            .u
            .object)
            .u
            .c
            .data;
    }
    std::ptr::null_mut::<libc::c_void>()
}
unsafe extern "C" fn stackidx(J: *mut js_State, mut idx: libc::c_int) -> *mut js_Value {
    static mut undefined: js_Value = js_Value {
        t: {
            C2RustUnnamed_6 {
                pad: [
                    0 as libc::c_int as libc::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                type_0: JS_TUNDEFINED as libc::c_int as libc::c_char,
            }
        },
    };
    idx = if idx < 0 as libc::c_int {
        (*J).top + idx
    } else {
        (*J).bot + idx
    };
    if idx < 0 as libc::c_int || idx >= (*J).top {
        return &mut undefined;
    }
    ((*J).stack).offset(idx as isize)
}
#[no_mangle]
pub unsafe extern "C" fn js_tovalue(J: *mut js_State, idx: libc::c_int) -> *mut js_Value {
    stackidx(J, idx)
}
#[no_mangle]
pub unsafe extern "C" fn js_isdefined(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    ((*stackidx(J, idx)).t.type_0 as libc::c_int != JS_TUNDEFINED as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isundefined(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    ((*stackidx(J, idx)).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isnull(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    ((*stackidx(J, idx)).t.type_0 as libc::c_int == JS_TNULL as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isboolean(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    ((*stackidx(J, idx)).t.type_0 as libc::c_int == JS_TBOOLEAN as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isnumber(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    ((*stackidx(J, idx)).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isstring(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let t: js_Type = (*stackidx(J, idx)).t.type_0 as js_Type;
    (t as libc::c_uint == JS_TSHRSTR as libc::c_int as libc::c_uint
        || t as libc::c_uint == JS_TLITSTR as libc::c_int as libc::c_uint
        || t as libc::c_uint == JS_TMEMSTR as libc::c_int as libc::c_uint) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isprimitive(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    ((*stackidx(J, idx)).t.type_0 as libc::c_int != JS_TOBJECT as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isobject(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    ((*stackidx(J, idx)).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_iscoercible(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as libc::c_int != JS_TUNDEFINED as libc::c_int
        && (*v).t.type_0 as libc::c_int != JS_TNULL as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_iscallable(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int {
        return ((*(*v).u.object).type_0 as libc::c_uint
            == JS_CFUNCTION as libc::c_int as libc::c_uint
            || (*(*v).u.object).type_0 as libc::c_uint == JS_CSCRIPT as libc::c_int as libc::c_uint
            || (*(*v).u.object).type_0 as libc::c_uint
                == JS_CCFUNCTION as libc::c_int as libc::c_uint) as libc::c_int;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isarray(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint)
        as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isregexp(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint)
        as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_isuserdata(
    J: *mut js_State,
    idx: libc::c_int,
    tag: *const libc::c_char,
) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CUSERDATA as libc::c_int as libc::c_uint
    {
        return (strcmp(tag, (*(*v).u.object).u.user.tag) == 0) as libc::c_int;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_iserror(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CERROR as libc::c_int as libc::c_uint)
        as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_typeof(J: *mut js_State, idx: libc::c_int) -> *const libc::c_char {
    let v: *mut js_Value = stackidx(J, idx);
    match (*v).t.type_0 as libc::c_int {
        1 => b"undefined\0" as *const u8 as *const libc::c_char,
        2 => b"object\0" as *const u8 as *const libc::c_char,
        3 => b"boolean\0" as *const u8 as *const libc::c_char,
        4 => b"number\0" as *const u8 as *const libc::c_char,
        5 => b"string\0" as *const u8 as *const libc::c_char,
        6 => b"string\0" as *const u8 as *const libc::c_char,
        7 => {
            if (*(*v).u.object).type_0 as libc::c_uint
                == JS_CFUNCTION as libc::c_int as libc::c_uint
                || (*(*v).u.object).type_0 as libc::c_uint
                    == JS_CCFUNCTION as libc::c_int as libc::c_uint
            {
                return b"function\0" as *const u8 as *const libc::c_char;
            }
            b"object\0" as *const u8 as *const libc::c_char
        }
        0 | _ => b"string\0" as *const u8 as *const libc::c_char,
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_type(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    match (*v).t.type_0 as libc::c_int {
        1 => JS_ISUNDEFINED as libc::c_int,
        2 => JS_ISNULL as libc::c_int,
        3 => JS_ISBOOLEAN as libc::c_int,
        4 => JS_ISNUMBER as libc::c_int,
        5 => JS_ISSTRING as libc::c_int,
        6 => JS_ISSTRING as libc::c_int,
        7 => {
            if (*(*v).u.object).type_0 as libc::c_uint
                == JS_CFUNCTION as libc::c_int as libc::c_uint
                || (*(*v).u.object).type_0 as libc::c_uint
                    == JS_CCFUNCTION as libc::c_int as libc::c_uint
            {
                return JS_ISFUNCTION as libc::c_int;
            }
            JS_ISOBJECT as libc::c_int
        }
        0 | _ => JS_ISSTRING as libc::c_int,
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_toboolean(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    jsV_toboolean(J, stackidx(J, idx))
}
#[no_mangle]
pub unsafe extern "C" fn js_tonumber(J: *mut js_State, idx: libc::c_int) -> libc::c_double {
    jsV_tonumber(J, stackidx(J, idx))
}
#[no_mangle]
pub unsafe extern "C" fn js_tointeger(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    jsV_numbertointeger(jsV_tonumber(J, stackidx(J, idx)))
}
#[no_mangle]
pub unsafe extern "C" fn js_toint32(J: *mut js_State, idx: libc::c_int) -> libc::c_int {
    jsV_numbertoint32(jsV_tonumber(J, stackidx(J, idx)))
}
#[no_mangle]
pub unsafe extern "C" fn js_touint32(J: *mut js_State, idx: libc::c_int) -> libc::c_uint {
    jsV_numbertouint32(jsV_tonumber(J, stackidx(J, idx)))
}
#[no_mangle]
pub unsafe extern "C" fn js_toint16(J: *mut js_State, idx: libc::c_int) -> libc::c_short {
    jsV_numbertoint16(jsV_tonumber(J, stackidx(J, idx)))
}
#[no_mangle]
pub unsafe extern "C" fn js_touint16(J: *mut js_State, idx: libc::c_int) -> libc::c_ushort {
    jsV_numbertouint16(jsV_tonumber(J, stackidx(J, idx)))
}
#[no_mangle]
pub unsafe extern "C" fn js_tostring(J: *mut js_State, idx: libc::c_int) -> *const libc::c_char {
    jsV_tostring(J, stackidx(J, idx))
}
#[no_mangle]
pub unsafe extern "C" fn js_toobject(J: *mut js_State, idx: libc::c_int) -> *mut js_Object {
    jsV_toobject(J, stackidx(J, idx))
}
#[no_mangle]
pub unsafe extern "C" fn js_toprimitive(J: *mut js_State, idx: libc::c_int, hint: libc::c_int) {
    jsV_toprimitive(J, stackidx(J, idx), hint);
}
#[no_mangle]
pub unsafe extern "C" fn js_toregexp(J: *mut js_State, idx: libc::c_int) -> *mut js_Regexp {
    let v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint
    {
        return &mut (*(*v).u.object).u.r;
    }
    js_typeerror(J, b"not a regexp\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn js_touserdata(
    J: *mut js_State,
    idx: libc::c_int,
    tag: *const libc::c_char,
) -> *mut libc::c_void {
    let v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CUSERDATA as libc::c_int as libc::c_uint
        && strcmp(tag, (*(*v).u.object).u.user.tag) == 0
    {
        return (*(*v).u.object).u.user.data;
    }
    js_typeerror(J, b"not a %s\0" as *const u8 as *const libc::c_char, tag);
}
unsafe extern "C" fn jsR_tofunction(J: *mut js_State, idx: libc::c_int) -> *mut js_Object {
    let v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int
        || (*v).t.type_0 as libc::c_int == JS_TNULL as libc::c_int
    {
        return std::ptr::null_mut::<js_Object>();
    }
    if (*v).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        && ((*(*v).u.object).type_0 as libc::c_uint == JS_CFUNCTION as libc::c_int as libc::c_uint
            || (*(*v).u.object).type_0 as libc::c_uint
                == JS_CCFUNCTION as libc::c_int as libc::c_uint)
    {
        return (*v).u.object;
    }
    js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn js_gettop(J: *mut js_State) -> libc::c_int {
    (*J).top - (*J).bot
}
#[no_mangle]
pub unsafe extern "C" fn js_pop(J: *mut js_State, n: libc::c_int) {
    (*J).top -= n;
    if (*J).top < (*J).bot {
        (*J).top = (*J).bot;
        js_error(J, b"stack underflow!\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_remove(J: *mut js_State, mut idx: libc::c_int) {
    idx = if idx < 0 as libc::c_int {
        (*J).top + idx
    } else {
        (*J).bot + idx
    };
    if idx < (*J).bot || idx >= (*J).top {
        js_error(J, b"stack error!\0" as *const u8 as *const libc::c_char);
    }
    while idx < (*J).top - 1 as libc::c_int {
        *((*J).stack).offset(idx as isize) =
            *((*J).stack).offset((idx + 1 as libc::c_int) as isize);
        idx += 1;
        idx;
    }
    (*J).top -= 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_insert(J: *mut js_State, idx: libc::c_int) {
    js_error(
        J,
        b"not implemented yet\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_replace(J: *mut js_State, mut idx: libc::c_int) {
    idx = if idx < 0 as libc::c_int {
        (*J).top + idx
    } else {
        (*J).bot + idx
    };
    if idx < (*J).bot || idx >= (*J).top {
        js_error(J, b"stack error!\0" as *const u8 as *const libc::c_char);
    }
    (*J).top -= 1;
    *((*J).stack).offset(idx as isize) = *((*J).stack).offset((*J).top as isize);
}
#[no_mangle]
pub unsafe extern "C" fn js_copy(J: *mut js_State, idx: libc::c_int) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) = *stackidx(J, idx);
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_dup(J: *mut js_State) {
    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) =
        *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_dup2(J: *mut js_State) {
    if (*J).top + 2 as libc::c_int >= 4096 as libc::c_int {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) =
        *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top + 1 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    (*J).top += 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot2(J: *mut js_State) {
    let tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot3(J: *mut js_State) {
    let tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 3 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 3 as libc::c_int) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot4(J: *mut js_State) {
    let tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 3 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 3 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 4 as libc::c_int) as isize);
    *((*J).stack).offset(((*J).top - 4 as libc::c_int) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot2pop1(J: *mut js_State) {
    *((*J).stack).offset(((*J).top - 2 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    (*J).top -= 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot3pop2(J: *mut js_State) {
    *((*J).stack).offset(((*J).top - 3 as libc::c_int) as isize) =
        *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    (*J).top -= 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot(J: *mut js_State, n: libc::c_int) {
    let mut i: libc::c_int = 0;
    let tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
    i = 1 as libc::c_int;
    while i < n {
        *((*J).stack).offset(((*J).top - i) as isize) =
            *((*J).stack).offset(((*J).top - i - 1 as libc::c_int) as isize);
        i += 1;
        i;
    }
    *((*J).stack).offset(((*J).top - i) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_isarrayindex(
    J: *mut js_State,
    mut p: *const libc::c_char,
    idx: *mut libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = 0 as libc::c_int;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32 {
        return if *p.offset(1 as libc::c_int as isize) as libc::c_int == 0 as libc::c_int {
            *idx = 0 as libc::c_int;
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    while *p != 0 {
        let fresh66 = p;
        p = p.offset(1);
        let c: libc::c_int = *fresh66 as libc::c_int;
        if c >= '0' as i32 && c <= '9' as i32 {
            if n >= 2147483647 as libc::c_int / 10 as libc::c_int {
                return 0 as libc::c_int;
            }
            n = n * 10 as libc::c_int + (c - '0' as i32);
        } else {
            return 0 as libc::c_int;
        }
    }
    *idx = n;
    1 as libc::c_int
}
unsafe extern "C" fn js_pushrune(J: *mut js_State, mut rune: Rune) {
    let mut buf: [libc::c_char; 5] = [0; 5];
    if rune >= 0 as libc::c_int {
        buf[jsU_runetochar(buf.as_mut_ptr(), &mut rune) as usize] =
            0 as libc::c_int as libc::c_char;
        js_pushstring(J, buf.as_mut_ptr());
    } else {
        js_pushundefined(J);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsR_unflattenarray(J: *mut js_State, obj: *mut js_Object) {
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        let mut ref_0: *mut js_Property = std::ptr::null_mut::<js_Property>();
        let mut i: libc::c_int = 0;
        let mut name: [libc::c_char; 32] = [0; 32];
        if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
            (*obj).properties = std::ptr::null_mut::<js_Property>();
            js_throw(J);
        }
        i = 0 as libc::c_int;
        while i < (*obj).u.a.flat_length {
            js_itoa(name.as_mut_ptr(), i);
            ref_0 = jsV_setproperty(J, obj, name.as_mut_ptr());
            (*ref_0).value = *((*obj).u.a.array).offset(i as isize);
            i += 1;
            i;
        }
        js_free(J, (*obj).u.a.array as *mut libc::c_void);
        (*obj).u.a.simple = 0 as libc::c_int;
        (*obj).u.a.flat_length = 0 as libc::c_int;
        (*obj).u.a.flat_capacity = 0 as libc::c_int;
        (*obj).u.a.array = std::ptr::null_mut::<js_Value>();
        js_endtry(J);
    }
}
unsafe extern "C" fn jsR_hasproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
) -> libc::c_int {
    let mut ref_0: *mut js_Property = std::ptr::null_mut::<js_Property>();
    let mut k: libc::c_int = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushnumber(J, (*obj).u.a.length as libc::c_double);
            return 1 as libc::c_int;
        }
        if (*obj).u.a.simple != 0 && js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as libc::c_int && k < (*obj).u.a.flat_length {
                js_pushvalue(J, *((*obj).u.a.array).offset(k as isize));
                return 1 as libc::c_int;
            }
            return 0 as libc::c_int;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushnumber(J, (*obj).u.s.length as libc::c_double);
            return 1 as libc::c_int;
        }
        if js_isarrayindex(J, name, &mut k) != 0 && k >= 0 as libc::c_int && k < (*obj).u.s.length {
            js_pushrune(J, js_runeat(J, (*obj).u.s.string, k));
            return 1 as libc::c_int;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushstring(J, (*obj).u.r.source);
            return 1 as libc::c_int;
        }
        if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushboolean(
                J,
                (*obj).u.r.flags as libc::c_int & JS_REGEXP_G as libc::c_int,
            );
            return 1 as libc::c_int;
        }
        if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushboolean(
                J,
                (*obj).u.r.flags as libc::c_int & JS_REGEXP_I as libc::c_int,
            );
            return 1 as libc::c_int;
        }
        if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushboolean(
                J,
                (*obj).u.r.flags as libc::c_int & JS_REGEXP_M as libc::c_int,
            );
            return 1 as libc::c_int;
        }
        if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushnumber(J, (*obj).u.r.last as libc::c_double);
            return 1 as libc::c_int;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as libc::c_int as libc::c_uint
        && ((*obj).u.user.has).is_some()
        && ((*obj).u.user.has).expect("non-null function pointer")(J, (*obj).u.user.data, name) != 0
    {
        return 1 as libc::c_int;
    }
    ref_0 = jsV_getproperty(J, obj, name);
    if !ref_0.is_null() {
        if !((*ref_0).getter).is_null() {
            js_pushobject(J, (*ref_0).getter);
            js_pushobject(J, obj);
            js_call(J, 0 as libc::c_int);
        } else {
            js_pushvalue(J, (*ref_0).value);
        }
        return 1 as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn jsR_getproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
) {
    if jsR_hasproperty(J, obj, name) == 0 {
        js_pushundefined(J);
    }
}
unsafe extern "C" fn jsR_hasindex(
    J: *mut js_State,
    obj: *mut js_Object,
    k: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 32] = [0; 32];
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        if k >= 0 as libc::c_int && k < (*obj).u.a.flat_length {
            js_pushvalue(J, *((*obj).u.a.array).offset(k as isize));
            return 1 as libc::c_int;
        }
        return 0 as libc::c_int;
    }
    jsR_hasproperty(J, obj, js_itoa(buf.as_mut_ptr(), k))
}
unsafe extern "C" fn jsR_getindex(J: *mut js_State, obj: *mut js_Object, k: libc::c_int) {
    if jsR_hasindex(J, obj, k) == 0 {
        js_pushundefined(J);
    }
}
unsafe extern "C" fn jsR_setarrayindex(
    J: *mut js_State,
    obj: *mut js_Object,
    k: libc::c_int,
    value: *mut js_Value,
) {
    let newlen: libc::c_int = k + 1 as libc::c_int;
    if (*obj).u.a.simple != 0 {
    } else {
        __assert_fail(
            b"obj->u.a.simple\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            11018 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_43107: {
        if (*obj).u.a.simple != 0 {
        } else {
            __assert_fail(
                b"obj->u.a.simple\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                11018 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if k >= 0 as libc::c_int {
    } else {
        __assert_fail(
            b"k >= 0\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            11019 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
            ))
            .as_ptr(),
        );
    }
    'c_43071: {
        if k >= 0 as libc::c_int {
        } else {
            __assert_fail(
                b"k >= 0\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                11019 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if newlen > (1 as libc::c_int) << 26 as libc::c_int {
        js_rangeerror(J, b"array too large\0" as *const u8 as *const libc::c_char);
    }
    if newlen > (*obj).u.a.flat_length {
        if newlen == (*obj).u.a.flat_length + 1 as libc::c_int {
        } else {
            __assert_fail(
                b"newlen == obj->u.a.flat_length + 1\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                11023 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                ))
                .as_ptr(),
            );
        }
        'c_42995: {
            if newlen == (*obj).u.a.flat_length + 1 as libc::c_int {
            } else {
                __assert_fail(
                    b"newlen == obj->u.a.flat_length + 1\0" as *const u8 as *const libc::c_char,
                    b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                    11023 as libc::c_int as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        if newlen > (*obj).u.a.flat_capacity {
            let mut newcap: libc::c_int = (*obj).u.a.flat_capacity;
            if newcap == 0 as libc::c_int {
                newcap = 8 as libc::c_int;
            }
            while newcap < newlen {
                newcap <<= 1 as libc::c_int;
            }
            (*obj).u.a.array = js_realloc(
                J,
                (*obj).u.a.array as *mut libc::c_void,
                (newcap as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<js_Value>() as libc::c_ulong)
                    as libc::c_int,
            ) as *mut js_Value;
            (*obj).u.a.flat_capacity = newcap;
        }
        (*obj).u.a.flat_length = newlen;
    }
    if newlen > (*obj).u.a.length {
        (*obj).u.a.length = newlen;
    }
    *((*obj).u.a.array).offset(k as isize) = *value;
}
unsafe extern "C" fn jsR_setproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
    transient: libc::c_int,
) {
    let mut current_block: u64;
    let value: *mut js_Value = stackidx(J, -(1 as libc::c_int));
    let mut ref_0: *mut js_Property = std::ptr::null_mut::<js_Property>();
    let mut k: libc::c_int = 0;
    let mut own: libc::c_int = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            let rawlen: libc::c_double = jsV_tonumber(J, value);
            let newlen: libc::c_int = jsV_numbertointeger(rawlen);
            if newlen as libc::c_double != rawlen || newlen < 0 as libc::c_int {
                js_rangeerror(
                    J,
                    b"invalid array length\0" as *const u8 as *const libc::c_char,
                );
            }
            if newlen > (1 as libc::c_int) << 26 as libc::c_int {
                js_rangeerror(J, b"array too large\0" as *const u8 as *const libc::c_char);
            }
            if (*obj).u.a.simple != 0 {
                (*obj).u.a.length = newlen;
                if newlen <= (*obj).u.a.flat_length {
                    (*obj).u.a.flat_length = newlen;
                }
            } else {
                jsV_resizearray(J, obj, newlen);
            }
            return;
        }
        if js_isarrayindex(J, name, &mut k) != 0 {
            if (*obj).u.a.simple != 0 {
                if k >= 0 as libc::c_int && k <= (*obj).u.a.flat_length {
                    jsR_setarrayindex(J, obj, k, value);
                } else {
                    jsR_unflattenarray(J, obj);
                    if (*obj).u.a.length < k + 1 as libc::c_int {
                        (*obj).u.a.length = k + 1 as libc::c_int;
                    }
                }
            } else if (*obj).u.a.length < k + 1 as libc::c_int {
                (*obj).u.a.length = k + 1 as libc::c_int;
            }
        }
        current_block = 4567019141635105728;
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as libc::c_int && k < (*obj).u.s.length {
                current_block = 5947843046271591061;
            } else {
                current_block = 4567019141635105728;
            }
        } else {
            current_block = 4567019141635105728;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else {
            if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
                (*obj).u.r.last = jsV_tointeger(J, value) as libc::c_ushort;
                return;
            }
            current_block = 4567019141635105728;
        }
    } else {
        if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as libc::c_int as libc::c_uint
            && ((*obj).u.user.put).is_some()
            && ((*obj).u.user.put).expect("non-null function pointer")(J, (*obj).u.user.data, name)
                != 0
        {
            return;
        }
        current_block = 4567019141635105728;
    }
    if current_block == 4567019141635105728 {
        ref_0 = jsV_getpropertyx(J, obj, name, &mut own);
        if !ref_0.is_null() {
            if !((*ref_0).setter).is_null() {
                js_pushobject(J, (*ref_0).setter);
                js_pushobject(J, obj);
                js_pushvalue(J, *value);
                js_call(J, 1 as libc::c_int);
                js_pop(J, 1 as libc::c_int);
                return;
            } else {
                if (*J).strict != 0 && !((*ref_0).getter).is_null() {
                    js_typeerror(
                        J,
                        b"setting property '%s' that only has a getter\0" as *const u8
                            as *const libc::c_char,
                        name,
                    );
                }
                if (*ref_0).atts & JS_READONLY as libc::c_int != 0 {
                    current_block = 5947843046271591061;
                } else {
                    current_block = 13826291924415791078;
                }
            }
        } else {
            current_block = 13826291924415791078;
        }
        match current_block {
            5947843046271591061 => {}
            _ => {
                if ref_0.is_null() || own == 0 {
                    if transient != 0 {
                        if (*J).strict != 0 {
                            js_typeerror(
                                J,
                                b"cannot create property '%s' on transient object\0" as *const u8
                                    as *const libc::c_char,
                                name,
                            );
                        }
                        return;
                    }
                    ref_0 = jsV_setproperty(J, obj, name);
                }
                if !ref_0.is_null() {
                    if (*ref_0).atts & JS_READONLY as libc::c_int == 0 {
                        (*ref_0).value = *value;
                        current_block = 1623252117315916725;
                    } else {
                        current_block = 5947843046271591061;
                    }
                } else {
                    current_block = 1623252117315916725;
                }
                match current_block {
                    5947843046271591061 => {}
                    _ => return,
                }
            }
        }
    }
    if (*J).strict != 0 {
        js_typeerror(
            J,
            b"'%s' is read-only\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
}
unsafe extern "C" fn jsR_setindex(
    J: *mut js_State,
    obj: *mut js_Object,
    k: libc::c_int,
    transient: libc::c_int,
) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
        && k >= 0 as libc::c_int
        && k <= (*obj).u.a.flat_length
    {
        jsR_setarrayindex(J, obj, k, stackidx(J, -(1 as libc::c_int)));
    } else {
        jsR_setproperty(J, obj, js_itoa(buf.as_mut_ptr(), k), transient);
    };
}
unsafe extern "C" fn jsR_defproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
    atts: libc::c_int,
    value: *mut js_Value,
    getter: *mut js_Object,
    setter: *mut js_Object,
    throw: libc::c_int,
) {
    let current_block: u64;
    let mut ref_0: *mut js_Property = std::ptr::null_mut::<js_Property>();
    let mut k: libc::c_int = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else {
            if (*obj).u.a.simple != 0 {
                jsR_unflattenarray(J, obj);
            }
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as libc::c_int && k < (*obj).u.s.length {
                current_block = 8261918236977660230;
            } else {
                current_block = 2370887241019905314;
            }
        } else {
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else {
            current_block = 2370887241019905314;
        }
    } else {
        if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as libc::c_int as libc::c_uint
            && ((*obj).u.user.put).is_some()
            && ((*obj).u.user.put).expect("non-null function pointer")(J, (*obj).u.user.data, name)
                != 0
        {
            return;
        }
        current_block = 2370887241019905314;
    }
    match current_block {
        8261918236977660230 => {
            if (*J).strict != 0 || throw != 0 {
                js_typeerror(
                    J,
                    b"'%s' is read-only or non-configurable\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
        }
        _ => {
            ref_0 = jsV_setproperty(J, obj, name);
            if !ref_0.is_null() {
                if !value.is_null() {
                    if (*ref_0).atts & JS_READONLY as libc::c_int == 0 {
                        (*ref_0).value = *value;
                    } else if (*J).strict != 0 {
                        js_typeerror(
                            J,
                            b"'%s' is read-only\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                }
                if !getter.is_null() {
                    if (*ref_0).atts & JS_DONTCONF as libc::c_int == 0 {
                        (*ref_0).getter = getter;
                    } else if (*J).strict != 0 {
                        js_typeerror(
                            J,
                            b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                }
                if !setter.is_null() {
                    if (*ref_0).atts & JS_DONTCONF as libc::c_int == 0 {
                        (*ref_0).setter = setter;
                    } else if (*J).strict != 0 {
                        js_typeerror(
                            J,
                            b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                }
                (*ref_0).atts |= atts;
            }
        }
    }
}
unsafe extern "C" fn jsR_delproperty(
    J: *mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut ref_0: *mut js_Property = std::ptr::null_mut::<js_Property>();
    let mut k: libc::c_int = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else {
            if (*obj).u.a.simple != 0 {
                jsR_unflattenarray(J, obj);
            }
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as libc::c_int as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as libc::c_int && k < (*obj).u.s.length {
                current_block = 15302371838142339756;
            } else {
                current_block = 2370887241019905314;
            }
        } else {
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as libc::c_int as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else {
            current_block = 2370887241019905314;
        }
    } else {
        if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as libc::c_int as libc::c_uint
            && ((*obj).u.user.delete).is_some()
            && ((*obj).u.user.delete).expect("non-null function pointer")(
                J,
                (*obj).u.user.data,
                name,
            ) != 0
        {
            return 1 as libc::c_int;
        }
        current_block = 2370887241019905314;
    }
    if current_block == 2370887241019905314 {
        ref_0 = jsV_getownproperty(J, obj, name);
        if !ref_0.is_null() {
            if (*ref_0).atts & JS_DONTCONF as libc::c_int != 0 {
                current_block = 15302371838142339756;
            } else {
                jsV_delproperty(J, obj, name);
                current_block = 14576567515993809846;
            }
        } else {
            current_block = 14576567515993809846;
        }
        match current_block {
            15302371838142339756 => {}
            _ => return 1 as libc::c_int,
        }
    }
    if (*J).strict != 0 {
        js_typeerror(
            J,
            b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    0 as libc::c_int
}
unsafe extern "C" fn jsR_delindex(J: *mut js_State, obj: *mut js_Object, k: libc::c_int) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as libc::c_int as libc::c_uint
        && (*obj).u.a.simple != 0
        && k == (*obj).u.a.flat_length - 1 as libc::c_int
    {
        (*obj).u.a.flat_length = k;
    } else {
        jsR_delproperty(J, obj, js_itoa(buf.as_mut_ptr(), k));
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_ref(J: *mut js_State) -> *const libc::c_char {
    let v: *mut js_Value = stackidx(J, -(1 as libc::c_int));
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut buf: [libc::c_char; 32] = [0; 32];
    match (*v).t.type_0 as libc::c_int {
        1 => {
            s = b"_Undefined\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"_Null\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = if (*v).u.boolean != 0 {
                b"_True\0" as *const u8 as *const libc::c_char
            } else {
                b"_False\0" as *const u8 as *const libc::c_char
            };
        }
        7 => {
            sprintf(
                buf.as_mut_ptr(),
                b"%p\0" as *const u8 as *const libc::c_char,
                (*v).u.object as *mut libc::c_void,
            );
            s = js_intern(J, buf.as_mut_ptr());
        }
        _ => {
            let fresh67 = (*J).nextref;
            (*J).nextref += 1;
            sprintf(
                buf.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                fresh67,
            );
            s = js_intern(J, buf.as_mut_ptr());
        }
    }
    js_setregistry(J, s);
    s
}
#[no_mangle]
pub unsafe extern "C" fn js_unref(J: *mut js_State, ref_0: *const libc::c_char) {
    js_delregistry(J, ref_0);
}
#[no_mangle]
pub unsafe extern "C" fn js_getregistry(J: *mut js_State, name: *const libc::c_char) {
    jsR_getproperty(J, (*J).R, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_setregistry(J: *mut js_State, name: *const libc::c_char) {
    jsR_setproperty(J, (*J).R, name, 0 as libc::c_int);
    js_pop(J, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_delregistry(J: *mut js_State, name: *const libc::c_char) {
    jsR_delproperty(J, (*J).R, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_getglobal(J: *mut js_State, name: *const libc::c_char) {
    jsR_getproperty(J, (*J).G, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_setglobal(J: *mut js_State, name: *const libc::c_char) {
    jsR_setproperty(J, (*J).G, name, 0 as libc::c_int);
    js_pop(J, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_defglobal(
    J: *mut js_State,
    name: *const libc::c_char,
    atts: libc::c_int,
) {
    jsR_defproperty(
        J,
        (*J).G,
        name,
        atts,
        stackidx(J, -(1 as libc::c_int)),
        std::ptr::null_mut::<js_Object>(),
        std::ptr::null_mut::<js_Object>(),
        0 as libc::c_int,
    );
    js_pop(J, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_delglobal(J: *mut js_State, name: *const libc::c_char) {
    jsR_delproperty(J, (*J).G, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_getproperty(
    J: *mut js_State,
    idx: libc::c_int,
    name: *const libc::c_char,
) {
    jsR_getproperty(J, js_toobject(J, idx), name);
}
#[no_mangle]
pub unsafe extern "C" fn js_setproperty(
    J: *mut js_State,
    idx: libc::c_int,
    name: *const libc::c_char,
) {
    jsR_setproperty(
        J,
        js_toobject(J, idx),
        name,
        (js_isobject(J, idx) == 0) as libc::c_int,
    );
    js_pop(J, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_defproperty(
    J: *mut js_State,
    idx: libc::c_int,
    name: *const libc::c_char,
    atts: libc::c_int,
) {
    jsR_defproperty(
        J,
        js_toobject(J, idx),
        name,
        atts,
        stackidx(J, -(1 as libc::c_int)),
        std::ptr::null_mut::<js_Object>(),
        std::ptr::null_mut::<js_Object>(),
        1 as libc::c_int,
    );
    js_pop(J, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_delproperty(
    J: *mut js_State,
    idx: libc::c_int,
    name: *const libc::c_char,
) {
    jsR_delproperty(J, js_toobject(J, idx), name);
}
#[no_mangle]
pub unsafe extern "C" fn js_defaccessor(
    J: *mut js_State,
    idx: libc::c_int,
    name: *const libc::c_char,
    atts: libc::c_int,
) {
    jsR_defproperty(
        J,
        js_toobject(J, idx),
        name,
        atts,
        std::ptr::null_mut::<js_Value>(),
        jsR_tofunction(J, -(2 as libc::c_int)),
        jsR_tofunction(J, -(1 as libc::c_int)),
        1 as libc::c_int,
    );
    js_pop(J, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_hasproperty(
    J: *mut js_State,
    idx: libc::c_int,
    name: *const libc::c_char,
) -> libc::c_int {
    jsR_hasproperty(J, js_toobject(J, idx), name)
}
#[no_mangle]
pub unsafe extern "C" fn js_getindex(J: *mut js_State, idx: libc::c_int, i: libc::c_int) {
    jsR_getindex(J, js_toobject(J, idx), i);
}
#[no_mangle]
pub unsafe extern "C" fn js_hasindex(
    J: *mut js_State,
    idx: libc::c_int,
    i: libc::c_int,
) -> libc::c_int {
    jsR_hasindex(J, js_toobject(J, idx), i)
}
#[no_mangle]
pub unsafe extern "C" fn js_setindex(J: *mut js_State, idx: libc::c_int, i: libc::c_int) {
    jsR_setindex(
        J,
        js_toobject(J, idx),
        i,
        (js_isobject(J, idx) == 0) as libc::c_int,
    );
    js_pop(J, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_delindex(J: *mut js_State, idx: libc::c_int, i: libc::c_int) {
    jsR_delindex(J, js_toobject(J, idx), i);
}
#[no_mangle]
pub unsafe extern "C" fn js_pushiterator(J: *mut js_State, idx: libc::c_int, own: libc::c_int) {
    js_pushobject(J, jsV_newiterator(J, js_toobject(J, idx), own));
}
#[no_mangle]
pub unsafe extern "C" fn js_nextiterator(
    J: *mut js_State,
    idx: libc::c_int,
) -> *const libc::c_char {
    jsV_nextiterator(J, js_toobject(J, idx))
}
#[no_mangle]
pub unsafe extern "C" fn jsR_newenvironment(
    J: *mut js_State,
    vars: *mut js_Object,
    outer: *mut js_Environment,
) -> *mut js_Environment {
    let E: *mut js_Environment = js_malloc(
        J,
        ::core::mem::size_of::<js_Environment>() as libc::c_ulong as libc::c_int,
    ) as *mut js_Environment;
    (*E).gcmark = 0 as libc::c_int;
    (*E).gcnext = (*J).gcenv;
    (*J).gcenv = E;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    (*E).outer = outer;
    (*E).variables = vars;
    E
}
unsafe extern "C" fn js_initvar(J: *mut js_State, name: *const libc::c_char, idx: libc::c_int) {
    jsR_defproperty(
        J,
        (*(*J).E).variables,
        name,
        JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
        stackidx(J, idx),
        std::ptr::null_mut::<js_Object>(),
        std::ptr::null_mut::<js_Object>(),
        0 as libc::c_int,
    );
}
unsafe extern "C" fn js_hasvar(J: *mut js_State, name: *const libc::c_char) -> libc::c_int {
    let mut E: *mut js_Environment = (*J).E;
    loop {
        let ref_0: *mut js_Property = jsV_getproperty(J, (*E).variables, name);
        if !ref_0.is_null() {
            if !((*ref_0).getter).is_null() {
                js_pushobject(J, (*ref_0).getter);
                js_pushobject(J, (*E).variables);
                js_call(J, 0 as libc::c_int);
            } else {
                js_pushvalue(J, (*ref_0).value);
            }
            return 1 as libc::c_int;
        }
        E = (*E).outer;
        if E.is_null() {
            break;
        }
    }
    0 as libc::c_int
}
unsafe extern "C" fn js_setvar(J: *mut js_State, name: *const libc::c_char) {
    let mut E: *mut js_Environment = (*J).E;
    loop {
        let ref_0: *mut js_Property = jsV_getproperty(J, (*E).variables, name);
        if !ref_0.is_null() {
            if !((*ref_0).setter).is_null() {
                js_pushobject(J, (*ref_0).setter);
                js_pushobject(J, (*E).variables);
                js_copy(J, -(3 as libc::c_int));
                js_call(J, 1 as libc::c_int);
                js_pop(J, 1 as libc::c_int);
                return;
            }
            if (*ref_0).atts & JS_READONLY as libc::c_int == 0 {
                (*ref_0).value = *stackidx(J, -(1 as libc::c_int));
            } else if (*J).strict != 0 {
                js_typeerror(
                    J,
                    b"'%s' is read-only\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
            return;
        }
        E = (*E).outer;
        if E.is_null() {
            break;
        }
    }
    if (*J).strict != 0 {
        js_referenceerror(
            J,
            b"assignment to undeclared variable '%s'\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    jsR_setproperty(J, (*J).G, name, 0 as libc::c_int);
}
unsafe extern "C" fn js_delvar(J: *mut js_State, name: *const libc::c_char) -> libc::c_int {
    let mut E: *mut js_Environment = (*J).E;
    loop {
        let ref_0: *mut js_Property = jsV_getownproperty(J, (*E).variables, name);
        if !ref_0.is_null() {
            if (*ref_0).atts & JS_DONTCONF as libc::c_int != 0 {
                if (*J).strict != 0 {
                    js_typeerror(
                        J,
                        b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
                return 0 as libc::c_int;
            }
            jsV_delproperty(J, (*E).variables, name);
            return 1 as libc::c_int;
        }
        E = (*E).outer;
        if E.is_null() {
            break;
        }
    }
    jsR_delproperty(J, (*J).G, name)
}
unsafe extern "C" fn jsR_savescope(J: *mut js_State, newE: *mut js_Environment) {
    if (*J).envtop + 1 as libc::c_int >= 1024 as libc::c_int {
        js_stackoverflow(J);
    }
    let fresh68 = (*J).envtop;
    (*J).envtop += 1;
    (*J).envstack[fresh68 as usize] = (*J).E;
    (*J).E = newE;
}
unsafe extern "C" fn jsR_restorescope(J: *mut js_State) {
    (*J).envtop -= 1;
    (*J).E = (*J).envstack[(*J).envtop as usize];
}
unsafe extern "C" fn jsR_calllwfunction(
    J: *mut js_State,
    mut n: libc::c_int,
    F: *mut js_Function,
    scope: *mut js_Environment,
) {
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    let mut i: libc::c_int = 0;
    jsR_savescope(J, scope);
    if n > (*F).numparams {
        js_pop(J, n - (*F).numparams);
        n = (*F).numparams;
    }
    i = n;
    while i < (*F).varlen {
        js_pushundefined(J);
        i += 1;
        i;
    }
    jsR_run(J, F);
    v = *stackidx(J, -(1 as libc::c_int));
    (*J).bot -= 1;
    (*J).top = (*J).bot;
    js_pushvalue(J, v);
    jsR_restorescope(J);
}
unsafe extern "C" fn jsR_callfunction(
    J: *mut js_State,
    n: libc::c_int,
    F: *mut js_Function,
    mut scope: *mut js_Environment,
) {
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    let mut i: libc::c_int = 0;
    scope = jsR_newenvironment(
        J,
        jsV_newobject(J, JS_COBJECT, std::ptr::null_mut::<js_Object>()),
        scope,
    );
    jsR_savescope(J, scope);
    if (*F).arguments != 0 {
        js_newarguments(J);
        if (*J).strict == 0 {
            js_currentfunction(J);
            js_defproperty(
                J,
                -(2 as libc::c_int),
                b"callee\0" as *const u8 as *const libc::c_char,
                JS_DONTENUM as libc::c_int,
            );
        }
        js_pushnumber(J, n as libc::c_double);
        js_defproperty(
            J,
            -(2 as libc::c_int),
            b"length\0" as *const u8 as *const libc::c_char,
            JS_DONTENUM as libc::c_int,
        );
        i = 0 as libc::c_int;
        while i < n {
            js_copy(J, i + 1 as libc::c_int);
            js_setindex(J, -(2 as libc::c_int), i);
            i += 1;
            i;
        }
        js_initvar(
            J,
            b"arguments\0" as *const u8 as *const libc::c_char,
            -(1 as libc::c_int),
        );
        js_pop(J, 1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < n && i < (*F).numparams {
        js_initvar(J, *((*F).vartab).offset(i as isize), i + 1 as libc::c_int);
        i += 1;
        i;
    }
    js_pop(J, n);
    while i < (*F).varlen {
        js_pushundefined(J);
        js_initvar(J, *((*F).vartab).offset(i as isize), -(1 as libc::c_int));
        js_pop(J, 1 as libc::c_int);
        i += 1;
        i;
    }
    jsR_run(J, F);
    v = *stackidx(J, -(1 as libc::c_int));
    (*J).bot -= 1;
    (*J).top = (*J).bot;
    js_pushvalue(J, v);
    jsR_restorescope(J);
}
unsafe extern "C" fn jsR_callscript(
    J: *mut js_State,
    n: libc::c_int,
    F: *mut js_Function,
    scope: *mut js_Environment,
) {
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    let mut i: libc::c_int = 0;
    if !scope.is_null() {
        jsR_savescope(J, scope);
    }
    js_pop(J, n);
    i = 0 as libc::c_int;
    while i < (*F).varlen {
        if js_hasvar(J, *((*F).vartab).offset(i as isize)) == 0 {
            js_pushundefined(J);
            js_initvar(J, *((*F).vartab).offset(i as isize), -(1 as libc::c_int));
            js_pop(J, 1 as libc::c_int);
        }
        i += 1;
        i;
    }
    jsR_run(J, F);
    v = *stackidx(J, -(1 as libc::c_int));
    (*J).bot -= 1;
    (*J).top = (*J).bot;
    js_pushvalue(J, v);
    if !scope.is_null() {
        jsR_restorescope(J);
    }
}
unsafe extern "C" fn jsR_callcfunction(
    J: *mut js_State,
    n: libc::c_int,
    min: libc::c_int,
    F: js_CFunction,
) {
    let mut save_top: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    i = n;
    while i < min {
        js_pushundefined(J);
        i += 1;
        i;
    }
    save_top = (*J).top;
    F.expect("non-null function pointer")(J);
    if (*J).top > save_top {
        v = *stackidx(J, -(1 as libc::c_int));
        (*J).bot -= 1;
        (*J).top = (*J).bot;
        js_pushvalue(J, v);
    } else {
        (*J).bot -= 1;
        (*J).top = (*J).bot;
        js_pushundefined(J);
    };
}
unsafe extern "C" fn jsR_pushtrace(
    J: *mut js_State,
    name: *const libc::c_char,
    file: *const libc::c_char,
    line: libc::c_int,
) {
    if (*J).tracetop + 1 as libc::c_int == 1024 as libc::c_int {
        js_error(
            J,
            b"call stack overflow\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).tracetop += 1;
    (*J).tracetop;
    (*J).trace[(*J).tracetop as usize].name = name;
    (*J).trace[(*J).tracetop as usize].file = file;
    (*J).trace[(*J).tracetop as usize].line = line;
}
#[no_mangle]
pub unsafe extern "C" fn js_call(J: *mut js_State, n: libc::c_int) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut savebot: libc::c_int = 0;
    if n < 0 as libc::c_int {
        js_rangeerror(
            J,
            b"number of arguments cannot be negative\0" as *const u8 as *const libc::c_char,
        );
    }
    if js_iscallable(J, -n - 2 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"%s is not callable\0" as *const u8 as *const libc::c_char,
            js_typeof(J, -n - 2 as libc::c_int),
        );
    }
    obj = js_toobject(J, -n - 2 as libc::c_int);
    savebot = (*J).bot;
    (*J).bot = (*J).top - n - 1 as libc::c_int;
    if (*obj).type_0 as libc::c_uint == JS_CFUNCTION as libc::c_int as libc::c_uint {
        jsR_pushtrace(
            J,
            (*(*obj).u.f.function).name,
            (*(*obj).u.f.function).filename,
            (*(*obj).u.f.function).line,
        );
        if (*(*obj).u.f.function).lightweight != 0 {
            jsR_calllwfunction(J, n, (*obj).u.f.function, (*obj).u.f.scope);
        } else {
            jsR_callfunction(J, n, (*obj).u.f.function, (*obj).u.f.scope);
        }
        (*J).tracetop -= 1;
        (*J).tracetop;
    } else if (*obj).type_0 as libc::c_uint == JS_CSCRIPT as libc::c_int as libc::c_uint {
        jsR_pushtrace(
            J,
            (*(*obj).u.f.function).name,
            (*(*obj).u.f.function).filename,
            (*(*obj).u.f.function).line,
        );
        jsR_callscript(J, n, (*obj).u.f.function, (*obj).u.f.scope);
        (*J).tracetop -= 1;
        (*J).tracetop;
    } else if (*obj).type_0 as libc::c_uint == JS_CCFUNCTION as libc::c_int as libc::c_uint {
        jsR_pushtrace(
            J,
            (*obj).u.c.name,
            b"native\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        jsR_callcfunction(J, n, (*obj).u.c.length, (*obj).u.c.function);
        (*J).tracetop -= 1;
        (*J).tracetop;
    }
    (*J).bot = savebot;
}
#[no_mangle]
pub unsafe extern "C" fn js_construct(J: *mut js_State, n: libc::c_int) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut prototype: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut newobj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_iscallable(J, -n - 1 as libc::c_int) == 0 {
        js_typeerror(
            J,
            b"%s is not callable\0" as *const u8 as *const libc::c_char,
            js_typeof(J, -n - 1 as libc::c_int),
        );
    }
    obj = js_toobject(J, -n - 1 as libc::c_int);
    if (*obj).type_0 as libc::c_uint == JS_CCFUNCTION as libc::c_int as libc::c_uint
        && ((*obj).u.c.constructor).is_some()
    {
        let savebot: libc::c_int = (*J).bot;
        js_pushnull(J);
        if n > 0 as libc::c_int {
            js_rot(J, n + 1 as libc::c_int);
        }
        (*J).bot = (*J).top - n - 1 as libc::c_int;
        jsR_pushtrace(
            J,
            (*obj).u.c.name,
            b"native\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        jsR_callcfunction(J, n, (*obj).u.c.length, (*obj).u.c.constructor);
        (*J).tracetop -= 1;
        (*J).tracetop;
        (*J).bot = savebot;
        return;
    }
    js_getproperty(
        J,
        -n - 1 as libc::c_int,
        b"prototype\0" as *const u8 as *const libc::c_char,
    );
    if js_isobject(J, -(1 as libc::c_int)) != 0 {
        prototype = js_toobject(J, -(1 as libc::c_int));
    } else {
        prototype = (*J).Object_prototype;
    }
    js_pop(J, 1 as libc::c_int);
    newobj = jsV_newobject(J, JS_COBJECT, prototype);
    js_pushobject(J, newobj);
    if n > 0 as libc::c_int {
        js_rot(J, n + 1 as libc::c_int);
    }
    js_pushobject(J, newobj);
    js_rot(J, n + 3 as libc::c_int);
    js_call(J, n);
    if js_isobject(J, -(1 as libc::c_int)) == 0 {
        js_pop(J, 1 as libc::c_int);
    } else {
        js_rot2pop1(J);
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_eval(J: *mut js_State) {
    if js_isstring(J, -(1 as libc::c_int)) == 0 {
        return;
    }
    js_loadeval(
        J,
        b"(eval)\0" as *const u8 as *const libc::c_char,
        js_tostring(J, -(1 as libc::c_int)),
    );
    js_rot2pop1(J);
    js_copy(J, 0 as libc::c_int);
    js_call(J, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_pconstruct(J: *mut js_State, n: libc::c_int) -> libc::c_int {
    let savetop: libc::c_int = (*J).top - n - 2 as libc::c_int;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        *((*J).stack).offset(savetop as isize) =
            *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
        (*J).top = savetop + 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    js_construct(J, n);
    js_endtry(J);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_pcall(J: *mut js_State, n: libc::c_int) -> libc::c_int {
    let savetop: libc::c_int = (*J).top - n - 2 as libc::c_int;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        *((*J).stack).offset(savetop as isize) =
            *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
        (*J).top = savetop + 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    js_call(J, n);
    js_endtry(J);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_savetrypc(
    J: *mut js_State,
    pc: *mut js_Instruction,
) -> *mut libc::c_void {
    if (*J).trytop == 64 as libc::c_int {
        js_trystackoverflow(J);
    }
    (*J).trybuf[(*J).trytop as usize].E = (*J).E;
    (*J).trybuf[(*J).trytop as usize].envtop = (*J).envtop;
    (*J).trybuf[(*J).trytop as usize].tracetop = (*J).tracetop;
    (*J).trybuf[(*J).trytop as usize].top = (*J).top;
    (*J).trybuf[(*J).trytop as usize].bot = (*J).bot;
    (*J).trybuf[(*J).trytop as usize].strict = (*J).strict;
    (*J).trybuf[(*J).trytop as usize].pc = pc;
    let fresh69 = (*J).trytop;
    (*J).trytop += 1;
    ((*J).trybuf[fresh69 as usize].buf).as_mut_ptr() as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn js_savetry(J: *mut js_State) -> *mut libc::c_void {
    if (*J).trytop == 64 as libc::c_int {
        js_trystackoverflow(J);
    }
    (*J).trybuf[(*J).trytop as usize].E = (*J).E;
    (*J).trybuf[(*J).trytop as usize].envtop = (*J).envtop;
    (*J).trybuf[(*J).trytop as usize].tracetop = (*J).tracetop;
    (*J).trybuf[(*J).trytop as usize].top = (*J).top;
    (*J).trybuf[(*J).trytop as usize].bot = (*J).bot;
    (*J).trybuf[(*J).trytop as usize].strict = (*J).strict;
    (*J).trybuf[(*J).trytop as usize].pc = std::ptr::null_mut::<js_Instruction>();
    let fresh70 = (*J).trytop;
    (*J).trytop += 1;
    ((*J).trybuf[fresh70 as usize].buf).as_mut_ptr() as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn js_endtry(J: *mut js_State) {
    if (*J).trytop == 0 as libc::c_int {
        js_error(
            J,
            b"endtry: exception stack underflow\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).trytop -= 1;
    (*J).trytop;
}
#[no_mangle]
pub unsafe extern "C" fn js_throw(J: *mut js_State) -> ! {
    if (*J).trytop > 0 as libc::c_int {
        let v: js_Value = *stackidx(J, -(1 as libc::c_int));
        (*J).trytop -= 1;
        (*J).trytop;
        (*J).E = (*J).trybuf[(*J).trytop as usize].E;
        (*J).envtop = (*J).trybuf[(*J).trytop as usize].envtop;
        (*J).tracetop = (*J).trybuf[(*J).trytop as usize].tracetop;
        (*J).top = (*J).trybuf[(*J).trytop as usize].top;
        (*J).bot = (*J).trybuf[(*J).trytop as usize].bot;
        (*J).strict = (*J).trybuf[(*J).trytop as usize].strict;
        js_pushvalue(J, v);
        longjmp(
            ((*J).trybuf[(*J).trytop as usize].buf).as_mut_ptr(),
            1 as libc::c_int,
        );
    }
    if ((*J).panic).is_some() {
        ((*J).panic).expect("non-null function pointer")(J);
    }
    abort();
}
unsafe extern "C" fn js_dumpvalue(J: *mut js_State, mut v: js_Value) {
    match v.t.type_0 as libc::c_int {
        1 => {
            printf(b"undefined\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"null\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(if v.u.boolean != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            });
        }
        4 => {
            printf(b"%.9g\0" as *const u8 as *const libc::c_char, v.u.number);
        }
        0 => {
            printf(
                b"'%s'\0" as *const u8 as *const libc::c_char,
                (v.u.shrstr).as_mut_ptr(),
            );
        }
        5 => {
            printf(b"'%s'\0" as *const u8 as *const libc::c_char, v.u.litstr);
        }
        6 => {
            printf(
                b"'%s'\0" as *const u8 as *const libc::c_char,
                ((*v.u.memstr).p).as_mut_ptr(),
            );
        }
        7 => {
            if v.u.object == (*J).G {
                printf(b"[Global]\0" as *const u8 as *const libc::c_char);
            } else {
                match (*v.u.object).type_0 as libc::c_uint {
                    0 => {
                        printf(
                            b"[Object %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    1 => {
                        printf(
                            b"[Array %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    2 => {
                        printf(
                            b"[Function %p, %s, %s:%d]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                            (*(*v.u.object).u.f.function).name,
                            (*(*v.u.object).u.f.function).filename,
                            (*(*v.u.object).u.f.function).line,
                        );
                    }
                    3 => {
                        printf(
                            b"[Script %s]\0" as *const u8 as *const libc::c_char,
                            (*(*v.u.object).u.f.function).filename,
                        );
                    }
                    4 => {
                        printf(
                            b"[CFunction %s]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.c.name,
                        );
                    }
                    6 => {
                        printf(
                            b"[Boolean %d]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.boolean,
                        );
                    }
                    7 => {
                        printf(
                            b"[Number %g]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.number,
                        );
                    }
                    8 => {
                        printf(
                            b"[String'%s']\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.s.string,
                        );
                    }
                    5 => {
                        printf(b"[Error]\0" as *const u8 as *const libc::c_char);
                    }
                    13 => {
                        printf(
                            b"[Arguments %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    14 => {
                        printf(
                            b"[Iterator %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    15 => {
                        printf(
                            b"[Userdata %s %p]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.user.tag,
                            (*v.u.object).u.user.data,
                        );
                    }
                    _ => {
                        printf(
                            b"[Object %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn js_stacktrace(J: *mut js_State) {
    let mut n: libc::c_int = 0;
    printf(b"stack trace:\n\0" as *const u8 as *const libc::c_char);
    n = (*J).tracetop;
    while n >= 0 as libc::c_int {
        let name: *const libc::c_char = (*J).trace[n as usize].name;
        let file: *const libc::c_char = (*J).trace[n as usize].file;
        let line: libc::c_int = (*J).trace[n as usize].line;
        if line > 0 as libc::c_int {
            if *name.offset(0 as libc::c_int as isize) != 0 {
                printf(
                    b"\tat %s (%s:%d)\n\0" as *const u8 as *const libc::c_char,
                    name,
                    file,
                    line,
                );
            } else {
                printf(
                    b"\tat %s:%d\n\0" as *const u8 as *const libc::c_char,
                    file,
                    line,
                );
            }
        } else {
            printf(
                b"\tat %s (%s)\n\0" as *const u8 as *const libc::c_char,
                name,
                file,
            );
        }
        n -= 1;
        n;
    }
}
unsafe extern "C" fn js_dumpstack(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    printf(b"stack {\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < (*J).top {
        putchar(if i == (*J).bot {
            '>' as i32
        } else {
            ' ' as i32
        });
        printf(b"%4d: \0" as *const u8 as *const libc::c_char, i);
        js_dumpvalue(J, *((*J).stack).offset(i as isize));
        putchar('\n' as i32);
        i += 1;
        i;
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn js_trap(J: *mut js_State, pc: libc::c_int) {
    js_dumpstack(J);
    js_stacktrace(J);
}
unsafe extern "C" fn jsR_isindex(
    J: *mut js_State,
    idx: libc::c_int,
    k: *mut libc::c_int,
) -> libc::c_int {
    let v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int {
        *k = (*v).u.number as libc::c_int;
        return (*k as libc::c_double == (*v).u.number && *k >= 0 as libc::c_int) as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn jsR_run(J: *mut js_State, F: *mut js_Function) {
    let FT: *mut *mut js_Function = (*F).funtab;
    // let mut VT: *mut *const libc::c_char = ((*F).vartab).offset(-1);
    let lightweight: libc::c_int = (*F).lightweight;
    let pcstart: *mut js_Instruction = (*F).code;
    let mut pc: *mut js_Instruction = (*F).code;
    let mut opcode: js_OpCode = OP_POP;
    let mut offset: libc::c_int = 0;
    let mut savestrict: libc::c_int = 0;
    let mut str: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut x: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut ux: libc::c_uint = 0;
    let mut uy: libc::c_uint = 0;
    let mut ix: libc::c_int = 0;
    let mut iy: libc::c_int = 0;
    let mut okay: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut transient: libc::c_int = 0;
    savestrict = (*J).strict;
    (*J).strict = (*F).strict;
    loop {
        if (*J).gccounter > (*J).gcthresh {
            js_gc(J, 0 as libc::c_int);
        }
        let fresh71 = pc;
        pc = pc.offset(1);
        (*J).trace[(*J).tracetop as usize].line = *fresh71 as libc::c_int;
        let fresh72 = pc;
        pc = pc.offset(1);
        opcode = *fresh72 as js_OpCode;
        match opcode as libc::c_uint {
            0 => {
                js_pop(J, 1 as libc::c_int);
            }
            1 => {
                js_dup(J);
            }
            2 => {
                js_dup2(J);
            }
            3 => {
                js_rot2(J);
            }
            4 => {
                js_rot3(J);
            }
            5 => {
                js_rot4(J);
            }
            6 => {
                let fresh73 = pc;
                pc = pc.offset(1);
                js_pushnumber(
                    J,
                    (*fresh73 as libc::c_int - 32768 as libc::c_int) as libc::c_double,
                );
            }
            7 => {
                memcpy(
                    &mut x as *mut libc::c_double as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                js_pushnumber(J, x);
            }
            8 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                js_pushliteral(J, str);
            }
            9 => {
                let fresh74 = pc;
                pc = pc.offset(1);
                js_newfunction(J, *FT.offset(*fresh74 as isize), (*J).E);
            }
            11 => {
                js_newobject(J);
            }
            10 => {
                js_newarray(J);
            }
            12 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                let fresh75 = pc;
                pc = pc.offset(1);
                js_newregexp(J, str, *fresh75 as libc::c_int);
            }
            13 => {
                js_pushundefined(J);
            }
            14 => {
                js_pushnull(J);
            }
            15 => {
                js_pushboolean(J, 1 as libc::c_int);
            }
            16 => {
                js_pushboolean(J, 0 as libc::c_int);
            }
            17 => {
                if (*J).strict != 0 {
                    js_copy(J, 0 as libc::c_int);
                } else if js_iscoercible(J, 0 as libc::c_int) != 0 {
                    js_copy(J, 0 as libc::c_int);
                } else {
                    js_pushglobal(J);
                }
            }
            18 => {
                js_currentfunction(J);
            }
            19 => {
                let VT: *mut *const libc::c_char = ((*F).vartab).offset(-1);
                if lightweight != 0 {
                    if (*J).top + 1 as libc::c_int >= 4096 as libc::c_int {
                        js_stackoverflow(J);
                    }
                    let fresh76 = pc;
                    pc = pc.offset(1);
                    let fresh77 = (*J).top;
                    (*J).top += 1;
                    *((*J).stack).offset(fresh77 as isize) =
                        *((*J).stack).offset(((*J).bot + *fresh76 as libc::c_int) as isize);
                } else {
                    let fresh78 = pc;
                    pc = pc.offset(1);
                    str = *VT.offset(*fresh78 as isize);
                    if js_hasvar(J, str) == 0 {
                        js_referenceerror(
                            J,
                            b"'%s' is not defined\0" as *const u8 as *const libc::c_char,
                            str,
                        );
                    }
                }
            }
            20 => {
                let VT: *mut *const libc::c_char = ((*F).vartab).offset(-1);
                if lightweight != 0 {
                    let fresh79 = pc;
                    pc = pc.offset(1);
                    *((*J).stack).offset(((*J).bot + *fresh79 as libc::c_int) as isize) =
                        *((*J).stack).offset(((*J).top - 1 as libc::c_int) as isize);
                } else {
                    let fresh80 = pc;
                    pc = pc.offset(1);
                    js_setvar(J, *VT.offset(*fresh80 as isize));
                }
            }
            21 => {
                let VT: *mut *const libc::c_char = ((*F).vartab).offset(-1);
                if lightweight != 0 {
                    pc = pc.offset(1);
                    pc;
                    js_pushboolean(J, 0 as libc::c_int);
                } else {
                    let fresh81 = pc;
                    pc = pc.offset(1);
                    b = js_delvar(J, *VT.offset(*fresh81 as isize));
                    js_pushboolean(J, b);
                }
            }
            23 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                if js_hasvar(J, str) == 0 {
                    js_referenceerror(
                        J,
                        b"'%s' is not defined\0" as *const u8 as *const libc::c_char,
                        str,
                    );
                }
            }
            22 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                if js_hasvar(J, str) == 0 {
                    js_pushundefined(J);
                }
            }
            24 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                js_setvar(J, str);
            }
            25 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                b = js_delvar(J, str);
                js_pushboolean(J, b);
            }
            26 => {
                str = js_tostring(J, -(2 as libc::c_int));
                if js_isobject(J, -(1 as libc::c_int)) == 0 {
                    js_typeerror(
                        J,
                        b"operand to 'in' is not an object\0" as *const u8 as *const libc::c_char,
                    );
                }
                b = js_hasproperty(J, -(1 as libc::c_int), str);
                js_pop(J, 2 as libc::c_int + b);
                js_pushboolean(J, b);
            }
            27 => {
                js_setlength(
                    J,
                    -(1 as libc::c_int),
                    js_getlength(J, -(1 as libc::c_int)) + 1 as libc::c_int,
                );
            }
            28 => {
                js_setindex(J, -(2 as libc::c_int), js_getlength(J, -(2 as libc::c_int)));
            }
            29 => {
                obj = js_toobject(J, -(3 as libc::c_int));
                str = js_tostring(J, -(2 as libc::c_int));
                jsR_setproperty(J, obj, str, 0 as libc::c_int);
                js_pop(J, 2 as libc::c_int);
            }
            30 => {
                obj = js_toobject(J, -(3 as libc::c_int));
                str = js_tostring(J, -(2 as libc::c_int));
                jsR_defproperty(
                    J,
                    obj,
                    str,
                    0 as libc::c_int,
                    std::ptr::null_mut::<js_Value>(),
                    jsR_tofunction(J, -(1 as libc::c_int)),
                    std::ptr::null_mut::<js_Object>(),
                    0 as libc::c_int,
                );
                js_pop(J, 2 as libc::c_int);
            }
            31 => {
                obj = js_toobject(J, -(3 as libc::c_int));
                str = js_tostring(J, -(2 as libc::c_int));
                jsR_defproperty(
                    J,
                    obj,
                    str,
                    0 as libc::c_int,
                    std::ptr::null_mut::<js_Value>(),
                    std::ptr::null_mut::<js_Object>(),
                    jsR_tofunction(J, -(1 as libc::c_int)),
                    0 as libc::c_int,
                );
                js_pop(J, 2 as libc::c_int);
            }
            32 => {
                if jsR_isindex(J, -(1 as libc::c_int), &mut ix) != 0 {
                    obj = js_toobject(J, -(2 as libc::c_int));
                    jsR_getindex(J, obj, ix);
                } else {
                    str = js_tostring(J, -(1 as libc::c_int));
                    obj = js_toobject(J, -(2 as libc::c_int));
                    jsR_getproperty(J, obj, str);
                }
                js_rot3pop2(J);
            }
            33 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                obj = js_toobject(J, -(1 as libc::c_int));
                jsR_getproperty(J, obj, str);
                js_rot2pop1(J);
            }
            34 => {
                if jsR_isindex(J, -(2 as libc::c_int), &mut ix) != 0 {
                    obj = js_toobject(J, -(3 as libc::c_int));
                    transient = (js_isobject(J, -(3 as libc::c_int)) == 0) as libc::c_int;
                    jsR_setindex(J, obj, ix, transient);
                } else {
                    str = js_tostring(J, -(2 as libc::c_int));
                    obj = js_toobject(J, -(3 as libc::c_int));
                    transient = (js_isobject(J, -(3 as libc::c_int)) == 0) as libc::c_int;
                    jsR_setproperty(J, obj, str, transient);
                }
                js_rot3pop2(J);
            }
            35 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                obj = js_toobject(J, -(2 as libc::c_int));
                transient = (js_isobject(J, -(2 as libc::c_int)) == 0) as libc::c_int;
                jsR_setproperty(J, obj, str, transient);
                js_rot2pop1(J);
            }
            36 => {
                str = js_tostring(J, -(1 as libc::c_int));
                obj = js_toobject(J, -(2 as libc::c_int));
                b = jsR_delproperty(J, obj, str);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, b);
            }
            37 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                obj = js_toobject(J, -(1 as libc::c_int));
                b = jsR_delproperty(J, obj, str);
                js_pop(J, 1 as libc::c_int);
                js_pushboolean(J, b);
            }
            38 => {
                if js_iscoercible(J, -(1 as libc::c_int)) != 0 {
                    obj = jsV_newiterator(J, js_toobject(J, -(1 as libc::c_int)), 0 as libc::c_int);
                    js_pop(J, 1 as libc::c_int);
                    js_pushobject(J, obj);
                }
            }
            39 => {
                if js_isobject(J, -(1 as libc::c_int)) != 0 {
                    obj = js_toobject(J, -(1 as libc::c_int));
                    str = jsV_nextiterator(J, obj);
                    if !str.is_null() {
                        js_pushstring(J, str);
                        js_pushboolean(J, 1 as libc::c_int);
                    } else {
                        js_pop(J, 1 as libc::c_int);
                        js_pushboolean(J, 0 as libc::c_int);
                    }
                } else {
                    js_pop(J, 1 as libc::c_int);
                    js_pushboolean(J, 0 as libc::c_int);
                }
            }
            40 => {
                js_eval(J);
            }
            41 => {
                let fresh82 = pc;
                pc = pc.offset(1);
                js_call(J, *fresh82 as libc::c_int);
            }
            42 => {
                let fresh83 = pc;
                pc = pc.offset(1);
                js_construct(J, *fresh83 as libc::c_int);
            }
            43 => {
                str = js_typeof(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushliteral(J, str);
            }
            44 => {
                x = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushnumber(J, x);
            }
            45 => {
                x = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushnumber(J, -x);
            }
            46 => {
                ix = js_toint32(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushnumber(J, !ix as libc::c_double);
            }
            47 => {
                b = js_toboolean(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushboolean(J, (b == 0) as libc::c_int);
            }
            48 => {
                x = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushnumber(J, x + 1 as libc::c_int as libc::c_double);
            }
            49 => {
                x = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushnumber(J, x - 1 as libc::c_int as libc::c_double);
            }
            50 => {
                x = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushnumber(J, x + 1 as libc::c_int as libc::c_double);
                js_pushnumber(J, x);
            }
            51 => {
                x = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                js_pushnumber(J, x - 1 as libc::c_int as libc::c_double);
                js_pushnumber(J, x);
            }
            52 => {
                x = js_tonumber(J, -(2 as libc::c_int));
                y = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(J, x * y);
            }
            53 => {
                x = js_tonumber(J, -(2 as libc::c_int));
                y = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(J, x / y);
            }
            54 => {
                x = js_tonumber(J, -(2 as libc::c_int));
                y = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(J, fmod(x, y));
            }
            55 => {
                js_concat(J);
            }
            56 => {
                x = js_tonumber(J, -(2 as libc::c_int));
                y = js_tonumber(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(J, x - y);
            }
            57 => {
                ix = js_toint32(J, -(2 as libc::c_int));
                uy = js_touint32(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(
                    J,
                    (ix << (uy & 0x1f as libc::c_int as libc::c_uint)) as libc::c_double,
                );
            }
            58 => {
                ix = js_toint32(J, -(2 as libc::c_int));
                uy = js_touint32(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(
                    J,
                    (ix >> (uy & 0x1f as libc::c_int as libc::c_uint)) as libc::c_double,
                );
            }
            59 => {
                ux = js_touint32(J, -(2 as libc::c_int));
                uy = js_touint32(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(
                    J,
                    (ux >> (uy & 0x1f as libc::c_int as libc::c_uint)) as libc::c_double,
                );
            }
            60 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, (okay != 0 && b < 0 as libc::c_int) as libc::c_int);
            }
            61 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, (okay != 0 && b > 0 as libc::c_int) as libc::c_int);
            }
            62 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, (okay != 0 && b <= 0 as libc::c_int) as libc::c_int);
            }
            63 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, (okay != 0 && b >= 0 as libc::c_int) as libc::c_int);
            }
            72 => {
                b = js_instanceof(J);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, b);
            }
            64 => {
                b = js_equal(J);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, b);
            }
            65 => {
                b = js_equal(J);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, (b == 0) as libc::c_int);
            }
            66 => {
                b = js_strictequal(J);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, b);
            }
            67 => {
                b = js_strictequal(J);
                js_pop(J, 2 as libc::c_int);
                js_pushboolean(J, (b == 0) as libc::c_int);
            }
            68 => {
                let fresh84 = pc;
                pc = pc.offset(1);
                offset = *fresh84 as libc::c_int;
                b = js_strictequal(J);
                if b != 0 {
                    js_pop(J, 2 as libc::c_int);
                    pc = pcstart.offset(offset as isize);
                } else {
                    js_pop(J, 1 as libc::c_int);
                }
            }
            69 => {
                ix = js_toint32(J, -(2 as libc::c_int));
                iy = js_toint32(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(J, (ix & iy) as libc::c_double);
            }
            70 => {
                ix = js_toint32(J, -(2 as libc::c_int));
                iy = js_toint32(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(J, (ix ^ iy) as libc::c_double);
            }
            71 => {
                ix = js_toint32(J, -(2 as libc::c_int));
                iy = js_toint32(J, -(1 as libc::c_int));
                js_pop(J, 2 as libc::c_int);
                js_pushnumber(J, (ix | iy) as libc::c_double);
            }
            73 => {
                js_throw(J);
            }
            74 => {
                let fresh85 = pc;
                pc = pc.offset(1);
                offset = *fresh85 as libc::c_int;
                if _setjmp(js_savetrypc(J, pc) as *mut __jmp_buf_tag) != 0 {
                    pc = (*J).trybuf[(*J).trytop as usize].pc;
                } else {
                    pc = pcstart.offset(offset as isize);
                }
            }
            75 => {
                js_endtry(J);
            }
            76 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as libc::c_ulong)
                        as isize,
                );
                obj = jsV_newobject(J, JS_COBJECT, std::ptr::null_mut::<js_Object>());
                js_pushobject(J, obj);
                js_rot2(J);
                js_setproperty(J, -(2 as libc::c_int), str);
                (*J).E = jsR_newenvironment(J, obj, (*J).E);
                js_pop(J, 1 as libc::c_int);
            }
            77 => {
                (*J).E = (*(*J).E).outer;
            }
            78 => {
                obj = js_toobject(J, -(1 as libc::c_int));
                (*J).E = jsR_newenvironment(J, obj, (*J).E);
                js_pop(J, 1 as libc::c_int);
            }
            79 => {
                (*J).E = (*(*J).E).outer;
            }
            80 => {
                js_trap(
                    J,
                    pc.offset_from(pcstart) as libc::c_long as libc::c_int - 1 as libc::c_int,
                );
            }
            81 => {
                pc = pcstart.offset(*pc as libc::c_int as isize);
            }
            82 => {
                let fresh86 = pc;
                pc = pc.offset(1);
                offset = *fresh86 as libc::c_int;
                b = js_toboolean(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                if b != 0 {
                    pc = pcstart.offset(offset as isize);
                }
            }
            83 => {
                let fresh87 = pc;
                pc = pc.offset(1);
                offset = *fresh87 as libc::c_int;
                b = js_toboolean(J, -(1 as libc::c_int));
                js_pop(J, 1 as libc::c_int);
                if b == 0 {
                    pc = pcstart.offset(offset as isize);
                }
            }
            84 => {
                (*J).strict = savestrict;
                return;
            }
            _ => {}
        }
    }
}
unsafe extern "C" fn js_ptry(J: *mut js_State) -> libc::c_int {
    if (*J).trytop == 64 as libc::c_int {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TLITSTR as libc::c_int as libc::c_char;
        let fresh88 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
        *fresh88 = b"exception stack overflow\0" as *const u8 as *const libc::c_char;
        (*J).top += 1;
        (*J).top;
        return 1 as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn js_defaultalloc(
    actx: *mut libc::c_void,
    ptr: *mut libc::c_void,
    size: libc::c_int,
) -> *mut libc::c_void {
    if size == 0 as libc::c_int {
        free(ptr);
        return std::ptr::null_mut::<libc::c_void>();
    }
    realloc(ptr, size as size_t)
}
unsafe extern "C" fn js_defaultreport(J: *mut js_State, message: *const libc::c_char) {
    fputs(message, stderr);
    fputc('\n' as i32, stderr);
}
unsafe extern "C" fn js_defaultpanic(J: *mut js_State) {
    js_report(
        J,
        b"uncaught exception\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_ploadstring(
    J: *mut js_State,
    filename: *const libc::c_char,
    source: *const libc::c_char,
) -> libc::c_int {
    if js_ptry(J) != 0 {
        return 1 as libc::c_int;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        return 1 as libc::c_int;
    }
    js_loadstring(J, filename, source);
    js_endtry(J);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_ploadfile(
    J: *mut js_State,
    filename: *const libc::c_char,
) -> libc::c_int {
    if js_ptry(J) != 0 {
        return 1 as libc::c_int;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        return 1 as libc::c_int;
    }
    js_loadfile(J, filename);
    js_endtry(J);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_trystring(
    J: *mut js_State,
    idx: libc::c_int,
    error: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    if js_ptry(J) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    s = js_tostring(J, idx);
    js_endtry(J);
    s
}
#[no_mangle]
pub unsafe extern "C" fn js_trynumber(
    J: *mut js_State,
    idx: libc::c_int,
    error: libc::c_double,
) -> libc::c_double {
    let mut v: libc::c_double = 0.;
    if js_ptry(J) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    v = js_tonumber(J, idx);
    js_endtry(J);
    v
}
#[no_mangle]
pub unsafe extern "C" fn js_tryinteger(
    J: *mut js_State,
    idx: libc::c_int,
    error: libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    if js_ptry(J) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    v = js_tointeger(J, idx);
    js_endtry(J);
    v
}
#[no_mangle]
pub unsafe extern "C" fn js_tryboolean(
    J: *mut js_State,
    idx: libc::c_int,
    error: libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    if js_ptry(J) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1 as libc::c_int);
        return error;
    }
    v = js_toboolean(J, idx);
    js_endtry(J);
    v
}
unsafe extern "C" fn js_loadstringx(
    J: *mut js_State,
    filename: *const libc::c_char,
    source: *const libc::c_char,
    iseval: libc::c_int,
) {
    let mut P: *mut js_Ast = std::ptr::null_mut::<js_Ast>();
    let mut F: *mut js_Function = std::ptr::null_mut::<js_Function>();
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        jsP_freeparse(J);
        js_throw(J);
    }
    P = jsP_parse(J, filename, source);
    F = jsC_compilescript(
        J,
        P,
        if iseval != 0 {
            (*J).strict
        } else {
            (*J).default_strict
        },
    );
    jsP_freeparse(J);
    js_newscript(
        J,
        F,
        if iseval != 0 {
            if (*J).strict != 0 {
                (*J).E
            } else {
                std::ptr::null_mut::<js_Environment>()
            }
        } else {
            (*J).GE
        },
    );
    js_endtry(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_loadeval(
    J: *mut js_State,
    filename: *const libc::c_char,
    source: *const libc::c_char,
) {
    js_loadstringx(J, filename, source, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_loadstring(
    J: *mut js_State,
    filename: *const libc::c_char,
    source: *const libc::c_char,
) {
    js_loadstringx(J, filename, source, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn js_loadfile(J: *mut js_State, filename: *const libc::c_char) {
    let mut f: *mut FILE = std::ptr::null_mut::<FILE>();
    let mut s: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut n: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    f = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        js_error(
            J,
            b"cannot open file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    if fseek(f, 0 as libc::c_int as libc::c_long, 2 as libc::c_int) < 0 as libc::c_int {
        fclose(f);
        js_error(
            J,
            b"cannot seek in file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    n = ftell(f) as libc::c_int;
    if n < 0 as libc::c_int {
        fclose(f);
        js_error(
            J,
            b"cannot tell in file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    if fseek(f, 0 as libc::c_int as libc::c_long, 0 as libc::c_int) < 0 as libc::c_int {
        fclose(f);
        js_error(
            J,
            b"cannot seek in file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        fclose(f);
        js_throw(J);
    }
    s = js_malloc(J, n + 1 as libc::c_int) as *mut libc::c_char;
    js_endtry(J);
    t = fread(
        s as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        n as size_t,
        f,
    ) as libc::c_int;
    if t != n {
        js_free(J, s as *mut libc::c_void);
        fclose(f);
        js_error(
            J,
            b"cannot read data from file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    *s.offset(n as isize) = 0 as libc::c_int as libc::c_char;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, s as *mut libc::c_void);
        fclose(f);
        js_throw(J);
    }
    p = s;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
        && *p.offset(1 as libc::c_int as isize) as libc::c_int == '!' as i32
    {
        p = p.offset(2 as libc::c_int as isize);
        while *p as libc::c_int != 0 && *p as libc::c_int != '\n' as i32 {
            p = p.offset(1);
            p;
        }
    }
    js_loadstring(J, filename, p);
    js_free(J, s as *mut libc::c_void);
    fclose(f);
    js_endtry(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_dostring(J: *mut js_State, source: *const libc::c_char) -> libc::c_int {
    if js_ptry(J) != 0 {
        js_report(
            J,
            b"exception stack overflow\0" as *const u8 as *const libc::c_char,
        );
        js_pop(J, 1 as libc::c_int);
        return 1 as libc::c_int;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_report(
            J,
            js_trystring(
                J,
                -(1 as libc::c_int),
                b"Error\0" as *const u8 as *const libc::c_char,
            ),
        );
        js_pop(J, 1 as libc::c_int);
        return 1 as libc::c_int;
    }
    js_loadstring(J, b"[string]\0" as *const u8 as *const libc::c_char, source);
    js_pushundefined(J);
    js_call(J, 0 as libc::c_int);
    js_pop(J, 1 as libc::c_int);
    js_endtry(J);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_dofile(J: *mut js_State, filename: *const libc::c_char) -> libc::c_int {
    if js_ptry(J) != 0 {
        js_report(
            J,
            b"exception stack overflow\0" as *const u8 as *const libc::c_char,
        );
        js_pop(J, 1 as libc::c_int);
        return 1 as libc::c_int;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_report(
            J,
            js_trystring(
                J,
                -(1 as libc::c_int),
                b"Error\0" as *const u8 as *const libc::c_char,
            ),
        );
        js_pop(J, 1 as libc::c_int);
        return 1 as libc::c_int;
    }
    js_loadfile(J, filename);
    js_pushundefined(J);
    js_call(J, 0 as libc::c_int);
    js_pop(J, 1 as libc::c_int);
    js_endtry(J);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_atpanic(J: *mut js_State, panic: js_Panic) -> js_Panic {
    let old: js_Panic = (*J).panic;
    (*J).panic = panic;
    old
}
#[no_mangle]
pub unsafe extern "C" fn js_report(J: *mut js_State, message: *const libc::c_char) {
    if ((*J).report).is_some() {
        ((*J).report).expect("non-null function pointer")(J, message);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_setreport(J: *mut js_State, report: js_Report) {
    (*J).report = report;
}
#[no_mangle]
pub unsafe extern "C" fn js_setcontext(J: *mut js_State, uctx: *mut libc::c_void) {
    (*J).uctx = uctx;
}
#[no_mangle]
pub unsafe extern "C" fn js_getcontext(J: *mut js_State) -> *mut libc::c_void {
    (*J).uctx
}
#[no_mangle]
pub unsafe extern "C" fn js_newstate(
    mut alloc: js_Alloc,
    actx: *mut libc::c_void,
    flags: libc::c_int,
) -> *mut js_State {
    let mut J: *mut js_State = std::ptr::null_mut::<js_State>();
    if ::core::mem::size_of::<js_Value>() as libc::c_ulong == 16 as libc::c_int as libc::c_ulong {
    } else {
        __assert_fail(
            b"sizeof(js_Value) == 16\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            12724 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"js_State *js_newstate(js_Alloc, void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_90618: {
        if ::core::mem::size_of::<js_Value>() as libc::c_ulong == 16 as libc::c_int as libc::c_ulong
        {
        } else {
            __assert_fail(
                b"sizeof(js_Value) == 16\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                12724 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"js_State *js_newstate(js_Alloc, void *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if 15 as libc::c_ulong as libc::c_int == 15 as libc::c_int {
    } else {
        __assert_fail(
            b"soffsetof(js_Value, t.type) == 15\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            12725 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"js_State *js_newstate(js_Alloc, void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    'c_90579: {
        if 15 as libc::c_ulong as libc::c_int == 15 as libc::c_int {
        } else {
            __assert_fail(
                b"soffsetof(js_Value, t.type) == 15\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                12725 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"js_State *js_newstate(js_Alloc, void *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if alloc.is_none() {
        alloc = Some(
            js_defaultalloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> *mut libc::c_void,
        );
    }
    J = alloc.expect("non-null function pointer")(
        actx,
        std::ptr::null_mut::<libc::c_void>(),
        ::core::mem::size_of::<js_State>() as libc::c_ulong as libc::c_int,
    ) as *mut js_State;
    if J.is_null() {
        return std::ptr::null_mut::<js_State>();
    }
    memset(
        J as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<js_State>() as libc::c_ulong,
    );
    (*J).actx = actx;
    (*J).alloc = alloc;
    if flags & JS_STRICT as libc::c_int != 0 {
        (*J).default_strict = 1 as libc::c_int;
        (*J).strict = (*J).default_strict;
    }
    (*J).trace[0 as libc::c_int as usize].name = b"-top-\0" as *const u8 as *const libc::c_char;
    (*J).trace[0 as libc::c_int as usize].file = b"native\0" as *const u8 as *const libc::c_char;
    (*J).trace[0 as libc::c_int as usize].line = 0 as libc::c_int;
    (*J).report =
        Some(js_defaultreport as unsafe extern "C" fn(*mut js_State, *const libc::c_char) -> ());
    (*J).panic = Some(js_defaultpanic as unsafe extern "C" fn(*mut js_State) -> ());
    (*J).stack = alloc.expect("non-null function pointer")(
        actx,
        std::ptr::null_mut::<libc::c_void>(),
        (4096 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<js_Value>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut js_Value;
    if ((*J).stack).is_null() {
        alloc.expect("non-null function pointer")(actx, J as *mut libc::c_void, 0 as libc::c_int);
        return std::ptr::null_mut::<js_State>();
    }
    (*J).gcmark = 1 as libc::c_int;
    (*J).nextref = 0 as libc::c_int;
    (*J).gcthresh = 0 as libc::c_int as libc::c_uint;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_freestate(J);
        return std::ptr::null_mut::<js_State>();
    }
    (*J).R = jsV_newobject(J, JS_COBJECT, std::ptr::null_mut::<js_Object>());
    (*J).G = jsV_newobject(J, JS_COBJECT, std::ptr::null_mut::<js_Object>());
    (*J).E = jsR_newenvironment(J, (*J).G, std::ptr::null_mut::<js_Environment>());
    (*J).GE = (*J).E;
    jsB_init(J);
    js_endtry(J);
    J
}
unsafe extern "C" fn js_doregexec(
    J: *mut js_State,
    prog: *mut Reprog,
    string: *const libc::c_char,
    sub: *mut Resub,
    eflags: libc::c_int,
) -> libc::c_int {
    let result: libc::c_int = js_regexec(prog, string, sub, eflags);
    if result < 0 as libc::c_int {
        js_error(J, b"regexec failed\0" as *const u8 as *const libc::c_char);
    }
    result
}
unsafe extern "C" fn checkstring(J: *mut js_State, idx: libc::c_int) -> *const libc::c_char {
    if js_iscoercible(J, idx) == 0 {
        js_typeerror(
            J,
            b"string function called on null or undefined\0" as *const u8 as *const libc::c_char,
        );
    }
    js_tostring(J, idx)
}
#[no_mangle]
pub unsafe extern "C" fn js_runeat(
    J: *mut js_State,
    mut s: *const libc::c_char,
    mut i: libc::c_int,
) -> libc::c_int {
    let mut rune: Rune = -(1 as libc::c_int);
    while i >= 0 as libc::c_int {
        rune = *(s as *mut libc::c_uchar) as Rune;
        if rune < Runeself as libc::c_int {
            if rune == 0 as libc::c_int {
                return -(1 as libc::c_int);
            }
            s = s.offset(1);
            s;
            i -= 1;
            i;
        } else {
            s = s.offset(jsU_chartorune(&mut rune, s) as isize);
            if rune >= 0x10000 as libc::c_int {
                i -= 2 as libc::c_int;
            } else {
                i -= 1;
                i;
            }
        }
    }
    if rune >= 0x10000 as libc::c_int {
        if i == -(2 as libc::c_int) {
            return 0xd800 as libc::c_int + ((rune - 0x10000 as libc::c_int) >> 10 as libc::c_int);
        } else {
            return 0xdc00 as libc::c_int
                + ((rune - 0x10000 as libc::c_int) & 0x3ff as libc::c_int);
        }
    }
    rune
}
#[no_mangle]
pub unsafe extern "C" fn js_utflen(mut s: *const libc::c_char) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut rune: Rune = 0;
    n = 0 as libc::c_int;
    loop {
        c = *(s as *mut libc::c_uchar) as libc::c_int;
        if c < Runeself as libc::c_int {
            if c == 0 as libc::c_int {
                return n;
            }
            s = s.offset(1);
            s;
            n += 1;
            n;
        } else {
            s = s.offset(jsU_chartorune(&mut rune, s) as isize);
            if rune >= 0x10000 as libc::c_int {
                n += 2 as libc::c_int;
            } else {
                n += 1;
                n;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_utfptrtoidx(
    mut s: *const libc::c_char,
    p: *const libc::c_char,
) -> libc::c_int {
    let mut rune: Rune = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while s < p {
        if (*(s as *mut libc::c_uchar) as libc::c_int) < Runeself as libc::c_int {
            s = s.offset(1);
            s;
        } else {
            s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        }
        if rune >= 0x10000 as libc::c_int {
            i += 2 as libc::c_int;
        } else {
            i += 1 as libc::c_int;
        }
    }
    i
}
unsafe extern "C" fn jsB_new_String(J: *mut js_State) {
    js_newstring(
        J,
        if js_gettop(J) > 1 as libc::c_int {
            js_tostring(J, 1 as libc::c_int)
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn jsB_String(J: *mut js_State) {
    js_pushstring(
        J,
        if js_gettop(J) > 1 as libc::c_int {
            js_tostring(J, 1 as libc::c_int)
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn Sp_toString(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    if (*self_0).type_0 as libc::c_uint != JS_CSTRING as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a string\0" as *const u8 as *const libc::c_char);
    }
    js_pushstring(J, (*self_0).u.s.string);
}
unsafe extern "C" fn Sp_valueOf(J: *mut js_State) {
    let self_0: *mut js_Object = js_toobject(J, 0 as libc::c_int);
    if (*self_0).type_0 as libc::c_uint != JS_CSTRING as libc::c_int as libc::c_uint {
        js_typeerror(J, b"not a string\0" as *const u8 as *const libc::c_char);
    }
    js_pushstring(J, (*self_0).u.s.string);
}
unsafe extern "C" fn Sp_charAt(J: *mut js_State) {
    let mut buf: [libc::c_char; 5] = [0; 5];
    let s: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let pos: libc::c_int = js_tointeger(J, 1 as libc::c_int);
    let mut rune: Rune = js_runeat(J, s, pos);
    if rune >= 0 as libc::c_int {
        buf[jsU_runetochar(buf.as_mut_ptr(), &mut rune) as usize] =
            0 as libc::c_int as libc::c_char;
        js_pushstring(J, buf.as_mut_ptr());
    } else {
        js_pushliteral(J, b"\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn Sp_charCodeAt(J: *mut js_State) {
    let s: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let pos: libc::c_int = js_tointeger(J, 1 as libc::c_int);
    let rune: Rune = js_runeat(J, s, pos);
    if rune >= 0 as libc::c_int {
        js_pushnumber(J, rune as libc::c_double);
    } else {
        js_pushnumber(J, ::core::f32::NAN as libc::c_double);
    };
}
unsafe extern "C" fn Sp_concat(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    let mut n: libc::c_int = 0;
    let mut out: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    if top == 1 as libc::c_int {
        return;
    }
    s = checkstring(J, 0 as libc::c_int);
    n = (1 as libc::c_int as libc::c_ulong).wrapping_add(strlen(s)) as libc::c_int;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, out as *mut libc::c_void);
        js_throw(J);
    }
    if n > (1 as libc::c_int) << 28 as libc::c_int {
        js_rangeerror(
            J,
            b"invalid string length\0" as *const u8 as *const libc::c_char,
        );
    }
    ::core::ptr::write_volatile(
        &mut out as *mut *mut libc::c_char,
        js_malloc(J, n) as *mut libc::c_char,
    );
    strcpy(out, s);
    i = 1 as libc::c_int;
    while i < top {
        s = js_tostring(J, i);
        n = (n as libc::c_ulong).wrapping_add(strlen(s)) as libc::c_int as libc::c_int;
        if n > (1 as libc::c_int) << 28 as libc::c_int {
            js_rangeerror(
                J,
                b"invalid string length\0" as *const u8 as *const libc::c_char,
            );
        }
        ::core::ptr::write_volatile(
            &mut out as *mut *mut libc::c_char,
            js_realloc(J, out as *mut libc::c_void, n) as *mut libc::c_char,
        );
        strcat(out, s);
        i += 1;
        i;
    }
    js_pushstring(J, out);
    js_endtry(J);
    js_free(J, out as *mut libc::c_void);
}
unsafe extern "C" fn Sp_indexOf(J: *mut js_State) {
    let mut haystack: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let needle: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    let pos: libc::c_int = js_tointeger(J, 2 as libc::c_int);
    let len: libc::c_int = strlen(needle) as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut rune: Rune = 0;
    while *haystack != 0 {
        if k >= pos && strncmp(haystack, needle, len as libc::c_ulong) == 0 {
            js_pushnumber(J, k as libc::c_double);
            return;
        }
        haystack = haystack.offset(jsU_chartorune(&mut rune, haystack) as isize);
        k += 1;
        k;
    }
    js_pushnumber(J, -(1 as libc::c_int) as libc::c_double);
}
unsafe extern "C" fn Sp_lastIndexOf(J: *mut js_State) {
    let mut haystack: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let needle: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    let pos: libc::c_int = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        strlen(haystack) as libc::c_int
    };
    let len: libc::c_int = strlen(needle) as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut last: libc::c_int = -(1 as libc::c_int);
    let mut rune: Rune = 0;
    while *haystack as libc::c_int != 0 && k <= pos {
        if strncmp(haystack, needle, len as libc::c_ulong) == 0 {
            last = k;
        }
        haystack = haystack.offset(jsU_chartorune(&mut rune, haystack) as isize);
        k += 1;
        k;
    }
    js_pushnumber(J, last as libc::c_double);
}
unsafe extern "C" fn Sp_localeCompare(J: *mut js_State) {
    let a: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let b: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    js_pushnumber(J, strcmp(a, b) as libc::c_double);
}
unsafe extern "C" fn Sp_substring_imp(
    J: *mut js_State,
    s: *const libc::c_char,
    a: libc::c_int,
    n: libc::c_int,
) {
    let mut head_rune: Rune = 0 as libc::c_int;
    let mut tail_rune: Rune = 0 as libc::c_int;
    let mut head: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut tail: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut head_len: libc::c_int = 0;
    let mut tail_len: libc::c_int = 0;
    head = s;
    i = 0 as libc::c_int;
    while i < a {
        head = head.offset(jsU_chartorune(&mut head_rune, head) as isize);
        if head_rune >= 0x10000 as libc::c_int {
            i += 1;
            i;
        }
        i += 1;
        i;
    }
    tail = head;
    k = i - a;
    while k < n {
        tail = tail.offset(jsU_chartorune(&mut tail_rune, tail) as isize);
        if tail_rune >= 0x10000 as libc::c_int {
            k += 1;
            k;
        }
        k += 1;
        k;
    }
    if i == a && k == n {
        js_pushlstring(
            J,
            head,
            tail.offset_from(head) as libc::c_long as libc::c_int,
        );
        return;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, p as *mut libc::c_void);
        js_throw(J);
    }
    p = js_malloc(
        J,
        (UTFmax as libc::c_int as libc::c_long + tail.offset_from(head) as libc::c_long)
            as libc::c_int,
    ) as *mut libc::c_char;
    if i > a {
        head_rune =
            0xdc00 as libc::c_int + ((head_rune - 0x10000 as libc::c_int) & 0x3ff as libc::c_int);
        head_len = jsU_runetochar(p, &mut head_rune);
        memcpy(
            p.offset(head_len as isize) as *mut libc::c_void,
            head as *const libc::c_void,
            tail.offset_from(head) as libc::c_long as libc::c_ulong,
        );
        js_pushlstring(
            J,
            p,
            (head_len as libc::c_long + tail.offset_from(head) as libc::c_long) as libc::c_int,
        );
    }
    if k > n {
        tail = tail.offset(-(jsU_runelen(tail_rune) as isize));
        memcpy(
            p as *mut libc::c_void,
            head as *const libc::c_void,
            tail.offset_from(head) as libc::c_long as libc::c_ulong,
        );
        tail_rune =
            0xd800 as libc::c_int + ((tail_rune - 0x10000 as libc::c_int) >> 10 as libc::c_int);
        tail_len = jsU_runetochar(
            p.offset(tail.offset_from(head) as libc::c_long as isize),
            &mut tail_rune,
        );
        js_pushlstring(
            J,
            p,
            (tail.offset_from(head) as libc::c_long + tail_len as libc::c_long) as libc::c_int,
        );
    }
    js_endtry(J);
    js_free(J, p as *mut libc::c_void);
}
unsafe extern "C" fn Sp_slice(J: *mut js_State) {
    let str: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let len: libc::c_int = js_utflen(str);
    let mut s: libc::c_int = js_tointeger(J, 1 as libc::c_int);
    let mut e: libc::c_int = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        len
    };
    s = if s < 0 as libc::c_int { s + len } else { s };
    e = if e < 0 as libc::c_int { e + len } else { e };
    s = if s < 0 as libc::c_int {
        0 as libc::c_int
    } else if s > len {
        len
    } else {
        s
    };
    e = if e < 0 as libc::c_int {
        0 as libc::c_int
    } else if e > len {
        len
    } else {
        e
    };
    if s < e {
        Sp_substring_imp(J, str, s, e - s);
    } else {
        Sp_substring_imp(J, str, e, s - e);
    };
}
unsafe extern "C" fn Sp_substring(J: *mut js_State) {
    let str: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let len: libc::c_int = js_utflen(str);
    let mut s: libc::c_int = js_tointeger(J, 1 as libc::c_int);
    let mut e: libc::c_int = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        len
    };
    s = if s < 0 as libc::c_int {
        0 as libc::c_int
    } else if s > len {
        len
    } else {
        s
    };
    e = if e < 0 as libc::c_int {
        0 as libc::c_int
    } else if e > len {
        len
    } else {
        e
    };
    if s < e {
        Sp_substring_imp(J, str, s, e - s);
    } else {
        Sp_substring_imp(J, str, e, s - e);
    };
}
unsafe extern "C" fn Sp_toLowerCase(J: *mut js_State) {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let s0: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let mut dst: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut d: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut rune: Rune = 0;
    let mut full: *const Rune = std::ptr::null::<Rune>();
    let mut n: libc::c_int = 0;
    n = 1 as libc::c_int;
    s = s0;
    while *s != 0 {
        s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        full = jsU_tolowerrune_full(rune);
        if !full.is_null() {
            while *full != 0 {
                n += jsU_runelen(*full);
                full = full.offset(1);
                full;
            }
        } else {
            rune = jsU_tolowerrune(rune);
            n += jsU_runelen(rune);
        }
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, dst as *mut libc::c_void);
        js_throw(J);
    }
    ::core::ptr::write_volatile(
        &mut dst as *mut *mut libc::c_char,
        js_malloc(J, n) as *mut libc::c_char,
    );
    d = ::core::ptr::read_volatile::<*mut libc::c_char>(&dst as *const *mut libc::c_char);
    s = s0;
    while *s != 0 {
        s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        full = jsU_tolowerrune_full(rune);
        if !full.is_null() {
            while *full != 0 {
                d = d.offset(jsU_runetochar(d, full) as isize);
                full = full.offset(1);
                full;
            }
        } else {
            rune = jsU_tolowerrune(rune);
            d = d.offset(jsU_runetochar(d, &mut rune) as isize);
        }
    }
    *d = 0 as libc::c_int as libc::c_char;
    js_pushstring(J, dst);
    js_endtry(J);
    js_free(J, dst as *mut libc::c_void);
}
unsafe extern "C" fn Sp_toUpperCase(J: *mut js_State) {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let s0: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let mut dst: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut d: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut full: *const Rune = std::ptr::null::<Rune>();
    let mut rune: Rune = 0;
    let mut n: libc::c_int = 0;
    n = 1 as libc::c_int;
    s = s0;
    while *s != 0 {
        s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        full = jsU_toupperrune_full(rune);
        if !full.is_null() {
            while *full != 0 {
                n += jsU_runelen(*full);
                full = full.offset(1);
                full;
            }
        } else {
            rune = jsU_toupperrune(rune);
            n += jsU_runelen(rune);
        }
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, dst as *mut libc::c_void);
        js_throw(J);
    }
    ::core::ptr::write_volatile(
        &mut dst as *mut *mut libc::c_char,
        js_malloc(J, n) as *mut libc::c_char,
    );
    d = ::core::ptr::read_volatile::<*mut libc::c_char>(&dst as *const *mut libc::c_char);
    s = s0;
    while *s != 0 {
        s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        full = jsU_toupperrune_full(rune);
        if !full.is_null() {
            while *full != 0 {
                d = d.offset(jsU_runetochar(d, full) as isize);
                full = full.offset(1);
                full;
            }
        } else {
            rune = jsU_toupperrune(rune);
            d = d.offset(jsU_runetochar(d, &mut rune) as isize);
        }
    }
    *d = 0 as libc::c_int as libc::c_char;
    js_pushstring(J, dst);
    js_endtry(J);
    js_free(J, dst as *mut libc::c_void);
}
unsafe extern "C" fn istrim(c: libc::c_int) -> libc::c_int {
    (c == 0x9 as libc::c_int
        || c == 0xb as libc::c_int
        || c == 0xc as libc::c_int
        || c == 0x20 as libc::c_int
        || c == 0xa0 as libc::c_int
        || c == 0xfeff as libc::c_int
        || c == 0xa as libc::c_int
        || c == 0xd as libc::c_int
        || c == 0x2028 as libc::c_int
        || c == 0x2029 as libc::c_int) as libc::c_int
}
unsafe extern "C" fn Sp_trim(J: *mut js_State) {
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut e: *const libc::c_char = std::ptr::null::<libc::c_char>();
    s = checkstring(J, 0 as libc::c_int);
    while istrim(*s as libc::c_int) != 0 {
        s = s.offset(1);
        s;
    }
    e = s.offset(strlen(s) as isize);
    while e > s && istrim(*e.offset(-(1 as libc::c_int) as isize) as libc::c_int) != 0 {
        e = e.offset(-1);
        e;
    }
    js_pushlstring(J, s, e.offset_from(s) as libc::c_long as libc::c_int);
}
unsafe extern "C" fn S_fromCharCode(J: *mut js_State) {
    let mut i: libc::c_int = 0;
    let top: libc::c_int = js_gettop(J);
    let mut s: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut c: Rune = 0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, s as *mut libc::c_void);
        js_throw(J);
    }
    p = js_malloc(
        J,
        (top - 1 as libc::c_int) * UTFmax as libc::c_int + 1 as libc::c_int,
    ) as *mut libc::c_char;
    ::core::ptr::write_volatile(&mut s as *mut *mut libc::c_char, p);
    i = 1 as libc::c_int;
    while i < top {
        c = js_touint32(J, i) as Rune;
        p = p.offset(jsU_runetochar(p, &mut c) as isize);
        i += 1;
        i;
    }
    *p = 0 as libc::c_int as libc::c_char;
    js_pushstring(J, s);
    js_endtry(J);
    js_free(J, s as *mut libc::c_void);
}
unsafe extern "C" fn Sp_match(J: *mut js_State) {
    let mut re: *mut js_Regexp = std::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut len: libc::c_int = 0;
    let mut a: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut b: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut c: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut e: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    text = checkstring(J, 0 as libc::c_int);
    if js_isregexp(J, 1 as libc::c_int) != 0 {
        js_copy(J, 1 as libc::c_int);
    } else if js_isundefined(J, 1 as libc::c_int) != 0 {
        js_newregexp(
            J,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        js_newregexp(J, js_tostring(J, 1 as libc::c_int), 0 as libc::c_int);
    }
    re = js_toregexp(J, -(1 as libc::c_int));
    if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int == 0 {
        js_RegExp_prototype_exec(J, re, text);
        return;
    }
    (*re).last = 0 as libc::c_int as libc::c_ushort;
    js_newarray(J);
    len = 0 as libc::c_int;
    a = text;
    e = text.offset(strlen(text) as isize);
    while a <= e {
        if js_doregexec(
            J,
            (*re).prog as *mut Reprog,
            a,
            &mut m,
            if a > text {
                REG_NOTBOL as libc::c_int
            } else {
                0 as libc::c_int
            },
        ) != 0
        {
            break;
        }
        b = m.sub[0 as libc::c_int as usize].sp;
        c = m.sub[0 as libc::c_int as usize].ep;
        js_pushlstring(J, b, c.offset_from(b) as libc::c_long as libc::c_int);
        let fresh89 = len;
        len += 1;
        js_setindex(J, -(2 as libc::c_int), fresh89);
        a = c;
        if c.offset_from(b) as libc::c_long == 0 as libc::c_int as libc::c_long {
            a = a.offset(1);
            a;
        }
    }
    if len == 0 as libc::c_int {
        js_pop(J, 1 as libc::c_int);
        js_pushnull(J);
    }
}
unsafe extern "C" fn Sp_search(J: *mut js_State) {
    let mut re: *mut js_Regexp = std::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    text = checkstring(J, 0 as libc::c_int);
    if js_isregexp(J, 1 as libc::c_int) != 0 {
        js_copy(J, 1 as libc::c_int);
    } else if js_isundefined(J, 1 as libc::c_int) != 0 {
        js_newregexp(
            J,
            b"\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else {
        js_newregexp(J, js_tostring(J, 1 as libc::c_int), 0 as libc::c_int);
    }
    re = js_toregexp(J, -(1 as libc::c_int));
    if js_doregexec(J, (*re).prog as *mut Reprog, text, &mut m, 0 as libc::c_int) == 0 {
        js_pushnumber(
            J,
            js_utfptrtoidx(text, m.sub[0 as libc::c_int as usize].sp) as libc::c_double,
        );
    } else {
        js_pushnumber(J, -(1 as libc::c_int) as libc::c_double);
    };
}
unsafe extern "C" fn Sp_replace_regexp(J: *mut js_State) {
    let mut re: *mut js_Regexp = std::ptr::null_mut::<js_Regexp>();
    let mut source: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut r: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    source = checkstring(J, 0 as libc::c_int);
    re = js_toregexp(J, 1 as libc::c_int);
    if js_doregexec(
        J,
        (*re).prog as *mut Reprog,
        source,
        &mut m,
        0 as libc::c_int,
    ) != 0
    {
        js_copy(J, 0 as libc::c_int);
        return;
    }
    (*re).last = 0 as libc::c_int as libc::c_ushort;
    loop {
        s = m.sub[0 as libc::c_int as usize].sp;
        n = (m.sub[0 as libc::c_int as usize].ep).offset_from(m.sub[0 as libc::c_int as usize].sp)
            as libc::c_long as libc::c_int;
        if js_iscallable(J, 2 as libc::c_int) != 0 {
            js_copy(J, 2 as libc::c_int);
            js_pushundefined(J);
            x = 0 as libc::c_int;
            while !(m.sub[x as usize].sp).is_null() {
                js_pushlstring(
                    J,
                    m.sub[x as usize].sp,
                    (m.sub[x as usize].ep).offset_from(m.sub[x as usize].sp) as libc::c_long
                        as libc::c_int,
                );
                x += 1;
                x;
            }
            js_pushnumber(J, s.offset_from(source) as libc::c_long as libc::c_double);
            js_copy(J, 0 as libc::c_int);
            js_call(J, 2 as libc::c_int + x);
            r = js_tostring(J, -(1 as libc::c_int));
            js_putm(J, &mut sb, source, s);
            js_puts(J, &mut sb, r);
            js_pop(J, 1 as libc::c_int);
        } else {
            r = js_tostring(J, 2 as libc::c_int);
            js_putm(J, &mut sb, source, s);
            while *r != 0 {
                if *r as libc::c_int == '$' as i32 {
                    let current_block_44: u64;
                    r = r.offset(1);
                    match *r as libc::c_int {
                        0 => {
                            r = r.offset(-1);
                            r;
                            current_block_44 = 5554089773008369846;
                        }
                        36 => {
                            current_block_44 = 5554089773008369846;
                        }
                        96 => {
                            js_putm(J, &mut sb, source, s);
                            current_block_44 = 9441801433784995173;
                        }
                        39 => {
                            js_puts(J, &mut sb, s.offset(n as isize));
                            current_block_44 = 9441801433784995173;
                        }
                        38 => {
                            js_putm(J, &mut sb, s, s.offset(n as isize));
                            current_block_44 = 9441801433784995173;
                        }
                        48..=57 => {
                            x = *r as libc::c_int - '0' as i32;
                            if *r.offset(1 as libc::c_int as isize) as libc::c_int >= '0' as i32
                                && *r.offset(1 as libc::c_int as isize) as libc::c_int <= '9' as i32
                            {
                                r = r.offset(1);
                                x = x * 10 as libc::c_int + *r as libc::c_int - '0' as i32;
                            }
                            if x > 0 as libc::c_int && x < m.nsub {
                                js_putm(J, &mut sb, m.sub[x as usize].sp, m.sub[x as usize].ep);
                            } else {
                                js_putc(J, &mut sb, '$' as i32);
                                if x > 10 as libc::c_int {
                                    js_putc(J, &mut sb, '0' as i32 + x / 10 as libc::c_int);
                                    js_putc(J, &mut sb, '0' as i32 + x % 10 as libc::c_int);
                                } else {
                                    js_putc(J, &mut sb, '0' as i32 + x);
                                }
                            }
                            current_block_44 = 9441801433784995173;
                        }
                        _ => {
                            js_putc(J, &mut sb, '$' as i32);
                            js_putc(J, &mut sb, *r as libc::c_int);
                            current_block_44 = 9441801433784995173;
                        }
                    }
                    if current_block_44 == 5554089773008369846 {
                        js_putc(J, &mut sb, '$' as i32);
                    }
                    r = r.offset(1);
                    r;
                } else {
                    let fresh90 = r;
                    r = r.offset(1);
                    js_putc(J, &mut sb, *fresh90 as libc::c_int);
                }
            }
        }
        if (*re).flags as libc::c_int & JS_REGEXP_G as libc::c_int == 0 {
            break;
        }
        source = m.sub[0 as libc::c_int as usize].ep;
        if n == 0 as libc::c_int {
            if *source == 0 {
                break;
            }
            let fresh91 = source;
            source = source.offset(1);
            js_putc(J, &mut sb, *fresh91 as libc::c_int);
        }
        if js_doregexec(
            J,
            (*re).prog as *mut Reprog,
            source,
            &mut m,
            REG_NOTBOL as libc::c_int,
        ) != 0
        {
            break;
        }
    }
    js_puts(J, &mut sb, s.offset(n as isize));
    js_putc(J, &mut sb, 0 as libc::c_int);
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    js_pushstring(
        J,
        if !sb.is_null() {
            ((*sb).s).as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
unsafe extern "C" fn Sp_replace_string(J: *mut js_State) {
    let mut source: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut needle: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut s: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut r: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut sb: *mut js_Buffer = std::ptr::null_mut::<js_Buffer>();
    let mut n: libc::c_int = 0;
    source = checkstring(J, 0 as libc::c_int);
    needle = js_tostring(J, 1 as libc::c_int);
    s = strstr(source, needle);
    if s.is_null() {
        js_copy(J, 0 as libc::c_int);
        return;
    }
    n = strlen(needle) as libc::c_int;
    if js_iscallable(J, 2 as libc::c_int) != 0 {
        js_copy(J, 2 as libc::c_int);
        js_pushundefined(J);
        js_pushlstring(J, s, n);
        js_pushnumber(J, s.offset_from(source) as libc::c_long as libc::c_double);
        js_copy(J, 0 as libc::c_int);
        js_call(J, 3 as libc::c_int);
        r = js_tostring(J, -(1 as libc::c_int));
        js_putm(J, &mut sb, source, s);
        js_puts(J, &mut sb, r);
        js_puts(J, &mut sb, s.offset(n as isize));
        js_putc(J, &mut sb, 0 as libc::c_int);
        js_pop(J, 1 as libc::c_int);
    } else {
        r = js_tostring(J, 2 as libc::c_int);
        js_putm(J, &mut sb, source, s);
        while *r != 0 {
            if *r as libc::c_int == '$' as i32 {
                let current_block_29: u64;
                r = r.offset(1);
                match *r as libc::c_int {
                    0 => {
                        r = r.offset(-1);
                        r;
                        current_block_29 = 6980996644429870198;
                    }
                    36 => {
                        current_block_29 = 6980996644429870198;
                    }
                    38 => {
                        js_putm(J, &mut sb, s, s.offset(n as isize));
                        current_block_29 = 15125582407903384992;
                    }
                    96 => {
                        js_putm(J, &mut sb, source, s);
                        current_block_29 = 15125582407903384992;
                    }
                    39 => {
                        js_puts(J, &mut sb, s.offset(n as isize));
                        current_block_29 = 15125582407903384992;
                    }
                    _ => {
                        js_putc(J, &mut sb, '$' as i32);
                        js_putc(J, &mut sb, *r as libc::c_int);
                        current_block_29 = 15125582407903384992;
                    }
                }
                if current_block_29 == 6980996644429870198 {
                    js_putc(J, &mut sb, '$' as i32);
                }
                r = r.offset(1);
                r;
            } else {
                let fresh92 = r;
                r = r.offset(1);
                js_putc(J, &mut sb, *fresh92 as libc::c_int);
            }
        }
        js_puts(J, &mut sb, s.offset(n as isize));
        js_putc(J, &mut sb, 0 as libc::c_int);
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    js_pushstring(
        J,
        if !sb.is_null() {
            ((*sb).s).as_mut_ptr() as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
unsafe extern "C" fn Sp_replace(J: *mut js_State) {
    if js_isregexp(J, 1 as libc::c_int) != 0 {
        Sp_replace_regexp(J);
    } else {
        Sp_replace_string(J);
    };
}
unsafe extern "C" fn Sp_split_regexp(J: *mut js_State) {
    let mut re: *mut js_Regexp = std::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut limit: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut a: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut b: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut c: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut e: *const libc::c_char = std::ptr::null::<libc::c_char>();
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    text = checkstring(J, 0 as libc::c_int);
    re = js_toregexp(J, 1 as libc::c_int);
    limit = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        (1 as libc::c_int) << 30 as libc::c_int
    };
    js_newarray(J);
    len = 0 as libc::c_int;
    if limit == 0 as libc::c_int {
        return;
    }
    e = text.offset(strlen(text) as isize);
    if e == text {
        if js_doregexec(J, (*re).prog as *mut Reprog, text, &mut m, 0 as libc::c_int) != 0 {
            js_pushliteral(J, b"\0" as *const u8 as *const libc::c_char);
            js_setindex(J, -(2 as libc::c_int), 0 as libc::c_int);
        }
        return;
    }
    a = text;
    p = a;
    while a < e {
        if js_doregexec(
            J,
            (*re).prog as *mut Reprog,
            a,
            &mut m,
            if a > text {
                REG_NOTBOL as libc::c_int
            } else {
                0 as libc::c_int
            },
        ) != 0
        {
            break;
        }
        b = m.sub[0 as libc::c_int as usize].sp;
        c = m.sub[0 as libc::c_int as usize].ep;
        if b == c && b == p {
            a = a.offset(1);
            a;
        } else {
            if len == limit {
                return;
            }
            js_pushlstring(J, p, b.offset_from(p) as libc::c_long as libc::c_int);
            let fresh93 = len;
            len += 1;
            js_setindex(J, -(2 as libc::c_int), fresh93);
            k = 1 as libc::c_int;
            while k < m.nsub {
                if len == limit {
                    return;
                }
                js_pushlstring(
                    J,
                    m.sub[k as usize].sp,
                    (m.sub[k as usize].ep).offset_from(m.sub[k as usize].sp) as libc::c_long
                        as libc::c_int,
                );
                let fresh94 = len;
                len += 1;
                js_setindex(J, -(2 as libc::c_int), fresh94);
                k += 1;
                k;
            }
            p = c;
            a = p;
        }
    }
    if len == limit {
        return;
    }
    js_pushstring(J, p);
    js_setindex(J, -(2 as libc::c_int), len);
}
unsafe extern "C" fn Sp_split_string(J: *mut js_State) {
    let mut str: *const libc::c_char = checkstring(J, 0 as libc::c_int);
    let sep: *const libc::c_char = js_tostring(J, 1 as libc::c_int);
    let limit: libc::c_int = if js_isdefined(J, 2 as libc::c_int) != 0 {
        js_tointeger(J, 2 as libc::c_int)
    } else {
        (1 as libc::c_int) << 30 as libc::c_int
    };
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    js_newarray(J);
    if limit == 0 as libc::c_int {
        return;
    }
    n = strlen(sep) as libc::c_int;
    if n == 0 as libc::c_int {
        let mut rune: Rune = 0;
        i = 0 as libc::c_int;
        while *str as libc::c_int != 0 && i < limit {
            n = jsU_chartorune(&mut rune, str);
            js_pushlstring(J, str, n);
            js_setindex(J, -(2 as libc::c_int), i);
            str = str.offset(n as isize);
            i += 1;
            i;
        }
        return;
    }
    i = 0 as libc::c_int;
    while !str.is_null() && i < limit {
        let s: *const libc::c_char = strstr(str, sep);
        if !s.is_null() {
            js_pushlstring(J, str, s.offset_from(str) as libc::c_long as libc::c_int);
            js_setindex(J, -(2 as libc::c_int), i);
            str = s.offset(n as isize);
        } else {
            js_pushstring(J, str);
            js_setindex(J, -(2 as libc::c_int), i);
            str = std::ptr::null::<libc::c_char>();
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn Sp_split(J: *mut js_State) {
    if js_isundefined(J, 1 as libc::c_int) != 0 {
        js_newarray(J);
        js_pushstring(J, js_tostring(J, 0 as libc::c_int));
        js_setindex(J, -(2 as libc::c_int), 0 as libc::c_int);
    } else if js_isregexp(J, 1 as libc::c_int) != 0 {
        Sp_split_regexp(J);
    } else {
        Sp_split_string(J);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initstring(J: *mut js_State) {
    (*(*J).String_prototype).u.s.shrstr[0 as libc::c_int as usize] =
        0 as libc::c_int as libc::c_char;
    (*(*J).String_prototype).u.s.string = ((*(*J).String_prototype).u.s.shrstr).as_mut_ptr();
    (*(*J).String_prototype).u.s.length = 0 as libc::c_int;
    js_pushobject(J, (*J).String_prototype);
    jsB_propf(
        J,
        b"String.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Sp_toString as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Sp_valueOf as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.charAt\0" as *const u8 as *const libc::c_char,
        Some(Sp_charAt as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.charCodeAt\0" as *const u8 as *const libc::c_char,
        Some(Sp_charCodeAt as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.concat\0" as *const u8 as *const libc::c_char,
        Some(Sp_concat as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.indexOf\0" as *const u8 as *const libc::c_char,
        Some(Sp_indexOf as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.lastIndexOf\0" as *const u8 as *const libc::c_char,
        Some(Sp_lastIndexOf as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.localeCompare\0" as *const u8 as *const libc::c_char,
        Some(Sp_localeCompare as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.match\0" as *const u8 as *const libc::c_char,
        Some(Sp_match as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.replace\0" as *const u8 as *const libc::c_char,
        Some(Sp_replace as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.search\0" as *const u8 as *const libc::c_char,
        Some(Sp_search as unsafe extern "C" fn(*mut js_State) -> ()),
        1 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.slice\0" as *const u8 as *const libc::c_char,
        Some(Sp_slice as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.split\0" as *const u8 as *const libc::c_char,
        Some(Sp_split as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.substring\0" as *const u8 as *const libc::c_char,
        Some(Sp_substring as unsafe extern "C" fn(*mut js_State) -> ()),
        2 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.toLowerCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toLowerCase as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.toLocaleLowerCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toLowerCase as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.toUpperCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toUpperCase as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.toLocaleUpperCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toUpperCase as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.prototype.trim\0" as *const u8 as *const libc::c_char,
        Some(Sp_trim as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    js_newcconstructor(
        J,
        Some(jsB_String as unsafe extern "C" fn(*mut js_State) -> ()),
        Some(jsB_new_String as unsafe extern "C" fn(*mut js_State) -> ()),
        b"String\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    jsB_propf(
        J,
        b"String.fromCharCode\0" as *const u8 as *const libc::c_char,
        Some(S_fromCharCode as unsafe extern "C" fn(*mut js_State) -> ()),
        0 as libc::c_int,
    );
    js_defglobal(
        J,
        b"String\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_strtol(
    mut s: *const libc::c_char,
    p: *mut *mut libc::c_char,
    base: libc::c_int,
) -> libc::c_double {
    static mut table: [libc::c_uchar; 256] = [
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        0 as libc::c_int as libc::c_uchar,
        1 as libc::c_int as libc::c_uchar,
        2 as libc::c_int as libc::c_uchar,
        3 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        8 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        29 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        10 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        13 as libc::c_int as libc::c_uchar,
        14 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        18 as libc::c_int as libc::c_uchar,
        19 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        22 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        24 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        26 as libc::c_int as libc::c_uchar,
        27 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        29 as libc::c_int as libc::c_uchar,
        30 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        32 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
    ];
    let mut x: libc::c_double = 0.;
    let mut c: libc::c_uchar = 0;
    if base == 10 as libc::c_int {
        x = 0 as libc::c_int as libc::c_double;
        let fresh95 = s;
        s = s.offset(1);
        c = *fresh95 as libc::c_uchar;
        while 0 as libc::c_int <= c as libc::c_int - '0' as i32
            && (c as libc::c_int - '0' as i32) < 10 as libc::c_int
        {
            x = x * 10 as libc::c_int as libc::c_double
                + (c as libc::c_int - '0' as i32) as libc::c_double;
            let fresh96 = s;
            s = s.offset(1);
            c = *fresh96 as libc::c_uchar;
        }
    } else {
        x = 0 as libc::c_int as libc::c_double;
        let fresh97 = s;
        s = s.offset(1);
        c = *fresh97 as libc::c_uchar;
        while (table[c as usize] as libc::c_int) < base {
            x = x * base as libc::c_double + table[c as usize] as libc::c_int as libc::c_double;
            let fresh98 = s;
            s = s.offset(1);
            c = *fresh98 as libc::c_uchar;
        }
    }
    if !p.is_null() {
        *p = (s as *mut libc::c_char).offset(-(1 as libc::c_int as isize));
    }
    x
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertointeger(mut n: libc::c_double) -> libc::c_int {
    if n == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int;
    }
    if n.is_nan() as i32 != 0 {
        return 0 as libc::c_int;
    }
    n = if n < 0 as libc::c_int as libc::c_double {
        -floor(-n)
    } else {
        floor(n)
    };
    if n < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    if n > 2147483647 as libc::c_int as libc::c_double {
        return 2147483647 as libc::c_int;
    }
    n as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertoint32(mut n: libc::c_double) -> libc::c_int {
    let two32: libc::c_double = 4294967296.0f64;
    let two31: libc::c_double = 2147483648.0f64;
    if n.is_finite() as i32 == 0 || n == 0 as libc::c_int as libc::c_double {
        return 0 as libc::c_int;
    }
    n = fmod(n, two32);
    n = if n >= 0 as libc::c_int as libc::c_double {
        floor(n)
    } else {
        ceil(n) + two32
    };
    if n >= two31 {
        (n - two32) as libc::c_int
    } else {
        n as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertouint32(n: libc::c_double) -> libc::c_uint {
    jsV_numbertoint32(n) as libc::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertoint16(n: libc::c_double) -> libc::c_short {
    jsV_numbertoint32(n) as libc::c_short
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertouint16(n: libc::c_double) -> libc::c_ushort {
    jsV_numbertoint32(n) as libc::c_ushort
}
unsafe extern "C" fn jsV_toString(J: *mut js_State, obj: *mut js_Object) -> libc::c_int {
    js_pushobject(J, obj);
    js_getproperty(
        J,
        -(1 as libc::c_int),
        b"toString\0" as *const u8 as *const libc::c_char,
    );
    if js_iscallable(J, -(1 as libc::c_int)) != 0 {
        js_rot2(J);
        js_call(J, 0 as libc::c_int);
        if js_isprimitive(J, -(1 as libc::c_int)) != 0 {
            return 1 as libc::c_int;
        }
        js_pop(J, 1 as libc::c_int);
        return 0 as libc::c_int;
    }
    js_pop(J, 2 as libc::c_int);
    0 as libc::c_int
}
unsafe extern "C" fn jsV_valueOf(J: *mut js_State, obj: *mut js_Object) -> libc::c_int {
    js_pushobject(J, obj);
    js_getproperty(
        J,
        -(1 as libc::c_int),
        b"valueOf\0" as *const u8 as *const libc::c_char,
    );
    if js_iscallable(J, -(1 as libc::c_int)) != 0 {
        js_rot2(J);
        js_call(J, 0 as libc::c_int);
        if js_isprimitive(J, -(1 as libc::c_int)) != 0 {
            return 1 as libc::c_int;
        }
        js_pop(J, 1 as libc::c_int);
        return 0 as libc::c_int;
    }
    js_pop(J, 2 as libc::c_int);
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsV_toprimitive(
    J: *mut js_State,
    v: *mut js_Value,
    mut preferred: libc::c_int,
) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if (*v).t.type_0 as libc::c_int != JS_TOBJECT as libc::c_int {
        return;
    }
    obj = (*v).u.object;
    if preferred == JS_HNONE as libc::c_int {
        preferred = if (*obj).type_0 as libc::c_uint == JS_CDATE as libc::c_int as libc::c_uint {
            JS_HSTRING as libc::c_int
        } else {
            JS_HNUMBER as libc::c_int
        };
    }
    if preferred == JS_HSTRING as libc::c_int {
        if jsV_toString(J, obj) != 0 || jsV_valueOf(J, obj) != 0 {
            *v = *js_tovalue(J, -(1 as libc::c_int));
            js_pop(J, 1 as libc::c_int);
            return;
        }
    } else if jsV_valueOf(J, obj) != 0 || jsV_toString(J, obj) != 0 {
        *v = *js_tovalue(J, -(1 as libc::c_int));
        js_pop(J, 1 as libc::c_int);
        return;
    }
    if (*J).strict != 0 {
        js_typeerror(
            J,
            b"cannot convert object to primitive\0" as *const u8 as *const libc::c_char,
        );
    }
    (*v).t.type_0 = JS_TLITSTR as libc::c_int as libc::c_char;
    (*v).u.litstr = b"[object]\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn jsV_toboolean(J: *mut js_State, v: *mut js_Value) -> libc::c_int {
    match (*v).t.type_0 as libc::c_int {
        1 => 0 as libc::c_int,
        2 => 0 as libc::c_int,
        3 => (*v).u.boolean,
        4 => {
            ((*v).u.number != 0 as libc::c_int as libc::c_double
                && ((*v).u.number).is_nan() as i32 == 0) as libc::c_int
        }
        5 => {
            (*((*v).u.litstr).offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int)
                as libc::c_int
        }
        6 => {
            (*((*(*v).u.memstr).p)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as libc::c_int
                != 0 as libc::c_int) as libc::c_int
        }
        7 => 1 as libc::c_int,
        0 | _ => {
            ((*v).u.shrstr[0 as libc::c_int as usize] as libc::c_int != 0 as libc::c_int)
                as libc::c_int
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_itoa(out: *mut libc::c_char, v: libc::c_int) -> *const libc::c_char {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut s: *mut libc::c_char = out;
    let mut a: libc::c_uint = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    if v < 0 as libc::c_int {
        a = -v as libc::c_uint;
        let fresh99 = s;
        s = s.offset(1);
        *fresh99 = '-' as i32 as libc::c_char;
    } else {
        a = v as libc::c_uint;
    }
    while a != 0 {
        let fresh100 = i;
        i += 1;
        buf[fresh100 as usize] =
            a.wrapping_rem(10 as libc::c_int as libc::c_uint)
                .wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
        a = a.wrapping_div(10 as libc::c_int as libc::c_uint);
    }
    if i == 0 as libc::c_int {
        let fresh101 = i;
        i += 1;
        buf[fresh101 as usize] = '0' as i32 as libc::c_char;
    }
    while i > 0 as libc::c_int {
        i -= 1;
        let fresh102 = s;
        s = s.offset(1);
        *fresh102 = buf[i as usize];
    }
    *s = 0 as libc::c_int as libc::c_char;
    out
}
#[no_mangle]
pub unsafe extern "C" fn js_stringtofloat(
    s: *const libc::c_char,
    ep: *mut *mut libc::c_char,
) -> libc::c_double {
    let mut end: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut n: libc::c_double = 0.;
    let mut e: *const libc::c_char = s;
    let mut isflt: libc::c_int = 0 as libc::c_int;
    if *e as libc::c_int == '+' as i32 || *e as libc::c_int == '-' as i32 {
        e = e.offset(1);
        e;
    }
    while *e as libc::c_int >= '0' as i32 && *e as libc::c_int <= '9' as i32 {
        e = e.offset(1);
        e;
    }
    if *e as libc::c_int == '.' as i32 {
        e = e.offset(1);
        e;
        isflt = 1 as libc::c_int;
    }
    while *e as libc::c_int >= '0' as i32 && *e as libc::c_int <= '9' as i32 {
        e = e.offset(1);
        e;
    }
    if *e as libc::c_int == 'e' as i32 || *e as libc::c_int == 'E' as i32 {
        e = e.offset(1);
        e;
        if *e as libc::c_int == '+' as i32 || *e as libc::c_int == '-' as i32 {
            e = e.offset(1);
            e;
        }
        while *e as libc::c_int >= '0' as i32 && *e as libc::c_int <= '9' as i32 {
            e = e.offset(1);
            e;
        }
        isflt = 1 as libc::c_int;
    }
    if isflt != 0 {
        n = js_strtod(s, &mut end);
    } else if *s as libc::c_int == '-' as i32 {
        n = -js_strtol(
            s.offset(1 as libc::c_int as isize),
            &mut end,
            10 as libc::c_int,
        );
    } else if *s as libc::c_int == '+' as i32 {
        n = js_strtol(
            s.offset(1 as libc::c_int as isize),
            &mut end,
            10 as libc::c_int,
        );
    } else {
        n = js_strtol(s, &mut end, 10 as libc::c_int);
    }
    if end == e as *mut libc::c_char {
        *ep = e as *mut libc::c_char;
        return n;
    }
    *ep = s as *mut libc::c_char;
    0 as libc::c_int as libc::c_double
}
#[no_mangle]
pub unsafe extern "C" fn jsV_stringtonumber(
    J: *mut js_State,
    mut s: *const libc::c_char,
) -> libc::c_double {
    let mut e: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut n: libc::c_double = 0.;
    while jsY_iswhite(*s as libc::c_int) != 0 || jsY_isnewline(*s as libc::c_int) != 0 {
        s = s.offset(1);
        s;
    }
    if *s.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
        && (*s.offset(1 as libc::c_int as isize) as libc::c_int == 'x' as i32
            || *s.offset(1 as libc::c_int as isize) as libc::c_int == 'X' as i32)
        && *s.offset(2 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int
    {
        n = js_strtol(
            s.offset(2 as libc::c_int as isize),
            &mut e,
            16 as libc::c_int,
        );
    } else if strncmp(
        s,
        b"Infinity\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        n = ::core::f32::INFINITY as libc::c_double;
        e = (s as *mut libc::c_char).offset(8 as libc::c_int as isize);
    } else if strncmp(
        s,
        b"+Infinity\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        n = ::core::f32::INFINITY as libc::c_double;
        e = (s as *mut libc::c_char).offset(9 as libc::c_int as isize);
    } else if strncmp(
        s,
        b"-Infinity\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        n = -::core::f32::INFINITY as libc::c_double;
        e = (s as *mut libc::c_char).offset(9 as libc::c_int as isize);
    } else {
        n = js_stringtofloat(s, &mut e);
    }
    while jsY_iswhite(*e as libc::c_int) != 0 || jsY_isnewline(*e as libc::c_int) != 0 {
        e = e.offset(1);
        e;
    }
    if *e != 0 {
        return ::core::f32::NAN as libc::c_double;
    }
    n
}
#[no_mangle]
pub unsafe extern "C" fn jsV_tonumber(J: *mut js_State, v: *mut js_Value) -> libc::c_double {
    match (*v).t.type_0 as libc::c_int {
        1 => ::core::f32::NAN as libc::c_double,
        2 => 0 as libc::c_int as libc::c_double,
        3 => (*v).u.boolean as libc::c_double,
        4 => (*v).u.number,
        5 => jsV_stringtonumber(J, (*v).u.litstr),
        6 => jsV_stringtonumber(J, ((*(*v).u.memstr).p).as_mut_ptr()),
        7 => {
            jsV_toprimitive(J, v, JS_HNUMBER as libc::c_int);
            jsV_tonumber(J, v)
        }
        0 | _ => jsV_stringtonumber(J, ((*v).u.shrstr).as_mut_ptr()),
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsV_tointeger(J: *mut js_State, v: *mut js_Value) -> libc::c_double {
    jsV_numbertointeger(jsV_tonumber(J, v)) as libc::c_double
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertostring(
    J: *mut js_State,
    buf: *mut libc::c_char,
    f: libc::c_double,
) -> *const libc::c_char {
    let mut digits: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = buf;
    let mut s: *mut libc::c_char = digits.as_mut_ptr();
    let mut exp_0: libc::c_int = 0;
    let mut ndigits: libc::c_int = 0;
    let mut point: libc::c_int = 0;
    if f == 0 as libc::c_int as libc::c_double {
        return b"0\0" as *const u8 as *const libc::c_char;
    }
    if f.is_nan() as i32 != 0 {
        return b"NaN\0" as *const u8 as *const libc::c_char;
    }
    if if f.is_infinite() {
        if f.is_sign_positive() {
            1
        } else {
            -1
        }
    } else {
        0
    } != 0
    {
        return if f < 0 as libc::c_int as libc::c_double {
            b"-Infinity\0" as *const u8 as *const libc::c_char
        } else {
            b"Infinity\0" as *const u8 as *const libc::c_char
        };
    }
    if f >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_double
        && f <= 2147483647 as libc::c_int as libc::c_double
    {
        let i: libc::c_int = f as libc::c_int;
        if i as libc::c_double == f {
            return js_itoa(buf, i);
        }
    }
    ndigits = js_grisu2(f, digits.as_mut_ptr(), &mut exp_0);
    point = ndigits + exp_0;
    if f.is_sign_negative() as libc::c_int != 0 {
        let fresh103 = p;
        p = p.offset(1);
        *fresh103 = '-' as i32 as libc::c_char;
    }
    if point < -(5 as libc::c_int) || point > 21 as libc::c_int {
        let fresh104 = s;
        s = s.offset(1);
        let fresh105 = p;
        p = p.offset(1);
        *fresh105 = *fresh104;
        if ndigits > 1 as libc::c_int {
            let mut n: libc::c_int = ndigits - 1 as libc::c_int;
            let fresh106 = p;
            p = p.offset(1);
            *fresh106 = '.' as i32 as libc::c_char;
            loop {
                let fresh107 = n;
                n -= 1;
                if fresh107 == 0 {
                    break;
                }
                let fresh108 = s;
                s = s.offset(1);
                let fresh109 = p;
                p = p.offset(1);
                *fresh109 = *fresh108;
            }
        }
        js_fmtexp(p, point - 1 as libc::c_int);
    } else if point <= 0 as libc::c_int {
        let fresh110 = p;
        p = p.offset(1);
        *fresh110 = '0' as i32 as libc::c_char;
        let fresh111 = p;
        p = p.offset(1);
        *fresh111 = '.' as i32 as libc::c_char;
        loop {
            let fresh112 = point;
            point += 1;
            if fresh112 >= 0 as libc::c_int {
                break;
            }
            let fresh113 = p;
            p = p.offset(1);
            *fresh113 = '0' as i32 as libc::c_char;
        }
        loop {
            let fresh114 = ndigits;
            ndigits -= 1;
            if fresh114 <= 0 as libc::c_int {
                break;
            }
            let fresh115 = s;
            s = s.offset(1);
            let fresh116 = p;
            p = p.offset(1);
            *fresh116 = *fresh115;
        }
        *p = 0 as libc::c_int as libc::c_char;
    } else {
        loop {
            let fresh117 = ndigits;
            ndigits -= 1;
            if fresh117 <= 0 as libc::c_int {
                break;
            }
            let fresh118 = s;
            s = s.offset(1);
            let fresh119 = p;
            p = p.offset(1);
            *fresh119 = *fresh118;
            point -= 1;
            if point == 0 as libc::c_int && ndigits > 0 as libc::c_int {
                let fresh120 = p;
                p = p.offset(1);
                *fresh120 = '.' as i32 as libc::c_char;
            }
        }
        loop {
            let fresh121 = point;
            point -= 1;
            if fresh121 <= 0 as libc::c_int {
                break;
            }
            let fresh122 = p;
            p = p.offset(1);
            *fresh122 = '0' as i32 as libc::c_char;
        }
        *p = 0 as libc::c_int as libc::c_char;
    }
    buf as *const libc::c_char
}
#[no_mangle]
pub unsafe extern "C" fn jsV_tostring(J: *mut js_State, v: *mut js_Value) -> *const libc::c_char {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut p: *const libc::c_char = std::ptr::null::<libc::c_char>();
    match (*v).t.type_0 as libc::c_int {
        1 => b"undefined\0" as *const u8 as *const libc::c_char,
        2 => b"null\0" as *const u8 as *const libc::c_char,
        3 => {
            if (*v).u.boolean != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            }
        }
        5 => (*v).u.litstr,
        6 => ((*(*v).u.memstr).p).as_mut_ptr(),
        4 => {
            p = jsV_numbertostring(J, buf.as_mut_ptr(), (*v).u.number);
            if p == buf.as_mut_ptr() as *const libc::c_char {
                let mut n: libc::c_int = strlen(p) as libc::c_int;
                if n <= 15 as libc::c_ulong as libc::c_int {
                    let mut s: *mut libc::c_char = ((*v).u.shrstr).as_mut_ptr();
                    loop {
                        let fresh123 = n;
                        n -= 1;
                        if fresh123 == 0 {
                            break;
                        }
                        let fresh124 = p;
                        p = p.offset(1);
                        let fresh125 = s;
                        s = s.offset(1);
                        *fresh125 = *fresh124;
                    }
                    *s = 0 as libc::c_int as libc::c_char;
                    (*v).t.type_0 = JS_TSHRSTR as libc::c_int as libc::c_char;
                    return ((*v).u.shrstr).as_mut_ptr();
                } else {
                    (*v).u.memstr = jsV_newmemstring(J, p, n);
                    (*v).t.type_0 = JS_TMEMSTR as libc::c_int as libc::c_char;
                    return ((*(*v).u.memstr).p).as_mut_ptr();
                }
            }
            p
        }
        7 => {
            jsV_toprimitive(J, v, JS_HSTRING as libc::c_int);
            jsV_tostring(J, v)
        }
        0 | _ => ((*v).u.shrstr).as_mut_ptr(),
    }
}
unsafe extern "C" fn jsV_newboolean(J: *mut js_State, v: libc::c_int) -> *mut js_Object {
    let obj: *mut js_Object = jsV_newobject(J, JS_CBOOLEAN, (*J).Boolean_prototype);
    (*obj).u.boolean = v;
    obj
}
unsafe extern "C" fn jsV_newnumber(J: *mut js_State, v: libc::c_double) -> *mut js_Object {
    let obj: *mut js_Object = jsV_newobject(J, JS_CNUMBER, (*J).Number_prototype);
    (*obj).u.number = v;
    obj
}
unsafe extern "C" fn jsV_newstring(J: *mut js_State, v: *const libc::c_char) -> *mut js_Object {
    let obj: *mut js_Object = jsV_newobject(J, JS_CSTRING, (*J).String_prototype);
    let n: size_t = strlen(v);
    if n < ::core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong {
        (*obj).u.s.string = ((*obj).u.s.shrstr).as_mut_ptr();
        memcpy(
            ((*obj).u.s.shrstr).as_mut_ptr() as *mut libc::c_void,
            v as *const libc::c_void,
            n.wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        (*obj).u.s.string = js_strdup(J, v);
    }
    (*obj).u.s.length = js_utflen(v);
    obj
}
#[no_mangle]
pub unsafe extern "C" fn jsV_toobject(J: *mut js_State, v: *mut js_Value) -> *mut js_Object {
    let mut o: *mut js_Object = std::ptr::null_mut::<js_Object>();
    match (*v).t.type_0 as libc::c_int {
        2 => {
            js_typeerror(
                J,
                b"cannot convert null to object\0" as *const u8 as *const libc::c_char,
            );
        }
        7 => return (*v).u.object,
        0 => {
            o = jsV_newstring(J, ((*v).u.shrstr).as_mut_ptr());
        }
        5 => {
            o = jsV_newstring(J, (*v).u.litstr);
        }
        6 => {
            o = jsV_newstring(J, ((*(*v).u.memstr).p).as_mut_ptr());
        }
        3 => {
            o = jsV_newboolean(J, (*v).u.boolean);
        }
        4 => {
            o = jsV_newnumber(J, (*v).u.number);
        }
        1 | _ => {
            js_typeerror(
                J,
                b"cannot convert undefined to object\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*v).t.type_0 = JS_TOBJECT as libc::c_int as libc::c_char;
    (*v).u.object = o;
    o
}
#[no_mangle]
pub unsafe extern "C" fn js_newobjectx(J: *mut js_State) {
    let mut prototype: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, -(1 as libc::c_int)) != 0 {
        prototype = js_toobject(J, -(1 as libc::c_int));
    }
    js_pop(J, 1 as libc::c_int);
    js_pushobject(J, jsV_newobject(J, JS_COBJECT, prototype));
}
#[no_mangle]
pub unsafe extern "C" fn js_newobject(J: *mut js_State) {
    js_pushobject(J, jsV_newobject(J, JS_COBJECT, (*J).Object_prototype));
}
#[no_mangle]
pub unsafe extern "C" fn js_newarguments(J: *mut js_State) {
    js_pushobject(J, jsV_newobject(J, JS_CARGUMENTS, (*J).Object_prototype));
}
#[no_mangle]
pub unsafe extern "C" fn js_newarray(J: *mut js_State) {
    let obj: *mut js_Object = jsV_newobject(J, JS_CARRAY, (*J).Array_prototype);
    (*obj).u.a.simple = 1 as libc::c_int;
    js_pushobject(J, obj);
}
#[no_mangle]
pub unsafe extern "C" fn js_newboolean(J: *mut js_State, v: libc::c_int) {
    js_pushobject(J, jsV_newboolean(J, v));
}
#[no_mangle]
pub unsafe extern "C" fn js_newnumber(J: *mut js_State, v: libc::c_double) {
    js_pushobject(J, jsV_newnumber(J, v));
}
#[no_mangle]
pub unsafe extern "C" fn js_newstring(J: *mut js_State, v: *const libc::c_char) {
    js_pushobject(J, jsV_newstring(J, v));
}
#[no_mangle]
pub unsafe extern "C" fn js_newfunction(
    J: *mut js_State,
    fun: *mut js_Function,
    scope: *mut js_Environment,
) {
    let obj: *mut js_Object = jsV_newobject(J, JS_CFUNCTION, (*J).Function_prototype);
    (*obj).u.f.function = fun;
    (*obj).u.f.scope = scope;
    js_pushobject(J, obj);
    js_pushnumber(J, (*fun).numparams as libc::c_double);
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"length\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    js_newobject(J);
    js_copy(J, -(2 as libc::c_int));
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"constructor\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"prototype\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newscript(
    J: *mut js_State,
    fun: *mut js_Function,
    scope: *mut js_Environment,
) {
    let obj: *mut js_Object = jsV_newobject(J, JS_CSCRIPT, std::ptr::null_mut::<js_Object>());
    (*obj).u.f.function = fun;
    (*obj).u.f.scope = scope;
    js_pushobject(J, obj);
}
#[no_mangle]
pub unsafe extern "C" fn js_newcfunctionx(
    J: *mut js_State,
    cfun: js_CFunction,
    name: *const libc::c_char,
    length: libc::c_int,
    data: *mut libc::c_void,
    finalize: js_Finalize,
) {
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        if finalize.is_some() {
            finalize.expect("non-null function pointer")(J, data);
        }
        js_throw(J);
    }
    obj = jsV_newobject(J, JS_CCFUNCTION, (*J).Function_prototype);
    (*obj).u.c.name = name;
    (*obj).u.c.function = cfun;
    (*obj).u.c.constructor = None;
    (*obj).u.c.length = length;
    (*obj).u.c.data = data;
    (*obj).u.c.finalize = finalize;
    js_endtry(J);
    js_pushobject(J, obj);
    js_pushnumber(J, length as libc::c_double);
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"length\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    js_newobject(J);
    js_copy(J, -(2 as libc::c_int));
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"constructor\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"prototype\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newcfunction(
    J: *mut js_State,
    cfun: js_CFunction,
    name: *const libc::c_char,
    length: libc::c_int,
) {
    js_newcfunctionx(
        J,
        cfun,
        name,
        length,
        std::ptr::null_mut::<libc::c_void>(),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newcconstructor(
    J: *mut js_State,
    cfun: js_CFunction,
    ccon: js_CFunction,
    name: *const libc::c_char,
    length: libc::c_int,
) {
    let obj: *mut js_Object = jsV_newobject(J, JS_CCFUNCTION, (*J).Function_prototype);
    (*obj).u.c.name = name;
    (*obj).u.c.function = cfun;
    (*obj).u.c.constructor = ccon;
    (*obj).u.c.length = length;
    js_pushobject(J, obj);
    js_pushnumber(J, length as libc::c_double);
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"length\0" as *const u8 as *const libc::c_char,
        JS_READONLY as libc::c_int | JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
    js_rot2(J);
    js_copy(J, -(2 as libc::c_int));
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"constructor\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int,
    );
    js_defproperty(
        J,
        -(2 as libc::c_int),
        b"prototype\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as libc::c_int | JS_DONTCONF as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newuserdatax(
    J: *mut js_State,
    tag: *const libc::c_char,
    data: *mut libc::c_void,
    has: js_HasProperty,
    put: js_Put,
    delete: js_Delete,
    finalize: js_Finalize,
) {
    let mut prototype: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut obj: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_isobject(J, -(1 as libc::c_int)) != 0 {
        prototype = js_toobject(J, -(1 as libc::c_int));
    }
    js_pop(J, 1 as libc::c_int);
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        if finalize.is_some() {
            finalize.expect("non-null function pointer")(J, data);
        }
        js_throw(J);
    }
    obj = jsV_newobject(J, JS_CUSERDATA, prototype);
    (*obj).u.user.tag = tag;
    (*obj).u.user.data = data;
    (*obj).u.user.has = has;
    (*obj).u.user.put = put;
    (*obj).u.user.delete = delete;
    (*obj).u.user.finalize = finalize;
    js_endtry(J);
    js_pushobject(J, obj);
}
#[no_mangle]
pub unsafe extern "C" fn js_newuserdata(
    J: *mut js_State,
    tag: *const libc::c_char,
    data: *mut libc::c_void,
    finalize: js_Finalize,
) {
    js_newuserdatax(J, tag, data, None, None, None, finalize);
}
#[no_mangle]
pub unsafe extern "C" fn js_instanceof(J: *mut js_State) -> libc::c_int {
    let mut O: *mut js_Object = std::ptr::null_mut::<js_Object>();
    let mut V: *mut js_Object = std::ptr::null_mut::<js_Object>();
    if js_iscallable(J, -(1 as libc::c_int)) == 0 {
        js_typeerror(
            J,
            b"instanceof: invalid operand\0" as *const u8 as *const libc::c_char,
        );
    }
    if js_isobject(J, -(2 as libc::c_int)) == 0 {
        return 0 as libc::c_int;
    }
    js_getproperty(
        J,
        -(1 as libc::c_int),
        b"prototype\0" as *const u8 as *const libc::c_char,
    );
    if js_isobject(J, -(1 as libc::c_int)) == 0 {
        js_typeerror(
            J,
            b"instanceof: 'prototype' property is not an object\0" as *const u8
                as *const libc::c_char,
        );
    }
    O = js_toobject(J, -(1 as libc::c_int));
    js_pop(J, 1 as libc::c_int);
    V = js_toobject(J, -(2 as libc::c_int));
    while !V.is_null() {
        V = (*V).prototype;
        if O == V {
            return 1 as libc::c_int;
        }
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_concat(J: *mut js_State) {
    js_toprimitive(J, -(2 as libc::c_int), JS_HNONE as libc::c_int);
    js_toprimitive(J, -(1 as libc::c_int), JS_HNONE as libc::c_int);
    if js_isstring(J, -(2 as libc::c_int)) != 0 || js_isstring(J, -(1 as libc::c_int)) != 0 {
        let sa: *const libc::c_char = js_tostring(J, -(2 as libc::c_int));
        let sb: *const libc::c_char = js_tostring(J, -(1 as libc::c_int));
        let mut sab: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
        if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
            js_free(J, sab as *mut libc::c_void);
            js_throw(J);
        }
        ::core::ptr::write_volatile(
            &mut sab as *mut *mut libc::c_char,
            js_malloc(
                J,
                (strlen(sa))
                    .wrapping_add(strlen(sb))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            ) as *mut libc::c_char,
        );
        strcpy(sab, sa);
        strcat(sab, sb);
        js_pop(J, 2 as libc::c_int);
        js_pushstring(J, sab);
        js_endtry(J);
        js_free(J, sab as *mut libc::c_void);
    } else {
        let x: libc::c_double = js_tonumber(J, -(2 as libc::c_int));
        let y: libc::c_double = js_tonumber(J, -(1 as libc::c_int));
        js_pop(J, 2 as libc::c_int);
        js_pushnumber(J, x + y);
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_compare(J: *mut js_State, okay: *mut libc::c_int) -> libc::c_int {
    js_toprimitive(J, -(2 as libc::c_int), JS_HNUMBER as libc::c_int);
    js_toprimitive(J, -(1 as libc::c_int), JS_HNUMBER as libc::c_int);
    *okay = 1 as libc::c_int;
    if js_isstring(J, -(2 as libc::c_int)) != 0 && js_isstring(J, -(1 as libc::c_int)) != 0 {
        strcmp(
            js_tostring(J, -(2 as libc::c_int)),
            js_tostring(J, -(1 as libc::c_int)),
        )
    } else {
        let x: libc::c_double = js_tonumber(J, -(2 as libc::c_int));
        let y: libc::c_double = js_tonumber(J, -(1 as libc::c_int));
        if x.is_nan() as i32 != 0 || y.is_nan() as i32 != 0 {
            *okay = 0 as libc::c_int;
        }
        if x < y {
            -(1 as libc::c_int)
        } else if x > y {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_equal(J: *mut js_State) -> libc::c_int {
    let x: *mut js_Value = js_tovalue(J, -(2 as libc::c_int));
    let y: *mut js_Value = js_tovalue(J, -(1 as libc::c_int));
    loop {
        if ((*x).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
            || (*x).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
            || (*x).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int)
            && ((*y).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
                || (*y).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
                || (*y).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int)
        {
            return (strcmp(
                if (*x).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int {
                    ((*x).u.shrstr).as_mut_ptr() as *const libc::c_char
                } else if (*x).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int {
                    (*x).u.litstr
                } else if (*x).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int {
                    ((*(*x).u.memstr).p).as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if (*y).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int {
                    ((*y).u.shrstr).as_mut_ptr() as *const libc::c_char
                } else if (*y).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int {
                    (*y).u.litstr
                } else if (*y).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int {
                    ((*(*y).u.memstr).p).as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            ) == 0) as libc::c_int;
        }
        if (*x).t.type_0 as libc::c_int == (*y).t.type_0 as libc::c_int {
            if (*x).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int {
                return 1 as libc::c_int;
            }
            if (*x).t.type_0 as libc::c_int == JS_TNULL as libc::c_int {
                return 1 as libc::c_int;
            }
            if (*x).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int {
                return ((*x).u.number == (*y).u.number) as libc::c_int;
            }
            if (*x).t.type_0 as libc::c_int == JS_TBOOLEAN as libc::c_int {
                return ((*x).u.boolean == (*y).u.boolean) as libc::c_int;
            }
            if (*x).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int {
                return ((*x).u.object == (*y).u.object) as libc::c_int;
            }
            return 0 as libc::c_int;
        }
        if (*x).t.type_0 as libc::c_int == JS_TNULL as libc::c_int
            && (*y).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if (*x).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int
            && (*y).t.type_0 as libc::c_int == JS_TNULL as libc::c_int
        {
            return 1 as libc::c_int;
        }
        if (*x).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int
            && ((*y).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
                || (*y).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
                || (*y).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int)
        {
            return ((*x).u.number == jsV_tonumber(J, y)) as libc::c_int;
        }
        if ((*x).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
            || (*x).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
            || (*x).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int)
            && (*y).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int
        {
            return (jsV_tonumber(J, x) == (*y).u.number) as libc::c_int;
        }
        if (*x).t.type_0 as libc::c_int == JS_TBOOLEAN as libc::c_int {
            (*x).t.type_0 = JS_TNUMBER as libc::c_int as libc::c_char;
            (*x).u.number = (if (*x).u.boolean != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_double;
        } else if (*y).t.type_0 as libc::c_int == JS_TBOOLEAN as libc::c_int {
            (*y).t.type_0 = JS_TNUMBER as libc::c_int as libc::c_char;
            (*y).u.number = (if (*y).u.boolean != 0 {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_double;
        } else if ((*x).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
            || (*x).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
            || (*x).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int
            || (*x).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int)
            && (*y).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
        {
            jsV_toprimitive(J, y, JS_HNONE as libc::c_int);
        } else {
            if !((*x).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int
                && ((*y).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
                    || (*y).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
                    || (*y).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int
                    || (*y).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int))
            {
                break;
            }
            jsV_toprimitive(J, x, JS_HNONE as libc::c_int);
        }
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn js_strictequal(J: *mut js_State) -> libc::c_int {
    let x: *mut js_Value = js_tovalue(J, -(2 as libc::c_int));
    let y: *mut js_Value = js_tovalue(J, -(1 as libc::c_int));
    if ((*x).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
        || (*x).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
        || (*x).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int)
        && ((*y).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int
            || (*y).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int
            || (*y).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int)
    {
        return (strcmp(
            if (*x).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int {
                ((*x).u.shrstr).as_mut_ptr() as *const libc::c_char
            } else if (*x).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int {
                (*x).u.litstr
            } else if (*x).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int {
                ((*(*x).u.memstr).p).as_mut_ptr() as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if (*y).t.type_0 as libc::c_int == JS_TSHRSTR as libc::c_int {
                ((*y).u.shrstr).as_mut_ptr() as *const libc::c_char
            } else if (*y).t.type_0 as libc::c_int == JS_TLITSTR as libc::c_int {
                (*y).u.litstr
            } else if (*y).t.type_0 as libc::c_int == JS_TMEMSTR as libc::c_int {
                ((*(*y).u.memstr).p).as_mut_ptr() as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        ) == 0) as libc::c_int;
    }
    if (*x).t.type_0 as libc::c_int != (*y).t.type_0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*x).t.type_0 as libc::c_int == JS_TUNDEFINED as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*x).t.type_0 as libc::c_int == JS_TNULL as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*x).t.type_0 as libc::c_int == JS_TNUMBER as libc::c_int {
        return ((*x).u.number == (*y).u.number) as libc::c_int;
    }
    if (*x).t.type_0 as libc::c_int == JS_TBOOLEAN as libc::c_int {
        return ((*x).u.boolean == (*y).u.boolean) as libc::c_int;
    }
    if (*x).t.type_0 as libc::c_int == JS_TOBJECT as libc::c_int {
        return ((*x).u.object == (*y).u.object) as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn die(g: *mut cstate, message: *const libc::c_char) {
    (*g).error = message;
    longjmp(((*g).kaboom).as_mut_ptr(), 1 as libc::c_int);
}
unsafe extern "C" fn canon(c: Rune) -> libc::c_int {
    let u: Rune = jsU_toupperrune(c);
    if c >= 128 as libc::c_int && u < 128 as libc::c_int {
        return c;
    }
    u
}
unsafe extern "C" fn hex(g: *mut cstate, c: libc::c_int) -> libc::c_int {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 0xa as libc::c_int;
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 0xa as libc::c_int;
    }
    die(
        g,
        b"invalid escape sequence\0" as *const u8 as *const libc::c_char,
    );
    0 as libc::c_int
}
unsafe extern "C" fn dec(g: *mut cstate, c: libc::c_int) -> libc::c_int {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    die(
        g,
        b"invalid quantifier\0" as *const u8 as *const libc::c_char,
    );
    0 as libc::c_int
}
unsafe extern "C" fn isunicodeletter(c: libc::c_int) -> libc::c_int {
    (c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || jsU_isalpharune(c) != 0) as libc::c_int
}
unsafe extern "C" fn nextrune(g: *mut cstate) -> libc::c_int {
    if *(*g).source == 0 {
        (*g).yychar = -(1 as libc::c_int);
        return 0 as libc::c_int;
    }
    (*g).source = ((*g).source).offset(jsU_chartorune(&mut (*g).yychar, (*g).source) as isize);
    if (*g).yychar == '\\' as i32 {
        if *(*g).source == 0 {
            die(
                g,
                b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
            );
        }
        (*g).source = ((*g).source).offset(jsU_chartorune(&mut (*g).yychar, (*g).source) as isize);
        match (*g).yychar {
            102 => {
                (*g).yychar = '\u{c}' as i32;
                return 0 as libc::c_int;
            }
            110 => {
                (*g).yychar = '\n' as i32;
                return 0 as libc::c_int;
            }
            114 => {
                (*g).yychar = '\r' as i32;
                return 0 as libc::c_int;
            }
            116 => {
                (*g).yychar = '\t' as i32;
                return 0 as libc::c_int;
            }
            118 => {
                (*g).yychar = '\u{b}' as i32;
                return 0 as libc::c_int;
            }
            99 => {
                if *((*g).source).offset(0 as libc::c_int as isize) == 0 {
                    die(
                        g,
                        b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh126 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = *fresh126 as libc::c_int & 31 as libc::c_int;
                return 0 as libc::c_int;
            }
            120 => {
                if *((*g).source).offset(0 as libc::c_int as isize) == 0
                    || *((*g).source).offset(1 as libc::c_int as isize) == 0
                {
                    die(
                        g,
                        b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh127 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = hex(g, *fresh127 as libc::c_int) << 4 as libc::c_int;
                let fresh128 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh128 as libc::c_int);
                if (*g).yychar == 0 as libc::c_int {
                    (*g).yychar = '0' as i32;
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
            117 => {
                if *((*g).source).offset(0 as libc::c_int as isize) == 0
                    || *((*g).source).offset(1 as libc::c_int as isize) == 0
                    || *((*g).source).offset(2 as libc::c_int as isize) == 0
                    || *((*g).source).offset(3 as libc::c_int as isize) == 0
                {
                    die(
                        g,
                        b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh129 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = hex(g, *fresh129 as libc::c_int) << 12 as libc::c_int;
                let fresh130 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh130 as libc::c_int) << 8 as libc::c_int;
                let fresh131 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh131 as libc::c_int) << 4 as libc::c_int;
                let fresh132 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh132 as libc::c_int);
                if (*g).yychar == 0 as libc::c_int {
                    (*g).yychar = '0' as i32;
                    return 1 as libc::c_int;
                }
                return 0 as libc::c_int;
            }
            0 => {
                (*g).yychar = '0' as i32;
                return 1 as libc::c_int;
            }
            _ => {}
        }
        if !(strchr(
            b"BbDdSsWw^$\\.*+?()[]{}|-0123456789\0" as *const u8 as *const libc::c_char,
            (*g).yychar,
        ))
        .is_null()
        {
            return 1 as libc::c_int;
        }
        if isunicodeletter((*g).yychar) != 0 || (*g).yychar == '_' as i32 {
            die(
                g,
                b"invalid escape character\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn lexcount(g: *mut cstate) -> libc::c_int {
    let fresh133 = (*g).source;
    (*g).source = ((*g).source).offset(1);
    (*g).yychar = *fresh133 as Rune;
    (*g).yymin = dec(g, (*g).yychar);
    let fresh134 = (*g).source;
    (*g).source = ((*g).source).offset(1);
    (*g).yychar = *fresh134 as Rune;
    while (*g).yychar != ',' as i32 && (*g).yychar != '}' as i32 {
        (*g).yymin = (*g).yymin * 10 as libc::c_int + dec(g, (*g).yychar);
        let fresh135 = (*g).source;
        (*g).source = ((*g).source).offset(1);
        (*g).yychar = *fresh135 as Rune;
        if (*g).yymin >= 255 as libc::c_int {
            die(g, b"numeric overflow\0" as *const u8 as *const libc::c_char);
        }
    }
    if (*g).yychar == ',' as i32 {
        let fresh136 = (*g).source;
        (*g).source = ((*g).source).offset(1);
        (*g).yychar = *fresh136 as Rune;
        if (*g).yychar == '}' as i32 {
            (*g).yymax = 255 as libc::c_int;
        } else {
            (*g).yymax = dec(g, (*g).yychar);
            let fresh137 = (*g).source;
            (*g).source = ((*g).source).offset(1);
            (*g).yychar = *fresh137 as Rune;
            while (*g).yychar != '}' as i32 {
                (*g).yymax = (*g).yymax * 10 as libc::c_int + dec(g, (*g).yychar);
                let fresh138 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = *fresh138 as Rune;
                if (*g).yymax >= 255 as libc::c_int {
                    die(g, b"numeric overflow\0" as *const u8 as *const libc::c_char);
                }
            }
        }
    } else {
        (*g).yymax = (*g).yymin;
    }
    L_COUNT as libc::c_int
}
unsafe extern "C" fn newcclass(g: *mut cstate) {
    if (*g).ncclass >= 128 as libc::c_int {
        die(
            g,
            b"too many character classes\0" as *const u8 as *const libc::c_char,
        );
    }
    let fresh139 = (*g).ncclass;
    (*g).ncclass += 1;
    (*g).yycc = ((*g).cclass).as_mut_ptr().offset(fresh139 as isize);
    (*(*g).yycc).end = ((*(*g).yycc).spans).as_mut_ptr();
}
unsafe extern "C" fn addrange(g: *mut cstate, a: Rune, b: Rune) {
    let cc: *mut Reclass = (*g).yycc;
    let mut p: *mut Rune = std::ptr::null_mut::<Rune>();
    if a > b {
        die(
            g,
            b"invalid character class range\0" as *const u8 as *const libc::c_char,
        );
    }
    p = ((*cc).spans).as_mut_ptr();
    while p < (*cc).end {
        if a >= *p.offset(0 as libc::c_int as isize) && b <= *p.offset(1 as libc::c_int as isize) {
            return;
        }
        if a < *p.offset(0 as libc::c_int as isize) && b >= *p.offset(1 as libc::c_int as isize) {
            *p.offset(0 as libc::c_int as isize) = a;
            *p.offset(1 as libc::c_int as isize) = b;
            return;
        }
        if b >= *p.offset(0 as libc::c_int as isize) - 1 as libc::c_int
            && b <= *p.offset(1 as libc::c_int as isize)
            && a < *p.offset(0 as libc::c_int as isize)
        {
            *p.offset(0 as libc::c_int as isize) = a;
            return;
        }
        if a >= *p.offset(0 as libc::c_int as isize)
            && a <= *p.offset(1 as libc::c_int as isize) + 1 as libc::c_int
            && b > *p.offset(1 as libc::c_int as isize)
        {
            *p.offset(1 as libc::c_int as isize) = b;
            return;
        }
        p = p.offset(2 as libc::c_int as isize);
    }
    if ((*cc).end).offset(2 as libc::c_int as isize)
        >= ((*cc).spans).as_mut_ptr().offset(
            (::core::mem::size_of::<[Rune; 64]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong)
                as libc::c_int as isize,
        )
    {
        die(
            g,
            b"too many character class ranges\0" as *const u8 as *const libc::c_char,
        );
    }
    let fresh140 = (*cc).end;
    (*cc).end = ((*cc).end).offset(1);
    *fresh140 = a;
    let fresh141 = (*cc).end;
    (*cc).end = ((*cc).end).offset(1);
    *fresh141 = b;
}
unsafe extern "C" fn addranges_d(g: *mut cstate) {
    addrange(g, '0' as i32, '9' as i32);
}
unsafe extern "C" fn addranges_D(g: *mut cstate) {
    addrange(g, 0 as libc::c_int, '0' as i32 - 1 as libc::c_int);
    addrange(g, '9' as i32 + 1 as libc::c_int, 0xffff as libc::c_int);
}
unsafe extern "C" fn addranges_s(g: *mut cstate) {
    addrange(g, 0x9 as libc::c_int, 0xd as libc::c_int);
    addrange(g, 0x20 as libc::c_int, 0x20 as libc::c_int);
    addrange(g, 0xa0 as libc::c_int, 0xa0 as libc::c_int);
    addrange(g, 0x2028 as libc::c_int, 0x2029 as libc::c_int);
    addrange(g, 0xfeff as libc::c_int, 0xfeff as libc::c_int);
}
unsafe extern "C" fn addranges_S(g: *mut cstate) {
    addrange(g, 0 as libc::c_int, 0x9 as libc::c_int - 1 as libc::c_int);
    addrange(
        g,
        0xd as libc::c_int + 1 as libc::c_int,
        0x20 as libc::c_int - 1 as libc::c_int,
    );
    addrange(
        g,
        0x20 as libc::c_int + 1 as libc::c_int,
        0xa0 as libc::c_int - 1 as libc::c_int,
    );
    addrange(
        g,
        0xa0 as libc::c_int + 1 as libc::c_int,
        0x2028 as libc::c_int - 1 as libc::c_int,
    );
    addrange(
        g,
        0x2029 as libc::c_int + 1 as libc::c_int,
        0xfeff as libc::c_int - 1 as libc::c_int,
    );
    addrange(
        g,
        0xfeff as libc::c_int + 1 as libc::c_int,
        0xffff as libc::c_int,
    );
}
unsafe extern "C" fn addranges_w(g: *mut cstate) {
    addrange(g, '0' as i32, '9' as i32);
    addrange(g, 'A' as i32, 'Z' as i32);
    addrange(g, '_' as i32, '_' as i32);
    addrange(g, 'a' as i32, 'z' as i32);
}
unsafe extern "C" fn addranges_W(g: *mut cstate) {
    addrange(g, 0 as libc::c_int, '0' as i32 - 1 as libc::c_int);
    addrange(
        g,
        '9' as i32 + 1 as libc::c_int,
        'A' as i32 - 1 as libc::c_int,
    );
    addrange(
        g,
        'Z' as i32 + 1 as libc::c_int,
        '_' as i32 - 1 as libc::c_int,
    );
    addrange(
        g,
        '_' as i32 + 1 as libc::c_int,
        'a' as i32 - 1 as libc::c_int,
    );
    addrange(g, 'z' as i32 + 1 as libc::c_int, 0xffff as libc::c_int);
}
unsafe extern "C" fn lexclass(g: *mut cstate) -> libc::c_int {
    let mut type_0: libc::c_int = L_CCLASS as libc::c_int;
    let mut quoted: libc::c_int = 0;
    let mut havesave: libc::c_int = 0;
    let mut havedash: libc::c_int = 0;
    let mut save: Rune = 0 as libc::c_int;
    newcclass(g);
    quoted = nextrune(g);
    if quoted == 0 && (*g).yychar == '^' as i32 {
        type_0 = L_NCCLASS as libc::c_int;
        quoted = nextrune(g);
    }
    havedash = 0 as libc::c_int;
    havesave = havedash;
    loop {
        if (*g).yychar == -(1 as libc::c_int) {
            die(
                g,
                b"unterminated character class\0" as *const u8 as *const libc::c_char,
            );
        }
        if quoted == 0 && (*g).yychar == ']' as i32 {
            break;
        }
        if quoted == 0 && (*g).yychar == '-' as i32 {
            if havesave != 0 {
                if havedash != 0 {
                    addrange(g, save, '-' as i32);
                    havedash = 0 as libc::c_int;
                    havesave = havedash;
                } else {
                    havedash = 1 as libc::c_int;
                }
            } else {
                save = '-' as i32;
                havesave = 1 as libc::c_int;
            }
        } else if quoted != 0
            && !(strchr(b"DSWdsw\0" as *const u8 as *const libc::c_char, (*g).yychar)).is_null()
        {
            if havesave != 0 {
                addrange(g, save, save);
                if havedash != 0 {
                    addrange(g, '-' as i32, '-' as i32);
                }
            }
            match (*g).yychar {
                100 => {
                    addranges_d(g);
                }
                115 => {
                    addranges_s(g);
                }
                119 => {
                    addranges_w(g);
                }
                68 => {
                    addranges_D(g);
                }
                83 => {
                    addranges_S(g);
                }
                87 => {
                    addranges_W(g);
                }
                _ => {}
            }
            havedash = 0 as libc::c_int;
            havesave = havedash;
        } else {
            if quoted != 0 {
                if (*g).yychar == 'b' as i32 {
                    (*g).yychar = '\u{8}' as i32;
                } else if (*g).yychar == '0' as i32 {
                    (*g).yychar = 0 as libc::c_int;
                }
            }
            if havesave != 0 {
                if havedash != 0 {
                    addrange(g, save, (*g).yychar);
                    havedash = 0 as libc::c_int;
                    havesave = havedash;
                } else {
                    addrange(g, save, save);
                    save = (*g).yychar;
                }
            } else {
                save = (*g).yychar;
                havesave = 1 as libc::c_int;
            }
        }
        quoted = nextrune(g);
    }
    if havesave != 0 {
        addrange(g, save, save);
        if havedash != 0 {
            addrange(g, '-' as i32, '-' as i32);
        }
    }
    type_0
}
unsafe extern "C" fn lex(g: *mut cstate) -> libc::c_int {
    let quoted: libc::c_int = nextrune(g);
    if quoted != 0 {
        match (*g).yychar {
            98 => return L_WORD as libc::c_int,
            66 => return L_NWORD as libc::c_int,
            100 => {
                newcclass(g);
                addranges_d(g);
                return L_CCLASS as libc::c_int;
            }
            115 => {
                newcclass(g);
                addranges_s(g);
                return L_CCLASS as libc::c_int;
            }
            119 => {
                newcclass(g);
                addranges_w(g);
                return L_CCLASS as libc::c_int;
            }
            68 => {
                newcclass(g);
                addranges_d(g);
                return L_NCCLASS as libc::c_int;
            }
            83 => {
                newcclass(g);
                addranges_s(g);
                return L_NCCLASS as libc::c_int;
            }
            87 => {
                newcclass(g);
                addranges_w(g);
                return L_NCCLASS as libc::c_int;
            }
            48 => {
                (*g).yychar = 0 as libc::c_int;
                return L_CHAR as libc::c_int;
            }
            _ => {}
        }
        if (*g).yychar >= '0' as i32 && (*g).yychar <= '9' as i32 {
            (*g).yychar -= '0' as i32;
            if *(*g).source as libc::c_int >= '0' as i32
                && *(*g).source as libc::c_int <= '9' as i32
            {
                let fresh142 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar =
                    (*g).yychar * 10 as libc::c_int + *fresh142 as libc::c_int - '0' as i32;
            }
            return L_REF as libc::c_int;
        }
        return L_CHAR as libc::c_int;
    }
    match (*g).yychar {
        -1 | 36 | 41 | 42 | 43 | 46 | 63 | 94 | 124 => return (*g).yychar,
        _ => {}
    }
    if (*g).yychar == '{' as i32 {
        return lexcount(g);
    }
    if (*g).yychar == '[' as i32 {
        return lexclass(g);
    }
    if (*g).yychar == '(' as i32 {
        if *((*g).source).offset(0 as libc::c_int as isize) as libc::c_int == '?' as i32 {
            if *((*g).source).offset(1 as libc::c_int as isize) as libc::c_int == ':' as i32 {
                (*g).source = ((*g).source).offset(2 as libc::c_int as isize);
                return L_NC as libc::c_int;
            }
            if *((*g).source).offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
                (*g).source = ((*g).source).offset(2 as libc::c_int as isize);
                return L_PLA as libc::c_int;
            }
            if *((*g).source).offset(1 as libc::c_int as isize) as libc::c_int == '!' as i32 {
                (*g).source = ((*g).source).offset(2 as libc::c_int as isize);
                return L_NLA as libc::c_int;
            }
        }
        return '(' as i32;
    }
    L_CHAR as libc::c_int
}
unsafe extern "C" fn newnode(g: *mut cstate, type_0: libc::c_int) -> *mut Renode {
    let fresh143 = (*g).pend;
    (*g).pend = ((*g).pend).offset(1);
    let node: *mut Renode = fresh143;
    (*node).type_0 = type_0 as libc::c_uchar;
    (*node).cc = -(1 as libc::c_int);
    (*node).c = 0 as libc::c_int;
    (*node).ng = 0 as libc::c_int as libc::c_uchar;
    (*node).m = 0 as libc::c_int as libc::c_uchar;
    (*node).n = 0 as libc::c_int as libc::c_uchar;
    (*node).y = std::ptr::null_mut::<Renode>();
    (*node).x = (*node).y;
    node
}
unsafe extern "C" fn empty(node: *mut Renode) -> libc::c_int {
    if node.is_null() {
        return 1 as libc::c_int;
    }
    match (*node).type_0 as libc::c_int {
        0 => (empty((*node).x) != 0 && empty((*node).y) != 0) as libc::c_int,
        1 => (empty((*node).x) != 0 || empty((*node).y) != 0) as libc::c_int,
        2 => (empty((*node).x) != 0 || (*node).m as libc::c_int == 0 as libc::c_int) as libc::c_int,
        7 => empty((*node).x),
        14 => empty((*node).x),
        10..=13 => 0 as libc::c_int,
        _ => 1 as libc::c_int,
    }
}
unsafe extern "C" fn newrep(
    g: *mut cstate,
    atom: *mut Renode,
    ng: libc::c_int,
    min: libc::c_int,
    max: libc::c_int,
) -> *mut Renode {
    let rep: *mut Renode = newnode(g, P_REP as libc::c_int);
    if max == 255 as libc::c_int && empty(atom) != 0 {
        die(
            g,
            b"infinite loop matching the empty string\0" as *const u8 as *const libc::c_char,
        );
    }
    (*rep).ng = ng as libc::c_uchar;
    (*rep).m = min as libc::c_uchar;
    (*rep).n = max as libc::c_uchar;
    (*rep).x = atom;
    rep
}
unsafe extern "C" fn regnext(g: *mut cstate) {
    (*g).lookahead = lex(g);
}
unsafe extern "C" fn regaccept(g: *mut cstate, t: libc::c_int) -> libc::c_int {
    if (*g).lookahead == t {
        regnext(g);
        return 1 as libc::c_int;
    }
    0 as libc::c_int
}
unsafe extern "C" fn parseatom(g: *mut cstate) -> *mut Renode {
    let mut atom: *mut Renode = std::ptr::null_mut::<Renode>();
    if (*g).lookahead == L_CHAR as libc::c_int {
        atom = newnode(g, P_CHAR as libc::c_int);
        (*atom).c = (*g).yychar;
        regnext(g);
        return atom;
    }
    if (*g).lookahead == L_CCLASS as libc::c_int {
        atom = newnode(g, P_CCLASS as libc::c_int);
        (*atom).cc =
            ((*g).yycc).offset_from(((*g).cclass).as_mut_ptr()) as libc::c_long as libc::c_int;
        regnext(g);
        return atom;
    }
    if (*g).lookahead == L_NCCLASS as libc::c_int {
        atom = newnode(g, P_NCCLASS as libc::c_int);
        (*atom).cc =
            ((*g).yycc).offset_from(((*g).cclass).as_mut_ptr()) as libc::c_long as libc::c_int;
        regnext(g);
        return atom;
    }
    if (*g).lookahead == L_REF as libc::c_int {
        atom = newnode(g, P_REF as libc::c_int);
        if (*g).yychar == 0 as libc::c_int
            || (*g).yychar >= (*g).nsub
            || ((*g).sub[(*g).yychar as usize]).is_null()
        {
            die(
                g,
                b"invalid back-reference\0" as *const u8 as *const libc::c_char,
            );
        }
        (*atom).n = (*g).yychar as libc::c_uchar;
        (*atom).x = (*g).sub[(*g).yychar as usize];
        regnext(g);
        return atom;
    }
    if regaccept(g, '.' as i32) != 0 {
        return newnode(g, P_ANY as libc::c_int);
    }
    if regaccept(g, '(' as i32) != 0 {
        atom = newnode(g, P_PAR as libc::c_int);
        if (*g).nsub == 16 as libc::c_int {
            die(
                g,
                b"too many captures\0" as *const u8 as *const libc::c_char,
            );
        }
        let fresh144 = (*g).nsub;
        (*g).nsub += 1;
        (*atom).n = fresh144 as libc::c_uchar;
        (*atom).x = parsealt(g);
        (*g).sub[(*atom).n as usize] = atom;
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    if regaccept(g, L_NC as libc::c_int) != 0 {
        atom = parsealt(g);
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    if regaccept(g, L_PLA as libc::c_int) != 0 {
        atom = newnode(g, P_PLA as libc::c_int);
        (*atom).x = parsealt(g);
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    if regaccept(g, L_NLA as libc::c_int) != 0 {
        atom = newnode(g, P_NLA as libc::c_int);
        (*atom).x = parsealt(g);
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    die(g, b"syntax error\0" as *const u8 as *const libc::c_char);
    std::ptr::null_mut::<Renode>()
}
unsafe extern "C" fn parserep(g: *mut cstate) -> *mut Renode {
    let mut atom: *mut Renode = std::ptr::null_mut::<Renode>();
    if regaccept(g, '^' as i32) != 0 {
        return newnode(g, P_BOL as libc::c_int);
    }
    if regaccept(g, '$' as i32) != 0 {
        return newnode(g, P_EOL as libc::c_int);
    }
    if regaccept(g, L_WORD as libc::c_int) != 0 {
        return newnode(g, P_WORD as libc::c_int);
    }
    if regaccept(g, L_NWORD as libc::c_int) != 0 {
        return newnode(g, P_NWORD as libc::c_int);
    }
    atom = parseatom(g);
    if (*g).lookahead == L_COUNT as libc::c_int {
        let min: libc::c_int = (*g).yymin;
        let max: libc::c_int = (*g).yymax;
        regnext(g);
        if max < min {
            die(
                g,
                b"invalid quantifier\0" as *const u8 as *const libc::c_char,
            );
        }
        return newrep(g, atom, regaccept(g, '?' as i32), min, max);
    }
    if regaccept(g, '*' as i32) != 0 {
        return newrep(
            g,
            atom,
            regaccept(g, '?' as i32),
            0 as libc::c_int,
            255 as libc::c_int,
        );
    }
    if regaccept(g, '+' as i32) != 0 {
        return newrep(
            g,
            atom,
            regaccept(g, '?' as i32),
            1 as libc::c_int,
            255 as libc::c_int,
        );
    }
    if regaccept(g, '?' as i32) != 0 {
        return newrep(
            g,
            atom,
            regaccept(g, '?' as i32),
            0 as libc::c_int,
            1 as libc::c_int,
        );
    }
    atom
}
unsafe extern "C" fn parsecat(g: *mut cstate) -> *mut Renode {
    let mut cat: *mut Renode = std::ptr::null_mut::<Renode>();
    let mut head: *mut Renode = std::ptr::null_mut::<Renode>();
    let mut tail: *mut *mut Renode = std::ptr::null_mut::<*mut Renode>();
    if (*g).lookahead != -(1 as libc::c_int)
        && (*g).lookahead != '|' as i32
        && (*g).lookahead != ')' as i32
    {
        head = parserep(g);
        tail = &mut head;
        while (*g).lookahead != -(1 as libc::c_int)
            && (*g).lookahead != '|' as i32
            && (*g).lookahead != ')' as i32
        {
            cat = newnode(g, P_CAT as libc::c_int);
            (*cat).x = *tail;
            (*cat).y = parserep(g);
            *tail = cat;
            tail = &mut (*cat).y;
        }
        return head;
    }
    std::ptr::null_mut::<Renode>()
}
unsafe extern "C" fn parsealt(g: *mut cstate) -> *mut Renode {
    let mut alt: *mut Renode = std::ptr::null_mut::<Renode>();
    let mut x: *mut Renode = std::ptr::null_mut::<Renode>();
    alt = parsecat(g);
    while regaccept(g, '|' as i32) != 0 {
        x = alt;
        alt = newnode(g, P_ALT as libc::c_int);
        (*alt).x = x;
        (*alt).y = parsecat(g);
    }
    alt
}
unsafe extern "C" fn count(
    g: *mut cstate,
    node: *mut Renode,
    mut depth: libc::c_int,
) -> libc::c_int {
    let mut min: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if node.is_null() {
        return 0 as libc::c_int;
    }
    depth += 1;
    if depth > 1024 as libc::c_int {
        die(g, b"stack overflow\0" as *const u8 as *const libc::c_char);
    }
    match (*node).type_0 as libc::c_int {
        0 => count(g, (*node).x, depth) + count(g, (*node).y, depth),
        1 => count(g, (*node).x, depth) + count(g, (*node).y, depth) + 2 as libc::c_int,
        2 => {
            min = (*node).m as libc::c_int;
            max = (*node).n as libc::c_int;
            if min == max {
                n = count(g, (*node).x, depth) * min;
            } else if max < 255 as libc::c_int {
                n = count(g, (*node).x, depth) * max + (max - min);
            } else {
                n = count(g, (*node).x, depth) * (min + 1 as libc::c_int) + 2 as libc::c_int;
            }
            if n < 0 as libc::c_int || n > (32 as libc::c_int) << 10 as libc::c_int {
                die(
                    g,
                    b"program too large\0" as *const u8 as *const libc::c_char,
                );
            }
            n
        }
        7 => count(g, (*node).x, depth) + 2 as libc::c_int,
        8 => count(g, (*node).x, depth) + 2 as libc::c_int,
        9 => count(g, (*node).x, depth) + 2 as libc::c_int,
        _ => 1 as libc::c_int,
    }
}
unsafe extern "C" fn regemit(prog: *mut Reprog, opcode: libc::c_int) -> *mut Reinst {
    let fresh145 = (*prog).end;
    (*prog).end = ((*prog).end).offset(1);
    let inst: *mut Reinst = fresh145;
    (*inst).opcode = opcode as libc::c_uchar;
    (*inst).n = 0 as libc::c_int as libc::c_uchar;
    (*inst).c = 0 as libc::c_int;
    (*inst).cc = std::ptr::null_mut::<Reclass>();
    (*inst).y = std::ptr::null_mut::<Reinst>();
    (*inst).x = (*inst).y;
    inst
}
unsafe extern "C" fn compile(prog: *mut Reprog, mut node: *mut Renode) {
    let current_block: u64;
    let mut inst: *mut Reinst = std::ptr::null_mut::<Reinst>();
    let mut split_0: *mut Reinst = std::ptr::null_mut::<Reinst>();
    let mut jump: *mut Reinst = std::ptr::null_mut::<Reinst>();
    let mut i: libc::c_int = 0;
    loop {
        if node.is_null() {
            return;
        }
        match (*node).type_0 as libc::c_int {
            0 => {
                compile(prog, (*node).x);
                node = (*node).y;
            }
            1 => {
                split_0 = regemit(prog, I_SPLIT as libc::c_int);
                compile(prog, (*node).x);
                jump = regemit(prog, I_JUMP as libc::c_int);
                compile(prog, (*node).y);
                (*split_0).x = split_0.offset(1 as libc::c_int as isize);
                (*split_0).y = jump.offset(1 as libc::c_int as isize);
                (*jump).x = (*prog).end;
                current_block = 13853033528615664019;
                break;
            }
            2 => {
                inst = std::ptr::null_mut::<Reinst>();
                i = 0 as libc::c_int;
                while i < (*node).m as libc::c_int {
                    inst = (*prog).end;
                    compile(prog, (*node).x);
                    i += 1;
                    i;
                }
                if (*node).m as libc::c_int == (*node).n as libc::c_int {
                    current_block = 13853033528615664019;
                    break;
                } else {
                    current_block = 8831408221741692167;
                    break;
                }
            }
            3 => {
                regemit(prog, I_BOL as libc::c_int);
                current_block = 13853033528615664019;
                break;
            }
            4 => {
                regemit(prog, I_EOL as libc::c_int);
                current_block = 13853033528615664019;
                break;
            }
            5 => {
                regemit(prog, I_WORD as libc::c_int);
                current_block = 13853033528615664019;
                break;
            }
            6 => {
                regemit(prog, I_NWORD as libc::c_int);
                current_block = 13853033528615664019;
                break;
            }
            7 => {
                inst = regemit(prog, I_LPAR as libc::c_int);
                (*inst).n = (*node).n;
                compile(prog, (*node).x);
                inst = regemit(prog, I_RPAR as libc::c_int);
                (*inst).n = (*node).n;
                current_block = 13853033528615664019;
                break;
            }
            8 => {
                split_0 = regemit(prog, I_PLA as libc::c_int);
                compile(prog, (*node).x);
                regemit(prog, I_END as libc::c_int);
                (*split_0).x = split_0.offset(1 as libc::c_int as isize);
                (*split_0).y = (*prog).end;
                current_block = 13853033528615664019;
                break;
            }
            9 => {
                split_0 = regemit(prog, I_NLA as libc::c_int);
                compile(prog, (*node).x);
                regemit(prog, I_END as libc::c_int);
                (*split_0).x = split_0.offset(1 as libc::c_int as isize);
                (*split_0).y = (*prog).end;
                current_block = 13853033528615664019;
                break;
            }
            10 => {
                regemit(prog, I_ANY as libc::c_int);
                current_block = 13853033528615664019;
                break;
            }
            11 => {
                inst = regemit(prog, I_CHAR as libc::c_int);
                (*inst).c = if (*prog).flags & REG_ICASE as libc::c_int != 0 {
                    canon((*node).c)
                } else {
                    (*node).c
                };
                current_block = 13853033528615664019;
                break;
            }
            12 => {
                inst = regemit(prog, I_CCLASS as libc::c_int);
                (*inst).cc = ((*prog).cclass).offset((*node).cc as isize);
                current_block = 13853033528615664019;
                break;
            }
            13 => {
                inst = regemit(prog, I_NCCLASS as libc::c_int);
                (*inst).cc = ((*prog).cclass).offset((*node).cc as isize);
                current_block = 13853033528615664019;
                break;
            }
            14 => {
                inst = regemit(prog, I_REF as libc::c_int);
                (*inst).n = (*node).n;
                current_block = 13853033528615664019;
                break;
            }
            _ => {
                current_block = 13853033528615664019;
                break;
            }
        }
    }
    if current_block == 8831408221741692167 {
        if ((*node).n as libc::c_int) < 255 as libc::c_int {
            i = (*node).m as libc::c_int;
            while i < (*node).n as libc::c_int {
                split_0 = regemit(prog, I_SPLIT as libc::c_int);
                compile(prog, (*node).x);
                if (*node).ng != 0 {
                    (*split_0).y = split_0.offset(1 as libc::c_int as isize);
                    (*split_0).x = (*prog).end;
                } else {
                    (*split_0).x = split_0.offset(1 as libc::c_int as isize);
                    (*split_0).y = (*prog).end;
                }
                i += 1;
                i;
            }
        } else if (*node).m as libc::c_int == 0 as libc::c_int {
            split_0 = regemit(prog, I_SPLIT as libc::c_int);
            compile(prog, (*node).x);
            jump = regemit(prog, I_JUMP as libc::c_int);
            if (*node).ng != 0 {
                (*split_0).y = split_0.offset(1 as libc::c_int as isize);
                (*split_0).x = (*prog).end;
            } else {
                (*split_0).x = split_0.offset(1 as libc::c_int as isize);
                (*split_0).y = (*prog).end;
            }
            (*jump).x = split_0;
        } else {
            split_0 = regemit(prog, I_SPLIT as libc::c_int);
            if (*node).ng != 0 {
                (*split_0).y = inst;
                (*split_0).x = (*prog).end;
            } else {
                (*split_0).x = inst;
                (*split_0).y = (*prog).end;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_regcompx(
    alloc: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            libc::c_int,
        ) -> *mut libc::c_void,
    >,
    ctx: *mut libc::c_void,
    pattern: *const libc::c_char,
    cflags: libc::c_int,
    errorp: *mut *const libc::c_char,
) -> *mut Reprog {
    let mut g: cstate = cstate {
        prog: std::ptr::null_mut::<Reprog>(),
        pstart: std::ptr::null_mut::<Renode>(),
        pend: std::ptr::null_mut::<Renode>(),
        source: std::ptr::null::<libc::c_char>(),
        ncclass: 0,
        nsub: 0,
        sub: [std::ptr::null_mut::<Renode>(); 16],
        lookahead: 0,
        yychar: 0,
        yycc: std::ptr::null_mut::<Reclass>(),
        yymin: 0,
        yymax: 0,
        error: std::ptr::null::<libc::c_char>(),
        kaboom: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        cclass: [Reclass {
            end: std::ptr::null_mut::<Rune>(),
            spans: [0; 64],
        }; 128],
    };
    let mut node: *mut Renode = std::ptr::null_mut::<Renode>();
    let mut split_0: *mut Reinst = std::ptr::null_mut::<Reinst>();
    let mut jump: *mut Reinst = std::ptr::null_mut::<Reinst>();
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    g.pstart = std::ptr::null_mut::<Renode>();
    g.prog = std::ptr::null_mut::<Reprog>();
    if _setjmp((g.kaboom).as_mut_ptr()) != 0 {
        if !errorp.is_null() {
            *errorp = g.error;
        }
        alloc.expect("non-null function pointer")(
            ctx,
            g.pstart as *mut libc::c_void,
            0 as libc::c_int,
        );
        if !(g.prog).is_null() {
            alloc.expect("non-null function pointer")(
                ctx,
                (*g.prog).cclass as *mut libc::c_void,
                0 as libc::c_int,
            );
            alloc.expect("non-null function pointer")(
                ctx,
                (*g.prog).start as *mut libc::c_void,
                0 as libc::c_int,
            );
            alloc.expect("non-null function pointer")(
                ctx,
                g.prog as *mut libc::c_void,
                0 as libc::c_int,
            );
        }
        return std::ptr::null_mut::<Reprog>();
    }
    g.prog = alloc.expect("non-null function pointer")(
        ctx,
        std::ptr::null_mut::<libc::c_void>(),
        ::core::mem::size_of::<Reprog>() as libc::c_ulong as libc::c_int,
    ) as *mut Reprog;
    if (g.prog).is_null() {
        die(
            &mut g,
            b"cannot allocate regular expression\0" as *const u8 as *const libc::c_char,
        );
    }
    (*g.prog).start = std::ptr::null_mut::<Reinst>();
    (*g.prog).cclass = std::ptr::null_mut::<Reclass>();
    n = (strlen(pattern)).wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    if n > (32 as libc::c_int) << 10 as libc::c_int {
        die(
            &mut g,
            b"program too large\0" as *const u8 as *const libc::c_char,
        );
    }
    if n > 0 as libc::c_int {
        g.pend = alloc.expect("non-null function pointer")(
            ctx,
            std::ptr::null_mut::<libc::c_void>(),
            (::core::mem::size_of::<Renode>() as libc::c_ulong).wrapping_mul(n as libc::c_ulong)
                as libc::c_int,
        ) as *mut Renode;
        g.pstart = g.pend;
        if (g.pstart).is_null() {
            die(
                &mut g,
                b"cannot allocate regular expression parse list\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    g.source = pattern;
    g.ncclass = 0 as libc::c_int;
    g.nsub = 1 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        g.sub[i as usize] = std::ptr::null_mut::<Renode>();
        i += 1;
        i;
    }
    (*g.prog).flags = cflags;
    regnext(&mut g);
    node = parsealt(&mut g);
    if g.lookahead == ')' as i32 {
        die(
            &mut g,
            b"unmatched ')'\0" as *const u8 as *const libc::c_char,
        );
    }
    if g.lookahead != -(1 as libc::c_int) {
        die(
            &mut g,
            b"syntax error\0" as *const u8 as *const libc::c_char,
        );
    }
    n = 6 as libc::c_int + count(&mut g, node, 0 as libc::c_int);
    if n < 0 as libc::c_int || n > (32 as libc::c_int) << 10 as libc::c_int {
        die(
            &mut g,
            b"program too large\0" as *const u8 as *const libc::c_char,
        );
    }
    (*g.prog).nsub = g.nsub;
    (*g.prog).end = alloc.expect("non-null function pointer")(
        ctx,
        std::ptr::null_mut::<libc::c_void>(),
        (n as libc::c_ulong).wrapping_mul(::core::mem::size_of::<Reinst>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut Reinst;
    (*g.prog).start = (*g.prog).end;
    if ((*g.prog).start).is_null() {
        die(
            &mut g,
            b"cannot allocate regular expression instruction list\0" as *const u8
                as *const libc::c_char,
        );
    }
    if g.ncclass > 0 as libc::c_int {
        (*g.prog).cclass = alloc.expect("non-null function pointer")(
            ctx,
            std::ptr::null_mut::<libc::c_void>(),
            (g.ncclass as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Reclass>() as libc::c_ulong)
                as libc::c_int,
        ) as *mut Reclass;
        if ((*g.prog).cclass).is_null() {
            die(
                &mut g,
                b"cannot allocate regular expression character class list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memcpy(
            (*g.prog).cclass as *mut libc::c_void,
            (g.cclass).as_mut_ptr() as *const libc::c_void,
            (g.ncclass as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<Reclass>() as libc::c_ulong),
        );
        i = 0 as libc::c_int;
        while i < g.ncclass {
            let fresh146 = &mut (*((*g.prog).cclass).offset(i as isize)).end;
            *fresh146 = ((*((*g.prog).cclass).offset(i as isize)).spans)
                .as_mut_ptr()
                .offset(
                    (g.cclass[i as usize].end)
                        .offset_from((g.cclass[i as usize].spans).as_mut_ptr())
                        as libc::c_long as isize,
                );
            i += 1;
            i;
        }
    }
    split_0 = regemit(g.prog, I_SPLIT as libc::c_int);
    (*split_0).x = split_0.offset(3 as libc::c_int as isize);
    (*split_0).y = split_0.offset(1 as libc::c_int as isize);
    regemit(g.prog, I_ANYNL as libc::c_int);
    jump = regemit(g.prog, I_JUMP as libc::c_int);
    (*jump).x = split_0;
    regemit(g.prog, I_LPAR as libc::c_int);
    compile(g.prog, node);
    regemit(g.prog, I_RPAR as libc::c_int);
    regemit(g.prog, I_END as libc::c_int);
    alloc.expect("non-null function pointer")(ctx, g.pstart as *mut libc::c_void, 0 as libc::c_int);
    if !errorp.is_null() {
        *errorp = std::ptr::null::<libc::c_char>();
    }
    g.prog
}
#[no_mangle]
pub unsafe extern "C" fn js_regfreex(
    alloc: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            libc::c_int,
        ) -> *mut libc::c_void,
    >,
    ctx: *mut libc::c_void,
    prog: *mut Reprog,
) {
    if !prog.is_null() {
        if !((*prog).cclass).is_null() {
            alloc.expect("non-null function pointer")(
                ctx,
                (*prog).cclass as *mut libc::c_void,
                0 as libc::c_int,
            );
        }
        alloc.expect("non-null function pointer")(
            ctx,
            (*prog).start as *mut libc::c_void,
            0 as libc::c_int,
        );
        alloc.expect("non-null function pointer")(ctx, prog as *mut libc::c_void, 0 as libc::c_int);
    }
}
unsafe extern "C" fn default_alloc(
    ctx: *mut libc::c_void,
    p: *mut libc::c_void,
    n: libc::c_int,
) -> *mut libc::c_void {
    if n == 0 as libc::c_int {
        free(p);
        return std::ptr::null_mut::<libc::c_void>();
    }
    realloc(p, n as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn js_regcomp(
    pattern: *const libc::c_char,
    cflags: libc::c_int,
    errorp: *mut *const libc::c_char,
) -> *mut Reprog {
    js_regcompx(
        Some(
            default_alloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> *mut libc::c_void,
        ),
        std::ptr::null_mut::<libc::c_void>(),
        pattern,
        cflags,
        errorp,
    )
}
#[no_mangle]
pub unsafe extern "C" fn js_regfree(prog: *mut Reprog) {
    js_regfreex(
        Some(
            default_alloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> *mut libc::c_void,
        ),
        std::ptr::null_mut::<libc::c_void>(),
        prog,
    );
}
unsafe extern "C" fn isnewline(c: libc::c_int) -> libc::c_int {
    (c == 0xa as libc::c_int
        || c == 0xd as libc::c_int
        || c == 0x2028 as libc::c_int
        || c == 0x2029 as libc::c_int) as libc::c_int
}
unsafe extern "C" fn iswordchar(c: libc::c_int) -> libc::c_int {
    (c == '_' as i32
        || c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= '0' as i32 && c <= '9' as i32) as libc::c_int
}
unsafe extern "C" fn incclass(cc: *mut Reclass, c: Rune) -> libc::c_int {
    let mut p: *mut Rune = std::ptr::null_mut::<Rune>();
    p = ((*cc).spans).as_mut_ptr();
    while p < (*cc).end {
        if *p.offset(0 as libc::c_int as isize) <= c && c <= *p.offset(1 as libc::c_int as isize) {
            return 1 as libc::c_int;
        }
        p = p.offset(2 as libc::c_int as isize);
    }
    0 as libc::c_int
}
unsafe extern "C" fn incclasscanon(cc: *mut Reclass, c: Rune) -> libc::c_int {
    let mut p: *mut Rune = std::ptr::null_mut::<Rune>();
    let mut r: Rune = 0;
    p = ((*cc).spans).as_mut_ptr();
    while p < (*cc).end {
        r = *p.offset(0 as libc::c_int as isize);
        while r <= *p.offset(1 as libc::c_int as isize) {
            if c == canon(r) {
                return 1 as libc::c_int;
            }
            r += 1;
            r;
        }
        p = p.offset(2 as libc::c_int as isize);
    }
    0 as libc::c_int
}
unsafe extern "C" fn strncmpcanon(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut ra: Rune = 0;
    let mut rb: Rune = 0;
    let mut c: libc::c_int = 0;
    loop {
        let fresh147 = n;
        n -= 1;
        if fresh147 == 0 {
            break;
        }
        if *a == 0 {
            return -(1 as libc::c_int);
        }
        if *b == 0 {
            return 1 as libc::c_int;
        }
        a = a.offset(jsU_chartorune(&mut ra, a) as isize);
        b = b.offset(jsU_chartorune(&mut rb, b) as isize);
        c = canon(ra) - canon(rb);
        if c != 0 {
            return c;
        }
    }
    0 as libc::c_int
}
unsafe extern "C" fn match_0(
    mut pc: *mut Reinst,
    mut sp: *const libc::c_char,
    bol: *const libc::c_char,
    flags: libc::c_int,
    out: *mut Resub,
    depth: libc::c_int,
) -> libc::c_int {
    let mut scratch: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    let mut result: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c: Rune = 0;
    if depth > 1024 as libc::c_int {
        return -(1 as libc::c_int);
    }
    loop {
        let current_block_97: u64;
        match (*pc).opcode as libc::c_int {
            0 => return 0 as libc::c_int,
            1 => {
                pc = (*pc).x;
            }
            2 => {
                scratch = *out;
                result = match_0(
                    (*pc).x,
                    sp,
                    bol,
                    flags,
                    &mut scratch,
                    depth + 1 as libc::c_int,
                );
                if result == -(1 as libc::c_int) {
                    return -(1 as libc::c_int);
                }
                if result == 0 as libc::c_int {
                    *out = scratch;
                    return 0 as libc::c_int;
                }
                pc = (*pc).y;
            }
            3 => {
                result = match_0((*pc).x, sp, bol, flags, out, depth + 1 as libc::c_int);
                if result == -(1 as libc::c_int) {
                    return -(1 as libc::c_int);
                }
                if result == 1 as libc::c_int {
                    return 1 as libc::c_int;
                }
                pc = (*pc).y;
            }
            4 => {
                scratch = *out;
                result = match_0(
                    (*pc).x,
                    sp,
                    bol,
                    flags,
                    &mut scratch,
                    depth + 1 as libc::c_int,
                );
                if result == -(1 as libc::c_int) {
                    return -(1 as libc::c_int);
                }
                if result == 0 as libc::c_int {
                    return 1 as libc::c_int;
                }
                pc = (*pc).y;
            }
            5 => {
                if *sp == 0 {
                    return 1 as libc::c_int;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                pc = pc.offset(1 as libc::c_int as isize);
            }
            6 => {
                if *sp == 0 {
                    return 1 as libc::c_int;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if isnewline(c) != 0 {
                    return 1 as libc::c_int;
                }
                pc = pc.offset(1 as libc::c_int as isize);
            }
            7 => {
                if *sp == 0 {
                    return 1 as libc::c_int;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if flags & REG_ICASE as libc::c_int != 0 {
                    c = canon(c);
                }
                if c != (*pc).c {
                    return 1 as libc::c_int;
                }
                pc = pc.offset(1 as libc::c_int as isize);
            }
            8 => {
                if *sp == 0 {
                    return 1 as libc::c_int;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if flags & REG_ICASE as libc::c_int != 0 {
                    if incclasscanon((*pc).cc, canon(c)) == 0 {
                        return 1 as libc::c_int;
                    }
                } else if incclass((*pc).cc, c) == 0 {
                    return 1 as libc::c_int;
                }
                pc = pc.offset(1 as libc::c_int as isize);
            }
            9 => {
                if *sp == 0 {
                    return 1 as libc::c_int;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if flags & REG_ICASE as libc::c_int != 0 {
                    if incclasscanon((*pc).cc, canon(c)) != 0 {
                        return 1 as libc::c_int;
                    }
                } else if incclass((*pc).cc, c) != 0 {
                    return 1 as libc::c_int;
                }
                pc = pc.offset(1 as libc::c_int as isize);
            }
            10 => {
                i = ((*out).sub[(*pc).n as usize].ep).offset_from((*out).sub[(*pc).n as usize].sp)
                    as libc::c_long as libc::c_int;
                if flags & REG_ICASE as libc::c_int != 0 {
                    if strncmpcanon(sp, (*out).sub[(*pc).n as usize].sp, i) != 0 {
                        return 1 as libc::c_int;
                    }
                } else if strncmp(sp, (*out).sub[(*pc).n as usize].sp, i as libc::c_ulong) != 0 {
                    return 1 as libc::c_int;
                }
                if i > 0 as libc::c_int {
                    sp = sp.offset(i as isize);
                }
                pc = pc.offset(1 as libc::c_int as isize);
            }
            11 => {
                if sp == bol && flags & REG_NOTBOL as libc::c_int == 0 {
                    pc = pc.offset(1 as libc::c_int as isize);
                } else {
                    if flags & REG_NEWLINE as libc::c_int != 0 {
                        if sp > bol
                            && isnewline(*sp.offset(-(1 as libc::c_int) as isize) as libc::c_int)
                                != 0
                        {
                            pc = pc.offset(1 as libc::c_int as isize);
                            current_block_97 = 6471821049853688503;
                        } else {
                            current_block_97 = 15462640364611497761;
                        }
                    } else {
                        current_block_97 = 15462640364611497761;
                    }
                    match current_block_97 {
                        6471821049853688503 => {}
                        _ => return 1 as libc::c_int,
                    }
                }
            }
            12 => {
                if *sp as libc::c_int == 0 as libc::c_int {
                    pc = pc.offset(1 as libc::c_int as isize);
                } else {
                    if flags & REG_NEWLINE as libc::c_int != 0 {
                        if isnewline(*sp as libc::c_int) != 0 {
                            pc = pc.offset(1 as libc::c_int as isize);
                            current_block_97 = 6471821049853688503;
                        } else {
                            current_block_97 = 5793491756164225964;
                        }
                    } else {
                        current_block_97 = 5793491756164225964;
                    }
                    match current_block_97 {
                        6471821049853688503 => {}
                        _ => return 1 as libc::c_int,
                    }
                }
            }
            13 => {
                i = (sp > bol
                    && iswordchar(*sp.offset(-(1 as libc::c_int) as isize) as libc::c_int) != 0)
                    as libc::c_int;
                i ^= iswordchar(*sp.offset(0 as libc::c_int as isize) as libc::c_int);
                if i == 0 {
                    return 1 as libc::c_int;
                }
                pc = pc.offset(1 as libc::c_int as isize);
            }
            14 => {
                i = (sp > bol
                    && iswordchar(*sp.offset(-(1 as libc::c_int) as isize) as libc::c_int) != 0)
                    as libc::c_int;
                i ^= iswordchar(*sp.offset(0 as libc::c_int as isize) as libc::c_int);
                if i != 0 {
                    return 1 as libc::c_int;
                }
                pc = pc.offset(1 as libc::c_int as isize);
            }
            15 => {
                (*out).sub[(*pc).n as usize].sp = sp;
                pc = pc.offset(1 as libc::c_int as isize);
            }
            16 => {
                (*out).sub[(*pc).n as usize].ep = sp;
                pc = pc.offset(1 as libc::c_int as isize);
            }
            _ => return 1 as libc::c_int,
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_regexec(
    prog: *mut Reprog,
    sp: *const libc::c_char,
    mut sub: *mut Resub,
    eflags: libc::c_int,
) -> libc::c_int {
    let mut scratch: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: std::ptr::null::<libc::c_char>(),
            ep: std::ptr::null::<libc::c_char>(),
        }; 16],
    };
    let mut i: libc::c_int = 0;
    if sub.is_null() {
        sub = &mut scratch;
    }
    (*sub).nsub = (*prog).nsub;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        (*sub).sub[i as usize].ep = std::ptr::null::<libc::c_char>();
        (*sub).sub[i as usize].sp = (*sub).sub[i as usize].ep;
        i += 1;
        i;
    }
    match_0(
        (*prog).start,
        sp,
        sp,
        (*prog).flags | eflags,
        sub,
        0 as libc::c_int,
    )
}
static mut ucd_alpha2: [Rune; 1046] = [
    0x41 as libc::c_int,
    0x5a as libc::c_int,
    0x61 as libc::c_int,
    0x7a as libc::c_int,
    0xc0 as libc::c_int,
    0xd6 as libc::c_int,
    0xd8 as libc::c_int,
    0xf6 as libc::c_int,
    0xf8 as libc::c_int,
    0x2c1 as libc::c_int,
    0x2c6 as libc::c_int,
    0x2d1 as libc::c_int,
    0x2e0 as libc::c_int,
    0x2e4 as libc::c_int,
    0x370 as libc::c_int,
    0x374 as libc::c_int,
    0x376 as libc::c_int,
    0x377 as libc::c_int,
    0x37a as libc::c_int,
    0x37d as libc::c_int,
    0x388 as libc::c_int,
    0x38a as libc::c_int,
    0x38e as libc::c_int,
    0x3a1 as libc::c_int,
    0x3a3 as libc::c_int,
    0x3f5 as libc::c_int,
    0x3f7 as libc::c_int,
    0x481 as libc::c_int,
    0x48a as libc::c_int,
    0x52f as libc::c_int,
    0x531 as libc::c_int,
    0x556 as libc::c_int,
    0x560 as libc::c_int,
    0x588 as libc::c_int,
    0x5d0 as libc::c_int,
    0x5ea as libc::c_int,
    0x5ef as libc::c_int,
    0x5f2 as libc::c_int,
    0x620 as libc::c_int,
    0x64a as libc::c_int,
    0x66e as libc::c_int,
    0x66f as libc::c_int,
    0x671 as libc::c_int,
    0x6d3 as libc::c_int,
    0x6e5 as libc::c_int,
    0x6e6 as libc::c_int,
    0x6ee as libc::c_int,
    0x6ef as libc::c_int,
    0x6fa as libc::c_int,
    0x6fc as libc::c_int,
    0x712 as libc::c_int,
    0x72f as libc::c_int,
    0x74d as libc::c_int,
    0x7a5 as libc::c_int,
    0x7ca as libc::c_int,
    0x7ea as libc::c_int,
    0x7f4 as libc::c_int,
    0x7f5 as libc::c_int,
    0x800 as libc::c_int,
    0x815 as libc::c_int,
    0x840 as libc::c_int,
    0x858 as libc::c_int,
    0x860 as libc::c_int,
    0x86a as libc::c_int,
    0x870 as libc::c_int,
    0x887 as libc::c_int,
    0x889 as libc::c_int,
    0x88e as libc::c_int,
    0x8a0 as libc::c_int,
    0x8c9 as libc::c_int,
    0x904 as libc::c_int,
    0x939 as libc::c_int,
    0x958 as libc::c_int,
    0x961 as libc::c_int,
    0x971 as libc::c_int,
    0x980 as libc::c_int,
    0x985 as libc::c_int,
    0x98c as libc::c_int,
    0x98f as libc::c_int,
    0x990 as libc::c_int,
    0x993 as libc::c_int,
    0x9a8 as libc::c_int,
    0x9aa as libc::c_int,
    0x9b0 as libc::c_int,
    0x9b6 as libc::c_int,
    0x9b9 as libc::c_int,
    0x9dc as libc::c_int,
    0x9dd as libc::c_int,
    0x9df as libc::c_int,
    0x9e1 as libc::c_int,
    0x9f0 as libc::c_int,
    0x9f1 as libc::c_int,
    0xa05 as libc::c_int,
    0xa0a as libc::c_int,
    0xa0f as libc::c_int,
    0xa10 as libc::c_int,
    0xa13 as libc::c_int,
    0xa28 as libc::c_int,
    0xa2a as libc::c_int,
    0xa30 as libc::c_int,
    0xa32 as libc::c_int,
    0xa33 as libc::c_int,
    0xa35 as libc::c_int,
    0xa36 as libc::c_int,
    0xa38 as libc::c_int,
    0xa39 as libc::c_int,
    0xa59 as libc::c_int,
    0xa5c as libc::c_int,
    0xa72 as libc::c_int,
    0xa74 as libc::c_int,
    0xa85 as libc::c_int,
    0xa8d as libc::c_int,
    0xa8f as libc::c_int,
    0xa91 as libc::c_int,
    0xa93 as libc::c_int,
    0xaa8 as libc::c_int,
    0xaaa as libc::c_int,
    0xab0 as libc::c_int,
    0xab2 as libc::c_int,
    0xab3 as libc::c_int,
    0xab5 as libc::c_int,
    0xab9 as libc::c_int,
    0xae0 as libc::c_int,
    0xae1 as libc::c_int,
    0xb05 as libc::c_int,
    0xb0c as libc::c_int,
    0xb0f as libc::c_int,
    0xb10 as libc::c_int,
    0xb13 as libc::c_int,
    0xb28 as libc::c_int,
    0xb2a as libc::c_int,
    0xb30 as libc::c_int,
    0xb32 as libc::c_int,
    0xb33 as libc::c_int,
    0xb35 as libc::c_int,
    0xb39 as libc::c_int,
    0xb5c as libc::c_int,
    0xb5d as libc::c_int,
    0xb5f as libc::c_int,
    0xb61 as libc::c_int,
    0xb85 as libc::c_int,
    0xb8a as libc::c_int,
    0xb8e as libc::c_int,
    0xb90 as libc::c_int,
    0xb92 as libc::c_int,
    0xb95 as libc::c_int,
    0xb99 as libc::c_int,
    0xb9a as libc::c_int,
    0xb9e as libc::c_int,
    0xb9f as libc::c_int,
    0xba3 as libc::c_int,
    0xba4 as libc::c_int,
    0xba8 as libc::c_int,
    0xbaa as libc::c_int,
    0xbae as libc::c_int,
    0xbb9 as libc::c_int,
    0xc05 as libc::c_int,
    0xc0c as libc::c_int,
    0xc0e as libc::c_int,
    0xc10 as libc::c_int,
    0xc12 as libc::c_int,
    0xc28 as libc::c_int,
    0xc2a as libc::c_int,
    0xc39 as libc::c_int,
    0xc58 as libc::c_int,
    0xc5a as libc::c_int,
    0xc60 as libc::c_int,
    0xc61 as libc::c_int,
    0xc85 as libc::c_int,
    0xc8c as libc::c_int,
    0xc8e as libc::c_int,
    0xc90 as libc::c_int,
    0xc92 as libc::c_int,
    0xca8 as libc::c_int,
    0xcaa as libc::c_int,
    0xcb3 as libc::c_int,
    0xcb5 as libc::c_int,
    0xcb9 as libc::c_int,
    0xcdd as libc::c_int,
    0xcde as libc::c_int,
    0xce0 as libc::c_int,
    0xce1 as libc::c_int,
    0xcf1 as libc::c_int,
    0xcf2 as libc::c_int,
    0xd04 as libc::c_int,
    0xd0c as libc::c_int,
    0xd0e as libc::c_int,
    0xd10 as libc::c_int,
    0xd12 as libc::c_int,
    0xd3a as libc::c_int,
    0xd54 as libc::c_int,
    0xd56 as libc::c_int,
    0xd5f as libc::c_int,
    0xd61 as libc::c_int,
    0xd7a as libc::c_int,
    0xd7f as libc::c_int,
    0xd85 as libc::c_int,
    0xd96 as libc::c_int,
    0xd9a as libc::c_int,
    0xdb1 as libc::c_int,
    0xdb3 as libc::c_int,
    0xdbb as libc::c_int,
    0xdc0 as libc::c_int,
    0xdc6 as libc::c_int,
    0xe01 as libc::c_int,
    0xe30 as libc::c_int,
    0xe32 as libc::c_int,
    0xe33 as libc::c_int,
    0xe40 as libc::c_int,
    0xe46 as libc::c_int,
    0xe81 as libc::c_int,
    0xe82 as libc::c_int,
    0xe86 as libc::c_int,
    0xe8a as libc::c_int,
    0xe8c as libc::c_int,
    0xea3 as libc::c_int,
    0xea7 as libc::c_int,
    0xeb0 as libc::c_int,
    0xeb2 as libc::c_int,
    0xeb3 as libc::c_int,
    0xec0 as libc::c_int,
    0xec4 as libc::c_int,
    0xedc as libc::c_int,
    0xedf as libc::c_int,
    0xf40 as libc::c_int,
    0xf47 as libc::c_int,
    0xf49 as libc::c_int,
    0xf6c as libc::c_int,
    0xf88 as libc::c_int,
    0xf8c as libc::c_int,
    0x1000 as libc::c_int,
    0x102a as libc::c_int,
    0x1050 as libc::c_int,
    0x1055 as libc::c_int,
    0x105a as libc::c_int,
    0x105d as libc::c_int,
    0x1065 as libc::c_int,
    0x1066 as libc::c_int,
    0x106e as libc::c_int,
    0x1070 as libc::c_int,
    0x1075 as libc::c_int,
    0x1081 as libc::c_int,
    0x10a0 as libc::c_int,
    0x10c5 as libc::c_int,
    0x10d0 as libc::c_int,
    0x10fa as libc::c_int,
    0x10fc as libc::c_int,
    0x1248 as libc::c_int,
    0x124a as libc::c_int,
    0x124d as libc::c_int,
    0x1250 as libc::c_int,
    0x1256 as libc::c_int,
    0x125a as libc::c_int,
    0x125d as libc::c_int,
    0x1260 as libc::c_int,
    0x1288 as libc::c_int,
    0x128a as libc::c_int,
    0x128d as libc::c_int,
    0x1290 as libc::c_int,
    0x12b0 as libc::c_int,
    0x12b2 as libc::c_int,
    0x12b5 as libc::c_int,
    0x12b8 as libc::c_int,
    0x12be as libc::c_int,
    0x12c2 as libc::c_int,
    0x12c5 as libc::c_int,
    0x12c8 as libc::c_int,
    0x12d6 as libc::c_int,
    0x12d8 as libc::c_int,
    0x1310 as libc::c_int,
    0x1312 as libc::c_int,
    0x1315 as libc::c_int,
    0x1318 as libc::c_int,
    0x135a as libc::c_int,
    0x1380 as libc::c_int,
    0x138f as libc::c_int,
    0x13a0 as libc::c_int,
    0x13f5 as libc::c_int,
    0x13f8 as libc::c_int,
    0x13fd as libc::c_int,
    0x1401 as libc::c_int,
    0x166c as libc::c_int,
    0x166f as libc::c_int,
    0x167f as libc::c_int,
    0x1681 as libc::c_int,
    0x169a as libc::c_int,
    0x16a0 as libc::c_int,
    0x16ea as libc::c_int,
    0x16f1 as libc::c_int,
    0x16f8 as libc::c_int,
    0x1700 as libc::c_int,
    0x1711 as libc::c_int,
    0x171f as libc::c_int,
    0x1731 as libc::c_int,
    0x1740 as libc::c_int,
    0x1751 as libc::c_int,
    0x1760 as libc::c_int,
    0x176c as libc::c_int,
    0x176e as libc::c_int,
    0x1770 as libc::c_int,
    0x1780 as libc::c_int,
    0x17b3 as libc::c_int,
    0x1820 as libc::c_int,
    0x1878 as libc::c_int,
    0x1880 as libc::c_int,
    0x1884 as libc::c_int,
    0x1887 as libc::c_int,
    0x18a8 as libc::c_int,
    0x18b0 as libc::c_int,
    0x18f5 as libc::c_int,
    0x1900 as libc::c_int,
    0x191e as libc::c_int,
    0x1950 as libc::c_int,
    0x196d as libc::c_int,
    0x1970 as libc::c_int,
    0x1974 as libc::c_int,
    0x1980 as libc::c_int,
    0x19ab as libc::c_int,
    0x19b0 as libc::c_int,
    0x19c9 as libc::c_int,
    0x1a00 as libc::c_int,
    0x1a16 as libc::c_int,
    0x1a20 as libc::c_int,
    0x1a54 as libc::c_int,
    0x1b05 as libc::c_int,
    0x1b33 as libc::c_int,
    0x1b45 as libc::c_int,
    0x1b4c as libc::c_int,
    0x1b83 as libc::c_int,
    0x1ba0 as libc::c_int,
    0x1bae as libc::c_int,
    0x1baf as libc::c_int,
    0x1bba as libc::c_int,
    0x1be5 as libc::c_int,
    0x1c00 as libc::c_int,
    0x1c23 as libc::c_int,
    0x1c4d as libc::c_int,
    0x1c4f as libc::c_int,
    0x1c5a as libc::c_int,
    0x1c7d as libc::c_int,
    0x1c80 as libc::c_int,
    0x1c8a as libc::c_int,
    0x1c90 as libc::c_int,
    0x1cba as libc::c_int,
    0x1cbd as libc::c_int,
    0x1cbf as libc::c_int,
    0x1ce9 as libc::c_int,
    0x1cec as libc::c_int,
    0x1cee as libc::c_int,
    0x1cf3 as libc::c_int,
    0x1cf5 as libc::c_int,
    0x1cf6 as libc::c_int,
    0x1d00 as libc::c_int,
    0x1dbf as libc::c_int,
    0x1e00 as libc::c_int,
    0x1f15 as libc::c_int,
    0x1f18 as libc::c_int,
    0x1f1d as libc::c_int,
    0x1f20 as libc::c_int,
    0x1f45 as libc::c_int,
    0x1f48 as libc::c_int,
    0x1f4d as libc::c_int,
    0x1f50 as libc::c_int,
    0x1f57 as libc::c_int,
    0x1f5f as libc::c_int,
    0x1f7d as libc::c_int,
    0x1f80 as libc::c_int,
    0x1fb4 as libc::c_int,
    0x1fb6 as libc::c_int,
    0x1fbc as libc::c_int,
    0x1fc2 as libc::c_int,
    0x1fc4 as libc::c_int,
    0x1fc6 as libc::c_int,
    0x1fcc as libc::c_int,
    0x1fd0 as libc::c_int,
    0x1fd3 as libc::c_int,
    0x1fd6 as libc::c_int,
    0x1fdb as libc::c_int,
    0x1fe0 as libc::c_int,
    0x1fec as libc::c_int,
    0x1ff2 as libc::c_int,
    0x1ff4 as libc::c_int,
    0x1ff6 as libc::c_int,
    0x1ffc as libc::c_int,
    0x2090 as libc::c_int,
    0x209c as libc::c_int,
    0x210a as libc::c_int,
    0x2113 as libc::c_int,
    0x2119 as libc::c_int,
    0x211d as libc::c_int,
    0x212a as libc::c_int,
    0x212d as libc::c_int,
    0x212f as libc::c_int,
    0x2139 as libc::c_int,
    0x213c as libc::c_int,
    0x213f as libc::c_int,
    0x2145 as libc::c_int,
    0x2149 as libc::c_int,
    0x2183 as libc::c_int,
    0x2184 as libc::c_int,
    0x2c00 as libc::c_int,
    0x2ce4 as libc::c_int,
    0x2ceb as libc::c_int,
    0x2cee as libc::c_int,
    0x2cf2 as libc::c_int,
    0x2cf3 as libc::c_int,
    0x2d00 as libc::c_int,
    0x2d25 as libc::c_int,
    0x2d30 as libc::c_int,
    0x2d67 as libc::c_int,
    0x2d80 as libc::c_int,
    0x2d96 as libc::c_int,
    0x2da0 as libc::c_int,
    0x2da6 as libc::c_int,
    0x2da8 as libc::c_int,
    0x2dae as libc::c_int,
    0x2db0 as libc::c_int,
    0x2db6 as libc::c_int,
    0x2db8 as libc::c_int,
    0x2dbe as libc::c_int,
    0x2dc0 as libc::c_int,
    0x2dc6 as libc::c_int,
    0x2dc8 as libc::c_int,
    0x2dce as libc::c_int,
    0x2dd0 as libc::c_int,
    0x2dd6 as libc::c_int,
    0x2dd8 as libc::c_int,
    0x2dde as libc::c_int,
    0x3005 as libc::c_int,
    0x3006 as libc::c_int,
    0x3031 as libc::c_int,
    0x3035 as libc::c_int,
    0x303b as libc::c_int,
    0x303c as libc::c_int,
    0x3041 as libc::c_int,
    0x3096 as libc::c_int,
    0x309d as libc::c_int,
    0x309f as libc::c_int,
    0x30a1 as libc::c_int,
    0x30fa as libc::c_int,
    0x30fc as libc::c_int,
    0x30ff as libc::c_int,
    0x3105 as libc::c_int,
    0x312f as libc::c_int,
    0x3131 as libc::c_int,
    0x318e as libc::c_int,
    0x31a0 as libc::c_int,
    0x31bf as libc::c_int,
    0x31f0 as libc::c_int,
    0x31ff as libc::c_int,
    0x9fff as libc::c_int,
    0xa48c as libc::c_int,
    0xa4d0 as libc::c_int,
    0xa4fd as libc::c_int,
    0xa500 as libc::c_int,
    0xa60c as libc::c_int,
    0xa610 as libc::c_int,
    0xa61f as libc::c_int,
    0xa62a as libc::c_int,
    0xa62b as libc::c_int,
    0xa640 as libc::c_int,
    0xa66e as libc::c_int,
    0xa67f as libc::c_int,
    0xa69d as libc::c_int,
    0xa6a0 as libc::c_int,
    0xa6e5 as libc::c_int,
    0xa717 as libc::c_int,
    0xa71f as libc::c_int,
    0xa722 as libc::c_int,
    0xa788 as libc::c_int,
    0xa78b as libc::c_int,
    0xa7cd as libc::c_int,
    0xa7d0 as libc::c_int,
    0xa7d1 as libc::c_int,
    0xa7d5 as libc::c_int,
    0xa7dc as libc::c_int,
    0xa7f2 as libc::c_int,
    0xa801 as libc::c_int,
    0xa803 as libc::c_int,
    0xa805 as libc::c_int,
    0xa807 as libc::c_int,
    0xa80a as libc::c_int,
    0xa80c as libc::c_int,
    0xa822 as libc::c_int,
    0xa840 as libc::c_int,
    0xa873 as libc::c_int,
    0xa882 as libc::c_int,
    0xa8b3 as libc::c_int,
    0xa8f2 as libc::c_int,
    0xa8f7 as libc::c_int,
    0xa8fd as libc::c_int,
    0xa8fe as libc::c_int,
    0xa90a as libc::c_int,
    0xa925 as libc::c_int,
    0xa930 as libc::c_int,
    0xa946 as libc::c_int,
    0xa960 as libc::c_int,
    0xa97c as libc::c_int,
    0xa984 as libc::c_int,
    0xa9b2 as libc::c_int,
    0xa9e0 as libc::c_int,
    0xa9e4 as libc::c_int,
    0xa9e6 as libc::c_int,
    0xa9ef as libc::c_int,
    0xa9fa as libc::c_int,
    0xa9fe as libc::c_int,
    0xaa00 as libc::c_int,
    0xaa28 as libc::c_int,
    0xaa40 as libc::c_int,
    0xaa42 as libc::c_int,
    0xaa44 as libc::c_int,
    0xaa4b as libc::c_int,
    0xaa60 as libc::c_int,
    0xaa76 as libc::c_int,
    0xaa7e as libc::c_int,
    0xaaaf as libc::c_int,
    0xaab5 as libc::c_int,
    0xaab6 as libc::c_int,
    0xaab9 as libc::c_int,
    0xaabd as libc::c_int,
    0xaadb as libc::c_int,
    0xaadd as libc::c_int,
    0xaae0 as libc::c_int,
    0xaaea as libc::c_int,
    0xaaf2 as libc::c_int,
    0xaaf4 as libc::c_int,
    0xab01 as libc::c_int,
    0xab06 as libc::c_int,
    0xab09 as libc::c_int,
    0xab0e as libc::c_int,
    0xab11 as libc::c_int,
    0xab16 as libc::c_int,
    0xab20 as libc::c_int,
    0xab26 as libc::c_int,
    0xab28 as libc::c_int,
    0xab2e as libc::c_int,
    0xab30 as libc::c_int,
    0xab5a as libc::c_int,
    0xab5c as libc::c_int,
    0xab69 as libc::c_int,
    0xab70 as libc::c_int,
    0xabe2 as libc::c_int,
    0xd7b0 as libc::c_int,
    0xd7c6 as libc::c_int,
    0xd7cb as libc::c_int,
    0xd7fb as libc::c_int,
    0xf900 as libc::c_int,
    0xfa6d as libc::c_int,
    0xfa70 as libc::c_int,
    0xfad9 as libc::c_int,
    0xfb00 as libc::c_int,
    0xfb06 as libc::c_int,
    0xfb13 as libc::c_int,
    0xfb17 as libc::c_int,
    0xfb1f as libc::c_int,
    0xfb28 as libc::c_int,
    0xfb2a as libc::c_int,
    0xfb36 as libc::c_int,
    0xfb38 as libc::c_int,
    0xfb3c as libc::c_int,
    0xfb40 as libc::c_int,
    0xfb41 as libc::c_int,
    0xfb43 as libc::c_int,
    0xfb44 as libc::c_int,
    0xfb46 as libc::c_int,
    0xfbb1 as libc::c_int,
    0xfbd3 as libc::c_int,
    0xfd3d as libc::c_int,
    0xfd50 as libc::c_int,
    0xfd8f as libc::c_int,
    0xfd92 as libc::c_int,
    0xfdc7 as libc::c_int,
    0xfdf0 as libc::c_int,
    0xfdfb as libc::c_int,
    0xfe70 as libc::c_int,
    0xfe74 as libc::c_int,
    0xfe76 as libc::c_int,
    0xfefc as libc::c_int,
    0xff21 as libc::c_int,
    0xff3a as libc::c_int,
    0xff41 as libc::c_int,
    0xff5a as libc::c_int,
    0xff66 as libc::c_int,
    0xffbe as libc::c_int,
    0xffc2 as libc::c_int,
    0xffc7 as libc::c_int,
    0xffca as libc::c_int,
    0xffcf as libc::c_int,
    0xffd2 as libc::c_int,
    0xffd7 as libc::c_int,
    0xffda as libc::c_int,
    0xffdc as libc::c_int,
    0x10000 as libc::c_int,
    0x1000b as libc::c_int,
    0x1000d as libc::c_int,
    0x10026 as libc::c_int,
    0x10028 as libc::c_int,
    0x1003a as libc::c_int,
    0x1003c as libc::c_int,
    0x1003d as libc::c_int,
    0x1003f as libc::c_int,
    0x1004d as libc::c_int,
    0x10050 as libc::c_int,
    0x1005d as libc::c_int,
    0x10080 as libc::c_int,
    0x100fa as libc::c_int,
    0x10280 as libc::c_int,
    0x1029c as libc::c_int,
    0x102a0 as libc::c_int,
    0x102d0 as libc::c_int,
    0x10300 as libc::c_int,
    0x1031f as libc::c_int,
    0x1032d as libc::c_int,
    0x10340 as libc::c_int,
    0x10342 as libc::c_int,
    0x10349 as libc::c_int,
    0x10350 as libc::c_int,
    0x10375 as libc::c_int,
    0x10380 as libc::c_int,
    0x1039d as libc::c_int,
    0x103a0 as libc::c_int,
    0x103c3 as libc::c_int,
    0x103c8 as libc::c_int,
    0x103cf as libc::c_int,
    0x10400 as libc::c_int,
    0x1049d as libc::c_int,
    0x104b0 as libc::c_int,
    0x104d3 as libc::c_int,
    0x104d8 as libc::c_int,
    0x104fb as libc::c_int,
    0x10500 as libc::c_int,
    0x10527 as libc::c_int,
    0x10530 as libc::c_int,
    0x10563 as libc::c_int,
    0x10570 as libc::c_int,
    0x1057a as libc::c_int,
    0x1057c as libc::c_int,
    0x1058a as libc::c_int,
    0x1058c as libc::c_int,
    0x10592 as libc::c_int,
    0x10594 as libc::c_int,
    0x10595 as libc::c_int,
    0x10597 as libc::c_int,
    0x105a1 as libc::c_int,
    0x105a3 as libc::c_int,
    0x105b1 as libc::c_int,
    0x105b3 as libc::c_int,
    0x105b9 as libc::c_int,
    0x105bb as libc::c_int,
    0x105bc as libc::c_int,
    0x105c0 as libc::c_int,
    0x105f3 as libc::c_int,
    0x10600 as libc::c_int,
    0x10736 as libc::c_int,
    0x10740 as libc::c_int,
    0x10755 as libc::c_int,
    0x10760 as libc::c_int,
    0x10767 as libc::c_int,
    0x10780 as libc::c_int,
    0x10785 as libc::c_int,
    0x10787 as libc::c_int,
    0x107b0 as libc::c_int,
    0x107b2 as libc::c_int,
    0x107ba as libc::c_int,
    0x10800 as libc::c_int,
    0x10805 as libc::c_int,
    0x1080a as libc::c_int,
    0x10835 as libc::c_int,
    0x10837 as libc::c_int,
    0x10838 as libc::c_int,
    0x1083f as libc::c_int,
    0x10855 as libc::c_int,
    0x10860 as libc::c_int,
    0x10876 as libc::c_int,
    0x10880 as libc::c_int,
    0x1089e as libc::c_int,
    0x108e0 as libc::c_int,
    0x108f2 as libc::c_int,
    0x108f4 as libc::c_int,
    0x108f5 as libc::c_int,
    0x10900 as libc::c_int,
    0x10915 as libc::c_int,
    0x10920 as libc::c_int,
    0x10939 as libc::c_int,
    0x10980 as libc::c_int,
    0x109b7 as libc::c_int,
    0x109be as libc::c_int,
    0x109bf as libc::c_int,
    0x10a10 as libc::c_int,
    0x10a13 as libc::c_int,
    0x10a15 as libc::c_int,
    0x10a17 as libc::c_int,
    0x10a19 as libc::c_int,
    0x10a35 as libc::c_int,
    0x10a60 as libc::c_int,
    0x10a7c as libc::c_int,
    0x10a80 as libc::c_int,
    0x10a9c as libc::c_int,
    0x10ac0 as libc::c_int,
    0x10ac7 as libc::c_int,
    0x10ac9 as libc::c_int,
    0x10ae4 as libc::c_int,
    0x10b00 as libc::c_int,
    0x10b35 as libc::c_int,
    0x10b40 as libc::c_int,
    0x10b55 as libc::c_int,
    0x10b60 as libc::c_int,
    0x10b72 as libc::c_int,
    0x10b80 as libc::c_int,
    0x10b91 as libc::c_int,
    0x10c00 as libc::c_int,
    0x10c48 as libc::c_int,
    0x10c80 as libc::c_int,
    0x10cb2 as libc::c_int,
    0x10cc0 as libc::c_int,
    0x10cf2 as libc::c_int,
    0x10d00 as libc::c_int,
    0x10d23 as libc::c_int,
    0x10d4a as libc::c_int,
    0x10d65 as libc::c_int,
    0x10d6f as libc::c_int,
    0x10d85 as libc::c_int,
    0x10e80 as libc::c_int,
    0x10ea9 as libc::c_int,
    0x10eb0 as libc::c_int,
    0x10eb1 as libc::c_int,
    0x10ec2 as libc::c_int,
    0x10ec4 as libc::c_int,
    0x10f00 as libc::c_int,
    0x10f1c as libc::c_int,
    0x10f30 as libc::c_int,
    0x10f45 as libc::c_int,
    0x10f70 as libc::c_int,
    0x10f81 as libc::c_int,
    0x10fb0 as libc::c_int,
    0x10fc4 as libc::c_int,
    0x10fe0 as libc::c_int,
    0x10ff6 as libc::c_int,
    0x11003 as libc::c_int,
    0x11037 as libc::c_int,
    0x11071 as libc::c_int,
    0x11072 as libc::c_int,
    0x11083 as libc::c_int,
    0x110af as libc::c_int,
    0x110d0 as libc::c_int,
    0x110e8 as libc::c_int,
    0x11103 as libc::c_int,
    0x11126 as libc::c_int,
    0x11150 as libc::c_int,
    0x11172 as libc::c_int,
    0x11183 as libc::c_int,
    0x111b2 as libc::c_int,
    0x111c1 as libc::c_int,
    0x111c4 as libc::c_int,
    0x11200 as libc::c_int,
    0x11211 as libc::c_int,
    0x11213 as libc::c_int,
    0x1122b as libc::c_int,
    0x1123f as libc::c_int,
    0x11240 as libc::c_int,
    0x11280 as libc::c_int,
    0x11286 as libc::c_int,
    0x1128a as libc::c_int,
    0x1128d as libc::c_int,
    0x1128f as libc::c_int,
    0x1129d as libc::c_int,
    0x1129f as libc::c_int,
    0x112a8 as libc::c_int,
    0x112b0 as libc::c_int,
    0x112de as libc::c_int,
    0x11305 as libc::c_int,
    0x1130c as libc::c_int,
    0x1130f as libc::c_int,
    0x11310 as libc::c_int,
    0x11313 as libc::c_int,
    0x11328 as libc::c_int,
    0x1132a as libc::c_int,
    0x11330 as libc::c_int,
    0x11332 as libc::c_int,
    0x11333 as libc::c_int,
    0x11335 as libc::c_int,
    0x11339 as libc::c_int,
    0x1135d as libc::c_int,
    0x11361 as libc::c_int,
    0x11380 as libc::c_int,
    0x11389 as libc::c_int,
    0x11390 as libc::c_int,
    0x113b5 as libc::c_int,
    0x11400 as libc::c_int,
    0x11434 as libc::c_int,
    0x11447 as libc::c_int,
    0x1144a as libc::c_int,
    0x1145f as libc::c_int,
    0x11461 as libc::c_int,
    0x11480 as libc::c_int,
    0x114af as libc::c_int,
    0x114c4 as libc::c_int,
    0x114c5 as libc::c_int,
    0x11580 as libc::c_int,
    0x115ae as libc::c_int,
    0x115d8 as libc::c_int,
    0x115db as libc::c_int,
    0x11600 as libc::c_int,
    0x1162f as libc::c_int,
    0x11680 as libc::c_int,
    0x116aa as libc::c_int,
    0x11700 as libc::c_int,
    0x1171a as libc::c_int,
    0x11740 as libc::c_int,
    0x11746 as libc::c_int,
    0x11800 as libc::c_int,
    0x1182b as libc::c_int,
    0x118a0 as libc::c_int,
    0x118df as libc::c_int,
    0x118ff as libc::c_int,
    0x11906 as libc::c_int,
    0x1190c as libc::c_int,
    0x11913 as libc::c_int,
    0x11915 as libc::c_int,
    0x11916 as libc::c_int,
    0x11918 as libc::c_int,
    0x1192f as libc::c_int,
    0x119a0 as libc::c_int,
    0x119a7 as libc::c_int,
    0x119aa as libc::c_int,
    0x119d0 as libc::c_int,
    0x11a0b as libc::c_int,
    0x11a32 as libc::c_int,
    0x11a5c as libc::c_int,
    0x11a89 as libc::c_int,
    0x11ab0 as libc::c_int,
    0x11af8 as libc::c_int,
    0x11bc0 as libc::c_int,
    0x11be0 as libc::c_int,
    0x11c00 as libc::c_int,
    0x11c08 as libc::c_int,
    0x11c0a as libc::c_int,
    0x11c2e as libc::c_int,
    0x11c72 as libc::c_int,
    0x11c8f as libc::c_int,
    0x11d00 as libc::c_int,
    0x11d06 as libc::c_int,
    0x11d08 as libc::c_int,
    0x11d09 as libc::c_int,
    0x11d0b as libc::c_int,
    0x11d30 as libc::c_int,
    0x11d60 as libc::c_int,
    0x11d65 as libc::c_int,
    0x11d67 as libc::c_int,
    0x11d68 as libc::c_int,
    0x11d6a as libc::c_int,
    0x11d89 as libc::c_int,
    0x11ee0 as libc::c_int,
    0x11ef2 as libc::c_int,
    0x11f04 as libc::c_int,
    0x11f10 as libc::c_int,
    0x11f12 as libc::c_int,
    0x11f33 as libc::c_int,
    0x12000 as libc::c_int,
    0x12399 as libc::c_int,
    0x12480 as libc::c_int,
    0x12543 as libc::c_int,
    0x12f90 as libc::c_int,
    0x12ff0 as libc::c_int,
    0x13000 as libc::c_int,
    0x1342f as libc::c_int,
    0x13441 as libc::c_int,
    0x13446 as libc::c_int,
    0x13460 as libc::c_int,
    0x143fa as libc::c_int,
    0x14400 as libc::c_int,
    0x14646 as libc::c_int,
    0x16100 as libc::c_int,
    0x1611d as libc::c_int,
    0x16800 as libc::c_int,
    0x16a38 as libc::c_int,
    0x16a40 as libc::c_int,
    0x16a5e as libc::c_int,
    0x16a70 as libc::c_int,
    0x16abe as libc::c_int,
    0x16ad0 as libc::c_int,
    0x16aed as libc::c_int,
    0x16b00 as libc::c_int,
    0x16b2f as libc::c_int,
    0x16b40 as libc::c_int,
    0x16b43 as libc::c_int,
    0x16b63 as libc::c_int,
    0x16b77 as libc::c_int,
    0x16b7d as libc::c_int,
    0x16b8f as libc::c_int,
    0x16d40 as libc::c_int,
    0x16d6c as libc::c_int,
    0x16e40 as libc::c_int,
    0x16e7f as libc::c_int,
    0x16f00 as libc::c_int,
    0x16f4a as libc::c_int,
    0x16f93 as libc::c_int,
    0x16f9f as libc::c_int,
    0x16fe0 as libc::c_int,
    0x16fe1 as libc::c_int,
    0x18800 as libc::c_int,
    0x18cd5 as libc::c_int,
    0x18cff as libc::c_int,
    0x18d00 as libc::c_int,
    0x1aff0 as libc::c_int,
    0x1aff3 as libc::c_int,
    0x1aff5 as libc::c_int,
    0x1affb as libc::c_int,
    0x1affd as libc::c_int,
    0x1affe as libc::c_int,
    0x1b000 as libc::c_int,
    0x1b122 as libc::c_int,
    0x1b150 as libc::c_int,
    0x1b152 as libc::c_int,
    0x1b164 as libc::c_int,
    0x1b167 as libc::c_int,
    0x1b170 as libc::c_int,
    0x1b2fb as libc::c_int,
    0x1bc00 as libc::c_int,
    0x1bc6a as libc::c_int,
    0x1bc70 as libc::c_int,
    0x1bc7c as libc::c_int,
    0x1bc80 as libc::c_int,
    0x1bc88 as libc::c_int,
    0x1bc90 as libc::c_int,
    0x1bc99 as libc::c_int,
    0x1d400 as libc::c_int,
    0x1d454 as libc::c_int,
    0x1d456 as libc::c_int,
    0x1d49c as libc::c_int,
    0x1d49e as libc::c_int,
    0x1d49f as libc::c_int,
    0x1d4a5 as libc::c_int,
    0x1d4a6 as libc::c_int,
    0x1d4a9 as libc::c_int,
    0x1d4ac as libc::c_int,
    0x1d4ae as libc::c_int,
    0x1d4b9 as libc::c_int,
    0x1d4bd as libc::c_int,
    0x1d4c3 as libc::c_int,
    0x1d4c5 as libc::c_int,
    0x1d505 as libc::c_int,
    0x1d507 as libc::c_int,
    0x1d50a as libc::c_int,
    0x1d50d as libc::c_int,
    0x1d514 as libc::c_int,
    0x1d516 as libc::c_int,
    0x1d51c as libc::c_int,
    0x1d51e as libc::c_int,
    0x1d539 as libc::c_int,
    0x1d53b as libc::c_int,
    0x1d53e as libc::c_int,
    0x1d540 as libc::c_int,
    0x1d544 as libc::c_int,
    0x1d54a as libc::c_int,
    0x1d550 as libc::c_int,
    0x1d552 as libc::c_int,
    0x1d6a5 as libc::c_int,
    0x1d6a8 as libc::c_int,
    0x1d6c0 as libc::c_int,
    0x1d6c2 as libc::c_int,
    0x1d6da as libc::c_int,
    0x1d6dc as libc::c_int,
    0x1d6fa as libc::c_int,
    0x1d6fc as libc::c_int,
    0x1d714 as libc::c_int,
    0x1d716 as libc::c_int,
    0x1d734 as libc::c_int,
    0x1d736 as libc::c_int,
    0x1d74e as libc::c_int,
    0x1d750 as libc::c_int,
    0x1d76e as libc::c_int,
    0x1d770 as libc::c_int,
    0x1d788 as libc::c_int,
    0x1d78a as libc::c_int,
    0x1d7a8 as libc::c_int,
    0x1d7aa as libc::c_int,
    0x1d7c2 as libc::c_int,
    0x1d7c4 as libc::c_int,
    0x1d7cb as libc::c_int,
    0x1df00 as libc::c_int,
    0x1df1e as libc::c_int,
    0x1df25 as libc::c_int,
    0x1df2a as libc::c_int,
    0x1e030 as libc::c_int,
    0x1e06d as libc::c_int,
    0x1e100 as libc::c_int,
    0x1e12c as libc::c_int,
    0x1e137 as libc::c_int,
    0x1e13d as libc::c_int,
    0x1e290 as libc::c_int,
    0x1e2ad as libc::c_int,
    0x1e2c0 as libc::c_int,
    0x1e2eb as libc::c_int,
    0x1e4d0 as libc::c_int,
    0x1e4eb as libc::c_int,
    0x1e5d0 as libc::c_int,
    0x1e5ed as libc::c_int,
    0x1e7e0 as libc::c_int,
    0x1e7e6 as libc::c_int,
    0x1e7e8 as libc::c_int,
    0x1e7eb as libc::c_int,
    0x1e7ed as libc::c_int,
    0x1e7ee as libc::c_int,
    0x1e7f0 as libc::c_int,
    0x1e7fe as libc::c_int,
    0x1e800 as libc::c_int,
    0x1e8c4 as libc::c_int,
    0x1e900 as libc::c_int,
    0x1e943 as libc::c_int,
    0x1ee00 as libc::c_int,
    0x1ee03 as libc::c_int,
    0x1ee05 as libc::c_int,
    0x1ee1f as libc::c_int,
    0x1ee21 as libc::c_int,
    0x1ee22 as libc::c_int,
    0x1ee29 as libc::c_int,
    0x1ee32 as libc::c_int,
    0x1ee34 as libc::c_int,
    0x1ee37 as libc::c_int,
    0x1ee4d as libc::c_int,
    0x1ee4f as libc::c_int,
    0x1ee51 as libc::c_int,
    0x1ee52 as libc::c_int,
    0x1ee61 as libc::c_int,
    0x1ee62 as libc::c_int,
    0x1ee67 as libc::c_int,
    0x1ee6a as libc::c_int,
    0x1ee6c as libc::c_int,
    0x1ee72 as libc::c_int,
    0x1ee74 as libc::c_int,
    0x1ee77 as libc::c_int,
    0x1ee79 as libc::c_int,
    0x1ee7c as libc::c_int,
    0x1ee80 as libc::c_int,
    0x1ee89 as libc::c_int,
    0x1ee8b as libc::c_int,
    0x1ee9b as libc::c_int,
    0x1eea1 as libc::c_int,
    0x1eea3 as libc::c_int,
    0x1eea5 as libc::c_int,
    0x1eea9 as libc::c_int,
    0x1eeab as libc::c_int,
    0x1eebb as libc::c_int,
    0x2f800 as libc::c_int,
    0x2fa1d as libc::c_int,
];
static mut ucd_alpha1: [Rune; 167] = [
    0xaa as libc::c_int,
    0xb5 as libc::c_int,
    0xba as libc::c_int,
    0x2ec as libc::c_int,
    0x2ee as libc::c_int,
    0x37f as libc::c_int,
    0x386 as libc::c_int,
    0x38c as libc::c_int,
    0x559 as libc::c_int,
    0x6d5 as libc::c_int,
    0x6ff as libc::c_int,
    0x710 as libc::c_int,
    0x7b1 as libc::c_int,
    0x7fa as libc::c_int,
    0x81a as libc::c_int,
    0x824 as libc::c_int,
    0x828 as libc::c_int,
    0x93d as libc::c_int,
    0x950 as libc::c_int,
    0x9b2 as libc::c_int,
    0x9bd as libc::c_int,
    0x9ce as libc::c_int,
    0x9fc as libc::c_int,
    0xa5e as libc::c_int,
    0xabd as libc::c_int,
    0xad0 as libc::c_int,
    0xaf9 as libc::c_int,
    0xb3d as libc::c_int,
    0xb71 as libc::c_int,
    0xb83 as libc::c_int,
    0xb9c as libc::c_int,
    0xbd0 as libc::c_int,
    0xc3d as libc::c_int,
    0xc5d as libc::c_int,
    0xc80 as libc::c_int,
    0xcbd as libc::c_int,
    0xd3d as libc::c_int,
    0xd4e as libc::c_int,
    0xdbd as libc::c_int,
    0xe84 as libc::c_int,
    0xea5 as libc::c_int,
    0xebd as libc::c_int,
    0xec6 as libc::c_int,
    0xf00 as libc::c_int,
    0x103f as libc::c_int,
    0x1061 as libc::c_int,
    0x108e as libc::c_int,
    0x10c7 as libc::c_int,
    0x10cd as libc::c_int,
    0x1258 as libc::c_int,
    0x12c0 as libc::c_int,
    0x17d7 as libc::c_int,
    0x17dc as libc::c_int,
    0x18aa as libc::c_int,
    0x1aa7 as libc::c_int,
    0x1cfa as libc::c_int,
    0x1f59 as libc::c_int,
    0x1f5b as libc::c_int,
    0x1f5d as libc::c_int,
    0x1fbe as libc::c_int,
    0x2071 as libc::c_int,
    0x207f as libc::c_int,
    0x2102 as libc::c_int,
    0x2107 as libc::c_int,
    0x2115 as libc::c_int,
    0x2124 as libc::c_int,
    0x2126 as libc::c_int,
    0x2128 as libc::c_int,
    0x214e as libc::c_int,
    0x2d27 as libc::c_int,
    0x2d2d as libc::c_int,
    0x2d6f as libc::c_int,
    0x2e2f as libc::c_int,
    0x3400 as libc::c_int,
    0x4dbf as libc::c_int,
    0x4e00 as libc::c_int,
    0xa7d3 as libc::c_int,
    0xa8fb as libc::c_int,
    0xa9cf as libc::c_int,
    0xaa7a as libc::c_int,
    0xaab1 as libc::c_int,
    0xaac0 as libc::c_int,
    0xaac2 as libc::c_int,
    0xac00 as libc::c_int,
    0xd7a3 as libc::c_int,
    0xfb1d as libc::c_int,
    0xfb3e as libc::c_int,
    0x10808 as libc::c_int,
    0x1083c as libc::c_int,
    0x10a00 as libc::c_int,
    0x10f27 as libc::c_int,
    0x11075 as libc::c_int,
    0x11144 as libc::c_int,
    0x11147 as libc::c_int,
    0x11176 as libc::c_int,
    0x111da as libc::c_int,
    0x111dc as libc::c_int,
    0x11288 as libc::c_int,
    0x1133d as libc::c_int,
    0x11350 as libc::c_int,
    0x1138b as libc::c_int,
    0x1138e as libc::c_int,
    0x113b7 as libc::c_int,
    0x113d1 as libc::c_int,
    0x113d3 as libc::c_int,
    0x114c7 as libc::c_int,
    0x11644 as libc::c_int,
    0x116b8 as libc::c_int,
    0x11909 as libc::c_int,
    0x1193f as libc::c_int,
    0x11941 as libc::c_int,
    0x119e1 as libc::c_int,
    0x119e3 as libc::c_int,
    0x11a00 as libc::c_int,
    0x11a3a as libc::c_int,
    0x11a50 as libc::c_int,
    0x11a9d as libc::c_int,
    0x11c40 as libc::c_int,
    0x11d46 as libc::c_int,
    0x11d98 as libc::c_int,
    0x11f02 as libc::c_int,
    0x11fb0 as libc::c_int,
    0x16f50 as libc::c_int,
    0x16fe3 as libc::c_int,
    0x17000 as libc::c_int,
    0x187f7 as libc::c_int,
    0x18d08 as libc::c_int,
    0x1b132 as libc::c_int,
    0x1b155 as libc::c_int,
    0x1d4a2 as libc::c_int,
    0x1d4bb as libc::c_int,
    0x1d546 as libc::c_int,
    0x1e14e as libc::c_int,
    0x1e5f0 as libc::c_int,
    0x1e94b as libc::c_int,
    0x1ee24 as libc::c_int,
    0x1ee27 as libc::c_int,
    0x1ee39 as libc::c_int,
    0x1ee3b as libc::c_int,
    0x1ee42 as libc::c_int,
    0x1ee47 as libc::c_int,
    0x1ee49 as libc::c_int,
    0x1ee4b as libc::c_int,
    0x1ee54 as libc::c_int,
    0x1ee57 as libc::c_int,
    0x1ee59 as libc::c_int,
    0x1ee5b as libc::c_int,
    0x1ee5d as libc::c_int,
    0x1ee5f as libc::c_int,
    0x1ee64 as libc::c_int,
    0x1ee7e as libc::c_int,
    0x20000 as libc::c_int,
    0x2a6df as libc::c_int,
    0x2a700 as libc::c_int,
    0x2b739 as libc::c_int,
    0x2b740 as libc::c_int,
    0x2b81d as libc::c_int,
    0x2b820 as libc::c_int,
    0x2cea1 as libc::c_int,
    0x2ceb0 as libc::c_int,
    0x2ebe0 as libc::c_int,
    0x2ebf0 as libc::c_int,
    0x2ee5d as libc::c_int,
    0x30000 as libc::c_int,
    0x3134a as libc::c_int,
    0x31350 as libc::c_int,
    0x323af as libc::c_int,
];
static mut ucd_tolower2: [Rune; 156] = [
    0x41 as libc::c_int,
    0x5a as libc::c_int,
    32 as libc::c_int,
    0xc0 as libc::c_int,
    0xd6 as libc::c_int,
    32 as libc::c_int,
    0xd8 as libc::c_int,
    0xde as libc::c_int,
    32 as libc::c_int,
    0x189 as libc::c_int,
    0x18a as libc::c_int,
    205 as libc::c_int,
    0x1b1 as libc::c_int,
    0x1b2 as libc::c_int,
    217 as libc::c_int,
    0x388 as libc::c_int,
    0x38a as libc::c_int,
    37 as libc::c_int,
    0x38e as libc::c_int,
    0x38f as libc::c_int,
    63 as libc::c_int,
    0x391 as libc::c_int,
    0x3a1 as libc::c_int,
    32 as libc::c_int,
    0x3a3 as libc::c_int,
    0x3ab as libc::c_int,
    32 as libc::c_int,
    0x3fd as libc::c_int,
    0x3ff as libc::c_int,
    -(130 as libc::c_int),
    0x400 as libc::c_int,
    0x40f as libc::c_int,
    80 as libc::c_int,
    0x410 as libc::c_int,
    0x42f as libc::c_int,
    32 as libc::c_int,
    0x531 as libc::c_int,
    0x556 as libc::c_int,
    48 as libc::c_int,
    0x10a0 as libc::c_int,
    0x10c5 as libc::c_int,
    7264 as libc::c_int,
    0x13a0 as libc::c_int,
    0x13ef as libc::c_int,
    38864 as libc::c_int,
    0x13f0 as libc::c_int,
    0x13f5 as libc::c_int,
    8 as libc::c_int,
    0x1c90 as libc::c_int,
    0x1cba as libc::c_int,
    -(3008 as libc::c_int),
    0x1cbd as libc::c_int,
    0x1cbf as libc::c_int,
    -(3008 as libc::c_int),
    0x1f08 as libc::c_int,
    0x1f0f as libc::c_int,
    -(8 as libc::c_int),
    0x1f18 as libc::c_int,
    0x1f1d as libc::c_int,
    -(8 as libc::c_int),
    0x1f28 as libc::c_int,
    0x1f2f as libc::c_int,
    -(8 as libc::c_int),
    0x1f38 as libc::c_int,
    0x1f3f as libc::c_int,
    -(8 as libc::c_int),
    0x1f48 as libc::c_int,
    0x1f4d as libc::c_int,
    -(8 as libc::c_int),
    0x1f68 as libc::c_int,
    0x1f6f as libc::c_int,
    -(8 as libc::c_int),
    0x1f88 as libc::c_int,
    0x1f8f as libc::c_int,
    -(8 as libc::c_int),
    0x1f98 as libc::c_int,
    0x1f9f as libc::c_int,
    -(8 as libc::c_int),
    0x1fa8 as libc::c_int,
    0x1faf as libc::c_int,
    -(8 as libc::c_int),
    0x1fb8 as libc::c_int,
    0x1fb9 as libc::c_int,
    -(8 as libc::c_int),
    0x1fba as libc::c_int,
    0x1fbb as libc::c_int,
    -(74 as libc::c_int),
    0x1fc8 as libc::c_int,
    0x1fcb as libc::c_int,
    -(86 as libc::c_int),
    0x1fd8 as libc::c_int,
    0x1fd9 as libc::c_int,
    -(8 as libc::c_int),
    0x1fda as libc::c_int,
    0x1fdb as libc::c_int,
    -(100 as libc::c_int),
    0x1fe8 as libc::c_int,
    0x1fe9 as libc::c_int,
    -(8 as libc::c_int),
    0x1fea as libc::c_int,
    0x1feb as libc::c_int,
    -(112 as libc::c_int),
    0x1ff8 as libc::c_int,
    0x1ff9 as libc::c_int,
    -(128 as libc::c_int),
    0x1ffa as libc::c_int,
    0x1ffb as libc::c_int,
    -(126 as libc::c_int),
    0x2160 as libc::c_int,
    0x216f as libc::c_int,
    16 as libc::c_int,
    0x24b6 as libc::c_int,
    0x24cf as libc::c_int,
    26 as libc::c_int,
    0x2c00 as libc::c_int,
    0x2c2f as libc::c_int,
    48 as libc::c_int,
    0x2c7e as libc::c_int,
    0x2c7f as libc::c_int,
    -(10815 as libc::c_int),
    0xff21 as libc::c_int,
    0xff3a as libc::c_int,
    32 as libc::c_int,
    0x10400 as libc::c_int,
    0x10427 as libc::c_int,
    40 as libc::c_int,
    0x104b0 as libc::c_int,
    0x104d3 as libc::c_int,
    40 as libc::c_int,
    0x10570 as libc::c_int,
    0x1057a as libc::c_int,
    39 as libc::c_int,
    0x1057c as libc::c_int,
    0x1058a as libc::c_int,
    39 as libc::c_int,
    0x1058c as libc::c_int,
    0x10592 as libc::c_int,
    39 as libc::c_int,
    0x10594 as libc::c_int,
    0x10595 as libc::c_int,
    39 as libc::c_int,
    0x10c80 as libc::c_int,
    0x10cb2 as libc::c_int,
    64 as libc::c_int,
    0x10d50 as libc::c_int,
    0x10d65 as libc::c_int,
    32 as libc::c_int,
    0x118a0 as libc::c_int,
    0x118bf as libc::c_int,
    32 as libc::c_int,
    0x16e40 as libc::c_int,
    0x16e5f as libc::c_int,
    32 as libc::c_int,
    0x1e900 as libc::c_int,
    0x1e921 as libc::c_int,
    34 as libc::c_int,
];
static mut ucd_tolower1: [Rune; 1244] = [
    0x100 as libc::c_int,
    1 as libc::c_int,
    0x102 as libc::c_int,
    1 as libc::c_int,
    0x104 as libc::c_int,
    1 as libc::c_int,
    0x106 as libc::c_int,
    1 as libc::c_int,
    0x108 as libc::c_int,
    1 as libc::c_int,
    0x10a as libc::c_int,
    1 as libc::c_int,
    0x10c as libc::c_int,
    1 as libc::c_int,
    0x10e as libc::c_int,
    1 as libc::c_int,
    0x110 as libc::c_int,
    1 as libc::c_int,
    0x112 as libc::c_int,
    1 as libc::c_int,
    0x114 as libc::c_int,
    1 as libc::c_int,
    0x116 as libc::c_int,
    1 as libc::c_int,
    0x118 as libc::c_int,
    1 as libc::c_int,
    0x11a as libc::c_int,
    1 as libc::c_int,
    0x11c as libc::c_int,
    1 as libc::c_int,
    0x11e as libc::c_int,
    1 as libc::c_int,
    0x120 as libc::c_int,
    1 as libc::c_int,
    0x122 as libc::c_int,
    1 as libc::c_int,
    0x124 as libc::c_int,
    1 as libc::c_int,
    0x126 as libc::c_int,
    1 as libc::c_int,
    0x128 as libc::c_int,
    1 as libc::c_int,
    0x12a as libc::c_int,
    1 as libc::c_int,
    0x12c as libc::c_int,
    1 as libc::c_int,
    0x12e as libc::c_int,
    1 as libc::c_int,
    0x130 as libc::c_int,
    -(199 as libc::c_int),
    0x132 as libc::c_int,
    1 as libc::c_int,
    0x134 as libc::c_int,
    1 as libc::c_int,
    0x136 as libc::c_int,
    1 as libc::c_int,
    0x139 as libc::c_int,
    1 as libc::c_int,
    0x13b as libc::c_int,
    1 as libc::c_int,
    0x13d as libc::c_int,
    1 as libc::c_int,
    0x13f as libc::c_int,
    1 as libc::c_int,
    0x141 as libc::c_int,
    1 as libc::c_int,
    0x143 as libc::c_int,
    1 as libc::c_int,
    0x145 as libc::c_int,
    1 as libc::c_int,
    0x147 as libc::c_int,
    1 as libc::c_int,
    0x14a as libc::c_int,
    1 as libc::c_int,
    0x14c as libc::c_int,
    1 as libc::c_int,
    0x14e as libc::c_int,
    1 as libc::c_int,
    0x150 as libc::c_int,
    1 as libc::c_int,
    0x152 as libc::c_int,
    1 as libc::c_int,
    0x154 as libc::c_int,
    1 as libc::c_int,
    0x156 as libc::c_int,
    1 as libc::c_int,
    0x158 as libc::c_int,
    1 as libc::c_int,
    0x15a as libc::c_int,
    1 as libc::c_int,
    0x15c as libc::c_int,
    1 as libc::c_int,
    0x15e as libc::c_int,
    1 as libc::c_int,
    0x160 as libc::c_int,
    1 as libc::c_int,
    0x162 as libc::c_int,
    1 as libc::c_int,
    0x164 as libc::c_int,
    1 as libc::c_int,
    0x166 as libc::c_int,
    1 as libc::c_int,
    0x168 as libc::c_int,
    1 as libc::c_int,
    0x16a as libc::c_int,
    1 as libc::c_int,
    0x16c as libc::c_int,
    1 as libc::c_int,
    0x16e as libc::c_int,
    1 as libc::c_int,
    0x170 as libc::c_int,
    1 as libc::c_int,
    0x172 as libc::c_int,
    1 as libc::c_int,
    0x174 as libc::c_int,
    1 as libc::c_int,
    0x176 as libc::c_int,
    1 as libc::c_int,
    0x178 as libc::c_int,
    -(121 as libc::c_int),
    0x179 as libc::c_int,
    1 as libc::c_int,
    0x17b as libc::c_int,
    1 as libc::c_int,
    0x17d as libc::c_int,
    1 as libc::c_int,
    0x181 as libc::c_int,
    210 as libc::c_int,
    0x182 as libc::c_int,
    1 as libc::c_int,
    0x184 as libc::c_int,
    1 as libc::c_int,
    0x186 as libc::c_int,
    206 as libc::c_int,
    0x187 as libc::c_int,
    1 as libc::c_int,
    0x18b as libc::c_int,
    1 as libc::c_int,
    0x18e as libc::c_int,
    79 as libc::c_int,
    0x18f as libc::c_int,
    202 as libc::c_int,
    0x190 as libc::c_int,
    203 as libc::c_int,
    0x191 as libc::c_int,
    1 as libc::c_int,
    0x193 as libc::c_int,
    205 as libc::c_int,
    0x194 as libc::c_int,
    207 as libc::c_int,
    0x196 as libc::c_int,
    211 as libc::c_int,
    0x197 as libc::c_int,
    209 as libc::c_int,
    0x198 as libc::c_int,
    1 as libc::c_int,
    0x19c as libc::c_int,
    211 as libc::c_int,
    0x19d as libc::c_int,
    213 as libc::c_int,
    0x19f as libc::c_int,
    214 as libc::c_int,
    0x1a0 as libc::c_int,
    1 as libc::c_int,
    0x1a2 as libc::c_int,
    1 as libc::c_int,
    0x1a4 as libc::c_int,
    1 as libc::c_int,
    0x1a6 as libc::c_int,
    218 as libc::c_int,
    0x1a7 as libc::c_int,
    1 as libc::c_int,
    0x1a9 as libc::c_int,
    218 as libc::c_int,
    0x1ac as libc::c_int,
    1 as libc::c_int,
    0x1ae as libc::c_int,
    218 as libc::c_int,
    0x1af as libc::c_int,
    1 as libc::c_int,
    0x1b3 as libc::c_int,
    1 as libc::c_int,
    0x1b5 as libc::c_int,
    1 as libc::c_int,
    0x1b7 as libc::c_int,
    219 as libc::c_int,
    0x1b8 as libc::c_int,
    1 as libc::c_int,
    0x1bc as libc::c_int,
    1 as libc::c_int,
    0x1c4 as libc::c_int,
    2 as libc::c_int,
    0x1c5 as libc::c_int,
    1 as libc::c_int,
    0x1c7 as libc::c_int,
    2 as libc::c_int,
    0x1c8 as libc::c_int,
    1 as libc::c_int,
    0x1ca as libc::c_int,
    2 as libc::c_int,
    0x1cb as libc::c_int,
    1 as libc::c_int,
    0x1cd as libc::c_int,
    1 as libc::c_int,
    0x1cf as libc::c_int,
    1 as libc::c_int,
    0x1d1 as libc::c_int,
    1 as libc::c_int,
    0x1d3 as libc::c_int,
    1 as libc::c_int,
    0x1d5 as libc::c_int,
    1 as libc::c_int,
    0x1d7 as libc::c_int,
    1 as libc::c_int,
    0x1d9 as libc::c_int,
    1 as libc::c_int,
    0x1db as libc::c_int,
    1 as libc::c_int,
    0x1de as libc::c_int,
    1 as libc::c_int,
    0x1e0 as libc::c_int,
    1 as libc::c_int,
    0x1e2 as libc::c_int,
    1 as libc::c_int,
    0x1e4 as libc::c_int,
    1 as libc::c_int,
    0x1e6 as libc::c_int,
    1 as libc::c_int,
    0x1e8 as libc::c_int,
    1 as libc::c_int,
    0x1ea as libc::c_int,
    1 as libc::c_int,
    0x1ec as libc::c_int,
    1 as libc::c_int,
    0x1ee as libc::c_int,
    1 as libc::c_int,
    0x1f1 as libc::c_int,
    2 as libc::c_int,
    0x1f2 as libc::c_int,
    1 as libc::c_int,
    0x1f4 as libc::c_int,
    1 as libc::c_int,
    0x1f6 as libc::c_int,
    -(97 as libc::c_int),
    0x1f7 as libc::c_int,
    -(56 as libc::c_int),
    0x1f8 as libc::c_int,
    1 as libc::c_int,
    0x1fa as libc::c_int,
    1 as libc::c_int,
    0x1fc as libc::c_int,
    1 as libc::c_int,
    0x1fe as libc::c_int,
    1 as libc::c_int,
    0x200 as libc::c_int,
    1 as libc::c_int,
    0x202 as libc::c_int,
    1 as libc::c_int,
    0x204 as libc::c_int,
    1 as libc::c_int,
    0x206 as libc::c_int,
    1 as libc::c_int,
    0x208 as libc::c_int,
    1 as libc::c_int,
    0x20a as libc::c_int,
    1 as libc::c_int,
    0x20c as libc::c_int,
    1 as libc::c_int,
    0x20e as libc::c_int,
    1 as libc::c_int,
    0x210 as libc::c_int,
    1 as libc::c_int,
    0x212 as libc::c_int,
    1 as libc::c_int,
    0x214 as libc::c_int,
    1 as libc::c_int,
    0x216 as libc::c_int,
    1 as libc::c_int,
    0x218 as libc::c_int,
    1 as libc::c_int,
    0x21a as libc::c_int,
    1 as libc::c_int,
    0x21c as libc::c_int,
    1 as libc::c_int,
    0x21e as libc::c_int,
    1 as libc::c_int,
    0x220 as libc::c_int,
    -(130 as libc::c_int),
    0x222 as libc::c_int,
    1 as libc::c_int,
    0x224 as libc::c_int,
    1 as libc::c_int,
    0x226 as libc::c_int,
    1 as libc::c_int,
    0x228 as libc::c_int,
    1 as libc::c_int,
    0x22a as libc::c_int,
    1 as libc::c_int,
    0x22c as libc::c_int,
    1 as libc::c_int,
    0x22e as libc::c_int,
    1 as libc::c_int,
    0x230 as libc::c_int,
    1 as libc::c_int,
    0x232 as libc::c_int,
    1 as libc::c_int,
    0x23a as libc::c_int,
    10795 as libc::c_int,
    0x23b as libc::c_int,
    1 as libc::c_int,
    0x23d as libc::c_int,
    -(163 as libc::c_int),
    0x23e as libc::c_int,
    10792 as libc::c_int,
    0x241 as libc::c_int,
    1 as libc::c_int,
    0x243 as libc::c_int,
    -(195 as libc::c_int),
    0x244 as libc::c_int,
    69 as libc::c_int,
    0x245 as libc::c_int,
    71 as libc::c_int,
    0x246 as libc::c_int,
    1 as libc::c_int,
    0x248 as libc::c_int,
    1 as libc::c_int,
    0x24a as libc::c_int,
    1 as libc::c_int,
    0x24c as libc::c_int,
    1 as libc::c_int,
    0x24e as libc::c_int,
    1 as libc::c_int,
    0x370 as libc::c_int,
    1 as libc::c_int,
    0x372 as libc::c_int,
    1 as libc::c_int,
    0x376 as libc::c_int,
    1 as libc::c_int,
    0x37f as libc::c_int,
    116 as libc::c_int,
    0x386 as libc::c_int,
    38 as libc::c_int,
    0x38c as libc::c_int,
    64 as libc::c_int,
    0x3cf as libc::c_int,
    8 as libc::c_int,
    0x3d8 as libc::c_int,
    1 as libc::c_int,
    0x3da as libc::c_int,
    1 as libc::c_int,
    0x3dc as libc::c_int,
    1 as libc::c_int,
    0x3de as libc::c_int,
    1 as libc::c_int,
    0x3e0 as libc::c_int,
    1 as libc::c_int,
    0x3e2 as libc::c_int,
    1 as libc::c_int,
    0x3e4 as libc::c_int,
    1 as libc::c_int,
    0x3e6 as libc::c_int,
    1 as libc::c_int,
    0x3e8 as libc::c_int,
    1 as libc::c_int,
    0x3ea as libc::c_int,
    1 as libc::c_int,
    0x3ec as libc::c_int,
    1 as libc::c_int,
    0x3ee as libc::c_int,
    1 as libc::c_int,
    0x3f4 as libc::c_int,
    -(60 as libc::c_int),
    0x3f7 as libc::c_int,
    1 as libc::c_int,
    0x3f9 as libc::c_int,
    -(7 as libc::c_int),
    0x3fa as libc::c_int,
    1 as libc::c_int,
    0x460 as libc::c_int,
    1 as libc::c_int,
    0x462 as libc::c_int,
    1 as libc::c_int,
    0x464 as libc::c_int,
    1 as libc::c_int,
    0x466 as libc::c_int,
    1 as libc::c_int,
    0x468 as libc::c_int,
    1 as libc::c_int,
    0x46a as libc::c_int,
    1 as libc::c_int,
    0x46c as libc::c_int,
    1 as libc::c_int,
    0x46e as libc::c_int,
    1 as libc::c_int,
    0x470 as libc::c_int,
    1 as libc::c_int,
    0x472 as libc::c_int,
    1 as libc::c_int,
    0x474 as libc::c_int,
    1 as libc::c_int,
    0x476 as libc::c_int,
    1 as libc::c_int,
    0x478 as libc::c_int,
    1 as libc::c_int,
    0x47a as libc::c_int,
    1 as libc::c_int,
    0x47c as libc::c_int,
    1 as libc::c_int,
    0x47e as libc::c_int,
    1 as libc::c_int,
    0x480 as libc::c_int,
    1 as libc::c_int,
    0x48a as libc::c_int,
    1 as libc::c_int,
    0x48c as libc::c_int,
    1 as libc::c_int,
    0x48e as libc::c_int,
    1 as libc::c_int,
    0x490 as libc::c_int,
    1 as libc::c_int,
    0x492 as libc::c_int,
    1 as libc::c_int,
    0x494 as libc::c_int,
    1 as libc::c_int,
    0x496 as libc::c_int,
    1 as libc::c_int,
    0x498 as libc::c_int,
    1 as libc::c_int,
    0x49a as libc::c_int,
    1 as libc::c_int,
    0x49c as libc::c_int,
    1 as libc::c_int,
    0x49e as libc::c_int,
    1 as libc::c_int,
    0x4a0 as libc::c_int,
    1 as libc::c_int,
    0x4a2 as libc::c_int,
    1 as libc::c_int,
    0x4a4 as libc::c_int,
    1 as libc::c_int,
    0x4a6 as libc::c_int,
    1 as libc::c_int,
    0x4a8 as libc::c_int,
    1 as libc::c_int,
    0x4aa as libc::c_int,
    1 as libc::c_int,
    0x4ac as libc::c_int,
    1 as libc::c_int,
    0x4ae as libc::c_int,
    1 as libc::c_int,
    0x4b0 as libc::c_int,
    1 as libc::c_int,
    0x4b2 as libc::c_int,
    1 as libc::c_int,
    0x4b4 as libc::c_int,
    1 as libc::c_int,
    0x4b6 as libc::c_int,
    1 as libc::c_int,
    0x4b8 as libc::c_int,
    1 as libc::c_int,
    0x4ba as libc::c_int,
    1 as libc::c_int,
    0x4bc as libc::c_int,
    1 as libc::c_int,
    0x4be as libc::c_int,
    1 as libc::c_int,
    0x4c0 as libc::c_int,
    15 as libc::c_int,
    0x4c1 as libc::c_int,
    1 as libc::c_int,
    0x4c3 as libc::c_int,
    1 as libc::c_int,
    0x4c5 as libc::c_int,
    1 as libc::c_int,
    0x4c7 as libc::c_int,
    1 as libc::c_int,
    0x4c9 as libc::c_int,
    1 as libc::c_int,
    0x4cb as libc::c_int,
    1 as libc::c_int,
    0x4cd as libc::c_int,
    1 as libc::c_int,
    0x4d0 as libc::c_int,
    1 as libc::c_int,
    0x4d2 as libc::c_int,
    1 as libc::c_int,
    0x4d4 as libc::c_int,
    1 as libc::c_int,
    0x4d6 as libc::c_int,
    1 as libc::c_int,
    0x4d8 as libc::c_int,
    1 as libc::c_int,
    0x4da as libc::c_int,
    1 as libc::c_int,
    0x4dc as libc::c_int,
    1 as libc::c_int,
    0x4de as libc::c_int,
    1 as libc::c_int,
    0x4e0 as libc::c_int,
    1 as libc::c_int,
    0x4e2 as libc::c_int,
    1 as libc::c_int,
    0x4e4 as libc::c_int,
    1 as libc::c_int,
    0x4e6 as libc::c_int,
    1 as libc::c_int,
    0x4e8 as libc::c_int,
    1 as libc::c_int,
    0x4ea as libc::c_int,
    1 as libc::c_int,
    0x4ec as libc::c_int,
    1 as libc::c_int,
    0x4ee as libc::c_int,
    1 as libc::c_int,
    0x4f0 as libc::c_int,
    1 as libc::c_int,
    0x4f2 as libc::c_int,
    1 as libc::c_int,
    0x4f4 as libc::c_int,
    1 as libc::c_int,
    0x4f6 as libc::c_int,
    1 as libc::c_int,
    0x4f8 as libc::c_int,
    1 as libc::c_int,
    0x4fa as libc::c_int,
    1 as libc::c_int,
    0x4fc as libc::c_int,
    1 as libc::c_int,
    0x4fe as libc::c_int,
    1 as libc::c_int,
    0x500 as libc::c_int,
    1 as libc::c_int,
    0x502 as libc::c_int,
    1 as libc::c_int,
    0x504 as libc::c_int,
    1 as libc::c_int,
    0x506 as libc::c_int,
    1 as libc::c_int,
    0x508 as libc::c_int,
    1 as libc::c_int,
    0x50a as libc::c_int,
    1 as libc::c_int,
    0x50c as libc::c_int,
    1 as libc::c_int,
    0x50e as libc::c_int,
    1 as libc::c_int,
    0x510 as libc::c_int,
    1 as libc::c_int,
    0x512 as libc::c_int,
    1 as libc::c_int,
    0x514 as libc::c_int,
    1 as libc::c_int,
    0x516 as libc::c_int,
    1 as libc::c_int,
    0x518 as libc::c_int,
    1 as libc::c_int,
    0x51a as libc::c_int,
    1 as libc::c_int,
    0x51c as libc::c_int,
    1 as libc::c_int,
    0x51e as libc::c_int,
    1 as libc::c_int,
    0x520 as libc::c_int,
    1 as libc::c_int,
    0x522 as libc::c_int,
    1 as libc::c_int,
    0x524 as libc::c_int,
    1 as libc::c_int,
    0x526 as libc::c_int,
    1 as libc::c_int,
    0x528 as libc::c_int,
    1 as libc::c_int,
    0x52a as libc::c_int,
    1 as libc::c_int,
    0x52c as libc::c_int,
    1 as libc::c_int,
    0x52e as libc::c_int,
    1 as libc::c_int,
    0x10c7 as libc::c_int,
    7264 as libc::c_int,
    0x10cd as libc::c_int,
    7264 as libc::c_int,
    0x1c89 as libc::c_int,
    1 as libc::c_int,
    0x1e00 as libc::c_int,
    1 as libc::c_int,
    0x1e02 as libc::c_int,
    1 as libc::c_int,
    0x1e04 as libc::c_int,
    1 as libc::c_int,
    0x1e06 as libc::c_int,
    1 as libc::c_int,
    0x1e08 as libc::c_int,
    1 as libc::c_int,
    0x1e0a as libc::c_int,
    1 as libc::c_int,
    0x1e0c as libc::c_int,
    1 as libc::c_int,
    0x1e0e as libc::c_int,
    1 as libc::c_int,
    0x1e10 as libc::c_int,
    1 as libc::c_int,
    0x1e12 as libc::c_int,
    1 as libc::c_int,
    0x1e14 as libc::c_int,
    1 as libc::c_int,
    0x1e16 as libc::c_int,
    1 as libc::c_int,
    0x1e18 as libc::c_int,
    1 as libc::c_int,
    0x1e1a as libc::c_int,
    1 as libc::c_int,
    0x1e1c as libc::c_int,
    1 as libc::c_int,
    0x1e1e as libc::c_int,
    1 as libc::c_int,
    0x1e20 as libc::c_int,
    1 as libc::c_int,
    0x1e22 as libc::c_int,
    1 as libc::c_int,
    0x1e24 as libc::c_int,
    1 as libc::c_int,
    0x1e26 as libc::c_int,
    1 as libc::c_int,
    0x1e28 as libc::c_int,
    1 as libc::c_int,
    0x1e2a as libc::c_int,
    1 as libc::c_int,
    0x1e2c as libc::c_int,
    1 as libc::c_int,
    0x1e2e as libc::c_int,
    1 as libc::c_int,
    0x1e30 as libc::c_int,
    1 as libc::c_int,
    0x1e32 as libc::c_int,
    1 as libc::c_int,
    0x1e34 as libc::c_int,
    1 as libc::c_int,
    0x1e36 as libc::c_int,
    1 as libc::c_int,
    0x1e38 as libc::c_int,
    1 as libc::c_int,
    0x1e3a as libc::c_int,
    1 as libc::c_int,
    0x1e3c as libc::c_int,
    1 as libc::c_int,
    0x1e3e as libc::c_int,
    1 as libc::c_int,
    0x1e40 as libc::c_int,
    1 as libc::c_int,
    0x1e42 as libc::c_int,
    1 as libc::c_int,
    0x1e44 as libc::c_int,
    1 as libc::c_int,
    0x1e46 as libc::c_int,
    1 as libc::c_int,
    0x1e48 as libc::c_int,
    1 as libc::c_int,
    0x1e4a as libc::c_int,
    1 as libc::c_int,
    0x1e4c as libc::c_int,
    1 as libc::c_int,
    0x1e4e as libc::c_int,
    1 as libc::c_int,
    0x1e50 as libc::c_int,
    1 as libc::c_int,
    0x1e52 as libc::c_int,
    1 as libc::c_int,
    0x1e54 as libc::c_int,
    1 as libc::c_int,
    0x1e56 as libc::c_int,
    1 as libc::c_int,
    0x1e58 as libc::c_int,
    1 as libc::c_int,
    0x1e5a as libc::c_int,
    1 as libc::c_int,
    0x1e5c as libc::c_int,
    1 as libc::c_int,
    0x1e5e as libc::c_int,
    1 as libc::c_int,
    0x1e60 as libc::c_int,
    1 as libc::c_int,
    0x1e62 as libc::c_int,
    1 as libc::c_int,
    0x1e64 as libc::c_int,
    1 as libc::c_int,
    0x1e66 as libc::c_int,
    1 as libc::c_int,
    0x1e68 as libc::c_int,
    1 as libc::c_int,
    0x1e6a as libc::c_int,
    1 as libc::c_int,
    0x1e6c as libc::c_int,
    1 as libc::c_int,
    0x1e6e as libc::c_int,
    1 as libc::c_int,
    0x1e70 as libc::c_int,
    1 as libc::c_int,
    0x1e72 as libc::c_int,
    1 as libc::c_int,
    0x1e74 as libc::c_int,
    1 as libc::c_int,
    0x1e76 as libc::c_int,
    1 as libc::c_int,
    0x1e78 as libc::c_int,
    1 as libc::c_int,
    0x1e7a as libc::c_int,
    1 as libc::c_int,
    0x1e7c as libc::c_int,
    1 as libc::c_int,
    0x1e7e as libc::c_int,
    1 as libc::c_int,
    0x1e80 as libc::c_int,
    1 as libc::c_int,
    0x1e82 as libc::c_int,
    1 as libc::c_int,
    0x1e84 as libc::c_int,
    1 as libc::c_int,
    0x1e86 as libc::c_int,
    1 as libc::c_int,
    0x1e88 as libc::c_int,
    1 as libc::c_int,
    0x1e8a as libc::c_int,
    1 as libc::c_int,
    0x1e8c as libc::c_int,
    1 as libc::c_int,
    0x1e8e as libc::c_int,
    1 as libc::c_int,
    0x1e90 as libc::c_int,
    1 as libc::c_int,
    0x1e92 as libc::c_int,
    1 as libc::c_int,
    0x1e94 as libc::c_int,
    1 as libc::c_int,
    0x1e9e as libc::c_int,
    -(7615 as libc::c_int),
    0x1ea0 as libc::c_int,
    1 as libc::c_int,
    0x1ea2 as libc::c_int,
    1 as libc::c_int,
    0x1ea4 as libc::c_int,
    1 as libc::c_int,
    0x1ea6 as libc::c_int,
    1 as libc::c_int,
    0x1ea8 as libc::c_int,
    1 as libc::c_int,
    0x1eaa as libc::c_int,
    1 as libc::c_int,
    0x1eac as libc::c_int,
    1 as libc::c_int,
    0x1eae as libc::c_int,
    1 as libc::c_int,
    0x1eb0 as libc::c_int,
    1 as libc::c_int,
    0x1eb2 as libc::c_int,
    1 as libc::c_int,
    0x1eb4 as libc::c_int,
    1 as libc::c_int,
    0x1eb6 as libc::c_int,
    1 as libc::c_int,
    0x1eb8 as libc::c_int,
    1 as libc::c_int,
    0x1eba as libc::c_int,
    1 as libc::c_int,
    0x1ebc as libc::c_int,
    1 as libc::c_int,
    0x1ebe as libc::c_int,
    1 as libc::c_int,
    0x1ec0 as libc::c_int,
    1 as libc::c_int,
    0x1ec2 as libc::c_int,
    1 as libc::c_int,
    0x1ec4 as libc::c_int,
    1 as libc::c_int,
    0x1ec6 as libc::c_int,
    1 as libc::c_int,
    0x1ec8 as libc::c_int,
    1 as libc::c_int,
    0x1eca as libc::c_int,
    1 as libc::c_int,
    0x1ecc as libc::c_int,
    1 as libc::c_int,
    0x1ece as libc::c_int,
    1 as libc::c_int,
    0x1ed0 as libc::c_int,
    1 as libc::c_int,
    0x1ed2 as libc::c_int,
    1 as libc::c_int,
    0x1ed4 as libc::c_int,
    1 as libc::c_int,
    0x1ed6 as libc::c_int,
    1 as libc::c_int,
    0x1ed8 as libc::c_int,
    1 as libc::c_int,
    0x1eda as libc::c_int,
    1 as libc::c_int,
    0x1edc as libc::c_int,
    1 as libc::c_int,
    0x1ede as libc::c_int,
    1 as libc::c_int,
    0x1ee0 as libc::c_int,
    1 as libc::c_int,
    0x1ee2 as libc::c_int,
    1 as libc::c_int,
    0x1ee4 as libc::c_int,
    1 as libc::c_int,
    0x1ee6 as libc::c_int,
    1 as libc::c_int,
    0x1ee8 as libc::c_int,
    1 as libc::c_int,
    0x1eea as libc::c_int,
    1 as libc::c_int,
    0x1eec as libc::c_int,
    1 as libc::c_int,
    0x1eee as libc::c_int,
    1 as libc::c_int,
    0x1ef0 as libc::c_int,
    1 as libc::c_int,
    0x1ef2 as libc::c_int,
    1 as libc::c_int,
    0x1ef4 as libc::c_int,
    1 as libc::c_int,
    0x1ef6 as libc::c_int,
    1 as libc::c_int,
    0x1ef8 as libc::c_int,
    1 as libc::c_int,
    0x1efa as libc::c_int,
    1 as libc::c_int,
    0x1efc as libc::c_int,
    1 as libc::c_int,
    0x1efe as libc::c_int,
    1 as libc::c_int,
    0x1f59 as libc::c_int,
    -(8 as libc::c_int),
    0x1f5b as libc::c_int,
    -(8 as libc::c_int),
    0x1f5d as libc::c_int,
    -(8 as libc::c_int),
    0x1f5f as libc::c_int,
    -(8 as libc::c_int),
    0x1fbc as libc::c_int,
    -(9 as libc::c_int),
    0x1fcc as libc::c_int,
    -(9 as libc::c_int),
    0x1fec as libc::c_int,
    -(7 as libc::c_int),
    0x1ffc as libc::c_int,
    -(9 as libc::c_int),
    0x2126 as libc::c_int,
    -(7517 as libc::c_int),
    0x212a as libc::c_int,
    -(8383 as libc::c_int),
    0x212b as libc::c_int,
    -(8262 as libc::c_int),
    0x2132 as libc::c_int,
    28 as libc::c_int,
    0x2183 as libc::c_int,
    1 as libc::c_int,
    0x2c60 as libc::c_int,
    1 as libc::c_int,
    0x2c62 as libc::c_int,
    -(10743 as libc::c_int),
    0x2c63 as libc::c_int,
    -(3814 as libc::c_int),
    0x2c64 as libc::c_int,
    -(10727 as libc::c_int),
    0x2c67 as libc::c_int,
    1 as libc::c_int,
    0x2c69 as libc::c_int,
    1 as libc::c_int,
    0x2c6b as libc::c_int,
    1 as libc::c_int,
    0x2c6d as libc::c_int,
    -(10780 as libc::c_int),
    0x2c6e as libc::c_int,
    -(10749 as libc::c_int),
    0x2c6f as libc::c_int,
    -(10783 as libc::c_int),
    0x2c70 as libc::c_int,
    -(10782 as libc::c_int),
    0x2c72 as libc::c_int,
    1 as libc::c_int,
    0x2c75 as libc::c_int,
    1 as libc::c_int,
    0x2c80 as libc::c_int,
    1 as libc::c_int,
    0x2c82 as libc::c_int,
    1 as libc::c_int,
    0x2c84 as libc::c_int,
    1 as libc::c_int,
    0x2c86 as libc::c_int,
    1 as libc::c_int,
    0x2c88 as libc::c_int,
    1 as libc::c_int,
    0x2c8a as libc::c_int,
    1 as libc::c_int,
    0x2c8c as libc::c_int,
    1 as libc::c_int,
    0x2c8e as libc::c_int,
    1 as libc::c_int,
    0x2c90 as libc::c_int,
    1 as libc::c_int,
    0x2c92 as libc::c_int,
    1 as libc::c_int,
    0x2c94 as libc::c_int,
    1 as libc::c_int,
    0x2c96 as libc::c_int,
    1 as libc::c_int,
    0x2c98 as libc::c_int,
    1 as libc::c_int,
    0x2c9a as libc::c_int,
    1 as libc::c_int,
    0x2c9c as libc::c_int,
    1 as libc::c_int,
    0x2c9e as libc::c_int,
    1 as libc::c_int,
    0x2ca0 as libc::c_int,
    1 as libc::c_int,
    0x2ca2 as libc::c_int,
    1 as libc::c_int,
    0x2ca4 as libc::c_int,
    1 as libc::c_int,
    0x2ca6 as libc::c_int,
    1 as libc::c_int,
    0x2ca8 as libc::c_int,
    1 as libc::c_int,
    0x2caa as libc::c_int,
    1 as libc::c_int,
    0x2cac as libc::c_int,
    1 as libc::c_int,
    0x2cae as libc::c_int,
    1 as libc::c_int,
    0x2cb0 as libc::c_int,
    1 as libc::c_int,
    0x2cb2 as libc::c_int,
    1 as libc::c_int,
    0x2cb4 as libc::c_int,
    1 as libc::c_int,
    0x2cb6 as libc::c_int,
    1 as libc::c_int,
    0x2cb8 as libc::c_int,
    1 as libc::c_int,
    0x2cba as libc::c_int,
    1 as libc::c_int,
    0x2cbc as libc::c_int,
    1 as libc::c_int,
    0x2cbe as libc::c_int,
    1 as libc::c_int,
    0x2cc0 as libc::c_int,
    1 as libc::c_int,
    0x2cc2 as libc::c_int,
    1 as libc::c_int,
    0x2cc4 as libc::c_int,
    1 as libc::c_int,
    0x2cc6 as libc::c_int,
    1 as libc::c_int,
    0x2cc8 as libc::c_int,
    1 as libc::c_int,
    0x2cca as libc::c_int,
    1 as libc::c_int,
    0x2ccc as libc::c_int,
    1 as libc::c_int,
    0x2cce as libc::c_int,
    1 as libc::c_int,
    0x2cd0 as libc::c_int,
    1 as libc::c_int,
    0x2cd2 as libc::c_int,
    1 as libc::c_int,
    0x2cd4 as libc::c_int,
    1 as libc::c_int,
    0x2cd6 as libc::c_int,
    1 as libc::c_int,
    0x2cd8 as libc::c_int,
    1 as libc::c_int,
    0x2cda as libc::c_int,
    1 as libc::c_int,
    0x2cdc as libc::c_int,
    1 as libc::c_int,
    0x2cde as libc::c_int,
    1 as libc::c_int,
    0x2ce0 as libc::c_int,
    1 as libc::c_int,
    0x2ce2 as libc::c_int,
    1 as libc::c_int,
    0x2ceb as libc::c_int,
    1 as libc::c_int,
    0x2ced as libc::c_int,
    1 as libc::c_int,
    0x2cf2 as libc::c_int,
    1 as libc::c_int,
    0xa640 as libc::c_int,
    1 as libc::c_int,
    0xa642 as libc::c_int,
    1 as libc::c_int,
    0xa644 as libc::c_int,
    1 as libc::c_int,
    0xa646 as libc::c_int,
    1 as libc::c_int,
    0xa648 as libc::c_int,
    1 as libc::c_int,
    0xa64a as libc::c_int,
    1 as libc::c_int,
    0xa64c as libc::c_int,
    1 as libc::c_int,
    0xa64e as libc::c_int,
    1 as libc::c_int,
    0xa650 as libc::c_int,
    1 as libc::c_int,
    0xa652 as libc::c_int,
    1 as libc::c_int,
    0xa654 as libc::c_int,
    1 as libc::c_int,
    0xa656 as libc::c_int,
    1 as libc::c_int,
    0xa658 as libc::c_int,
    1 as libc::c_int,
    0xa65a as libc::c_int,
    1 as libc::c_int,
    0xa65c as libc::c_int,
    1 as libc::c_int,
    0xa65e as libc::c_int,
    1 as libc::c_int,
    0xa660 as libc::c_int,
    1 as libc::c_int,
    0xa662 as libc::c_int,
    1 as libc::c_int,
    0xa664 as libc::c_int,
    1 as libc::c_int,
    0xa666 as libc::c_int,
    1 as libc::c_int,
    0xa668 as libc::c_int,
    1 as libc::c_int,
    0xa66a as libc::c_int,
    1 as libc::c_int,
    0xa66c as libc::c_int,
    1 as libc::c_int,
    0xa680 as libc::c_int,
    1 as libc::c_int,
    0xa682 as libc::c_int,
    1 as libc::c_int,
    0xa684 as libc::c_int,
    1 as libc::c_int,
    0xa686 as libc::c_int,
    1 as libc::c_int,
    0xa688 as libc::c_int,
    1 as libc::c_int,
    0xa68a as libc::c_int,
    1 as libc::c_int,
    0xa68c as libc::c_int,
    1 as libc::c_int,
    0xa68e as libc::c_int,
    1 as libc::c_int,
    0xa690 as libc::c_int,
    1 as libc::c_int,
    0xa692 as libc::c_int,
    1 as libc::c_int,
    0xa694 as libc::c_int,
    1 as libc::c_int,
    0xa696 as libc::c_int,
    1 as libc::c_int,
    0xa698 as libc::c_int,
    1 as libc::c_int,
    0xa69a as libc::c_int,
    1 as libc::c_int,
    0xa722 as libc::c_int,
    1 as libc::c_int,
    0xa724 as libc::c_int,
    1 as libc::c_int,
    0xa726 as libc::c_int,
    1 as libc::c_int,
    0xa728 as libc::c_int,
    1 as libc::c_int,
    0xa72a as libc::c_int,
    1 as libc::c_int,
    0xa72c as libc::c_int,
    1 as libc::c_int,
    0xa72e as libc::c_int,
    1 as libc::c_int,
    0xa732 as libc::c_int,
    1 as libc::c_int,
    0xa734 as libc::c_int,
    1 as libc::c_int,
    0xa736 as libc::c_int,
    1 as libc::c_int,
    0xa738 as libc::c_int,
    1 as libc::c_int,
    0xa73a as libc::c_int,
    1 as libc::c_int,
    0xa73c as libc::c_int,
    1 as libc::c_int,
    0xa73e as libc::c_int,
    1 as libc::c_int,
    0xa740 as libc::c_int,
    1 as libc::c_int,
    0xa742 as libc::c_int,
    1 as libc::c_int,
    0xa744 as libc::c_int,
    1 as libc::c_int,
    0xa746 as libc::c_int,
    1 as libc::c_int,
    0xa748 as libc::c_int,
    1 as libc::c_int,
    0xa74a as libc::c_int,
    1 as libc::c_int,
    0xa74c as libc::c_int,
    1 as libc::c_int,
    0xa74e as libc::c_int,
    1 as libc::c_int,
    0xa750 as libc::c_int,
    1 as libc::c_int,
    0xa752 as libc::c_int,
    1 as libc::c_int,
    0xa754 as libc::c_int,
    1 as libc::c_int,
    0xa756 as libc::c_int,
    1 as libc::c_int,
    0xa758 as libc::c_int,
    1 as libc::c_int,
    0xa75a as libc::c_int,
    1 as libc::c_int,
    0xa75c as libc::c_int,
    1 as libc::c_int,
    0xa75e as libc::c_int,
    1 as libc::c_int,
    0xa760 as libc::c_int,
    1 as libc::c_int,
    0xa762 as libc::c_int,
    1 as libc::c_int,
    0xa764 as libc::c_int,
    1 as libc::c_int,
    0xa766 as libc::c_int,
    1 as libc::c_int,
    0xa768 as libc::c_int,
    1 as libc::c_int,
    0xa76a as libc::c_int,
    1 as libc::c_int,
    0xa76c as libc::c_int,
    1 as libc::c_int,
    0xa76e as libc::c_int,
    1 as libc::c_int,
    0xa779 as libc::c_int,
    1 as libc::c_int,
    0xa77b as libc::c_int,
    1 as libc::c_int,
    0xa77d as libc::c_int,
    -(35332 as libc::c_int),
    0xa77e as libc::c_int,
    1 as libc::c_int,
    0xa780 as libc::c_int,
    1 as libc::c_int,
    0xa782 as libc::c_int,
    1 as libc::c_int,
    0xa784 as libc::c_int,
    1 as libc::c_int,
    0xa786 as libc::c_int,
    1 as libc::c_int,
    0xa78b as libc::c_int,
    1 as libc::c_int,
    0xa78d as libc::c_int,
    -(42280 as libc::c_int),
    0xa790 as libc::c_int,
    1 as libc::c_int,
    0xa792 as libc::c_int,
    1 as libc::c_int,
    0xa796 as libc::c_int,
    1 as libc::c_int,
    0xa798 as libc::c_int,
    1 as libc::c_int,
    0xa79a as libc::c_int,
    1 as libc::c_int,
    0xa79c as libc::c_int,
    1 as libc::c_int,
    0xa79e as libc::c_int,
    1 as libc::c_int,
    0xa7a0 as libc::c_int,
    1 as libc::c_int,
    0xa7a2 as libc::c_int,
    1 as libc::c_int,
    0xa7a4 as libc::c_int,
    1 as libc::c_int,
    0xa7a6 as libc::c_int,
    1 as libc::c_int,
    0xa7a8 as libc::c_int,
    1 as libc::c_int,
    0xa7aa as libc::c_int,
    -(42308 as libc::c_int),
    0xa7ab as libc::c_int,
    -(42319 as libc::c_int),
    0xa7ac as libc::c_int,
    -(42315 as libc::c_int),
    0xa7ad as libc::c_int,
    -(42305 as libc::c_int),
    0xa7ae as libc::c_int,
    -(42308 as libc::c_int),
    0xa7b0 as libc::c_int,
    -(42258 as libc::c_int),
    0xa7b1 as libc::c_int,
    -(42282 as libc::c_int),
    0xa7b2 as libc::c_int,
    -(42261 as libc::c_int),
    0xa7b3 as libc::c_int,
    928 as libc::c_int,
    0xa7b4 as libc::c_int,
    1 as libc::c_int,
    0xa7b6 as libc::c_int,
    1 as libc::c_int,
    0xa7b8 as libc::c_int,
    1 as libc::c_int,
    0xa7ba as libc::c_int,
    1 as libc::c_int,
    0xa7bc as libc::c_int,
    1 as libc::c_int,
    0xa7be as libc::c_int,
    1 as libc::c_int,
    0xa7c0 as libc::c_int,
    1 as libc::c_int,
    0xa7c2 as libc::c_int,
    1 as libc::c_int,
    0xa7c4 as libc::c_int,
    -(48 as libc::c_int),
    0xa7c5 as libc::c_int,
    -(42307 as libc::c_int),
    0xa7c6 as libc::c_int,
    -(35384 as libc::c_int),
    0xa7c7 as libc::c_int,
    1 as libc::c_int,
    0xa7c9 as libc::c_int,
    1 as libc::c_int,
    0xa7cb as libc::c_int,
    -(42343 as libc::c_int),
    0xa7cc as libc::c_int,
    1 as libc::c_int,
    0xa7d0 as libc::c_int,
    1 as libc::c_int,
    0xa7d6 as libc::c_int,
    1 as libc::c_int,
    0xa7d8 as libc::c_int,
    1 as libc::c_int,
    0xa7da as libc::c_int,
    1 as libc::c_int,
    0xa7dc as libc::c_int,
    -(42561 as libc::c_int),
    0xa7f5 as libc::c_int,
    1 as libc::c_int,
];
static mut ucd_toupper2: [Rune; 159] = [
    0x61 as libc::c_int,
    0x7a as libc::c_int,
    -(32 as libc::c_int),
    0xe0 as libc::c_int,
    0xf6 as libc::c_int,
    -(32 as libc::c_int),
    0xf8 as libc::c_int,
    0xfe as libc::c_int,
    -(32 as libc::c_int),
    0x23f as libc::c_int,
    0x240 as libc::c_int,
    10815 as libc::c_int,
    0x256 as libc::c_int,
    0x257 as libc::c_int,
    -(205 as libc::c_int),
    0x28a as libc::c_int,
    0x28b as libc::c_int,
    -(217 as libc::c_int),
    0x37b as libc::c_int,
    0x37d as libc::c_int,
    130 as libc::c_int,
    0x3ad as libc::c_int,
    0x3af as libc::c_int,
    -(37 as libc::c_int),
    0x3b1 as libc::c_int,
    0x3c1 as libc::c_int,
    -(32 as libc::c_int),
    0x3c3 as libc::c_int,
    0x3cb as libc::c_int,
    -(32 as libc::c_int),
    0x3cd as libc::c_int,
    0x3ce as libc::c_int,
    -(63 as libc::c_int),
    0x430 as libc::c_int,
    0x44f as libc::c_int,
    -(32 as libc::c_int),
    0x450 as libc::c_int,
    0x45f as libc::c_int,
    -(80 as libc::c_int),
    0x561 as libc::c_int,
    0x586 as libc::c_int,
    -(48 as libc::c_int),
    0x10d0 as libc::c_int,
    0x10fa as libc::c_int,
    3008 as libc::c_int,
    0x10fd as libc::c_int,
    0x10ff as libc::c_int,
    3008 as libc::c_int,
    0x13f8 as libc::c_int,
    0x13fd as libc::c_int,
    -(8 as libc::c_int),
    0x1c83 as libc::c_int,
    0x1c84 as libc::c_int,
    -(6242 as libc::c_int),
    0x1f00 as libc::c_int,
    0x1f07 as libc::c_int,
    8 as libc::c_int,
    0x1f10 as libc::c_int,
    0x1f15 as libc::c_int,
    8 as libc::c_int,
    0x1f20 as libc::c_int,
    0x1f27 as libc::c_int,
    8 as libc::c_int,
    0x1f30 as libc::c_int,
    0x1f37 as libc::c_int,
    8 as libc::c_int,
    0x1f40 as libc::c_int,
    0x1f45 as libc::c_int,
    8 as libc::c_int,
    0x1f60 as libc::c_int,
    0x1f67 as libc::c_int,
    8 as libc::c_int,
    0x1f70 as libc::c_int,
    0x1f71 as libc::c_int,
    74 as libc::c_int,
    0x1f72 as libc::c_int,
    0x1f75 as libc::c_int,
    86 as libc::c_int,
    0x1f76 as libc::c_int,
    0x1f77 as libc::c_int,
    100 as libc::c_int,
    0x1f78 as libc::c_int,
    0x1f79 as libc::c_int,
    128 as libc::c_int,
    0x1f7a as libc::c_int,
    0x1f7b as libc::c_int,
    112 as libc::c_int,
    0x1f7c as libc::c_int,
    0x1f7d as libc::c_int,
    126 as libc::c_int,
    0x1f80 as libc::c_int,
    0x1f87 as libc::c_int,
    8 as libc::c_int,
    0x1f90 as libc::c_int,
    0x1f97 as libc::c_int,
    8 as libc::c_int,
    0x1fa0 as libc::c_int,
    0x1fa7 as libc::c_int,
    8 as libc::c_int,
    0x1fb0 as libc::c_int,
    0x1fb1 as libc::c_int,
    8 as libc::c_int,
    0x1fd0 as libc::c_int,
    0x1fd1 as libc::c_int,
    8 as libc::c_int,
    0x1fe0 as libc::c_int,
    0x1fe1 as libc::c_int,
    8 as libc::c_int,
    0x2170 as libc::c_int,
    0x217f as libc::c_int,
    -(16 as libc::c_int),
    0x24d0 as libc::c_int,
    0x24e9 as libc::c_int,
    -(26 as libc::c_int),
    0x2c30 as libc::c_int,
    0x2c5f as libc::c_int,
    -(48 as libc::c_int),
    0x2d00 as libc::c_int,
    0x2d25 as libc::c_int,
    -(7264 as libc::c_int),
    0xab70 as libc::c_int,
    0xabbf as libc::c_int,
    -(38864 as libc::c_int),
    0xff41 as libc::c_int,
    0xff5a as libc::c_int,
    -(32 as libc::c_int),
    0x10428 as libc::c_int,
    0x1044f as libc::c_int,
    -(40 as libc::c_int),
    0x104d8 as libc::c_int,
    0x104fb as libc::c_int,
    -(40 as libc::c_int),
    0x10597 as libc::c_int,
    0x105a1 as libc::c_int,
    -(39 as libc::c_int),
    0x105a3 as libc::c_int,
    0x105b1 as libc::c_int,
    -(39 as libc::c_int),
    0x105b3 as libc::c_int,
    0x105b9 as libc::c_int,
    -(39 as libc::c_int),
    0x105bb as libc::c_int,
    0x105bc as libc::c_int,
    -(39 as libc::c_int),
    0x10cc0 as libc::c_int,
    0x10cf2 as libc::c_int,
    -(64 as libc::c_int),
    0x10d70 as libc::c_int,
    0x10d85 as libc::c_int,
    -(32 as libc::c_int),
    0x118c0 as libc::c_int,
    0x118df as libc::c_int,
    -(32 as libc::c_int),
    0x16e60 as libc::c_int,
    0x16e7f as libc::c_int,
    -(32 as libc::c_int),
    0x1e922 as libc::c_int,
    0x1e943 as libc::c_int,
    -(34 as libc::c_int),
];
static mut ucd_toupper1: [Rune; 1274] = [
    0xb5 as libc::c_int,
    743 as libc::c_int,
    0xff as libc::c_int,
    121 as libc::c_int,
    0x101 as libc::c_int,
    -(1 as libc::c_int),
    0x103 as libc::c_int,
    -(1 as libc::c_int),
    0x105 as libc::c_int,
    -(1 as libc::c_int),
    0x107 as libc::c_int,
    -(1 as libc::c_int),
    0x109 as libc::c_int,
    -(1 as libc::c_int),
    0x10b as libc::c_int,
    -(1 as libc::c_int),
    0x10d as libc::c_int,
    -(1 as libc::c_int),
    0x10f as libc::c_int,
    -(1 as libc::c_int),
    0x111 as libc::c_int,
    -(1 as libc::c_int),
    0x113 as libc::c_int,
    -(1 as libc::c_int),
    0x115 as libc::c_int,
    -(1 as libc::c_int),
    0x117 as libc::c_int,
    -(1 as libc::c_int),
    0x119 as libc::c_int,
    -(1 as libc::c_int),
    0x11b as libc::c_int,
    -(1 as libc::c_int),
    0x11d as libc::c_int,
    -(1 as libc::c_int),
    0x11f as libc::c_int,
    -(1 as libc::c_int),
    0x121 as libc::c_int,
    -(1 as libc::c_int),
    0x123 as libc::c_int,
    -(1 as libc::c_int),
    0x125 as libc::c_int,
    -(1 as libc::c_int),
    0x127 as libc::c_int,
    -(1 as libc::c_int),
    0x129 as libc::c_int,
    -(1 as libc::c_int),
    0x12b as libc::c_int,
    -(1 as libc::c_int),
    0x12d as libc::c_int,
    -(1 as libc::c_int),
    0x12f as libc::c_int,
    -(1 as libc::c_int),
    0x131 as libc::c_int,
    -(232 as libc::c_int),
    0x133 as libc::c_int,
    -(1 as libc::c_int),
    0x135 as libc::c_int,
    -(1 as libc::c_int),
    0x137 as libc::c_int,
    -(1 as libc::c_int),
    0x13a as libc::c_int,
    -(1 as libc::c_int),
    0x13c as libc::c_int,
    -(1 as libc::c_int),
    0x13e as libc::c_int,
    -(1 as libc::c_int),
    0x140 as libc::c_int,
    -(1 as libc::c_int),
    0x142 as libc::c_int,
    -(1 as libc::c_int),
    0x144 as libc::c_int,
    -(1 as libc::c_int),
    0x146 as libc::c_int,
    -(1 as libc::c_int),
    0x148 as libc::c_int,
    -(1 as libc::c_int),
    0x14b as libc::c_int,
    -(1 as libc::c_int),
    0x14d as libc::c_int,
    -(1 as libc::c_int),
    0x14f as libc::c_int,
    -(1 as libc::c_int),
    0x151 as libc::c_int,
    -(1 as libc::c_int),
    0x153 as libc::c_int,
    -(1 as libc::c_int),
    0x155 as libc::c_int,
    -(1 as libc::c_int),
    0x157 as libc::c_int,
    -(1 as libc::c_int),
    0x159 as libc::c_int,
    -(1 as libc::c_int),
    0x15b as libc::c_int,
    -(1 as libc::c_int),
    0x15d as libc::c_int,
    -(1 as libc::c_int),
    0x15f as libc::c_int,
    -(1 as libc::c_int),
    0x161 as libc::c_int,
    -(1 as libc::c_int),
    0x163 as libc::c_int,
    -(1 as libc::c_int),
    0x165 as libc::c_int,
    -(1 as libc::c_int),
    0x167 as libc::c_int,
    -(1 as libc::c_int),
    0x169 as libc::c_int,
    -(1 as libc::c_int),
    0x16b as libc::c_int,
    -(1 as libc::c_int),
    0x16d as libc::c_int,
    -(1 as libc::c_int),
    0x16f as libc::c_int,
    -(1 as libc::c_int),
    0x171 as libc::c_int,
    -(1 as libc::c_int),
    0x173 as libc::c_int,
    -(1 as libc::c_int),
    0x175 as libc::c_int,
    -(1 as libc::c_int),
    0x177 as libc::c_int,
    -(1 as libc::c_int),
    0x17a as libc::c_int,
    -(1 as libc::c_int),
    0x17c as libc::c_int,
    -(1 as libc::c_int),
    0x17e as libc::c_int,
    -(1 as libc::c_int),
    0x17f as libc::c_int,
    -(300 as libc::c_int),
    0x180 as libc::c_int,
    195 as libc::c_int,
    0x183 as libc::c_int,
    -(1 as libc::c_int),
    0x185 as libc::c_int,
    -(1 as libc::c_int),
    0x188 as libc::c_int,
    -(1 as libc::c_int),
    0x18c as libc::c_int,
    -(1 as libc::c_int),
    0x192 as libc::c_int,
    -(1 as libc::c_int),
    0x195 as libc::c_int,
    97 as libc::c_int,
    0x199 as libc::c_int,
    -(1 as libc::c_int),
    0x19a as libc::c_int,
    163 as libc::c_int,
    0x19b as libc::c_int,
    42561 as libc::c_int,
    0x19e as libc::c_int,
    130 as libc::c_int,
    0x1a1 as libc::c_int,
    -(1 as libc::c_int),
    0x1a3 as libc::c_int,
    -(1 as libc::c_int),
    0x1a5 as libc::c_int,
    -(1 as libc::c_int),
    0x1a8 as libc::c_int,
    -(1 as libc::c_int),
    0x1ad as libc::c_int,
    -(1 as libc::c_int),
    0x1b0 as libc::c_int,
    -(1 as libc::c_int),
    0x1b4 as libc::c_int,
    -(1 as libc::c_int),
    0x1b6 as libc::c_int,
    -(1 as libc::c_int),
    0x1b9 as libc::c_int,
    -(1 as libc::c_int),
    0x1bd as libc::c_int,
    -(1 as libc::c_int),
    0x1bf as libc::c_int,
    56 as libc::c_int,
    0x1c5 as libc::c_int,
    -(1 as libc::c_int),
    0x1c6 as libc::c_int,
    -(2 as libc::c_int),
    0x1c8 as libc::c_int,
    -(1 as libc::c_int),
    0x1c9 as libc::c_int,
    -(2 as libc::c_int),
    0x1cb as libc::c_int,
    -(1 as libc::c_int),
    0x1cc as libc::c_int,
    -(2 as libc::c_int),
    0x1ce as libc::c_int,
    -(1 as libc::c_int),
    0x1d0 as libc::c_int,
    -(1 as libc::c_int),
    0x1d2 as libc::c_int,
    -(1 as libc::c_int),
    0x1d4 as libc::c_int,
    -(1 as libc::c_int),
    0x1d6 as libc::c_int,
    -(1 as libc::c_int),
    0x1d8 as libc::c_int,
    -(1 as libc::c_int),
    0x1da as libc::c_int,
    -(1 as libc::c_int),
    0x1dc as libc::c_int,
    -(1 as libc::c_int),
    0x1dd as libc::c_int,
    -(79 as libc::c_int),
    0x1df as libc::c_int,
    -(1 as libc::c_int),
    0x1e1 as libc::c_int,
    -(1 as libc::c_int),
    0x1e3 as libc::c_int,
    -(1 as libc::c_int),
    0x1e5 as libc::c_int,
    -(1 as libc::c_int),
    0x1e7 as libc::c_int,
    -(1 as libc::c_int),
    0x1e9 as libc::c_int,
    -(1 as libc::c_int),
    0x1eb as libc::c_int,
    -(1 as libc::c_int),
    0x1ed as libc::c_int,
    -(1 as libc::c_int),
    0x1ef as libc::c_int,
    -(1 as libc::c_int),
    0x1f2 as libc::c_int,
    -(1 as libc::c_int),
    0x1f3 as libc::c_int,
    -(2 as libc::c_int),
    0x1f5 as libc::c_int,
    -(1 as libc::c_int),
    0x1f9 as libc::c_int,
    -(1 as libc::c_int),
    0x1fb as libc::c_int,
    -(1 as libc::c_int),
    0x1fd as libc::c_int,
    -(1 as libc::c_int),
    0x1ff as libc::c_int,
    -(1 as libc::c_int),
    0x201 as libc::c_int,
    -(1 as libc::c_int),
    0x203 as libc::c_int,
    -(1 as libc::c_int),
    0x205 as libc::c_int,
    -(1 as libc::c_int),
    0x207 as libc::c_int,
    -(1 as libc::c_int),
    0x209 as libc::c_int,
    -(1 as libc::c_int),
    0x20b as libc::c_int,
    -(1 as libc::c_int),
    0x20d as libc::c_int,
    -(1 as libc::c_int),
    0x20f as libc::c_int,
    -(1 as libc::c_int),
    0x211 as libc::c_int,
    -(1 as libc::c_int),
    0x213 as libc::c_int,
    -(1 as libc::c_int),
    0x215 as libc::c_int,
    -(1 as libc::c_int),
    0x217 as libc::c_int,
    -(1 as libc::c_int),
    0x219 as libc::c_int,
    -(1 as libc::c_int),
    0x21b as libc::c_int,
    -(1 as libc::c_int),
    0x21d as libc::c_int,
    -(1 as libc::c_int),
    0x21f as libc::c_int,
    -(1 as libc::c_int),
    0x223 as libc::c_int,
    -(1 as libc::c_int),
    0x225 as libc::c_int,
    -(1 as libc::c_int),
    0x227 as libc::c_int,
    -(1 as libc::c_int),
    0x229 as libc::c_int,
    -(1 as libc::c_int),
    0x22b as libc::c_int,
    -(1 as libc::c_int),
    0x22d as libc::c_int,
    -(1 as libc::c_int),
    0x22f as libc::c_int,
    -(1 as libc::c_int),
    0x231 as libc::c_int,
    -(1 as libc::c_int),
    0x233 as libc::c_int,
    -(1 as libc::c_int),
    0x23c as libc::c_int,
    -(1 as libc::c_int),
    0x242 as libc::c_int,
    -(1 as libc::c_int),
    0x247 as libc::c_int,
    -(1 as libc::c_int),
    0x249 as libc::c_int,
    -(1 as libc::c_int),
    0x24b as libc::c_int,
    -(1 as libc::c_int),
    0x24d as libc::c_int,
    -(1 as libc::c_int),
    0x24f as libc::c_int,
    -(1 as libc::c_int),
    0x250 as libc::c_int,
    10783 as libc::c_int,
    0x251 as libc::c_int,
    10780 as libc::c_int,
    0x252 as libc::c_int,
    10782 as libc::c_int,
    0x253 as libc::c_int,
    -(210 as libc::c_int),
    0x254 as libc::c_int,
    -(206 as libc::c_int),
    0x259 as libc::c_int,
    -(202 as libc::c_int),
    0x25b as libc::c_int,
    -(203 as libc::c_int),
    0x25c as libc::c_int,
    42319 as libc::c_int,
    0x260 as libc::c_int,
    -(205 as libc::c_int),
    0x261 as libc::c_int,
    42315 as libc::c_int,
    0x263 as libc::c_int,
    -(207 as libc::c_int),
    0x264 as libc::c_int,
    42343 as libc::c_int,
    0x265 as libc::c_int,
    42280 as libc::c_int,
    0x266 as libc::c_int,
    42308 as libc::c_int,
    0x268 as libc::c_int,
    -(209 as libc::c_int),
    0x269 as libc::c_int,
    -(211 as libc::c_int),
    0x26a as libc::c_int,
    42308 as libc::c_int,
    0x26b as libc::c_int,
    10743 as libc::c_int,
    0x26c as libc::c_int,
    42305 as libc::c_int,
    0x26f as libc::c_int,
    -(211 as libc::c_int),
    0x271 as libc::c_int,
    10749 as libc::c_int,
    0x272 as libc::c_int,
    -(213 as libc::c_int),
    0x275 as libc::c_int,
    -(214 as libc::c_int),
    0x27d as libc::c_int,
    10727 as libc::c_int,
    0x280 as libc::c_int,
    -(218 as libc::c_int),
    0x282 as libc::c_int,
    42307 as libc::c_int,
    0x283 as libc::c_int,
    -(218 as libc::c_int),
    0x287 as libc::c_int,
    42282 as libc::c_int,
    0x288 as libc::c_int,
    -(218 as libc::c_int),
    0x289 as libc::c_int,
    -(69 as libc::c_int),
    0x28c as libc::c_int,
    -(71 as libc::c_int),
    0x292 as libc::c_int,
    -(219 as libc::c_int),
    0x29d as libc::c_int,
    42261 as libc::c_int,
    0x29e as libc::c_int,
    42258 as libc::c_int,
    0x345 as libc::c_int,
    84 as libc::c_int,
    0x371 as libc::c_int,
    -(1 as libc::c_int),
    0x373 as libc::c_int,
    -(1 as libc::c_int),
    0x377 as libc::c_int,
    -(1 as libc::c_int),
    0x3ac as libc::c_int,
    -(38 as libc::c_int),
    0x3c2 as libc::c_int,
    -(31 as libc::c_int),
    0x3cc as libc::c_int,
    -(64 as libc::c_int),
    0x3d0 as libc::c_int,
    -(62 as libc::c_int),
    0x3d1 as libc::c_int,
    -(57 as libc::c_int),
    0x3d5 as libc::c_int,
    -(47 as libc::c_int),
    0x3d6 as libc::c_int,
    -(54 as libc::c_int),
    0x3d7 as libc::c_int,
    -(8 as libc::c_int),
    0x3d9 as libc::c_int,
    -(1 as libc::c_int),
    0x3db as libc::c_int,
    -(1 as libc::c_int),
    0x3dd as libc::c_int,
    -(1 as libc::c_int),
    0x3df as libc::c_int,
    -(1 as libc::c_int),
    0x3e1 as libc::c_int,
    -(1 as libc::c_int),
    0x3e3 as libc::c_int,
    -(1 as libc::c_int),
    0x3e5 as libc::c_int,
    -(1 as libc::c_int),
    0x3e7 as libc::c_int,
    -(1 as libc::c_int),
    0x3e9 as libc::c_int,
    -(1 as libc::c_int),
    0x3eb as libc::c_int,
    -(1 as libc::c_int),
    0x3ed as libc::c_int,
    -(1 as libc::c_int),
    0x3ef as libc::c_int,
    -(1 as libc::c_int),
    0x3f0 as libc::c_int,
    -(86 as libc::c_int),
    0x3f1 as libc::c_int,
    -(80 as libc::c_int),
    0x3f2 as libc::c_int,
    7 as libc::c_int,
    0x3f3 as libc::c_int,
    -(116 as libc::c_int),
    0x3f5 as libc::c_int,
    -(96 as libc::c_int),
    0x3f8 as libc::c_int,
    -(1 as libc::c_int),
    0x3fb as libc::c_int,
    -(1 as libc::c_int),
    0x461 as libc::c_int,
    -(1 as libc::c_int),
    0x463 as libc::c_int,
    -(1 as libc::c_int),
    0x465 as libc::c_int,
    -(1 as libc::c_int),
    0x467 as libc::c_int,
    -(1 as libc::c_int),
    0x469 as libc::c_int,
    -(1 as libc::c_int),
    0x46b as libc::c_int,
    -(1 as libc::c_int),
    0x46d as libc::c_int,
    -(1 as libc::c_int),
    0x46f as libc::c_int,
    -(1 as libc::c_int),
    0x471 as libc::c_int,
    -(1 as libc::c_int),
    0x473 as libc::c_int,
    -(1 as libc::c_int),
    0x475 as libc::c_int,
    -(1 as libc::c_int),
    0x477 as libc::c_int,
    -(1 as libc::c_int),
    0x479 as libc::c_int,
    -(1 as libc::c_int),
    0x47b as libc::c_int,
    -(1 as libc::c_int),
    0x47d as libc::c_int,
    -(1 as libc::c_int),
    0x47f as libc::c_int,
    -(1 as libc::c_int),
    0x481 as libc::c_int,
    -(1 as libc::c_int),
    0x48b as libc::c_int,
    -(1 as libc::c_int),
    0x48d as libc::c_int,
    -(1 as libc::c_int),
    0x48f as libc::c_int,
    -(1 as libc::c_int),
    0x491 as libc::c_int,
    -(1 as libc::c_int),
    0x493 as libc::c_int,
    -(1 as libc::c_int),
    0x495 as libc::c_int,
    -(1 as libc::c_int),
    0x497 as libc::c_int,
    -(1 as libc::c_int),
    0x499 as libc::c_int,
    -(1 as libc::c_int),
    0x49b as libc::c_int,
    -(1 as libc::c_int),
    0x49d as libc::c_int,
    -(1 as libc::c_int),
    0x49f as libc::c_int,
    -(1 as libc::c_int),
    0x4a1 as libc::c_int,
    -(1 as libc::c_int),
    0x4a3 as libc::c_int,
    -(1 as libc::c_int),
    0x4a5 as libc::c_int,
    -(1 as libc::c_int),
    0x4a7 as libc::c_int,
    -(1 as libc::c_int),
    0x4a9 as libc::c_int,
    -(1 as libc::c_int),
    0x4ab as libc::c_int,
    -(1 as libc::c_int),
    0x4ad as libc::c_int,
    -(1 as libc::c_int),
    0x4af as libc::c_int,
    -(1 as libc::c_int),
    0x4b1 as libc::c_int,
    -(1 as libc::c_int),
    0x4b3 as libc::c_int,
    -(1 as libc::c_int),
    0x4b5 as libc::c_int,
    -(1 as libc::c_int),
    0x4b7 as libc::c_int,
    -(1 as libc::c_int),
    0x4b9 as libc::c_int,
    -(1 as libc::c_int),
    0x4bb as libc::c_int,
    -(1 as libc::c_int),
    0x4bd as libc::c_int,
    -(1 as libc::c_int),
    0x4bf as libc::c_int,
    -(1 as libc::c_int),
    0x4c2 as libc::c_int,
    -(1 as libc::c_int),
    0x4c4 as libc::c_int,
    -(1 as libc::c_int),
    0x4c6 as libc::c_int,
    -(1 as libc::c_int),
    0x4c8 as libc::c_int,
    -(1 as libc::c_int),
    0x4ca as libc::c_int,
    -(1 as libc::c_int),
    0x4cc as libc::c_int,
    -(1 as libc::c_int),
    0x4ce as libc::c_int,
    -(1 as libc::c_int),
    0x4cf as libc::c_int,
    -(15 as libc::c_int),
    0x4d1 as libc::c_int,
    -(1 as libc::c_int),
    0x4d3 as libc::c_int,
    -(1 as libc::c_int),
    0x4d5 as libc::c_int,
    -(1 as libc::c_int),
    0x4d7 as libc::c_int,
    -(1 as libc::c_int),
    0x4d9 as libc::c_int,
    -(1 as libc::c_int),
    0x4db as libc::c_int,
    -(1 as libc::c_int),
    0x4dd as libc::c_int,
    -(1 as libc::c_int),
    0x4df as libc::c_int,
    -(1 as libc::c_int),
    0x4e1 as libc::c_int,
    -(1 as libc::c_int),
    0x4e3 as libc::c_int,
    -(1 as libc::c_int),
    0x4e5 as libc::c_int,
    -(1 as libc::c_int),
    0x4e7 as libc::c_int,
    -(1 as libc::c_int),
    0x4e9 as libc::c_int,
    -(1 as libc::c_int),
    0x4eb as libc::c_int,
    -(1 as libc::c_int),
    0x4ed as libc::c_int,
    -(1 as libc::c_int),
    0x4ef as libc::c_int,
    -(1 as libc::c_int),
    0x4f1 as libc::c_int,
    -(1 as libc::c_int),
    0x4f3 as libc::c_int,
    -(1 as libc::c_int),
    0x4f5 as libc::c_int,
    -(1 as libc::c_int),
    0x4f7 as libc::c_int,
    -(1 as libc::c_int),
    0x4f9 as libc::c_int,
    -(1 as libc::c_int),
    0x4fb as libc::c_int,
    -(1 as libc::c_int),
    0x4fd as libc::c_int,
    -(1 as libc::c_int),
    0x4ff as libc::c_int,
    -(1 as libc::c_int),
    0x501 as libc::c_int,
    -(1 as libc::c_int),
    0x503 as libc::c_int,
    -(1 as libc::c_int),
    0x505 as libc::c_int,
    -(1 as libc::c_int),
    0x507 as libc::c_int,
    -(1 as libc::c_int),
    0x509 as libc::c_int,
    -(1 as libc::c_int),
    0x50b as libc::c_int,
    -(1 as libc::c_int),
    0x50d as libc::c_int,
    -(1 as libc::c_int),
    0x50f as libc::c_int,
    -(1 as libc::c_int),
    0x511 as libc::c_int,
    -(1 as libc::c_int),
    0x513 as libc::c_int,
    -(1 as libc::c_int),
    0x515 as libc::c_int,
    -(1 as libc::c_int),
    0x517 as libc::c_int,
    -(1 as libc::c_int),
    0x519 as libc::c_int,
    -(1 as libc::c_int),
    0x51b as libc::c_int,
    -(1 as libc::c_int),
    0x51d as libc::c_int,
    -(1 as libc::c_int),
    0x51f as libc::c_int,
    -(1 as libc::c_int),
    0x521 as libc::c_int,
    -(1 as libc::c_int),
    0x523 as libc::c_int,
    -(1 as libc::c_int),
    0x525 as libc::c_int,
    -(1 as libc::c_int),
    0x527 as libc::c_int,
    -(1 as libc::c_int),
    0x529 as libc::c_int,
    -(1 as libc::c_int),
    0x52b as libc::c_int,
    -(1 as libc::c_int),
    0x52d as libc::c_int,
    -(1 as libc::c_int),
    0x52f as libc::c_int,
    -(1 as libc::c_int),
    0x1c80 as libc::c_int,
    -(6254 as libc::c_int),
    0x1c81 as libc::c_int,
    -(6253 as libc::c_int),
    0x1c82 as libc::c_int,
    -(6244 as libc::c_int),
    0x1c85 as libc::c_int,
    -(6243 as libc::c_int),
    0x1c86 as libc::c_int,
    -(6236 as libc::c_int),
    0x1c87 as libc::c_int,
    -(6181 as libc::c_int),
    0x1c88 as libc::c_int,
    35266 as libc::c_int,
    0x1c8a as libc::c_int,
    -(1 as libc::c_int),
    0x1d79 as libc::c_int,
    35332 as libc::c_int,
    0x1d7d as libc::c_int,
    3814 as libc::c_int,
    0x1d8e as libc::c_int,
    35384 as libc::c_int,
    0x1e01 as libc::c_int,
    -(1 as libc::c_int),
    0x1e03 as libc::c_int,
    -(1 as libc::c_int),
    0x1e05 as libc::c_int,
    -(1 as libc::c_int),
    0x1e07 as libc::c_int,
    -(1 as libc::c_int),
    0x1e09 as libc::c_int,
    -(1 as libc::c_int),
    0x1e0b as libc::c_int,
    -(1 as libc::c_int),
    0x1e0d as libc::c_int,
    -(1 as libc::c_int),
    0x1e0f as libc::c_int,
    -(1 as libc::c_int),
    0x1e11 as libc::c_int,
    -(1 as libc::c_int),
    0x1e13 as libc::c_int,
    -(1 as libc::c_int),
    0x1e15 as libc::c_int,
    -(1 as libc::c_int),
    0x1e17 as libc::c_int,
    -(1 as libc::c_int),
    0x1e19 as libc::c_int,
    -(1 as libc::c_int),
    0x1e1b as libc::c_int,
    -(1 as libc::c_int),
    0x1e1d as libc::c_int,
    -(1 as libc::c_int),
    0x1e1f as libc::c_int,
    -(1 as libc::c_int),
    0x1e21 as libc::c_int,
    -(1 as libc::c_int),
    0x1e23 as libc::c_int,
    -(1 as libc::c_int),
    0x1e25 as libc::c_int,
    -(1 as libc::c_int),
    0x1e27 as libc::c_int,
    -(1 as libc::c_int),
    0x1e29 as libc::c_int,
    -(1 as libc::c_int),
    0x1e2b as libc::c_int,
    -(1 as libc::c_int),
    0x1e2d as libc::c_int,
    -(1 as libc::c_int),
    0x1e2f as libc::c_int,
    -(1 as libc::c_int),
    0x1e31 as libc::c_int,
    -(1 as libc::c_int),
    0x1e33 as libc::c_int,
    -(1 as libc::c_int),
    0x1e35 as libc::c_int,
    -(1 as libc::c_int),
    0x1e37 as libc::c_int,
    -(1 as libc::c_int),
    0x1e39 as libc::c_int,
    -(1 as libc::c_int),
    0x1e3b as libc::c_int,
    -(1 as libc::c_int),
    0x1e3d as libc::c_int,
    -(1 as libc::c_int),
    0x1e3f as libc::c_int,
    -(1 as libc::c_int),
    0x1e41 as libc::c_int,
    -(1 as libc::c_int),
    0x1e43 as libc::c_int,
    -(1 as libc::c_int),
    0x1e45 as libc::c_int,
    -(1 as libc::c_int),
    0x1e47 as libc::c_int,
    -(1 as libc::c_int),
    0x1e49 as libc::c_int,
    -(1 as libc::c_int),
    0x1e4b as libc::c_int,
    -(1 as libc::c_int),
    0x1e4d as libc::c_int,
    -(1 as libc::c_int),
    0x1e4f as libc::c_int,
    -(1 as libc::c_int),
    0x1e51 as libc::c_int,
    -(1 as libc::c_int),
    0x1e53 as libc::c_int,
    -(1 as libc::c_int),
    0x1e55 as libc::c_int,
    -(1 as libc::c_int),
    0x1e57 as libc::c_int,
    -(1 as libc::c_int),
    0x1e59 as libc::c_int,
    -(1 as libc::c_int),
    0x1e5b as libc::c_int,
    -(1 as libc::c_int),
    0x1e5d as libc::c_int,
    -(1 as libc::c_int),
    0x1e5f as libc::c_int,
    -(1 as libc::c_int),
    0x1e61 as libc::c_int,
    -(1 as libc::c_int),
    0x1e63 as libc::c_int,
    -(1 as libc::c_int),
    0x1e65 as libc::c_int,
    -(1 as libc::c_int),
    0x1e67 as libc::c_int,
    -(1 as libc::c_int),
    0x1e69 as libc::c_int,
    -(1 as libc::c_int),
    0x1e6b as libc::c_int,
    -(1 as libc::c_int),
    0x1e6d as libc::c_int,
    -(1 as libc::c_int),
    0x1e6f as libc::c_int,
    -(1 as libc::c_int),
    0x1e71 as libc::c_int,
    -(1 as libc::c_int),
    0x1e73 as libc::c_int,
    -(1 as libc::c_int),
    0x1e75 as libc::c_int,
    -(1 as libc::c_int),
    0x1e77 as libc::c_int,
    -(1 as libc::c_int),
    0x1e79 as libc::c_int,
    -(1 as libc::c_int),
    0x1e7b as libc::c_int,
    -(1 as libc::c_int),
    0x1e7d as libc::c_int,
    -(1 as libc::c_int),
    0x1e7f as libc::c_int,
    -(1 as libc::c_int),
    0x1e81 as libc::c_int,
    -(1 as libc::c_int),
    0x1e83 as libc::c_int,
    -(1 as libc::c_int),
    0x1e85 as libc::c_int,
    -(1 as libc::c_int),
    0x1e87 as libc::c_int,
    -(1 as libc::c_int),
    0x1e89 as libc::c_int,
    -(1 as libc::c_int),
    0x1e8b as libc::c_int,
    -(1 as libc::c_int),
    0x1e8d as libc::c_int,
    -(1 as libc::c_int),
    0x1e8f as libc::c_int,
    -(1 as libc::c_int),
    0x1e91 as libc::c_int,
    -(1 as libc::c_int),
    0x1e93 as libc::c_int,
    -(1 as libc::c_int),
    0x1e95 as libc::c_int,
    -(1 as libc::c_int),
    0x1e9b as libc::c_int,
    -(59 as libc::c_int),
    0x1ea1 as libc::c_int,
    -(1 as libc::c_int),
    0x1ea3 as libc::c_int,
    -(1 as libc::c_int),
    0x1ea5 as libc::c_int,
    -(1 as libc::c_int),
    0x1ea7 as libc::c_int,
    -(1 as libc::c_int),
    0x1ea9 as libc::c_int,
    -(1 as libc::c_int),
    0x1eab as libc::c_int,
    -(1 as libc::c_int),
    0x1ead as libc::c_int,
    -(1 as libc::c_int),
    0x1eaf as libc::c_int,
    -(1 as libc::c_int),
    0x1eb1 as libc::c_int,
    -(1 as libc::c_int),
    0x1eb3 as libc::c_int,
    -(1 as libc::c_int),
    0x1eb5 as libc::c_int,
    -(1 as libc::c_int),
    0x1eb7 as libc::c_int,
    -(1 as libc::c_int),
    0x1eb9 as libc::c_int,
    -(1 as libc::c_int),
    0x1ebb as libc::c_int,
    -(1 as libc::c_int),
    0x1ebd as libc::c_int,
    -(1 as libc::c_int),
    0x1ebf as libc::c_int,
    -(1 as libc::c_int),
    0x1ec1 as libc::c_int,
    -(1 as libc::c_int),
    0x1ec3 as libc::c_int,
    -(1 as libc::c_int),
    0x1ec5 as libc::c_int,
    -(1 as libc::c_int),
    0x1ec7 as libc::c_int,
    -(1 as libc::c_int),
    0x1ec9 as libc::c_int,
    -(1 as libc::c_int),
    0x1ecb as libc::c_int,
    -(1 as libc::c_int),
    0x1ecd as libc::c_int,
    -(1 as libc::c_int),
    0x1ecf as libc::c_int,
    -(1 as libc::c_int),
    0x1ed1 as libc::c_int,
    -(1 as libc::c_int),
    0x1ed3 as libc::c_int,
    -(1 as libc::c_int),
    0x1ed5 as libc::c_int,
    -(1 as libc::c_int),
    0x1ed7 as libc::c_int,
    -(1 as libc::c_int),
    0x1ed9 as libc::c_int,
    -(1 as libc::c_int),
    0x1edb as libc::c_int,
    -(1 as libc::c_int),
    0x1edd as libc::c_int,
    -(1 as libc::c_int),
    0x1edf as libc::c_int,
    -(1 as libc::c_int),
    0x1ee1 as libc::c_int,
    -(1 as libc::c_int),
    0x1ee3 as libc::c_int,
    -(1 as libc::c_int),
    0x1ee5 as libc::c_int,
    -(1 as libc::c_int),
    0x1ee7 as libc::c_int,
    -(1 as libc::c_int),
    0x1ee9 as libc::c_int,
    -(1 as libc::c_int),
    0x1eeb as libc::c_int,
    -(1 as libc::c_int),
    0x1eed as libc::c_int,
    -(1 as libc::c_int),
    0x1eef as libc::c_int,
    -(1 as libc::c_int),
    0x1ef1 as libc::c_int,
    -(1 as libc::c_int),
    0x1ef3 as libc::c_int,
    -(1 as libc::c_int),
    0x1ef5 as libc::c_int,
    -(1 as libc::c_int),
    0x1ef7 as libc::c_int,
    -(1 as libc::c_int),
    0x1ef9 as libc::c_int,
    -(1 as libc::c_int),
    0x1efb as libc::c_int,
    -(1 as libc::c_int),
    0x1efd as libc::c_int,
    -(1 as libc::c_int),
    0x1eff as libc::c_int,
    -(1 as libc::c_int),
    0x1f51 as libc::c_int,
    8 as libc::c_int,
    0x1f53 as libc::c_int,
    8 as libc::c_int,
    0x1f55 as libc::c_int,
    8 as libc::c_int,
    0x1f57 as libc::c_int,
    8 as libc::c_int,
    0x1fb3 as libc::c_int,
    9 as libc::c_int,
    0x1fbe as libc::c_int,
    -(7205 as libc::c_int),
    0x1fc3 as libc::c_int,
    9 as libc::c_int,
    0x1fe5 as libc::c_int,
    7 as libc::c_int,
    0x1ff3 as libc::c_int,
    9 as libc::c_int,
    0x214e as libc::c_int,
    -(28 as libc::c_int),
    0x2184 as libc::c_int,
    -(1 as libc::c_int),
    0x2c61 as libc::c_int,
    -(1 as libc::c_int),
    0x2c65 as libc::c_int,
    -(10795 as libc::c_int),
    0x2c66 as libc::c_int,
    -(10792 as libc::c_int),
    0x2c68 as libc::c_int,
    -(1 as libc::c_int),
    0x2c6a as libc::c_int,
    -(1 as libc::c_int),
    0x2c6c as libc::c_int,
    -(1 as libc::c_int),
    0x2c73 as libc::c_int,
    -(1 as libc::c_int),
    0x2c76 as libc::c_int,
    -(1 as libc::c_int),
    0x2c81 as libc::c_int,
    -(1 as libc::c_int),
    0x2c83 as libc::c_int,
    -(1 as libc::c_int),
    0x2c85 as libc::c_int,
    -(1 as libc::c_int),
    0x2c87 as libc::c_int,
    -(1 as libc::c_int),
    0x2c89 as libc::c_int,
    -(1 as libc::c_int),
    0x2c8b as libc::c_int,
    -(1 as libc::c_int),
    0x2c8d as libc::c_int,
    -(1 as libc::c_int),
    0x2c8f as libc::c_int,
    -(1 as libc::c_int),
    0x2c91 as libc::c_int,
    -(1 as libc::c_int),
    0x2c93 as libc::c_int,
    -(1 as libc::c_int),
    0x2c95 as libc::c_int,
    -(1 as libc::c_int),
    0x2c97 as libc::c_int,
    -(1 as libc::c_int),
    0x2c99 as libc::c_int,
    -(1 as libc::c_int),
    0x2c9b as libc::c_int,
    -(1 as libc::c_int),
    0x2c9d as libc::c_int,
    -(1 as libc::c_int),
    0x2c9f as libc::c_int,
    -(1 as libc::c_int),
    0x2ca1 as libc::c_int,
    -(1 as libc::c_int),
    0x2ca3 as libc::c_int,
    -(1 as libc::c_int),
    0x2ca5 as libc::c_int,
    -(1 as libc::c_int),
    0x2ca7 as libc::c_int,
    -(1 as libc::c_int),
    0x2ca9 as libc::c_int,
    -(1 as libc::c_int),
    0x2cab as libc::c_int,
    -(1 as libc::c_int),
    0x2cad as libc::c_int,
    -(1 as libc::c_int),
    0x2caf as libc::c_int,
    -(1 as libc::c_int),
    0x2cb1 as libc::c_int,
    -(1 as libc::c_int),
    0x2cb3 as libc::c_int,
    -(1 as libc::c_int),
    0x2cb5 as libc::c_int,
    -(1 as libc::c_int),
    0x2cb7 as libc::c_int,
    -(1 as libc::c_int),
    0x2cb9 as libc::c_int,
    -(1 as libc::c_int),
    0x2cbb as libc::c_int,
    -(1 as libc::c_int),
    0x2cbd as libc::c_int,
    -(1 as libc::c_int),
    0x2cbf as libc::c_int,
    -(1 as libc::c_int),
    0x2cc1 as libc::c_int,
    -(1 as libc::c_int),
    0x2cc3 as libc::c_int,
    -(1 as libc::c_int),
    0x2cc5 as libc::c_int,
    -(1 as libc::c_int),
    0x2cc7 as libc::c_int,
    -(1 as libc::c_int),
    0x2cc9 as libc::c_int,
    -(1 as libc::c_int),
    0x2ccb as libc::c_int,
    -(1 as libc::c_int),
    0x2ccd as libc::c_int,
    -(1 as libc::c_int),
    0x2ccf as libc::c_int,
    -(1 as libc::c_int),
    0x2cd1 as libc::c_int,
    -(1 as libc::c_int),
    0x2cd3 as libc::c_int,
    -(1 as libc::c_int),
    0x2cd5 as libc::c_int,
    -(1 as libc::c_int),
    0x2cd7 as libc::c_int,
    -(1 as libc::c_int),
    0x2cd9 as libc::c_int,
    -(1 as libc::c_int),
    0x2cdb as libc::c_int,
    -(1 as libc::c_int),
    0x2cdd as libc::c_int,
    -(1 as libc::c_int),
    0x2cdf as libc::c_int,
    -(1 as libc::c_int),
    0x2ce1 as libc::c_int,
    -(1 as libc::c_int),
    0x2ce3 as libc::c_int,
    -(1 as libc::c_int),
    0x2cec as libc::c_int,
    -(1 as libc::c_int),
    0x2cee as libc::c_int,
    -(1 as libc::c_int),
    0x2cf3 as libc::c_int,
    -(1 as libc::c_int),
    0x2d27 as libc::c_int,
    -(7264 as libc::c_int),
    0x2d2d as libc::c_int,
    -(7264 as libc::c_int),
    0xa641 as libc::c_int,
    -(1 as libc::c_int),
    0xa643 as libc::c_int,
    -(1 as libc::c_int),
    0xa645 as libc::c_int,
    -(1 as libc::c_int),
    0xa647 as libc::c_int,
    -(1 as libc::c_int),
    0xa649 as libc::c_int,
    -(1 as libc::c_int),
    0xa64b as libc::c_int,
    -(1 as libc::c_int),
    0xa64d as libc::c_int,
    -(1 as libc::c_int),
    0xa64f as libc::c_int,
    -(1 as libc::c_int),
    0xa651 as libc::c_int,
    -(1 as libc::c_int),
    0xa653 as libc::c_int,
    -(1 as libc::c_int),
    0xa655 as libc::c_int,
    -(1 as libc::c_int),
    0xa657 as libc::c_int,
    -(1 as libc::c_int),
    0xa659 as libc::c_int,
    -(1 as libc::c_int),
    0xa65b as libc::c_int,
    -(1 as libc::c_int),
    0xa65d as libc::c_int,
    -(1 as libc::c_int),
    0xa65f as libc::c_int,
    -(1 as libc::c_int),
    0xa661 as libc::c_int,
    -(1 as libc::c_int),
    0xa663 as libc::c_int,
    -(1 as libc::c_int),
    0xa665 as libc::c_int,
    -(1 as libc::c_int),
    0xa667 as libc::c_int,
    -(1 as libc::c_int),
    0xa669 as libc::c_int,
    -(1 as libc::c_int),
    0xa66b as libc::c_int,
    -(1 as libc::c_int),
    0xa66d as libc::c_int,
    -(1 as libc::c_int),
    0xa681 as libc::c_int,
    -(1 as libc::c_int),
    0xa683 as libc::c_int,
    -(1 as libc::c_int),
    0xa685 as libc::c_int,
    -(1 as libc::c_int),
    0xa687 as libc::c_int,
    -(1 as libc::c_int),
    0xa689 as libc::c_int,
    -(1 as libc::c_int),
    0xa68b as libc::c_int,
    -(1 as libc::c_int),
    0xa68d as libc::c_int,
    -(1 as libc::c_int),
    0xa68f as libc::c_int,
    -(1 as libc::c_int),
    0xa691 as libc::c_int,
    -(1 as libc::c_int),
    0xa693 as libc::c_int,
    -(1 as libc::c_int),
    0xa695 as libc::c_int,
    -(1 as libc::c_int),
    0xa697 as libc::c_int,
    -(1 as libc::c_int),
    0xa699 as libc::c_int,
    -(1 as libc::c_int),
    0xa69b as libc::c_int,
    -(1 as libc::c_int),
    0xa723 as libc::c_int,
    -(1 as libc::c_int),
    0xa725 as libc::c_int,
    -(1 as libc::c_int),
    0xa727 as libc::c_int,
    -(1 as libc::c_int),
    0xa729 as libc::c_int,
    -(1 as libc::c_int),
    0xa72b as libc::c_int,
    -(1 as libc::c_int),
    0xa72d as libc::c_int,
    -(1 as libc::c_int),
    0xa72f as libc::c_int,
    -(1 as libc::c_int),
    0xa733 as libc::c_int,
    -(1 as libc::c_int),
    0xa735 as libc::c_int,
    -(1 as libc::c_int),
    0xa737 as libc::c_int,
    -(1 as libc::c_int),
    0xa739 as libc::c_int,
    -(1 as libc::c_int),
    0xa73b as libc::c_int,
    -(1 as libc::c_int),
    0xa73d as libc::c_int,
    -(1 as libc::c_int),
    0xa73f as libc::c_int,
    -(1 as libc::c_int),
    0xa741 as libc::c_int,
    -(1 as libc::c_int),
    0xa743 as libc::c_int,
    -(1 as libc::c_int),
    0xa745 as libc::c_int,
    -(1 as libc::c_int),
    0xa747 as libc::c_int,
    -(1 as libc::c_int),
    0xa749 as libc::c_int,
    -(1 as libc::c_int),
    0xa74b as libc::c_int,
    -(1 as libc::c_int),
    0xa74d as libc::c_int,
    -(1 as libc::c_int),
    0xa74f as libc::c_int,
    -(1 as libc::c_int),
    0xa751 as libc::c_int,
    -(1 as libc::c_int),
    0xa753 as libc::c_int,
    -(1 as libc::c_int),
    0xa755 as libc::c_int,
    -(1 as libc::c_int),
    0xa757 as libc::c_int,
    -(1 as libc::c_int),
    0xa759 as libc::c_int,
    -(1 as libc::c_int),
    0xa75b as libc::c_int,
    -(1 as libc::c_int),
    0xa75d as libc::c_int,
    -(1 as libc::c_int),
    0xa75f as libc::c_int,
    -(1 as libc::c_int),
    0xa761 as libc::c_int,
    -(1 as libc::c_int),
    0xa763 as libc::c_int,
    -(1 as libc::c_int),
    0xa765 as libc::c_int,
    -(1 as libc::c_int),
    0xa767 as libc::c_int,
    -(1 as libc::c_int),
    0xa769 as libc::c_int,
    -(1 as libc::c_int),
    0xa76b as libc::c_int,
    -(1 as libc::c_int),
    0xa76d as libc::c_int,
    -(1 as libc::c_int),
    0xa76f as libc::c_int,
    -(1 as libc::c_int),
    0xa77a as libc::c_int,
    -(1 as libc::c_int),
    0xa77c as libc::c_int,
    -(1 as libc::c_int),
    0xa77f as libc::c_int,
    -(1 as libc::c_int),
    0xa781 as libc::c_int,
    -(1 as libc::c_int),
    0xa783 as libc::c_int,
    -(1 as libc::c_int),
    0xa785 as libc::c_int,
    -(1 as libc::c_int),
    0xa787 as libc::c_int,
    -(1 as libc::c_int),
    0xa78c as libc::c_int,
    -(1 as libc::c_int),
    0xa791 as libc::c_int,
    -(1 as libc::c_int),
    0xa793 as libc::c_int,
    -(1 as libc::c_int),
    0xa794 as libc::c_int,
    48 as libc::c_int,
    0xa797 as libc::c_int,
    -(1 as libc::c_int),
    0xa799 as libc::c_int,
    -(1 as libc::c_int),
    0xa79b as libc::c_int,
    -(1 as libc::c_int),
    0xa79d as libc::c_int,
    -(1 as libc::c_int),
    0xa79f as libc::c_int,
    -(1 as libc::c_int),
    0xa7a1 as libc::c_int,
    -(1 as libc::c_int),
    0xa7a3 as libc::c_int,
    -(1 as libc::c_int),
    0xa7a5 as libc::c_int,
    -(1 as libc::c_int),
    0xa7a7 as libc::c_int,
    -(1 as libc::c_int),
    0xa7a9 as libc::c_int,
    -(1 as libc::c_int),
    0xa7b5 as libc::c_int,
    -(1 as libc::c_int),
    0xa7b7 as libc::c_int,
    -(1 as libc::c_int),
    0xa7b9 as libc::c_int,
    -(1 as libc::c_int),
    0xa7bb as libc::c_int,
    -(1 as libc::c_int),
    0xa7bd as libc::c_int,
    -(1 as libc::c_int),
    0xa7bf as libc::c_int,
    -(1 as libc::c_int),
    0xa7c1 as libc::c_int,
    -(1 as libc::c_int),
    0xa7c3 as libc::c_int,
    -(1 as libc::c_int),
    0xa7c8 as libc::c_int,
    -(1 as libc::c_int),
    0xa7ca as libc::c_int,
    -(1 as libc::c_int),
    0xa7cd as libc::c_int,
    -(1 as libc::c_int),
    0xa7d1 as libc::c_int,
    -(1 as libc::c_int),
    0xa7d7 as libc::c_int,
    -(1 as libc::c_int),
    0xa7d9 as libc::c_int,
    -(1 as libc::c_int),
    0xa7db as libc::c_int,
    -(1 as libc::c_int),
    0xa7f6 as libc::c_int,
    -(1 as libc::c_int),
    0xab53 as libc::c_int,
    -(928 as libc::c_int),
];
static mut ucd_tolower_full: [Rune; 112] = [
    0x130 as libc::c_int,
    0x69 as libc::c_int,
    0x307 as libc::c_int,
    0 as libc::c_int,
    0x1f88 as libc::c_int,
    0x1f80 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f89 as libc::c_int,
    0x1f81 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8a as libc::c_int,
    0x1f82 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8b as libc::c_int,
    0x1f83 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8c as libc::c_int,
    0x1f84 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8d as libc::c_int,
    0x1f85 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8e as libc::c_int,
    0x1f86 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8f as libc::c_int,
    0x1f87 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f98 as libc::c_int,
    0x1f90 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f99 as libc::c_int,
    0x1f91 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9a as libc::c_int,
    0x1f92 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9b as libc::c_int,
    0x1f93 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9c as libc::c_int,
    0x1f94 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9d as libc::c_int,
    0x1f95 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9e as libc::c_int,
    0x1f96 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9f as libc::c_int,
    0x1f97 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa8 as libc::c_int,
    0x1fa0 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa9 as libc::c_int,
    0x1fa1 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1faa as libc::c_int,
    0x1fa2 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fab as libc::c_int,
    0x1fa3 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fac as libc::c_int,
    0x1fa4 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fad as libc::c_int,
    0x1fa5 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fae as libc::c_int,
    0x1fa6 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1faf as libc::c_int,
    0x1fa7 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fbc as libc::c_int,
    0x1fb3 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fcc as libc::c_int,
    0x1fc3 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1ffc as libc::c_int,
    0x1ff3 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
static mut ucd_toupper_full: [Rune; 510] = [
    0xdf as libc::c_int,
    0x53 as libc::c_int,
    0x53 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x149 as libc::c_int,
    0x2bc as libc::c_int,
    0x4e as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f0 as libc::c_int,
    0x4a as libc::c_int,
    0x30c as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x390 as libc::c_int,
    0x399 as libc::c_int,
    0x308 as libc::c_int,
    0x301 as libc::c_int,
    0 as libc::c_int,
    0x3b0 as libc::c_int,
    0x3a5 as libc::c_int,
    0x308 as libc::c_int,
    0x301 as libc::c_int,
    0 as libc::c_int,
    0x587 as libc::c_int,
    0x535 as libc::c_int,
    0x552 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1e96 as libc::c_int,
    0x48 as libc::c_int,
    0x331 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1e97 as libc::c_int,
    0x54 as libc::c_int,
    0x308 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1e98 as libc::c_int,
    0x57 as libc::c_int,
    0x30a as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1e99 as libc::c_int,
    0x59 as libc::c_int,
    0x30a as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1e9a as libc::c_int,
    0x41 as libc::c_int,
    0x2be as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f50 as libc::c_int,
    0x3a5 as libc::c_int,
    0x313 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f52 as libc::c_int,
    0x3a5 as libc::c_int,
    0x313 as libc::c_int,
    0x300 as libc::c_int,
    0 as libc::c_int,
    0x1f54 as libc::c_int,
    0x3a5 as libc::c_int,
    0x313 as libc::c_int,
    0x301 as libc::c_int,
    0 as libc::c_int,
    0x1f56 as libc::c_int,
    0x3a5 as libc::c_int,
    0x313 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0x1f80 as libc::c_int,
    0x1f08 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f81 as libc::c_int,
    0x1f09 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f82 as libc::c_int,
    0x1f0a as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f83 as libc::c_int,
    0x1f0b as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f84 as libc::c_int,
    0x1f0c as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f85 as libc::c_int,
    0x1f0d as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f86 as libc::c_int,
    0x1f0e as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f87 as libc::c_int,
    0x1f0f as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f88 as libc::c_int,
    0x1f08 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f89 as libc::c_int,
    0x1f09 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8a as libc::c_int,
    0x1f0a as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8b as libc::c_int,
    0x1f0b as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8c as libc::c_int,
    0x1f0c as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8d as libc::c_int,
    0x1f0d as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8e as libc::c_int,
    0x1f0e as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f8f as libc::c_int,
    0x1f0f as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f90 as libc::c_int,
    0x1f28 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f91 as libc::c_int,
    0x1f29 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f92 as libc::c_int,
    0x1f2a as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f93 as libc::c_int,
    0x1f2b as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f94 as libc::c_int,
    0x1f2c as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f95 as libc::c_int,
    0x1f2d as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f96 as libc::c_int,
    0x1f2e as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f97 as libc::c_int,
    0x1f2f as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f98 as libc::c_int,
    0x1f28 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f99 as libc::c_int,
    0x1f29 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9a as libc::c_int,
    0x1f2a as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9b as libc::c_int,
    0x1f2b as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9c as libc::c_int,
    0x1f2c as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9d as libc::c_int,
    0x1f2d as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9e as libc::c_int,
    0x1f2e as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1f9f as libc::c_int,
    0x1f2f as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa0 as libc::c_int,
    0x1f68 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa1 as libc::c_int,
    0x1f69 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa2 as libc::c_int,
    0x1f6a as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa3 as libc::c_int,
    0x1f6b as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa4 as libc::c_int,
    0x1f6c as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa5 as libc::c_int,
    0x1f6d as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa6 as libc::c_int,
    0x1f6e as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa7 as libc::c_int,
    0x1f6f as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa8 as libc::c_int,
    0x1f68 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fa9 as libc::c_int,
    0x1f69 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1faa as libc::c_int,
    0x1f6a as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fab as libc::c_int,
    0x1f6b as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fac as libc::c_int,
    0x1f6c as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fad as libc::c_int,
    0x1f6d as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fae as libc::c_int,
    0x1f6e as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1faf as libc::c_int,
    0x1f6f as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fb2 as libc::c_int,
    0x1fba as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fb3 as libc::c_int,
    0x391 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fb4 as libc::c_int,
    0x386 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fb6 as libc::c_int,
    0x391 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fb7 as libc::c_int,
    0x391 as libc::c_int,
    0x342 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0x1fbc as libc::c_int,
    0x391 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fc2 as libc::c_int,
    0x1fca as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fc3 as libc::c_int,
    0x397 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fc4 as libc::c_int,
    0x389 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fc6 as libc::c_int,
    0x397 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fc7 as libc::c_int,
    0x397 as libc::c_int,
    0x342 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0x1fcc as libc::c_int,
    0x397 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fd2 as libc::c_int,
    0x399 as libc::c_int,
    0x308 as libc::c_int,
    0x300 as libc::c_int,
    0 as libc::c_int,
    0x1fd3 as libc::c_int,
    0x399 as libc::c_int,
    0x308 as libc::c_int,
    0x301 as libc::c_int,
    0 as libc::c_int,
    0x1fd6 as libc::c_int,
    0x399 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fd7 as libc::c_int,
    0x399 as libc::c_int,
    0x308 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0x1fe2 as libc::c_int,
    0x3a5 as libc::c_int,
    0x308 as libc::c_int,
    0x300 as libc::c_int,
    0 as libc::c_int,
    0x1fe3 as libc::c_int,
    0x3a5 as libc::c_int,
    0x308 as libc::c_int,
    0x301 as libc::c_int,
    0 as libc::c_int,
    0x1fe4 as libc::c_int,
    0x3a1 as libc::c_int,
    0x313 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fe6 as libc::c_int,
    0x3a5 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1fe7 as libc::c_int,
    0x3a5 as libc::c_int,
    0x308 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0x1ff2 as libc::c_int,
    0x1ffa as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1ff3 as libc::c_int,
    0x3a9 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1ff4 as libc::c_int,
    0x38f as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1ff6 as libc::c_int,
    0x3a9 as libc::c_int,
    0x342 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0x1ff7 as libc::c_int,
    0x3a9 as libc::c_int,
    0x342 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0x1ffc as libc::c_int,
    0x3a9 as libc::c_int,
    0x399 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb00 as libc::c_int,
    0x46 as libc::c_int,
    0x46 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb01 as libc::c_int,
    0x46 as libc::c_int,
    0x49 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb02 as libc::c_int,
    0x46 as libc::c_int,
    0x4c as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb03 as libc::c_int,
    0x46 as libc::c_int,
    0x46 as libc::c_int,
    0x49 as libc::c_int,
    0 as libc::c_int,
    0xfb04 as libc::c_int,
    0x46 as libc::c_int,
    0x46 as libc::c_int,
    0x4c as libc::c_int,
    0 as libc::c_int,
    0xfb05 as libc::c_int,
    0x53 as libc::c_int,
    0x54 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb06 as libc::c_int,
    0x53 as libc::c_int,
    0x54 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb13 as libc::c_int,
    0x544 as libc::c_int,
    0x546 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb14 as libc::c_int,
    0x544 as libc::c_int,
    0x535 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb15 as libc::c_int,
    0x544 as libc::c_int,
    0x53b as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb16 as libc::c_int,
    0x54e as libc::c_int,
    0x546 as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
    0xfb17 as libc::c_int,
    0x544 as libc::c_int,
    0x53d as libc::c_int,
    0 as libc::c_int,
    0 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn jsU_chartorune(rune: *mut Rune, str: *const libc::c_char) -> libc::c_int {
    let mut c: libc::c_int = 0;
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut c3: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    if *str.offset(0 as libc::c_int as isize) as uchar as libc::c_int == 0xc0 as libc::c_int
        && *str.offset(1 as libc::c_int as isize) as uchar as libc::c_int == 0x80 as libc::c_int
    {
        *rune = 0 as libc::c_int;
        return 2 as libc::c_int;
    }
    c = *(str as *mut uchar) as libc::c_int;
    if c < Tx as libc::c_int {
        *rune = c;
        return 1 as libc::c_int;
    }
    c1 = *(str.offset(1 as libc::c_int as isize) as *mut uchar) as libc::c_int ^ Tx as libc::c_int;
    if c1 & Testx as libc::c_int == 0 {
        if c < T3 as libc::c_int {
            if c >= T2 as libc::c_int {
                l = (c << Bitx as libc::c_int | c1) & Rune2 as libc::c_int;
                if l > Rune1 as libc::c_int {
                    *rune = l;
                    return 2 as libc::c_int;
                }
            }
        } else {
            c2 = *(str.offset(2 as libc::c_int as isize) as *mut uchar) as libc::c_int
                ^ Tx as libc::c_int;
            if c2 & Testx as libc::c_int == 0 {
                if c < T4 as libc::c_int {
                    l = ((c << Bitx as libc::c_int | c1) << Bitx as libc::c_int | c2)
                        & Rune3 as libc::c_int;
                    if l > Rune2 as libc::c_int {
                        *rune = l;
                        return 3 as libc::c_int;
                    }
                } else if UTFmax as libc::c_int >= 4 as libc::c_int {
                    c3 = *(str.offset(3 as libc::c_int as isize) as *mut uchar) as libc::c_int
                        ^ Tx as libc::c_int;
                    if c3 & Testx as libc::c_int == 0 && c < T5 as libc::c_int {
                        l = (((c << Bitx as libc::c_int | c1) << Bitx as libc::c_int | c2)
                            << Bitx as libc::c_int
                            | c3)
                            & Rune4 as libc::c_int;
                        if l > Rune3 as libc::c_int && l <= Runemax as libc::c_int {
                            *rune = l;
                            return 4 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    *rune = Bad as libc::c_int;
    1 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsU_runetochar(str: *mut libc::c_char, rune: *const Rune) -> libc::c_int {
    let mut c: libc::c_int = *rune;
    if c == 0 as libc::c_int {
        *str.offset(0 as libc::c_int as isize) = 0xc0 as libc::c_int as libc::c_char;
        *str.offset(1 as libc::c_int as isize) = 0x80 as libc::c_int as libc::c_char;
        return 2 as libc::c_int;
    }
    if c <= Rune1 as libc::c_int {
        *str.offset(0 as libc::c_int as isize) = c as libc::c_char;
        return 1 as libc::c_int;
    }
    if c <= Rune2 as libc::c_int {
        *str.offset(0 as libc::c_int as isize) =
            (T2 as libc::c_int | c >> (1 as libc::c_int * Bitx as libc::c_int)) as libc::c_char;
        *str.offset(1 as libc::c_int as isize) =
            (Tx as libc::c_int | c & Maskx as libc::c_int) as libc::c_char;
        return 2 as libc::c_int;
    }
    if c > Runemax as libc::c_int {
        c = Runeerror as libc::c_int;
    }
    if c <= Rune3 as libc::c_int {
        *str.offset(0 as libc::c_int as isize) =
            (T3 as libc::c_int | c >> (2 as libc::c_int * Bitx as libc::c_int)) as libc::c_char;
        *str.offset(1 as libc::c_int as isize) = (Tx as libc::c_int
            | c >> (1 as libc::c_int * Bitx as libc::c_int) & Maskx as libc::c_int)
            as libc::c_char;
        *str.offset(2 as libc::c_int as isize) =
            (Tx as libc::c_int | c & Maskx as libc::c_int) as libc::c_char;
        return 3 as libc::c_int;
    }
    *str.offset(0 as libc::c_int as isize) =
        (T4 as libc::c_int | c >> (3 as libc::c_int * Bitx as libc::c_int)) as libc::c_char;
    *str.offset(1 as libc::c_int as isize) = (Tx as libc::c_int
        | c >> (2 as libc::c_int * Bitx as libc::c_int) & Maskx as libc::c_int)
        as libc::c_char;
    *str.offset(2 as libc::c_int as isize) = (Tx as libc::c_int
        | c >> (1 as libc::c_int * Bitx as libc::c_int) & Maskx as libc::c_int)
        as libc::c_char;
    *str.offset(3 as libc::c_int as isize) =
        (Tx as libc::c_int | c & Maskx as libc::c_int) as libc::c_char;
    4 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsU_runelen(c: libc::c_int) -> libc::c_int {
    let mut rune: Rune = 0;
    let mut str: [libc::c_char; 10] = [0; 10];
    rune = c;
    jsU_runetochar(str.as_mut_ptr(), &mut rune)
}
unsafe extern "C" fn ucd_bsearch(
    c: Rune,
    mut t: *const Rune,
    mut n: libc::c_int,
    ne: libc::c_int,
) -> *const Rune {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    let mut m: libc::c_int = 0;
    while n > 1 as libc::c_int {
        m = n / 2 as libc::c_int;
        p = t.offset((m * ne) as isize);
        if c >= *p.offset(0 as libc::c_int as isize) {
            t = p;
            n -= m;
        } else {
            n = m;
        }
    }
    if n != 0 && c >= *t.offset(0 as libc::c_int as isize) {
        return t;
    }
    std::ptr::null::<Rune>()
}
#[no_mangle]
pub unsafe extern "C" fn jsU_tolowerrune(c: Rune) -> Rune {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        ucd_tolower2.as_ptr(),
        (::core::mem::size_of::<[Rune; 156]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 3 as libc::c_int,
        3 as libc::c_int,
    );
    if !p.is_null()
        && c >= *p.offset(0 as libc::c_int as isize)
        && c <= *p.offset(1 as libc::c_int as isize)
    {
        return c + *p.offset(2 as libc::c_int as isize);
    }
    p = ucd_bsearch(
        c,
        ucd_tolower1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1244]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 2 as libc::c_int,
        2 as libc::c_int,
    );
    if !p.is_null() && c == *p.offset(0 as libc::c_int as isize) {
        return c + *p.offset(1 as libc::c_int as isize);
    }
    c
}
#[no_mangle]
pub unsafe extern "C" fn jsU_toupperrune(c: Rune) -> Rune {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        ucd_toupper2.as_ptr(),
        (::core::mem::size_of::<[Rune; 159]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 3 as libc::c_int,
        3 as libc::c_int,
    );
    if !p.is_null()
        && c >= *p.offset(0 as libc::c_int as isize)
        && c <= *p.offset(1 as libc::c_int as isize)
    {
        return c + *p.offset(2 as libc::c_int as isize);
    }
    p = ucd_bsearch(
        c,
        ucd_toupper1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1274]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 2 as libc::c_int,
        2 as libc::c_int,
    );
    if !p.is_null() && c == *p.offset(0 as libc::c_int as isize) {
        return c + *p.offset(1 as libc::c_int as isize);
    }
    c
}
#[no_mangle]
pub unsafe extern "C" fn jsU_islowerrune(c: Rune) -> libc::c_int {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        ucd_toupper2.as_ptr(),
        (::core::mem::size_of::<[Rune; 159]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 3 as libc::c_int,
        3 as libc::c_int,
    );
    if !p.is_null()
        && c >= *p.offset(0 as libc::c_int as isize)
        && c <= *p.offset(1 as libc::c_int as isize)
    {
        return 1 as libc::c_int;
    }
    p = ucd_bsearch(
        c,
        ucd_toupper1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1274]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 2 as libc::c_int,
        2 as libc::c_int,
    );
    if !p.is_null() && c == *p.offset(0 as libc::c_int as isize) {
        return 1 as libc::c_int;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsU_isupperrune(c: Rune) -> libc::c_int {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        ucd_tolower2.as_ptr(),
        (::core::mem::size_of::<[Rune; 156]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 3 as libc::c_int,
        3 as libc::c_int,
    );
    if !p.is_null()
        && c >= *p.offset(0 as libc::c_int as isize)
        && c <= *p.offset(1 as libc::c_int as isize)
    {
        return 1 as libc::c_int;
    }
    p = ucd_bsearch(
        c,
        ucd_tolower1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1244]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 2 as libc::c_int,
        2 as libc::c_int,
    );
    if !p.is_null() && c == *p.offset(0 as libc::c_int as isize) {
        return 1 as libc::c_int;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsU_isalpharune(c: Rune) -> libc::c_int {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        ucd_alpha2.as_ptr(),
        (::core::mem::size_of::<[Rune; 1046]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 2 as libc::c_int,
        2 as libc::c_int,
    );
    if !p.is_null()
        && c >= *p.offset(0 as libc::c_int as isize)
        && c <= *p.offset(1 as libc::c_int as isize)
    {
        return 1 as libc::c_int;
    }
    p = ucd_bsearch(
        c,
        ucd_alpha1.as_ptr(),
        (::core::mem::size_of::<[Rune; 167]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int,
        1 as libc::c_int,
    );
    if !p.is_null() && c == *p.offset(0 as libc::c_int as isize) {
        return 1 as libc::c_int;
    }
    0 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn jsU_tolowerrune_full(c: Rune) -> *const Rune {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        ucd_tolower_full.as_ptr(),
        (::core::mem::size_of::<[Rune; 112]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 4 as libc::c_int,
        4 as libc::c_int,
    );
    if !p.is_null() && c == *p.offset(0 as libc::c_int as isize) {
        return p.offset(1 as libc::c_int as isize);
    }
    std::ptr::null::<Rune>()
}
#[no_mangle]
pub unsafe extern "C" fn jsU_toupperrune_full(c: Rune) -> *const Rune {
    let mut p: *const Rune = std::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        ucd_toupper_full.as_ptr(),
        (::core::mem::size_of::<[Rune; 510]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<Rune>() as libc::c_ulong) as libc::c_int
            / 5 as libc::c_int,
        5 as libc::c_int,
    );
    if !p.is_null() && c == *p.offset(0 as libc::c_int as isize) {
        return p.offset(1 as libc::c_int as isize);
    }
    std::ptr::null::<Rune>()
}
