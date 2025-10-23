use crate::*;

pub unsafe extern "C" fn escaperegexp(
    J: &mut js_State,
    mut pattern: *const libc::c_char,
) -> *mut libc::c_char {
    let mut copy: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut n: i32 = 0;
    s = pattern;
    while *s != 0 {
        if *s as i32 == '/' as i32 {
            n += 1;
        }
        n += 1;

        s = s.offset(1);
    }
    p = js_malloc(J, n + 1) as *mut libc::c_char;
    copy = p;
    s = pattern;
    while *s != 0 {
        if *s as i32 == '/' as i32 {
            let fresh50 = p;
            p = p.offset(1);
            *fresh50 = '\\' as i32 as libc::c_char;
        }
        let fresh51 = p;
        p = p.offset(1);
        *fresh51 = *s;
        s = s.offset(1);
    }
    *p = 0;
    copy
}

pub unsafe extern "C" fn js_newregexpx(
    J: &mut js_State,
    mut pattern: *const libc::c_char,
    mut flags: i32,
    mut is_clone: i32,
) {
    let mut error: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut prog: *mut Reprog = core::ptr::null_mut::<Reprog>();
    let mut opts: i32 = 0;
    obj = jsV_newobject(J, JS_CREGEXP, (*J).RegExp_prototype);
    opts = 0;
    if flags & JS_REGEXP_I as i32 != 0 {
        opts |= REG_ICASE as i32;
    }
    if flags & JS_REGEXP_M as i32 != 0 {
        opts |= REG_NEWLINE as i32;
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
    (*obj).u.r.last = 0;
    js_pushobject(J, obj);
}

#[no_mangle]
pub unsafe extern "C" fn js_newregexp(
    J: &mut js_State,
    mut pattern: *const libc::c_char,
    mut flags: i32,
) {
    js_newregexpx(J, pattern, flags, 0);
}

#[no_mangle]
pub unsafe extern "C" fn js_RegExp_prototype_exec(
    J: &mut js_State,
    mut re: *mut js_Regexp,
    mut text: *const libc::c_char,
) {
    let mut haystack: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut result: i32 = 0;
    let mut i: i32 = 0;
    let mut opts: i32 = 0;
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    haystack = text;
    opts = 0;
    if (*re).flags as i32 & JS_REGEXP_G as i32 != 0 {
        if (*re).last as u32 > strlen(haystack) {
            (*re).last = 0;
            js_pushnull(J);
            return;
        }
        if (*re).last as i32 > 0 {
            haystack = text.offset((*re).last as i32 as isize);
            opts |= REG_NOTBOL as i32;
        }
    }
    result = js_regexec((*re).prog as *mut Reprog, haystack, &mut m, opts);
    if result < 0 {
        js_error(J, b"regexec failed\0" as *const u8 as *const libc::c_char);
    }
    if result == 0 {
        js_newarray(J);
        js_pushstring(J, text);
        js_setproperty(
            J,
            -(2 as i32),
            b"input\0" as *const u8 as *const libc::c_char,
        );
        js_pushnumber(J, js_utfptrtoidx(text, m.sub[0 as usize].sp) as f64);
        js_setproperty(
            J,
            -(2 as i32),
            b"index\0" as *const u8 as *const libc::c_char,
        );
        i = 0;
        while i < m.nsub {
            js_pushlstring(
                J,
                m.sub[i as usize].sp,
                (m.sub[i as usize].ep).offset_from(m.sub[i as usize].sp) as i32,
            );
            js_setindex(J, -(2 as i32), i);
            i += 1;
        }
        if (*re).flags as i32 & JS_REGEXP_G as i32 != 0 {
            (*re).last = (m.sub[0 as usize].ep).offset_from(text) as libc::c_ushort;
        }
        return;
    }
    if (*re).flags as i32 & JS_REGEXP_G as i32 != 0 {
        (*re).last = 0;
    }
    js_pushnull(J);
}

unsafe extern "C" fn Rp_test(J: &mut js_State) {
    let mut re: *mut js_Regexp = core::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut result: i32 = 0;
    let mut opts: i32 = 0;
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    re = js_toregexp(J, 0);
    text = js_tostring(J, 1);
    opts = 0;
    if (*re).flags as i32 & JS_REGEXP_G as i32 != 0 {
        if (*re).last as u32 > strlen(text) {
            (*re).last = 0;
            js_pushboolean(J, 0);
            return;
        }
        if (*re).last as i32 > 0 {
            text = text.offset((*re).last as i32 as isize);
            opts |= REG_NOTBOL as i32;
        }
    }
    result = js_regexec((*re).prog as *mut Reprog, text, &mut m, opts);
    if result < 0 {
        js_error(J, b"regexec failed\0" as *const u8 as *const libc::c_char);
    }
    if result == 0 {
        if (*re).flags as i32 & JS_REGEXP_G as i32 != 0 {
            (*re).last = ((*re).last as isize + (m.sub[0].ep).offset_from(text)) as libc::c_ushort;
        }
        js_pushboolean(J, 1);
        return;
    }
    if (*re).flags as i32 & JS_REGEXP_G as i32 != 0 {
        (*re).last = 0;
    }
    js_pushboolean(J, 0);
}
unsafe extern "C" fn jsB_new_RegExp(J: &mut js_State) {
    let mut old: *mut js_Regexp = core::ptr::null_mut::<js_Regexp>();
    let mut pattern: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut flags: i32 = 0;
    let mut is_clone: i32 = 0;
    if js_isregexp(J, 1) != 0 {
        if js_isdefined(J, 2) != 0 {
            js_typeerror(
                J,
                b"cannot supply flags when creating one RegExp from another\0" as *const u8
                    as *const libc::c_char,
            );
        }
        old = js_toregexp(J, 1);
        pattern = (*old).source;
        flags = (*old).flags as i32;
        is_clone = 1;
    } else if js_isundefined(J, 1) != 0 {
        pattern = b"(?:)\0" as *const u8 as *const libc::c_char;
        flags = 0;
    } else {
        pattern = js_tostring(J, 1);
        flags = 0;
    }
    if strlen(pattern) == 0 {
        pattern = b"(?:)\0" as *const u8 as *const libc::c_char;
    }
    if js_isdefined(J, 2) != 0 {
        let mut s: *const libc::c_char = js_tostring(J, 2);
        let mut g: i32 = 0;
        let mut i: i32 = 0;
        let mut m: i32 = 0;
        while *s != 0 {
            if *s as i32 == 'g' as i32 {
                g += 1;
            } else if *s as i32 == 'i' as i32 {
                i += 1;
            } else if *s as i32 == 'm' as i32 {
                m += 1;
            } else {
                js_syntaxerror(
                    J,
                    b"invalid regular expression flag: '%c'\0" as *const u8 as *const libc::c_char,
                    *s as i32,
                );
            }
            s = s.offset(1);
        }
        if g > 1 {
            js_syntaxerror(
                J,
                b"invalid regular expression flag: 'g'\0" as *const u8 as *const libc::c_char,
            );
        }
        if i > 1 {
            js_syntaxerror(
                J,
                b"invalid regular expression flag: 'i'\0" as *const u8 as *const libc::c_char,
            );
        }
        if m > 1 {
            js_syntaxerror(
                J,
                b"invalid regular expression flag: 'm'\0" as *const u8 as *const libc::c_char,
            );
        }
        if g != 0 {
            flags |= JS_REGEXP_G as i32;
        }
        if i != 0 {
            flags |= JS_REGEXP_I as i32;
        }
        if m != 0 {
            flags |= JS_REGEXP_M as i32;
        }
    }
    js_newregexpx(J, pattern, flags, is_clone);
}
unsafe extern "C" fn jsB_RegExp(J: &mut js_State) {
    if js_isregexp(J, 1) != 0 {
        return;
    }
    jsB_new_RegExp(J);
}
unsafe extern "C" fn Rp_toString(J: &mut js_State) {
    let mut re: *mut js_Regexp = core::ptr::null_mut::<js_Regexp>();
    let mut out: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    re = js_toregexp(J, 0);
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, out as *mut libc::c_void);
        js_throw(J);
    }
    ::core::ptr::write_volatile(
        &mut out as *mut *mut libc::c_char,
        js_malloc(
            J,
            (strlen((*re).source)).wrapping_add(6 as i32 as u32) as i32,
        ) as *mut libc::c_char,
    );
    strcpy(out, b"/\0" as *const u8 as *const libc::c_char);
    strcat(out, (*re).source);
    strcat(out, b"/\0" as *const u8 as *const libc::c_char);
    if (*re).flags as i32 & JS_REGEXP_G as i32 != 0 {
        strcat(out, b"g\0" as *const u8 as *const libc::c_char);
    }
    if (*re).flags as i32 & JS_REGEXP_I as i32 != 0 {
        strcat(out, b"i\0" as *const u8 as *const libc::c_char);
    }
    if (*re).flags as i32 & JS_REGEXP_M as i32 != 0 {
        strcat(out, b"m\0" as *const u8 as *const libc::c_char);
    }
    js_pop(J, 0);
    js_pushstring(J, out);
    js_endtry(J);
    js_free(J, out as *mut libc::c_void);
}
unsafe extern "C" fn Rp_exec(J: &mut js_State) {
    let re = js_toregexp(J, 0);
    let text = js_tostring(J, 1);
    js_RegExp_prototype_exec(J, re, text);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initregexp(J: &mut js_State) {
    js_pushobject(J, (*J).RegExp_prototype);
    jsB_propf(
        J,
        b"RegExp.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Rp_toString),
        0,
    );
    jsB_propf(
        J,
        b"RegExp.prototype.test\0" as *const u8 as *const libc::c_char,
        Some(Rp_test),
        0,
    );
    jsB_propf(
        J,
        b"RegExp.prototype.exec\0" as *const u8 as *const libc::c_char,
        Some(Rp_exec),
        0,
    );
    js_newcconstructor(
        J,
        Some(jsB_RegExp),
        Some(jsB_new_RegExp),
        b"RegExp\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"RegExp\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
