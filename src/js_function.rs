use crate::*;

unsafe extern "C" fn jsB_Function(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut body: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut parse: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut fun: *mut js_Function = core::ptr::null_mut::<js_Function>();
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        jsP_freeparse(J);
        js_throw(J);
    }
    if top > 2 {
        i = 1;
        while i < top - 1 {
            if i > 1 {
                js_putc(J, &mut sb, ',' as i32);
            }
            let s = js_tostring(J, i);
            js_puts(J, &mut sb, s);
            i += 1;
        }
        js_putc(J, &mut sb, ')' as i32);
        js_putc(J, &mut sb, 0);
    }
    body = if js_isdefined(J, top - 1) != 0 {
        js_tostring(J, top - 1)
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    parse = jsP_parsefunction(
        J,
        b"[string]\0" as *const u8 as *const libc::c_char,
        if !sb.is_null() {
            ((*sb).s).as_mut_ptr() as *const libc::c_char
        } else {
            core::ptr::null_mut::<libc::c_char>()
        },
        body,
    );
    fun = jsC_compilefunction(J, parse);
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
    jsP_freeparse(J);
    js_newfunction(J, fun, (*J).GE);
}
unsafe extern "C" fn jsB_Function_prototype(J: &mut js_State) {
    js_pushundefined(J);
}
unsafe extern "C" fn Fp_toString(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut i: i32 = 0;
    if js_iscallable(J, 0) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    if (*self_0).type_0 as libc::c_uint == JS_CFUNCTION as i32 as libc::c_uint
        || (*self_0).type_0 as libc::c_uint == JS_CSCRIPT as i32 as libc::c_uint
    {
        let mut F: *mut js_Function = (*self_0).u.f.function;
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
        i = 0;
        while i < (*F).numparams {
            if i > 0 {
                js_putc(J, &mut sb, ',' as i32);
            }
            js_puts(J, &mut sb, *((*F).vartab).offset(i as isize));
            i += 1;
        }
        js_puts(
            J,
            &mut sb,
            b") { [byte code] }\0" as *const u8 as *const libc::c_char,
        );
        js_putc(J, &mut sb, 0);
        js_pushstring(J, ((*sb).s).as_mut_ptr() as *const libc::c_char);
        js_endtry(J);
        js_free(J, sb as *mut libc::c_void);
    } else if (*self_0).type_0 as libc::c_uint == JS_CCFUNCTION as i32 as libc::c_uint {
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
        js_putc(J, &mut sb, 0);
        js_pushstring(J, ((*sb).s).as_mut_ptr() as *const libc::c_char);
        js_endtry(J);
        js_free(J, sb as *mut libc::c_void);
    } else {
        js_pushliteral(J, b"function () { }\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn Fp_apply(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    if js_iscallable(J, 0) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    js_copy(J, 0);
    js_copy(J, 1);
    if js_isnull(J, 2) != 0 || js_isundefined(J, 2) != 0 {
        n = 0;
    } else {
        n = js_getlength(J, 2);
        if n < 0 {
            n = 0;
        }
        i = 0;
        while i < n {
            js_getindex(J, 2, i);
            i += 1;
        }
    }
    js_call(J, n);
}
unsafe extern "C" fn Fp_call(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    if js_iscallable(J, 0) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    i = 0;
    while i < top {
        js_copy(J, i);
        i += 1;
    }
    js_call(J, top - 2);
}
unsafe extern "C" fn callbound(J: &mut js_State) {
    let mut top: i32 = js_gettop(J);
    let mut i: i32 = 0;
    let mut fun: i32 = 0;
    let mut args: i32 = 0;
    let mut n: i32 = 0;
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
    if n < 0 {
        n = 0;
    }
    i = 0;
    while i < n {
        js_getindex(J, args, i);
        i += 1;
    }
    js_remove(J, args);
    i = 1;
    while i < top {
        js_copy(J, i);
        i += 1;
    }
    js_call(J, n + top - 1);
}
unsafe extern "C" fn constructbound(J: &mut js_State) {
    let mut top: i32 = js_gettop(J);
    let mut i: i32 = 0;
    let mut fun: i32 = 0;
    let mut args: i32 = 0;
    let mut n: i32 = 0;
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
    if n < 0 {
        n = 0;
    }
    i = 0;
    while i < n {
        js_getindex(J, args, i);
        i += 1;
    }
    js_remove(J, args);
    i = 1;
    while i < top {
        js_copy(J, i);
        i += 1;
    }
    js_construct(J, n + top - 1);
}
unsafe extern "C" fn Fp_bind(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    let mut n: i32 = 0;
    if js_iscallable(J, 0) == 0 {
        js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
    }
    n = js_getlength(J, 0);
    if n > top - 2 {
        n -= top - 2;
    } else {
        n = 0;
    }
    js_getproperty(
        J,
        0,
        b"prototype\0" as *const u8 as *const libc::c_char,
    );
    js_newcconstructor(
        J,
        Some(callbound),
        Some(constructbound),
        b"[bind]\0" as *const u8 as *const libc::c_char,
        n,
    );
    js_copy(J, 0);
    js_defproperty(
        J,
        -(2 as i32),
        b"__TargetFunction__\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    js_copy(J, 1);
    js_defproperty(
        J,
        -(2 as i32),
        b"__BoundThis__\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    js_newarray(J);
    i = 2;
    while i < top {
        js_copy(J, i);
        js_setindex(J, -(2 as i32), i - 2);
        i += 1;
    }
    js_defproperty(
        J,
        -(2 as i32),
        b"__BoundArguments__\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initfunction(J: &mut js_State) {
    (*(*J).Function_prototype).u.c.name =
        b"Function.prototype\0" as *const u8 as *const libc::c_char;
    (*(*J).Function_prototype).u.c.function = Some(jsB_Function_prototype);
    (*(*J).Function_prototype).u.c.constructor = None;
    (*(*J).Function_prototype).u.c.length = 0;
    js_pushobject(J, (*J).Function_prototype);
    jsB_propf(
        J,
        b"Function.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Fp_toString),
        2,
    );
    jsB_propf(
        J,
        b"Function.prototype.apply\0" as *const u8 as *const libc::c_char,
        Some(Fp_apply),
        2,
    );
    jsB_propf(
        J,
        b"Function.prototype.call\0" as *const u8 as *const libc::c_char,
        Some(Fp_call),
        1,
    );
    jsB_propf(
        J,
        b"Function.prototype.bind\0" as *const u8 as *const libc::c_char,
        Some(Fp_bind),
        1,
    );
    js_newcconstructor(
        J,
        Some(jsB_Function),
        Some(jsB_Function),
        b"Function\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"Function\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
