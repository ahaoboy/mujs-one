use core::ptr::NonNull;

use crate::*;

unsafe extern "C" fn js_ptry(J: &mut js_State) -> i32 {
    if (*J).trytop == 64 as i32 {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as i32 as libc::c_char;
        let fresh88 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
        *fresh88 = b"exception stack overflow\0" as *const u8 as *const libc::c_char;
        (*J).top += 1;
        (*J).top;
        return 1;
    }
    0
}
unsafe extern "C" fn js_defaultalloc(
    mut actx: *mut libc::c_void,
    mut ptr: *mut libc::c_void,
    mut size: i32,
) -> *mut libc::c_void {
    if size == 0 {
        free(ptr);
        return core::ptr::null_mut::<libc::c_void>();
    }
    realloc(ptr, size as size_t)
}
unsafe extern "C" fn js_defaultreport(J: &mut js_State, mut message: *const libc::c_char) {
    fputs(message, stderr);
    fputc('\n' as i32, stderr);
}
unsafe extern "C" fn js_defaultpanic(J: &mut js_State) {
    js_report(
        J,
        b"uncaught exception\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_ploadstring(
    J: &mut js_State,
    mut filename: *const libc::c_char,
    mut source: *const libc::c_char,
) -> i32 {
    if js_ptry(J) != 0 {
        return 1;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        return 1;
    }
    js_loadstring(J, filename, source);
    js_endtry(J);
    0
}
#[no_mangle]
pub unsafe extern "C" fn js_ploadfile(J: &mut js_State, mut filename: *const libc::c_char) -> i32 {
    if js_ptry(J) != 0 {
        return 1;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        return 1;
    }
    js_loadfile(J, filename);
    js_endtry(J);
    0
}
#[no_mangle]
pub unsafe extern "C" fn js_trystring(
    J: &mut js_State,
    mut idx: i32,
    mut error: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    if js_ptry(J) != 0 {
        js_pop(J, 1);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1);
        return error;
    }
    s = js_tostring(J, idx);
    js_endtry(J);
    s
}
#[no_mangle]
pub unsafe extern "C" fn js_trynumber(J: &mut js_State, mut idx: i32, mut error: f64) -> f64 {
    let mut v: f64 = 0.;
    if js_ptry(J) != 0 {
        js_pop(J, 1);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1);
        return error;
    }
    v = js_tonumber(J, idx);
    js_endtry(J);
    v
}
#[no_mangle]
pub unsafe extern "C" fn js_tryinteger(J: &mut js_State, mut idx: i32, mut error: i32) -> i32 {
    let mut v: i32 = 0;
    if js_ptry(J) != 0 {
        js_pop(J, 1);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1);
        return error;
    }
    v = js_tointeger(J, idx);
    js_endtry(J);
    v
}
#[no_mangle]
pub unsafe extern "C" fn js_tryboolean(J: &mut js_State, mut idx: i32, mut error: i32) -> i32 {
    let mut v: i32 = 0;
    if js_ptry(J) != 0 {
        js_pop(J, 1);
        return error;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1);
        return error;
    }
    v = js_toboolean(J, idx);
    js_endtry(J);
    v
}
unsafe extern "C" fn js_loadstringx(
    J: &mut js_State,
    mut filename: *const libc::c_char,
    mut source: *const libc::c_char,
    mut iseval: i32,
) {
    let mut P: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut F: *mut js_Function = core::ptr::null_mut::<js_Function>();
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
                core::ptr::null_mut::<js_Environment>()
            }
        } else {
            (*J).GE
        },
    );
    js_endtry(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_loadeval(
    J: &mut js_State,
    mut filename: *const libc::c_char,
    mut source: *const libc::c_char,
) {
    js_loadstringx(J, filename, source, 1);
}
#[no_mangle]
pub unsafe extern "C" fn js_loadstring(
    J: &mut js_State,
    mut filename: *const libc::c_char,
    mut source: *const libc::c_char,
) {
    js_loadstringx(J, filename, source, 0);
}
#[no_mangle]
pub unsafe extern "C" fn js_loadfile(J: &mut js_State, mut filename: *const libc::c_char) {
    let mut f: *mut FILE = core::ptr::null_mut::<FILE>();
    let mut s: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut n: i32 = 0;
    let mut t: i32 = 0;
    f = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        js_error(
            J,
            b"cannot open file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    if fseek(f, 0, 2) < 0 {
        fclose(f);
        js_error(
            J,
            b"cannot seek in file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    n = ftell(f) as i32;
    if n < 0 {
        fclose(f);
        js_error(
            J,
            b"cannot tell in file '%s': %s\0" as *const u8 as *const libc::c_char,
            filename,
            strerror(*__errno_location()),
        );
    }
    if fseek(f, 0, 0) < 0 {
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
    s = js_malloc(J, n + 1) as *mut libc::c_char;
    js_endtry(J);
    t = fread(s as *mut libc::c_void, 1, n as size_t, f) as i32;
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
    *s.offset(n as isize) = 0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, s as *mut libc::c_void);
        fclose(f);
        js_throw(J);
    }
    p = s;
    if *p.offset(0 as isize) as i32 == '#' as i32
        && *p.offset(1 as i32 as isize) as i32 == '!' as i32
    {
        p = p.offset(2 as i32 as isize);
        while *p as i32 != 0 && *p as i32 != '\n' as i32 {
            p = p.offset(1);
        }
    }
    js_loadstring(J, filename, p);
    js_free(J, s as *mut libc::c_void);
    fclose(f);
    js_endtry(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_dostring(J: &mut js_State, mut source: *const libc::c_char) -> i32 {
    if js_ptry(J) != 0 {
        js_report(
            J,
            b"exception stack overflow\0" as *const u8 as *const libc::c_char,
        );
        js_pop(J, 1);
        return 1;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        let msg = js_trystring(
            J,
            -(1 as i32),
            b"Error\0" as *const u8 as *const libc::c_char,
        );
        js_report(J, msg);
        js_pop(J, 1);
        return 1;
    }
    js_loadstring(J, b"[string]\0" as *const u8 as *const libc::c_char, source);
    js_pushundefined(J);
    js_call(J, 0);
    js_pop(J, 1);
    js_endtry(J);
    0
}
#[no_mangle]
pub unsafe extern "C" fn js_dofile(J: &mut js_State, mut filename: *const libc::c_char) -> i32 {
    if js_ptry(J) != 0 {
        js_report(
            J,
            b"exception stack overflow\0" as *const u8 as *const libc::c_char,
        );
        js_pop(J, 1);
        return 1;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        let msg = js_trystring(
            J,
            -(1 as i32),
            b"Error\0" as *const u8 as *const libc::c_char,
        );
        js_report(J, msg);
        js_pop(J, 1);
        return 1;
    }
    js_loadfile(J, filename);
    js_pushundefined(J);
    js_call(J, 0);
    js_pop(J, 1);
    js_endtry(J);
    0
}
#[no_mangle]
pub unsafe extern "C" fn js_atpanic(J: &mut js_State, mut panic: js_Panic) -> js_Panic {
    let mut old: js_Panic = (*J).panic;
    (*J).panic = panic;
    old
}
#[no_mangle]
pub unsafe extern "C" fn js_report(J: &mut js_State, mut message: *const libc::c_char) {
    if ((*J).report).is_some() {
        ((*J).report).expect("non-null function pointer")(J, message);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_setreport(J: &mut js_State, mut report: js_Report) {
    (*J).report = report;
}
#[no_mangle]
pub unsafe extern "C" fn js_setcontext(J: &mut js_State, mut uctx: *mut libc::c_void) {
    (*J).uctx = uctx;
}
#[no_mangle]
pub unsafe extern "C" fn js_getcontext(J: &mut js_State) -> *mut libc::c_void {
    (*J).uctx
}
#[no_mangle]
pub unsafe extern "C" fn js_newstate(
    mut alloc: js_Alloc,
    mut actx: *mut libc::c_void,
    mut flags: i32,
) -> Option<NonNull<js_State>> {
    let mut J: *mut js_State = core::ptr::null_mut::<js_State>();
    if ::core::mem::size_of::<js_Value>() as u32 == 16 as i32 as u32 {
    } else {
        __assert_fail(
            b"sizeof(js_Value) == 16\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            12724 as i32 as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"js_State *js_newstate(js_Alloc, void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    {
        if ::core::mem::size_of::<js_Value>() as u32 == 16 as i32 as u32 {
        } else {
            __assert_fail(
                b"sizeof(js_Value) == 16\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                12724 as i32 as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                    b"js_State *js_newstate(js_Alloc, void *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if 15 as u32 as i32 == 15 as i32 {
    } else {
        __assert_fail(
            b"soffsetof(js_Value, t.type) == 15\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            12725 as i32 as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 45], &[libc::c_char; 45]>(
                b"js_State *js_newstate(js_Alloc, void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    {
        if 15 as u32 as i32 == 15 as i32 {
        } else {
            __assert_fail(
                b"soffsetof(js_Value, t.type) == 15\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                12725 as i32 as libc::c_uint,
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
                    i32,
                ) -> *mut libc::c_void,
        );
    }
    J = alloc.expect("non-null function pointer")(
        actx,
        core::ptr::null_mut::<libc::c_void>(),
        ::core::mem::size_of::<js_State>() as u32 as i32,
    ) as *mut js_State;
    if J.is_null() {
        return None;
    }
    memset(
        J as *mut libc::c_void,
        0,
        ::core::mem::size_of::<js_State>() as u32,
    );
    let J = &mut *J;
    (*J).actx = actx;
    (*J).alloc = alloc;
    if flags & JS_STRICT as i32 != 0 {
        (*J).default_strict = 1;
        (*J).strict = (*J).default_strict;
    }
    (*J).trace[0 as usize].name = b"-top-\0" as *const u8 as *const libc::c_char;
    (*J).trace[0 as usize].file = b"native\0" as *const u8 as *const libc::c_char;
    (*J).trace[0 as usize].line = 0;
    (*J).report = Some(js_defaultreport);
    (*J).panic = Some(js_defaultpanic);
    (*J).stack = alloc.expect("non-null function pointer")(
        actx,
        core::ptr::null_mut::<libc::c_void>(),
        (4096 as i32 as u32).wrapping_mul(::core::mem::size_of::<js_Value>() as u32) as i32,
    ) as *mut js_Value;
    if ((*J).stack).is_null() {
        alloc.expect("non-null function pointer")(actx, J as *mut js_State as *mut libc::c_void, 0);
        return None;
    }
    (*J).gcmark = 1;
    (*J).nextref = 0;
    (*J).gcthresh = 0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_freestate(J);
        return None;
    }
    (*J).R = jsV_newobject(J, JS_COBJECT, core::ptr::null_mut::<js_Object>());
    (*J).G = jsV_newobject(J, JS_COBJECT, core::ptr::null_mut::<js_Object>());
    (*J).E = jsR_newenvironment(J, (*J).G, core::ptr::null_mut::<js_Environment>());
    (*J).GE = (*J).E;
    jsB_init(J);
    js_endtry(J);
    Some(NonNull::from(J))
}
