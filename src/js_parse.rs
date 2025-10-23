use crate::*;

#[no_mangle]
pub unsafe extern "C" fn js_isnumberobject(J: &mut js_State, mut idx: i32) -> i32 {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint == JS_CNUMBER as i32 as libc::c_uint)
        as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isstringobject(J: &mut js_State, mut idx: i32) -> i32 {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint)
        as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isbooleanobject(J: &mut js_State, mut idx: i32) -> i32 {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint == JS_CBOOLEAN as i32 as libc::c_uint)
        as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isdateobject(J: &mut js_State, mut idx: i32) -> i32 {
    (js_isobject(J, idx) != 0
        && (*js_toobject(J, idx)).type_0 as libc::c_uint == JS_CDATE as i32 as libc::c_uint)
        as i32
}
unsafe extern "C" fn jsonnext(J: &mut js_State) {
    (*J).lookahead = jsY_lexjson(J);
}
unsafe extern "C" fn jsonaccept(J: &mut js_State, mut t: i32) -> i32 {
    if (*J).lookahead == t {
        jsonnext(J);
        return 1;
    }
    0
}
unsafe extern "C" fn jsonexpect(J: &mut js_State, mut t: i32) {
    if jsonaccept(J, t) == 0 {
        js_syntaxerror(
            J,
            b"JSON: unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(t),
        );
    }
}
unsafe extern "C" fn jsonvalue(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut name: *const libc::c_char = core::ptr::null::<libc::c_char>();
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
                if (*J).lookahead != TK_STRING as i32 {
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
                js_setproperty(J, -(2 as i32), name);
                if jsonaccept(J, ',' as i32) == 0 {
                    break;
                }
            }
            jsonexpect(J, '}' as i32);
        }
        91 => {
            js_newarray(J);
            jsonnext(J);
            i = 0;
            if jsonaccept(J, ']' as i32) != 0 {
                return;
            }
            loop {
                jsonvalue(J);
                let fresh48 = i;
                i += 1;
                js_setindex(J, -(2 as i32), fresh48);
                if jsonaccept(J, ',' as i32) == 0 {
                    break;
                }
            }
            jsonexpect(J, ']' as i32);
        }
        306 => {
            js_pushboolean(J, 1);
            jsonnext(J);
        }
        293 => {
            js_pushboolean(J, 0);
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
unsafe extern "C" fn jsonrevive(J: &mut js_State, mut name: *const libc::c_char) {
    let mut key: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut buf: [libc::c_char; 32] = [0; 32];
    js_getproperty(J, -(1 as i32), name);
    if js_isobject(J, -(1 as i32)) != 0 {
        if js_isarray(J, -(1 as i32)) != 0 {
            let mut i: i32 = 0;
            let mut n: i32 = js_getlength(J, -(1 as i32));
            i = 0;
            while i < n {
                jsonrevive(J, js_itoa(buf.as_mut_ptr(), i));
                if js_isundefined(J, -(1 as i32)) != 0 {
                    js_pop(J, 1);
                    js_delproperty(J, -(1 as i32), buf.as_mut_ptr());
                } else {
                    js_setproperty(J, -(2 as i32), buf.as_mut_ptr());
                }
                i += 1;
            }
        } else {
            js_pushiterator(J, -(1 as i32), 1);
            loop {
                key = js_nextiterator(J, -(1 as i32));
                if key.is_null() {
                    break;
                }
                js_rot2(J);
                jsonrevive(J, key);
                if js_isundefined(J, -(1 as i32)) != 0 {
                    js_pop(J, 1);
                    js_delproperty(J, -(1 as i32), key);
                } else {
                    js_setproperty(J, -(2 as i32), key);
                }
                js_rot2(J);
            }
            js_pop(J, 1);
        }
    }
    js_copy(J, 2);
    js_copy(J, -(3 as i32));
    js_pushstring(J, name);
    js_copy(J, -(4 as i32));
    js_call(J, 2);
    js_rot2pop1(J);
}
unsafe extern "C" fn JSON_parse(J: &mut js_State) {
    let mut source: *const libc::c_char = js_tostring(J, 1);
    jsY_initlex(J, b"JSON\0" as *const u8 as *const libc::c_char, source);
    jsonnext(J);
    if js_iscallable(J, 2) != 0 {
        js_newobject(J);
        jsonvalue(J);
        js_defproperty(J, -(2 as i32), b"\0" as *const u8 as *const libc::c_char, 0);
        jsonrevive(J, b"\0" as *const u8 as *const libc::c_char);
    } else {
        jsonvalue(J);
    };
}
unsafe extern "C" fn fmtnum(J: &mut js_State, mut sb: *mut *mut js_Buffer, mut n: f64) {
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
    } else if n == 0.0 {
        js_puts(J, sb, b"0\0" as *const u8 as *const libc::c_char);
    } else {
        let mut buf: [libc::c_char; 40] = [0; 40];
        js_puts(J, sb, jsV_numbertostring(buf.as_mut_ptr(), n));
    };
}
unsafe extern "C" fn fmtstr(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut s: *const libc::c_char,
) {
    const HEX: *const libc::c_char = c"0123456789abcdef".as_ptr();
    let mut i: i32 = 0;
    let mut n: i32 = 0;
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
                if c < ' ' as i32 || c >= 0xd800 && c <= 0xdfff as i32 {
                    js_putc(J, sb, '\\' as i32);
                    js_putc(J, sb, 'u' as i32);
                    js_putc(
                        J,
                        sb,
                        *HEX.offset((c >> 12 as i32 & 15 as i32) as isize) as i32,
                    );
                    js_putc(J, sb, *HEX.offset((c >> 8 & 15 as i32) as isize) as i32);
                    js_putc(J, sb, *HEX.offset((c >> 4 & 15 as i32) as isize) as i32);
                    js_putc(J, sb, *HEX.offset((c & 15 as i32) as isize) as i32);
                } else if c < 128 as i32 {
                    js_putc(J, sb, c);
                } else {
                    i = 0;
                    while i < n {
                        js_putc(J, sb, *s.offset(i as isize) as i32);
                        i += 1;
                    }
                }
            }
        }
        s = s.offset(n as isize);
    }
    js_putc(J, sb, '"' as i32);
}
unsafe extern "C" fn fmtindent(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut gap: *const libc::c_char,
    mut level: i32,
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
unsafe extern "C" fn filterprop(J: &mut js_State, mut key: *const libc::c_char) -> i32 {
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut found: i32 = 0;
    if js_isarray(J, 2) != 0 {
        found = 0;
        n = js_getlength(J, 2);
        i = 0;
        while i < n && found == 0 {
            js_getindex(J, 2, i);
            if js_isstring(J, -(1 as i32)) != 0
                || js_isnumber(J, -(1 as i32)) != 0
                || js_isstringobject(J, -(1 as i32)) != 0
                || js_isnumberobject(J, -(1 as i32)) != 0
            {
                found = (strcmp(key, js_tostring(J, -(1 as i32))) == 0) as i32;
            }
            js_pop(J, 1);
            i += 1;
        }
        return found;
    }
    1
}
unsafe extern "C" fn fmtobject(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut obj: *mut js_Object,
    mut gap: *const libc::c_char,
    mut level: i32,
) {
    let mut key: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut save: i32 = 0;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    n = js_gettop(J) - 1;
    i = 4;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as i32)) {
            js_typeerror(
                J,
                b"cyclic object value\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
    }
    n = 0;
    js_putc(J, sb, '{' as i32);
    js_pushiterator(J, -(1 as i32), 1);
    loop {
        key = js_nextiterator(J, -(1 as i32));
        if key.is_null() {
            break;
        }
        if filterprop(J, key) != 0 {
            save = (**sb).s.len() as i32;
            if n != 0 {
                js_putc(J, sb, ',' as i32);
            }
            if !gap.is_null() {
                fmtindent(J, sb, gap, level + 1);
            }
            fmtstr(J, sb, key);
            js_putc(J, sb, ':' as i32);
            if !gap.is_null() {
                js_putc(J, sb, ' ' as i32);
            }
            js_rot2(J);
            if fmtvalue(J, sb, key, gap, level + 1) == 0 {
                (**sb).s.set_len(save as usize);
            } else {
                n += 1;
            }
            js_rot2(J);
        }
    }
    js_pop(J, 1);
    if !gap.is_null() && n != 0 {
        fmtindent(J, sb, gap, level);
    }
    js_putc(J, sb, '}' as i32);
}
unsafe extern "C" fn fmtarray(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut gap: *const libc::c_char,
    mut level: i32,
) {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut buf: [libc::c_char; 32] = [0; 32];
    n = js_gettop(J) - 1;
    i = 4;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as i32)) {
            js_typeerror(
                J,
                b"cyclic object value\0" as *const u8 as *const libc::c_char,
            );
        }
        i += 1;
    }
    js_putc(J, sb, '[' as i32);
    n = js_getlength(J, -(1 as i32));
    i = 0;
    while i < n {
        if i != 0 {
            js_putc(J, sb, ',' as i32);
        }
        if !gap.is_null() {
            fmtindent(J, sb, gap, level + 1);
        }
        if fmtvalue(J, sb, js_itoa(buf.as_mut_ptr(), i), gap, level + 1) == 0 {
            js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    if !gap.is_null() && n != 0 {
        fmtindent(J, sb, gap, level);
    }
    js_putc(J, sb, ']' as i32);
}
unsafe extern "C" fn fmtvalue(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut key: *const libc::c_char,
    mut gap: *const libc::c_char,
    mut level: i32,
) -> i32 {
    js_getproperty(J, -(1 as i32), key);
    if js_isobject(J, -(1 as i32)) != 0
        && js_hasproperty(
            J,
            -(1 as i32),
            b"toJSON\0" as *const u8 as *const libc::c_char,
        ) != 0
    {
        if js_iscallable(J, -(1 as i32)) != 0 {
            js_copy(J, -(2 as i32));
            js_pushstring(J, key);
            js_call(J, 1);
            js_rot2pop1(J);
        } else {
            js_pop(J, 1);
        }
    }
    if js_iscallable(J, 2) != 0 {
        js_copy(J, 2);
        js_copy(J, -(3 as i32));
        js_pushstring(J, key);
        js_copy(J, -(4 as i32));
        js_call(J, 2);
        js_rot2pop1(J);
    }
    if js_isobject(J, -(1 as i32)) != 0 && js_iscallable(J, -(1 as i32)) == 0 {
        let mut obj: *mut js_Object = js_toobject(J, -(1 as i32));
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
    } else if js_isboolean(J, -(1 as i32)) != 0 {
        let b = js_toboolean(J, -(1 as i32));
        js_puts(
            J,
            sb,
            if b != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            },
        );
    } else if js_isnumber(J, -(1 as i32)) != 0 {
        let n = js_tonumber(J, -(1 as i32));
        fmtnum(J, sb, n);
    } else if js_isstring(J, -(1 as i32)) != 0 {
        let s = js_tostring(J, -(1 as i32));
        fmtstr(J, sb, s);
    } else if js_isnull(J, -(1 as i32)) != 0 {
        js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
    } else {
        js_pop(J, 1);
        return 0;
    }
    js_pop(J, 1);
    1
}
unsafe extern "C" fn JSON_stringify(J: &mut js_State) {
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut buf: [libc::c_char; 12] = [0; 12];
    let mut gap: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut n: i32 = 0;
    ::core::ptr::write_volatile(
        &mut gap as *mut *const libc::c_char,
        core::ptr::null::<libc::c_char>(),
    );
    if js_isnumber(J, 3) != 0 || js_isnumberobject(J, 3) != 0 {
        n = js_tointeger(J, 3);
        if n < 0 {
            n = 0;
        }
        if n > 10 {
            n = 10;
        }
        memset(buf.as_mut_ptr() as *mut libc::c_void, ' ' as i32, n as u32);
        buf[n as usize] = 0;
        if n > 0 {
            ::core::ptr::write_volatile(&mut gap as *mut *const libc::c_char, buf.as_mut_ptr());
        }
    } else if js_isstring(J, 3) != 0 || js_isstringobject(J, 3) != 0 {
        s = js_tostring(J, 3);
        n = strlen(s) as i32;
        if n > 10 {
            n = 10;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            n as u32,
        );
        buf[n as usize] = 0;
        if n > 0 {
            ::core::ptr::write_volatile(&mut gap as *mut *const libc::c_char, buf.as_mut_ptr());
        }
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    js_newobject(J);
    js_copy(J, 1);
    js_defproperty(J, -(2 as i32), b"\0" as *const u8 as *const libc::c_char, 0);
    if fmtvalue(
        J,
        &mut sb,
        b"\0" as *const u8 as *const libc::c_char,
        gap,
        0,
    ) == 0
    {
        js_pushundefined(J);
    } else {
        js_putc(J, &mut sb, 0);
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
pub unsafe extern "C" fn jsB_initjson(J: &mut js_State) {
    let v = jsV_newobject(J, JS_CJSON, (*J).Object_prototype);
    js_pushobject(J, v);
    jsB_propf(
        J,
        b"JSON.parse\0" as *const u8 as *const libc::c_char,
        Some(JSON_parse),
        2,
    );
    jsB_propf(
        J,
        b"JSON.stringify\0" as *const u8 as *const libc::c_char,
        Some(JSON_stringify),
        3,
    );
    js_defglobal(
        J,
        b"JSON\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
unsafe extern "C" fn jsP_error(J: &mut js_State, mut fmt: *const libc::c_char, mut args: ...) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut msgbuf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(msgbuf.as_mut_ptr(), 256 as i32 as u32, fmt, ap.as_va_list());
    snprintf(
        buf.as_mut_ptr(),
        256 as i32 as u32,
        b"%s:%d: \0" as *const u8 as *const libc::c_char,
        (*J).filename,
        (*J).lexline,
    );
    strcat(buf.as_mut_ptr(), msgbuf.as_mut_ptr());
    js_newsyntaxerror(J, buf.as_mut_ptr());
    js_throw(J);
}
unsafe extern "C" fn jsP_warning(J: &mut js_State, mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut msg: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
        fmt,
        ap.as_va_list(),
    );
    snprintf(
        buf.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 512]>() as u32,
        b"%s:%d: warning: %s\0" as *const u8 as *const libc::c_char,
        (*J).filename,
        (*J).lexline,
        msg.as_mut_ptr(),
    );
    js_report(J, buf.as_mut_ptr());
}
unsafe extern "C" fn jsP_newnode(
    J: &mut js_State,
    mut type_0: js_AstType,
    mut line: i32,
    mut a: *mut js_Ast,
    mut b: *mut js_Ast,
    mut c: *mut js_Ast,
    mut d: *mut js_Ast,
) -> *mut js_Ast {
    let mut node: *mut js_Ast =
        js_malloc(J, ::core::mem::size_of::<js_Ast>() as u32 as i32) as *mut js_Ast;
    (*node).type_0 = type_0;
    (*node).line = line;
    (*node).a = a;
    (*node).b = b;
    (*node).c = c;
    (*node).d = d;
    (*node).number = 0.0;
    (*node).string = core::ptr::null::<libc::c_char>();
    (*node).jumps = core::ptr::null_mut::<js_JumpList>();
    (*node).casejump = 0;
    (*node).parent = core::ptr::null_mut::<js_Ast>();
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
unsafe extern "C" fn jsP_list(mut head: *mut js_Ast) -> *mut js_Ast {
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
    J: &mut js_State,
    mut type_0: js_AstType,
    mut s: *const libc::c_char,
) -> *mut js_Ast {
    let mut node: *mut js_Ast = jsP_newnode(
        J,
        type_0,
        (*J).lexline,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    (*node).string = s;
    node
}
unsafe extern "C" fn jsP_newnumnode(
    J: &mut js_State,
    mut type_0: js_AstType,
    mut n: f64,
) -> *mut js_Ast {
    let mut node: *mut js_Ast = jsP_newnode(
        J,
        type_0,
        (*J).lexline,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    (*node).number = n;
    node
}
unsafe extern "C" fn jsP_freejumps(J: &mut js_State, mut node: *mut js_JumpList) {
    while !node.is_null() {
        let mut next_0: *mut js_JumpList = (*node).next;
        js_free(J, node as *mut libc::c_void);
        node = next_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsP_freeparse(J: &mut js_State) {
    let mut node: *mut js_Ast = (*J).gcast;
    while !node.is_null() {
        let mut next_0: *mut js_Ast = (*node).gcnext;
        jsP_freejumps(J, (*node).jumps);
        js_free(J, node as *mut libc::c_void);
        node = next_0;
    }
    (*J).gcast = core::ptr::null_mut::<js_Ast>();
}
unsafe extern "C" fn jsP_next(J: &mut js_State) {
    (*J).lookahead = jsY_lex(J);
}
unsafe extern "C" fn semicolon(J: &mut js_State) {
    if (*J).lookahead == ';' as i32 {
        jsP_next(J);
        return;
    }
    if (*J).newline != 0 || (*J).lookahead == '}' as i32 || (*J).lookahead == 0 {
        return;
    }
    jsP_error(
        J,
        b"unexpected token: %s (expected ';')\0" as *const u8 as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn identifier(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == TK_IDENTIFIER as i32 {
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
unsafe extern "C" fn identifieropt(J: &mut js_State) -> *mut js_Ast {
    if (*J).lookahead == TK_IDENTIFIER as i32 {
        return identifier(J);
    }
    core::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn identifiername(J: &mut js_State) -> *mut js_Ast {
    if J.lookahead == TK_IDENTIFIER as i32 || J.lookahead >= TK_BREAK as i32 {
        let mut a: *mut js_Ast = jsP_newstrnode(J, AST_IDENTIFIER, J.text);
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
unsafe extern "C" fn arrayelement(J: &mut js_State) -> *mut js_Ast {
    let mut line: i32 = (*J).lexline;
    if (*J).lookahead == ',' as i32 {
        return jsP_newnode(
            J,
            EXP_ELISION,
            line,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    assignment(J, 0)
}
unsafe extern "C" fn arrayliteral(J: &mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == ']' as i32 {
        return core::ptr::null_mut::<js_Ast>();
    }
    let a = arrayelement(J);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if (*J).lookahead != ']' as i32 {
            let a = arrayelement(J);
            (*tail).b = jsP_newnode(
                J,
                AST_LIST,
                0,
                a,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
            tail = (*tail).b;
        }
    }
    jsP_list(head)
}
unsafe extern "C" fn propname(J: &mut js_State) -> *mut js_Ast {
    let mut name: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == TK_NUMBER as i32 {
        name = jsP_newnumnode(J, EXP_NUMBER, (*J).number);
        jsP_next(J);
    } else if (*J).lookahead == TK_STRING as i32 {
        name = jsP_newstrnode(J, EXP_STRING, (*J).text);
        jsP_next(J);
    } else {
        name = identifiername(J);
    }
    name
}
unsafe extern "C" fn propassign(J: &mut js_State) -> *mut js_Ast {
    let mut name: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut value: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut arg: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut body: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = (*J).lexline;
    name = propname(J);
    if (*J).lookahead != ':' as i32
        && (*name).type_0 as libc::c_uint == AST_IDENTIFIER as i32 as libc::c_uint
    {
        if strcmp((*name).string, b"get\0" as *const u8 as *const libc::c_char) == 0 {
            name = propname(J);
            if if (*J).lookahead == '(' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
                1
            } else {
                0
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
                core::ptr::null_mut::<js_Ast>(),
                body,
                core::ptr::null_mut::<js_Ast>(),
            );
        }
        if strcmp((*name).string, b"set\0" as *const u8 as *const libc::c_char) == 0 {
            name = propname(J);
            if if (*J).lookahead == '(' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
                1
            } else {
                0
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
            let b = jsP_newnode(
                J,
                AST_LIST,
                0,
                arg,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
            return jsP_newnode(
                J,
                EXP_PROP_SET,
                line,
                name,
                b,
                body,
                core::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    if if (*J).lookahead == ':' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring(':' as i32),
        );
    }
    value = assignment(J, 0);
    jsP_newnode(
        J,
        EXP_PROP_VAL,
        line,
        name,
        value,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn objectliteral(J: &mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == '}' as i32 {
        return core::ptr::null_mut::<js_Ast>();
    }
    let a = propassign(J);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if (*J).lookahead == '}' as i32 {
            break;
        }
        let a = propassign(J);
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn parameters(J: &mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == ')' as i32 {
        return core::ptr::null_mut::<js_Ast>();
    }
    let a = identifier(J);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let a = identifier(J);
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn fundec(J: &mut js_State, mut line: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    a = identifier(J);
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1
    } else {
        0
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
        1
    } else {
        0
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
        AST_FUNDEC,
        line,
        a,
        b,
        c,
        core::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn funstm(J: &mut js_State, mut line: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    a = identifier(J);
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1
    } else {
        0
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
        1
    } else {
        0
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
    let b = jsP_newnode(J, EXP_FUN, line, a, b, c, core::ptr::null_mut::<js_Ast>());
    let a = jsP_newnode(
        J,
        EXP_VAR,
        line,
        a,
        b,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    jsP_newnode(
        J,
        STM_VAR,
        line,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn funexp(J: &mut js_State, mut line: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    a = identifieropt(J);
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1
    } else {
        0
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
        1
    } else {
        0
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
    jsP_newnode(J, EXP_FUN, line, a, b, c, core::ptr::null_mut::<js_Ast>())
}
unsafe extern "C" fn primary(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = (*J).lexline;
    if (*J).lookahead == TK_IDENTIFIER as i32 {
        a = jsP_newstrnode(J, EXP_IDENTIFIER, (*J).text);
        jsP_next(J);
        return a;
    }
    if (*J).lookahead == TK_STRING as i32 {
        a = jsP_newstrnode(J, EXP_STRING, (*J).text);
        jsP_next(J);
        return a;
    }
    if (*J).lookahead == TK_REGEXP as i32 {
        a = jsP_newstrnode(J, EXP_REGEXP, (*J).text);
        (*a).number = (*J).number;
        jsP_next(J);
        return a;
    }
    if (*J).lookahead == TK_NUMBER as i32 {
        a = jsP_newnumnode(J, EXP_NUMBER, (*J).number);
        jsP_next(J);
        return a;
    }
    if if (*J).lookahead == TK_THIS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_THIS,
            line,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_NULL as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_NULL,
            line,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_TRUE as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_TRUE,
            line,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_FALSE as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        return jsP_newnode(
            J,
            EXP_FALSE,
            line,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == '{' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = objectliteral(J);
        a = jsP_newnode(
            J,
            EXP_OBJECT,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        if if (*J).lookahead == '}' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
        1
    } else {
        0
    } != 0
    {
        let aa = arrayliteral(J);
        a = jsP_newnode(
            J,
            EXP_ARRAY,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        if if (*J).lookahead == ']' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
        1
    } else {
        0
    } != 0
    {
        a = expression(J, 0);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
unsafe extern "C" fn arguments(J: &mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == ')' as i32 {
        return core::ptr::null_mut::<js_Ast>();
    }
    let a = assignment(J, 0);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let a = assignment(J, 0);
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn newexp(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == TK_NEW as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = memberexp(J);
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            b = arguments(J);
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        }
        return jsP_newnode(
            J,
            EXP_NEW,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_FUNCTION as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        return funexp(J, line);
    }
    primary(J)
}
unsafe extern "C" fn memberexp(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = newexp(J);
    let mut line: i32 = 0;
    let mut SAVE: i32 = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '.' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = identifiername(J);
            a = jsP_newnode(
                J,
                EXP_MEMBER,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == '[' as i32 {
                jsP_next(J);
                1
            } else {
                0
            } == 0)
            {
                break;
            }
            let b = expression(J, 0);
            a = jsP_newnode(
                J,
                EXP_INDEX,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
            if if (*J).lookahead == ']' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
unsafe extern "C" fn callexp(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = newexp(J);
    let mut line: i32 = 0;
    let mut SAVE: i32 = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '.' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = identifiername(J);
            a = jsP_newnode(
                J,
                EXP_MEMBER,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == '[' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = expression(J, 0);
            a = jsP_newnode(
                J,
                EXP_INDEX,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
            if if (*J).lookahead == ']' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
                1
            } else {
                0
            } == 0)
            {
                break;
            }
            let b = arguments(J);
            a = jsP_newnode(
                J,
                EXP_CALL,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
unsafe extern "C" fn postfix(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = callexp(J);
    let mut line: i32 = (*J).lexline;
    if (*J).newline == 0
        && (if (*J).lookahead == TK_INC as i32 {
            jsP_next(J);
            1
        } else {
            0
        }) != 0
    {
        return jsP_newnode(
            J,
            EXP_POSTINC,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    if (*J).newline == 0
        && (if (*J).lookahead == TK_DEC as i32 {
            jsP_next(J);
            1
        } else {
            0
        }) != 0
    {
        return jsP_newnode(
            J,
            EXP_POSTDEC,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    a
}
unsafe extern "C" fn unary(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = (*J).lexline;
    (*J).astdepth += 1;
    if (*J).astdepth > 400 {
        jsP_error(
            J,
            b"too much recursion\0" as *const u8 as *const libc::c_char,
        );
    }
    if if (*J).lookahead == TK_DELETE as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = unary(J);
        a = jsP_newnode(
            J,
            EXP_DELETE,
            line,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_VOID as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = unary(J);
        a = jsP_newnode(
            J,
            EXP_VOID,
            line,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_TYPEOF as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = unary(J);
        a = jsP_newnode(
            J,
            EXP_TYPEOF,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_INC as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = unary(J);
        a = jsP_newnode(
            J,
            EXP_PREINC,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_DEC as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = unary(J);
        a = jsP_newnode(
            J,
            EXP_PREDEC,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '+' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = unary(J);
        a = jsP_newnode(
            J,
            EXP_POS,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '-' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = unary(J);
        a = jsP_newnode(
            J,
            EXP_NEG,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '~' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = unary(J);
        a = jsP_newnode(
            J,
            EXP_BITNOT,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == '!' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let aa = unary(J);
        a = jsP_newnode(
            J,
            EXP_LOGNOT,
            line,
            aa,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else {
        a = postfix(J);
    }
    (*J).astdepth -= 1;
    (*J).astdepth;
    a
}
unsafe extern "C" fn multiplicative(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = unary(J);
    let mut line: i32 = 0;
    let mut SAVE: i32 = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '*' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = unary(J);
            a = jsP_newnode(
                J,
                EXP_MUL,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == '/' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = unary(J);
            a = jsP_newnode(
                J,
                EXP_DIV,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == '%' as i32 {
                jsP_next(J);
                1
            } else {
                0
            } == 0)
            {
                break;
            }
            let b = unary(J);
            a = jsP_newnode(
                J,
                EXP_MOD,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn additive(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = multiplicative(J);
    let mut line: i32 = 0;
    let mut SAVE: i32 = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '+' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = multiplicative(J);
            a = jsP_newnode(
                J,
                EXP_ADD,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == '-' as i32 {
                jsP_next(J);
                1
            } else {
                0
            } == 0)
            {
                break;
            }
            let b = multiplicative(J);
            a = jsP_newnode(
                J,
                EXP_SUB,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn shift(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = additive(J);
    let mut line: i32 = 0;
    let mut SAVE: i32 = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == TK_SHL as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = additive(J);
            a = jsP_newnode(
                J,
                EXP_SHL,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_SHR as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = additive(J);
            a = jsP_newnode(
                J,
                EXP_SHR,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == TK_USHR as i32 {
                jsP_next(J);
                1
            } else {
                0
            } == 0)
            {
                break;
            }
            let b = additive(J);
            a = jsP_newnode(
                J,
                EXP_USHR,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn relational(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = shift(J);
    let mut line: i32 = 0;
    let mut SAVE: i32 = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == '<' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = shift(J);
            a = jsP_newnode(
                J,
                EXP_LT,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == '>' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = shift(J);
            a = jsP_newnode(
                J,
                EXP_GT,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_LE as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = shift(J);
            a = jsP_newnode(
                J,
                EXP_LE,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_GE as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = shift(J);
            a = jsP_newnode(
                J,
                EXP_GE,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_INSTANCEOF as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = shift(J);
            a = jsP_newnode(
                J,
                EXP_INSTANCEOF,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if !(notin == 0
                && (if (*J).lookahead == TK_IN as i32 {
                    jsP_next(J);
                    1
                } else {
                    0
                }) != 0)
            {
                break;
            }
            let b = shift(J);
            a = jsP_newnode(
                J,
                EXP_IN,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn equality(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = relational(J, notin);
    let mut line: i32 = 0;
    let mut SAVE: i32 = (*J).astdepth;
    loop {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        line = (*J).lexline;
        if if (*J).lookahead == TK_EQ as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = relational(J, notin);
            a = jsP_newnode(
                J,
                EXP_EQ,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_NE as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = relational(J, notin);
            a = jsP_newnode(
                J,
                EXP_NE,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else if if (*J).lookahead == TK_STRICTEQ as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            let b = relational(J, notin);
            a = jsP_newnode(
                J,
                EXP_STRICTEQ,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else {
            if (if (*J).lookahead == TK_STRICTNE as i32 {
                jsP_next(J);
                1
            } else {
                0
            } == 0)
            {
                break;
            }
            let b = relational(J, notin);
            a = jsP_newnode(
                J,
                EXP_STRICTNE,
                line,
                a,
                b,
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        }
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn bitand(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = equality(J, notin);
    let mut SAVE: i32 = (*J).astdepth;
    let mut line: i32 = (*J).lexline;
    while if (*J).lookahead == '&' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        let b = equality(J, notin);
        a = jsP_newnode(
            J,
            EXP_BITAND,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn bitxor(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = bitand(J, notin);
    let mut SAVE: i32 = (*J).astdepth;
    let mut line: i32 = (*J).lexline;
    while if (*J).lookahead == '^' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        let b = bitand(J, notin);
        a = jsP_newnode(
            J,
            EXP_BITXOR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn bitor(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = bitxor(J, notin);
    let mut SAVE: i32 = (*J).astdepth;
    let mut line: i32 = (*J).lexline;
    while if (*J).lookahead == '|' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        let b = bitxor(J, notin);
        a = jsP_newnode(
            J,
            EXP_BITOR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn logand(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = bitor(J, notin);
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == TK_AND as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        let b = logand(J, notin);
        a = jsP_newnode(
            J,
            EXP_LOGAND,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        (*J).astdepth -= 1;
        (*J).astdepth;
    }
    a
}
unsafe extern "C" fn logor(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = logand(J, notin);
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == TK_OR as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        let b = logor(J, notin);
        a = jsP_newnode(
            J,
            EXP_LOGOR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        (*J).astdepth -= 1;
        (*J).astdepth;
    }
    a
}
unsafe extern "C" fn conditional(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = logor(J, notin);
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == '?' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
        let mut c: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        b = assignment(J, 0);
        if if (*J).lookahead == ':' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
        return jsP_newnode(J, EXP_COND, line, a, b, c, core::ptr::null_mut::<js_Ast>());
    }
    a
}
unsafe extern "C" fn assignment(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = conditional(J, notin);
    let mut line: i32 = (*J).lexline;
    (*J).astdepth += 1;
    if (*J).astdepth > 400 {
        jsP_error(
            J,
            b"too much recursion\0" as *const u8 as *const libc::c_char,
        );
    }
    if if (*J).lookahead == '=' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_MUL_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_MUL,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_DIV_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_DIV,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_MOD_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_MOD,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_ADD_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_ADD,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SUB_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_SUB,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SHL_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_SHL,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SHR_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_SHR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_USHR_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_USHR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_AND_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_BITAND,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_XOR_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_BITXOR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_OR_ASS as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_ASS_BITOR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    (*J).astdepth -= 1;
    (*J).astdepth;
    a
}
unsafe extern "C" fn expression(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = assignment(J, notin);
    let mut SAVE: i32 = (*J).astdepth;
    let mut line: i32 = (*J).lexline;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        (*J).astdepth += 1;
        if (*J).astdepth > 400 {
            jsP_error(
                J,
                b"too much recursion\0" as *const u8 as *const libc::c_char,
            );
        }
        let b = assignment(J, notin);
        a = jsP_newnode(
            J,
            EXP_COMMA,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        line = (*J).lexline;
    }
    (*J).astdepth = SAVE;
    a
}
unsafe extern "C" fn vardec(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = identifier(J);
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == '=' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let b = assignment(J, notin);
        return jsP_newnode(
            J,
            EXP_VAR,
            line,
            a,
            b,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    jsP_newnode(
        J,
        EXP_VAR,
        line,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn vardeclist(J: &mut js_State, mut notin: i32) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let a = vardec(J, notin);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while if (*J).lookahead == ',' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        let a = vardec(J, notin);
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn statementlist(J: &mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == '}' as i32
        || (*J).lookahead == TK_CASE as i32
        || (*J).lookahead == TK_DEFAULT as i32
    {
        return core::ptr::null_mut::<js_Ast>();
    }
    let a = statement(J);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while (*J).lookahead != '}' as i32
        && (*J).lookahead != TK_CASE as i32
        && (*J).lookahead != TK_DEFAULT as i32
    {
        let a = statement(J);
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn caseclause(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == TK_CASE as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = expression(J, 0);
        if if (*J).lookahead == ':' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    if if (*J).lookahead == TK_DEFAULT as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if if (*J).lookahead == ':' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    jsP_error(
        J,
        b"unexpected token in switch: %s (expected 'case' or 'default')\0" as *const u8
            as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn caselist(J: &mut js_State) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == '}' as i32 {
        return core::ptr::null_mut::<js_Ast>();
    }
    let a = caseclause(J);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while (*J).lookahead != '}' as i32 {
        let a = caseclause(J);
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn block(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == '{' as i32 {
        jsP_next(J);
        1
    } else {
        0
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
        1
    } else {
        0
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
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    )
}
unsafe extern "C" fn forexpression(J: &mut js_State, mut end: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead != end {
        a = expression(J, 0);
    }
    if if (*J).lookahead == end {
        jsP_next(J);
        1
    } else {
        0
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
unsafe extern "C" fn forstatement(J: &mut js_State, mut line: i32) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut d: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if if (*J).lookahead == '(' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } == 0
    {
        jsP_error(
            J,
            b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
            jsY_tokenstring('(' as i32),
        );
    }
    if if (*J).lookahead == TK_VAR as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = vardeclist(J, 1);
        if if (*J).lookahead == ';' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            b = forexpression(J, ';' as i32);
            c = forexpression(J, ')' as i32);
            d = statement(J);
            return jsP_newnode(J, STM_FOR_VAR, line, a, b, c, d);
        }
        if if (*J).lookahead == TK_IN as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            b = expression(J, 0);
            if if (*J).lookahead == ')' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
                core::ptr::null_mut::<js_Ast>(),
            );
        }
        jsP_error(
            J,
            b"unexpected token in for-var-statement: %s\0" as *const u8 as *const libc::c_char,
            jsY_tokenstring((*J).lookahead),
        );
    }
    if (*J).lookahead != ';' as i32 {
        a = expression(J, 1);
    } else {
        a = core::ptr::null_mut::<js_Ast>();
    }
    if if (*J).lookahead == ';' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        b = forexpression(J, ';' as i32);
        c = forexpression(J, ')' as i32);
        d = statement(J);
        return jsP_newnode(J, STM_FOR, line, a, b, c, d);
    }
    if if (*J).lookahead == TK_IN as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        b = expression(J, 0);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
            STM_FOR_IN,
            line,
            a,
            b,
            c,
            core::ptr::null_mut::<js_Ast>(),
        );
    }
    jsP_error(
        J,
        b"unexpected token in for-statement: %s\0" as *const u8 as *const libc::c_char,
        jsY_tokenstring((*J).lookahead),
    );
}
unsafe extern "C" fn statement(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut b: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut c: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut d: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut stm: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = (*J).lexline;
    (*J).astdepth += 1;
    if (*J).astdepth > 400 {
        jsP_error(
            J,
            b"too much recursion\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*J).lookahead == '{' as i32 {
        stm = block(J);
    } else if if (*J).lookahead == TK_VAR as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = vardeclist(J, 0);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_VAR,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == ';' as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        stm = jsP_newnode(
            J,
            STM_EMPTY,
            line,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_IF as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
        if if (*J).lookahead == TK_ELSE as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            c = statement(J);
        } else {
            c = core::ptr::null_mut::<js_Ast>();
        }
        stm = jsP_newnode(J, STM_IF, line, a, b, c, core::ptr::null_mut::<js_Ast>());
    } else if if (*J).lookahead == TK_DO as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = statement(J);
        if if (*J).lookahead == TK_WHILE as i32 {
            jsP_next(J);
            1
        } else {
            0
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring(TK_WHILE as i32),
            );
        }
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        b = expression(J, 0);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_WHILE as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_FOR as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        stm = forstatement(J, line);
    } else if if (*J).lookahead == TK_CONTINUE as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = identifieropt(J);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_CONTINUE,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_BREAK as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = identifieropt(J);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_BREAK,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_RETURN as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if (*J).lookahead != ';' as i32 && (*J).lookahead != '}' as i32 && (*J).lookahead != 0 {
            a = expression(J, 0);
        } else {
            a = core::ptr::null_mut::<js_Ast>();
        }
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_RETURN,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_WITH as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_SWITCH as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        if if (*J).lookahead == '(' as i32 {
            jsP_next(J);
            1
        } else {
            0
        } == 0
        {
            jsP_error(
                J,
                b"unexpected token: %s (expected %s)\0" as *const u8 as *const libc::c_char,
                jsY_tokenstring((*J).lookahead),
                jsY_tokenstring('(' as i32),
            );
        }
        a = expression(J, 0);
        if if (*J).lookahead == ')' as i32 {
            jsP_next(J);
            1
        } else {
            0
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
            1
        } else {
            0
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
            1
        } else {
            0
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
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_THROW as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = expression(J, 0);
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_THROW,
            line,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_TRY as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        a = block(J);
        d = core::ptr::null_mut::<js_Ast>();
        c = d;
        b = c;
        if if (*J).lookahead == TK_CATCH as i32 {
            jsP_next(J);
            1
        } else {
            0
        } != 0
        {
            if if (*J).lookahead == '(' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
                1
            } else {
                0
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
        if if (*J).lookahead == TK_FINALLY as i32 {
            jsP_next(J);
            1
        } else {
            0
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
    } else if if (*J).lookahead == TK_DEBUGGER as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        semicolon(J);
        stm = jsP_newnode(
            J,
            STM_DEBUGGER,
            line,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
    } else if if (*J).lookahead == TK_FUNCTION as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        jsP_warning(
            J,
            b"function statements are not standard\0" as *const u8 as *const libc::c_char,
        );
        stm = funstm(J, line);
    } else if (*J).lookahead == TK_IDENTIFIER as i32 {
        a = expression(J, 0);
        if (*a).type_0 as libc::c_uint == EXP_IDENTIFIER as i32 as libc::c_uint
            && (if (*J).lookahead == ':' as i32 {
                jsP_next(J);
                1
            } else {
                0
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
                core::ptr::null_mut::<js_Ast>(),
                core::ptr::null_mut::<js_Ast>(),
            );
        } else {
            semicolon(J);
            stm = a;
        }
    } else {
        stm = expression(J, 0);
        semicolon(J);
    }
    (*J).astdepth -= 1;
    (*J).astdepth;
    stm
}
unsafe extern "C" fn scriptelement(J: &mut js_State) -> *mut js_Ast {
    let mut line: i32 = (*J).lexline;
    if if (*J).lookahead == TK_FUNCTION as i32 {
        jsP_next(J);
        1
    } else {
        0
    } != 0
    {
        return fundec(J, line);
    }
    statement(J)
}
unsafe extern "C" fn script(J: &mut js_State, mut terminator: i32) -> *mut js_Ast {
    let mut head: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut tail: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if (*J).lookahead == terminator {
        return core::ptr::null_mut::<js_Ast>();
    }
    let a = scriptelement(J);
    tail = jsP_newnode(
        J,
        AST_LIST,
        0,
        a,
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
    );
    head = tail;
    while (*J).lookahead != terminator {
        let a = scriptelement(J);
        (*tail).b = jsP_newnode(
            J,
            AST_LIST,
            0,
            a,
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
            core::ptr::null_mut::<js_Ast>(),
        );
        tail = (*tail).b;
    }
    jsP_list(head)
}
unsafe extern "C" fn funbody(J: &mut js_State) -> *mut js_Ast {
    let mut a: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    if if (*J).lookahead == '{' as i32 {
        jsP_next(J);
        1
    } else {
        0
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
        1
    } else {
        0
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
unsafe extern "C" fn toint32(mut d: f64) -> i32 {
    let mut two32: f64 = 4294967296.0f64;
    let mut two31: f64 = 2147483648.0f64;
    if d.is_finite() as i32 == 0 || d == 0.0 {
        return 0;
    }
    d = fmod(d, two32);
    d = if d >= 0.0 { floor(d) } else { ceil(d) + two32 };
    if d >= two31 {
        (d - two32) as i32
    } else {
        d as i32
    }
}
unsafe extern "C" fn touint32(mut d: f64) -> libc::c_uint {
    toint32(d) as libc::c_uint
}
unsafe extern "C" fn jsP_setnumnode(mut node: *mut js_Ast, mut x: f64) -> i32 {
    (*node).type_0 = EXP_NUMBER;
    (*node).number = x;
    (*node).d = core::ptr::null_mut::<js_Ast>();
    (*node).c = (*node).d;
    (*node).b = (*node).c;
    (*node).a = (*node).b;
    1
}
unsafe extern "C" fn jsP_foldconst(mut node: *mut js_Ast) -> i32 {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    if (*node).type_0 as libc::c_uint == AST_LIST as i32 as libc::c_uint {
        while !node.is_null() {
            jsP_foldconst((*node).a);
            node = (*node).b;
        }
        return 0;
    }
    if (*node).type_0 as libc::c_uint == EXP_NUMBER as i32 as libc::c_uint {
        return 1;
    }
    a = if !((*node).a).is_null() {
        jsP_foldconst((*node).a)
    } else {
        0
    };
    b = if !((*node).b).is_null() {
        jsP_foldconst((*node).b)
    } else {
        0
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
            31 => return jsP_setnumnode(node, !toint32(x) as f64),
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
                        (toint32(x) << (touint32(y) & 0x1f as i32 as libc::c_uint)) as f64,
                    );
                }
                39 => {
                    return jsP_setnumnode(
                        node,
                        (toint32(x) >> (touint32(y) & 0x1f as i32 as libc::c_uint)) as f64,
                    );
                }
                38 => {
                    return jsP_setnumnode(
                        node,
                        (touint32(x) >> (touint32(y) & 0x1f as i32 as libc::c_uint)) as f64,
                    );
                }
                51 => {
                    return jsP_setnumnode(node, (toint32(x) & toint32(y)) as f64);
                }
                52 => {
                    return jsP_setnumnode(node, (toint32(x) ^ toint32(y)) as f64);
                }
                53 => {
                    return jsP_setnumnode(node, (toint32(x) | toint32(y)) as f64);
                }
                _ => {}
            }
        }
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn jsP_parse(
    J: &mut js_State,
    mut filename: *const libc::c_char,
    mut source: *const libc::c_char,
) -> *mut js_Ast {
    let mut p: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    jsY_initlex(J, filename, source);
    jsP_next(J);
    (*J).astdepth = 0;
    p = script(J, 0);
    if !p.is_null() {
        jsP_foldconst(p);
    }
    p
}
#[no_mangle]
pub unsafe extern "C" fn jsP_parsefunction(
    J: &mut js_State,
    mut filename: *const libc::c_char,
    mut params: *const libc::c_char,
    mut body: *const libc::c_char,
) -> *mut js_Ast {
    let mut p: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut line: i32 = 0;
    if !params.is_null() {
        jsY_initlex(J, filename, params);
        jsP_next(J);
        (*J).astdepth = 0;
        p = parameters(J);
    }
    let c = jsP_parse(J, filename, body);
    jsP_newnode(
        J,
        EXP_FUN,
        line,
        core::ptr::null_mut::<js_Ast>(),
        p,
        c,
        core::ptr::null_mut::<js_Ast>(),
    )
}
