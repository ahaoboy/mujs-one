use crate::*;

unsafe extern "C" fn reprnum(J: &mut js_State, mut sb: *mut *mut js_Buffer, mut n: f64) {
    let mut buf: [libc::c_char; 40] = [0; 40];
    if n == 0.0 && n.is_sign_negative() as i32 != 0 {
        js_puts(J, sb, b"-0\0" as *const u8 as *const libc::c_char);
    } else {
        js_puts(J, sb, jsV_numbertostring(buf.as_mut_ptr(), n));
    };
}
unsafe extern "C" fn reprstr(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut s: *const libc::c_char,
) {
    const HEX: *const libc::c_char = c"0123456789ABCDEF".as_ptr();
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
                if c < ' ' as i32 {
                    js_putc(J, sb, '\\' as i32);
                    js_putc(J, sb, 'x' as i32);
                    js_putc(J, sb, *HEX.offset((c >> 4 & 15 as i32) as isize) as i32);
                    js_putc(J, sb, *HEX.offset((c & 15 as i32) as isize) as i32);
                } else if c < 128 as i32 {
                    js_putc(J, sb, c);
                } else if c < 0x10000 {
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
unsafe extern "C" fn reprident(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut name: *const libc::c_char,
) {
    let mut p: *const libc::c_char = name;
    if *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
        while *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32 {
            p = p.offset(1);
        }
    } else if *p as i32 >= 'a' as i32 && *p as i32 <= 'z' as i32
        || *p as i32 >= 'A' as i32 && *p as i32 <= 'Z' as i32
        || *p as i32 == '_' as i32
    {
        while *p as i32 >= '0' as i32 && *p as i32 <= '9' as i32
            || (*p as i32 >= 'a' as i32 && *p as i32 <= 'z' as i32
                || *p as i32 >= 'A' as i32 && *p as i32 <= 'Z' as i32)
            || *p as i32 == '_' as i32
        {
            p = p.offset(1);
        }
    }
    if p > name && *p as i32 == 0 {
        js_puts(J, sb, name);
    } else {
        reprstr(J, sb, name);
    };
}
unsafe extern "C" fn reprobject(J: &mut js_State, mut sb: *mut *mut js_Buffer) {
    let mut key: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    n = js_gettop(J) - 1;
    i = 0;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as i32)) {
            js_puts(J, sb, b"{}\0" as *const u8 as *const libc::c_char);
            return;
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
        let fresh52 = n;
        n += 1;
        if fresh52 > 0 {
            js_puts(J, sb, b", \0" as *const u8 as *const libc::c_char);
        }
        reprident(J, sb, key);
        js_puts(J, sb, b": \0" as *const u8 as *const libc::c_char);
        js_getproperty(J, -(2 as i32), key);
        reprvalue(J, sb);
        js_pop(J, 1);
    }
    js_pop(J, 1);
    js_putc(J, sb, '}' as i32);
}
unsafe extern "C" fn reprarray(J: &mut js_State, mut sb: *mut *mut js_Buffer) {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    n = js_gettop(J) - 1;
    i = 0;
    while i < n {
        if js_isobject(J, i) != 0 && js_toobject(J, i) == js_toobject(J, -(1 as i32)) {
            js_puts(J, sb, b"[]\0" as *const u8 as *const libc::c_char);
            return;
        }
        i += 1;
    }
    js_putc(J, sb, '[' as i32);
    n = js_getlength(J, -(1 as i32));
    i = 0;
    while i < n {
        if i > 0 {
            js_puts(J, sb, b", \0" as *const u8 as *const libc::c_char);
        }
        if js_hasindex(J, -(1 as i32), i) != 0 {
            reprvalue(J, sb);
            js_pop(J, 1);
        }
        i += 1;
    }
    js_putc(J, sb, ']' as i32);
}
unsafe extern "C" fn reprfun(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut fun: *mut js_Function,
) {
    let mut i: i32 = 0;
    js_puts(J, sb, b"function \0" as *const u8 as *const libc::c_char);
    js_puts(J, sb, (*fun).name);
    js_putc(J, sb, '(' as i32);
    i = 0;
    while i < (*fun).numparams {
        if i > 0 {
            js_puts(J, sb, b", \0" as *const u8 as *const libc::c_char);
        }
        js_puts(J, sb, *((*fun).vartab).offset(i as isize));
        i += 1;
    }
    js_puts(
        J,
        sb,
        b") { [byte code] }\0" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn reprvalue(J: &mut js_State, mut sb: *mut *mut js_Buffer) {
    if js_isundefined(J, -(1 as i32)) != 0 {
        js_puts(J, sb, b"undefined\0" as *const u8 as *const libc::c_char);
    } else if js_isnull(J, -(1 as i32)) != 0 {
        js_puts(J, sb, b"null\0" as *const u8 as *const libc::c_char);
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
        reprnum(J, sb, n);
    } else if js_isstring(J, -(1 as i32)) != 0 {
        let s = js_tostring(J, -(1 as i32));
        reprstr(J, sb, s);
    } else if js_isobject(J, -(1 as i32)) != 0 {
        let mut obj: *mut js_Object = js_toobject(J, -(1 as i32));
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
                if (*obj).u.r.flags as i32 & JS_REGEXP_G as i32 != 0 {
                    js_putc(J, sb, 'g' as i32);
                }
                if (*obj).u.r.flags as i32 & JS_REGEXP_I as i32 != 0 {
                    js_putc(J, sb, 'i' as i32);
                }
                if (*obj).u.r.flags as i32 & JS_REGEXP_M as i32 != 0 {
                    js_putc(J, sb, 'm' as i32);
                }
            }
            10 => {
                let mut buf: [libc::c_char; 40] = [0; 40];
                js_puts(J, sb, b"(new Date(\0" as *const u8 as *const libc::c_char);
                js_puts(J, sb, jsV_numbertostring(buf.as_mut_ptr(), (*obj).u.number));
                js_puts(J, sb, b"))\0" as *const u8 as *const libc::c_char);
            }
            5 => {
                js_puts(J, sb, b"(new \0" as *const u8 as *const libc::c_char);
                js_getproperty(
                    J,
                    -(1 as i32),
                    b"name\0" as *const u8 as *const libc::c_char,
                );
                let s = js_tostring(J, -(1 as i32));
                js_puts(J, sb, s);
                js_pop(J, 1);
                js_putc(J, sb, '(' as i32);
                if js_hasproperty(
                    J,
                    -(1 as i32),
                    b"message\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    reprvalue(J, sb);
                    js_pop(J, 1);
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
pub unsafe extern "C" fn js_repr(J: &mut js_State, mut idx: i32) {
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut savebot: i32 = 0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    js_copy(J, idx);
    savebot = (*J).bot;
    (*J).bot = (*J).top - 1;
    reprvalue(J, &mut sb);
    (*J).bot = savebot;
    js_pop(J, 1);
    js_putc(J, &mut sb, 0);
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
pub unsafe extern "C" fn js_torepr(J: &mut js_State, mut idx: i32) -> *const libc::c_char {
    js_repr(J, idx);
    js_replace(J, if idx < 0 { idx - 1 } else { idx });
    js_tostring(J, idx)
}
#[no_mangle]
pub unsafe extern "C" fn js_tryrepr(
    J: &mut js_State,
    mut idx: i32,
    mut error: *const libc::c_char,
) -> *const libc::c_char {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_pop(J, 1);
        return error;
    }
    s = js_torepr(J, idx);
    js_endtry(J);
    s
}
