use crate::*;

unsafe extern "C" fn jsB_stacktrace(J: &mut js_State, mut skip: i32) -> i32 {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut n: i32 = (*J).tracetop - skip;
    if n <= 0 {
        return 0;
    }
    while n > 0 {
        let mut name: *const libc::c_char = (*J).trace[n as usize].name;
        let mut file: *const libc::c_char = (*J).trace[n as usize].file;
        let mut line: i32 = (*J).trace[n as usize].line;
        if line > 0 {
            if *name.offset(0 as isize) != 0 {
                snprintf(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
                    b"\n\tat %s (%s:%d)\0" as *const u8 as *const libc::c_char,
                    name,
                    file,
                    line,
                );
            } else {
                snprintf(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
                    b"\n\tat %s:%d\0" as *const u8 as *const libc::c_char,
                    file,
                    line,
                );
            }
        } else {
            snprintf(
                buf.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
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
    }
    1
}
unsafe extern "C" fn Ep_toString(J: &mut js_State) {
    let mut name: *const libc::c_char = b"Error\0" as *const u8 as *const libc::c_char;
    let mut message: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
    if js_isobject(J, -(1 as i32)) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    if js_hasproperty(
        J,
        0,
        b"name\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        name = js_tostring(J, -(1 as i32));
    }
    if js_hasproperty(
        J,
        0,
        b"message\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        message = js_tostring(J, -(1 as i32));
    }
    if *name.offset(0 as isize) as i32 == 0 {
        js_pushstring(J, message);
    } else if *message.offset(0 as isize) as i32 == 0 {
        js_pushstring(J, name);
    } else {
        js_pushstring(J, name);
        js_pushstring(J, b": \0" as *const u8 as *const libc::c_char);
        js_concat(J);
        js_pushstring(J, message);
        js_concat(J);
    };
}
unsafe extern "C" fn jsB_ErrorX(J: &mut js_State, mut prototype: *mut js_Object) -> i32 {
    let v = jsV_newobject(J, JS_CERROR, prototype);
    js_pushobject(J, v);
    if js_isdefined(J, 1) != 0 {
        let v = js_tostring(J, 1);
        js_pushstring(J, v);
        js_defproperty(
            J,
            -(2 as i32),
            b"message\0" as *const u8 as *const libc::c_char,
            JS_DONTENUM as i32,
        );
    }
    if jsB_stacktrace(J, 1) != 0 {
        js_defproperty(
            J,
            -(2 as i32),
            b"stackTrace\0" as *const u8 as *const libc::c_char,
            JS_DONTENUM as i32,
        );
    }
    1
}
unsafe extern "C" fn js_newerrorx(
    J: &mut js_State,
    mut message: *const libc::c_char,
    mut prototype: *mut js_Object,
) {
    let v = jsV_newobject(J, JS_CERROR, prototype);
    js_pushobject(J, v);
    js_pushstring(J, message);
    js_setproperty(
        J,
        -(2 as i32),
        b"message\0" as *const u8 as *const libc::c_char,
    );
    if jsB_stacktrace(J, 0) != 0 {
        js_setproperty(
            J,
            -(2 as i32),
            b"stackTrace\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_newerror(J: &mut js_State, mut s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).Error_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_error(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).Error_prototype);
    js_throw(J);
}
unsafe extern "C" fn jsB_Error(J: &mut js_State) {
    jsB_ErrorX(J, (*J).Error_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_evalerror(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).EvalError_prototype);
    js_throw(J);
}
unsafe extern "C" fn jsB_EvalError(J: &mut js_State) {
    jsB_ErrorX(J, (*J).EvalError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_newevalerror(J: &mut js_State, mut s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).EvalError_prototype);
}
unsafe extern "C" fn jsB_RangeError(J: &mut js_State) {
    jsB_ErrorX(J, (*J).RangeError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_rangeerror(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).RangeError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newrangeerror(J: &mut js_State, mut s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).RangeError_prototype);
}
unsafe extern "C" fn jsB_ReferenceError(J: &mut js_State) {
    jsB_ErrorX(J, (*J).ReferenceError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_newreferenceerror(J: &mut js_State, mut s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).ReferenceError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_referenceerror(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).ReferenceError_prototype);
    js_throw(J);
}
unsafe extern "C" fn jsB_SyntaxError(J: &mut js_State) {
    jsB_ErrorX(J, (*J).SyntaxError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_syntaxerror(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).SyntaxError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newsyntaxerror(J: &mut js_State, mut s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).SyntaxError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_typeerror(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).TypeError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newtypeerror(J: &mut js_State, mut s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).TypeError_prototype);
}
unsafe extern "C" fn jsB_TypeError(J: &mut js_State) {
    jsB_ErrorX(J, (*J).TypeError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn js_urierror(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    js_newerrorx(J, buf.as_mut_ptr(), (*J).URIError_prototype);
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_newurierror(J: &mut js_State, mut s: *const libc::c_char) {
    js_newerrorx(J, s, (*J).URIError_prototype);
}
unsafe extern "C" fn jsB_URIError(J: &mut js_State) {
    jsB_ErrorX(J, (*J).URIError_prototype);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initerror(J: &mut js_State) {
    js_pushobject(J, (*J).Error_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"Error\0" as *const u8 as *const libc::c_char,
    );
    jsB_propf(
        J,
        b"Error.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Ep_toString),
        0,
    );
    js_newcconstructor(
        J,
        Some(jsB_Error),
        Some(jsB_Error),
        b"Error\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"Error\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_pushobject(J, (*J).EvalError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"EvalError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_EvalError),
        Some(jsB_EvalError),
        b"EvalError\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"EvalError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_pushobject(J, (*J).RangeError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"RangeError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_RangeError),
        Some(jsB_RangeError),
        b"RangeError\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"RangeError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_pushobject(J, (*J).ReferenceError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"ReferenceError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_ReferenceError),
        Some(jsB_ReferenceError),
        b"ReferenceError\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"ReferenceError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_pushobject(J, (*J).SyntaxError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"SyntaxError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_SyntaxError),
        Some(jsB_SyntaxError),
        b"SyntaxError\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"SyntaxError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_pushobject(J, (*J).TypeError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"TypeError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_TypeError),
        Some(jsB_TypeError),
        b"TypeError\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"TypeError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_pushobject(J, (*J).URIError_prototype);
    jsB_props(
        J,
        b"name\0" as *const u8 as *const libc::c_char,
        b"URIError\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(jsB_URIError),
        Some(jsB_URIError),
        b"URIError\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"URIError\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
