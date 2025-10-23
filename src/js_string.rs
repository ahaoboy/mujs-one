use crate::*;

unsafe extern "C" fn js_doregexec(
    J: &mut js_State,
    mut prog: *mut Reprog,
    mut string: *const libc::c_char,
    mut sub: *mut Resub,
    mut eflags: i32,
) -> i32 {
    let mut result: i32 = js_regexec(prog, string, sub, eflags);
    if result < 0 {
        js_error(J, b"regexec failed\0" as *const u8 as *const libc::c_char);
    }
    result
}
unsafe extern "C" fn checkstring(J: &mut js_State, mut idx: i32) -> *const libc::c_char {
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
    J: &mut js_State,
    mut s: *const libc::c_char,
    mut i: i32,
) -> i32 {
    let mut rune: Rune = -(1 as i32);
    while i >= 0 {
        rune = *(s as *mut libc::c_uchar) as Rune;
        if rune < Runeself as i32 {
            if rune == 0 {
                return -(1 as i32);
            }
            s = s.offset(1);

            i -= 1;
        } else {
            s = s.offset(jsU_chartorune(&mut rune, s) as isize);
            if rune >= 0x10000 {
                i -= 2;
            } else {
                i -= 1;
            }
        }
    }
    if rune >= 0x10000 {
        if i == -(2) {
            return 0xd800 + ((rune - 0x10000) >> 10);
        } else {
            return 0xdc00 + ((rune - 0x10000) & 0x3ff as i32);
        }
    }
    rune
}
#[no_mangle]
pub unsafe extern "C" fn js_utflen(mut s: *const libc::c_char) -> i32 {
    let mut c: i32 = 0;
    let mut n: i32 = 0;
    let mut rune: Rune = 0;
    n = 0;
    loop {
        c = *(s as *mut libc::c_uchar) as i32;
        if c < Runeself as i32 {
            if c == 0 {
                return n;
            }
            s = s.offset(1);

            n += 1;
        } else {
            s = s.offset(jsU_chartorune(&mut rune, s) as isize);
            if rune >= 0x10000 {
                n += 2;
            } else {
                n += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_utfptrtoidx(
    mut s: *const libc::c_char,
    mut p: *const libc::c_char,
) -> i32 {
    let mut rune: Rune = 0;
    let mut i: i32 = 0;
    while s < p {
        if (*(s as *mut libc::c_uchar) as i32) < Runeself as i32 {
            s = s.offset(1);
        } else {
            s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        }
        if rune >= 0x10000 {
            i += 2;
        } else {
            i += 1;
        }
    }
    i
}
unsafe extern "C" fn jsB_new_String(J: &mut js_State) {
    let v = if js_gettop(J) > 1 {
        js_tostring(J, 1)
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    js_newstring(J, v);
}
unsafe extern "C" fn jsB_String(J: &mut js_State) {
    let v = if js_gettop(J) > 1 {
        js_tostring(J, 1)
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    js_pushstring(J, v);
}
unsafe extern "C" fn Sp_toString(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    if (*self_0).type_0 as libc::c_uint != JS_CSTRING as i32 as libc::c_uint {
        js_typeerror(J, b"not a string\0" as *const u8 as *const libc::c_char);
    }
    js_pushstring(J, (*self_0).u.s.string);
}
unsafe extern "C" fn Sp_valueOf(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    if (*self_0).type_0 as libc::c_uint != JS_CSTRING as i32 as libc::c_uint {
        js_typeerror(J, b"not a string\0" as *const u8 as *const libc::c_char);
    }
    js_pushstring(J, (*self_0).u.s.string);
}
unsafe extern "C" fn Sp_charAt(J: &mut js_State) {
    let mut buf: [libc::c_char; 5] = [0; 5];
    let mut s: *const libc::c_char = checkstring(J, 0);
    let mut pos: i32 = js_tointeger(J, 1);
    let mut rune: Rune = js_runeat(J, s, pos);
    if rune >= 0 {
        buf[jsU_runetochar(buf.as_mut_ptr(), &mut rune) as usize] = 0;
        js_pushstring(J, buf.as_mut_ptr());
    } else {
        js_pushliteral(J, b"\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn Sp_charCodeAt(J: &mut js_State) {
    let mut s: *const libc::c_char = checkstring(J, 0);
    let mut pos: i32 = js_tointeger(J, 1);
    let mut rune: Rune = js_runeat(J, s, pos);
    if rune >= 0 {
        js_pushnumber(J, rune as f64);
    } else {
        js_pushnumber(J, ::core::f32::NAN as f64);
    };
}
unsafe extern "C" fn Sp_concat(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    let mut n: i32 = 0;
    let mut out: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    if top == 1 {
        return;
    }
    s = checkstring(J, 0);
    n = (1 as i32 as u32).wrapping_add(strlen(s)) as i32;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, out as *mut libc::c_void);
        js_throw(J);
    }
    if n > (1 as i32) << 28 as i32 {
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
    i = 1;
    while i < top {
        s = js_tostring(J, i);
        n = (n as u32).wrapping_add(strlen(s)) as i32 as i32;
        if n > (1 as i32) << 28 as i32 {
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
    }
    js_pushstring(J, out);
    js_endtry(J);
    js_free(J, out as *mut libc::c_void);
}
unsafe extern "C" fn Sp_indexOf(J: &mut js_State) {
    let mut haystack: *const libc::c_char = checkstring(J, 0);
    let mut needle: *const libc::c_char = js_tostring(J, 1);
    let mut pos: i32 = js_tointeger(J, 2);
    let mut len: i32 = strlen(needle) as i32;
    let mut k: i32 = 0;
    let mut rune: Rune = 0;
    while *haystack != 0 {
        if k >= pos && strncmp(haystack, needle, len as u32) == 0 {
            js_pushnumber(J, k as f64);
            return;
        }
        haystack = haystack.offset(jsU_chartorune(&mut rune, haystack) as isize);
        k += 1;
    }
    js_pushnumber(J, -(1 as i32) as f64);
}
unsafe extern "C" fn Sp_lastIndexOf(J: &mut js_State) {
    let mut haystack: *const libc::c_char = checkstring(J, 0);
    let mut needle: *const libc::c_char = js_tostring(J, 1);
    let mut pos: i32 = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        strlen(haystack) as i32
    };
    let mut len: i32 = strlen(needle) as i32;
    let mut k: i32 = 0;
    let mut last: i32 = -(1 as i32);
    let mut rune: Rune = 0;
    while *haystack as i32 != 0 && k <= pos {
        if strncmp(haystack, needle, len as u32) == 0 {
            last = k;
        }
        haystack = haystack.offset(jsU_chartorune(&mut rune, haystack) as isize);
        k += 1;
    }
    js_pushnumber(J, last as f64);
}
unsafe extern "C" fn Sp_localeCompare(J: &mut js_State) {
    let mut a: *const libc::c_char = checkstring(J, 0);
    let mut b: *const libc::c_char = js_tostring(J, 1);
    js_pushnumber(J, strcmp(a, b) as f64);
}
unsafe extern "C" fn Sp_substring_imp(
    J: &mut js_State,
    mut s: *const libc::c_char,
    mut a: i32,
    mut n: i32,
) {
    let mut head_rune: Rune = 0;
    let mut tail_rune: Rune = 0;
    let mut head: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut tail: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut p: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut head_len: i32 = 0;
    let mut tail_len: i32 = 0;
    head = s;
    i = 0;
    while i < a {
        head = head.offset(jsU_chartorune(&mut head_rune, head) as isize);
        if head_rune >= 0x10000 {
            i += 1;
        }
        i += 1;
    }
    tail = head;
    k = i - a;
    while k < n {
        tail = tail.offset(jsU_chartorune(&mut tail_rune, tail) as isize);
        if tail_rune >= 0x10000 {
            k += 1;
        }
        k += 1;
    }
    if i == a && k == n {
        js_pushlstring(J, head, tail.offset_from(head) as i32);
        return;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, p as *mut libc::c_void);
        js_throw(J);
    }
    p = js_malloc(J, (UTFmax as isize + tail.offset_from(head)) as i32) as *mut libc::c_char;
    if i > a {
        head_rune = 0xdc00 + ((head_rune - 0x10000) & 0x3ff as i32);
        head_len = jsU_runetochar(p, &mut head_rune);
        memcpy(
            p.offset(head_len as isize) as *mut libc::c_void,
            head as *const libc::c_void,
            tail.offset_from(head) as u32,
        );
        js_pushlstring(J, p, (head_len as isize + tail.offset_from(head)) as i32);
    }
    if k > n {
        tail = tail.offset(-(jsU_runelen(tail_rune) as isize));
        memcpy(
            p as *mut libc::c_void,
            head as *const libc::c_void,
            tail.offset_from(head) as u32,
        );
        tail_rune = 0xd800 + ((tail_rune - 0x10000) >> 10);
        tail_len = jsU_runetochar(p.offset(tail.offset_from(head) as isize), &mut tail_rune);
        js_pushlstring(J, p, (tail.offset_from(head) + tail_len as isize) as i32);
    }
    js_endtry(J);
    js_free(J, p as *mut libc::c_void);
}
unsafe extern "C" fn Sp_slice(J: &mut js_State) {
    let mut str: *const libc::c_char = checkstring(J, 0);
    let mut len: i32 = js_utflen(str);
    let mut s: i32 = js_tointeger(J, 1);
    let mut e: i32 = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        len
    };
    s = if s < 0 { s + len } else { s };
    e = if e < 0 { e + len } else { e };
    s = if s < 0 {
        0
    } else if s > len {
        len
    } else {
        s
    };
    e = if e < 0 {
        0
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
unsafe extern "C" fn Sp_substring(J: &mut js_State) {
    let mut str: *const libc::c_char = checkstring(J, 0);
    let mut len: i32 = js_utflen(str);
    let mut s: i32 = js_tointeger(J, 1);
    let mut e: i32 = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        len
    };
    s = if s < 0 {
        0
    } else if s > len {
        len
    } else {
        s
    };
    e = if e < 0 {
        0
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
unsafe extern "C" fn Sp_toLowerCase(J: &mut js_State) {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut s0: *const libc::c_char = checkstring(J, 0);
    let mut dst: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut d: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut rune: Rune = 0;
    let mut full: *const Rune = core::ptr::null::<Rune>();
    let mut n: i32 = 0;
    n = 1;
    s = s0;
    while *s != 0 {
        s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        full = jsU_tolowerrune_full(rune);
        if !full.is_null() {
            while *full != 0 {
                n += jsU_runelen(*full);
                full = full.offset(1);
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
            }
        } else {
            rune = jsU_tolowerrune(rune);
            d = d.offset(jsU_runetochar(d, &mut rune) as isize);
        }
    }
    *d = 0;
    js_pushstring(J, dst);
    js_endtry(J);
    js_free(J, dst as *mut libc::c_void);
}
unsafe extern "C" fn Sp_toUpperCase(J: &mut js_State) {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut s0: *const libc::c_char = checkstring(J, 0);
    let mut dst: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut d: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut full: *const Rune = core::ptr::null::<Rune>();
    let mut rune: Rune = 0;
    let mut n: i32 = 0;
    n = 1;
    s = s0;
    while *s != 0 {
        s = s.offset(jsU_chartorune(&mut rune, s) as isize);
        full = jsU_toupperrune_full(rune);
        if !full.is_null() {
            while *full != 0 {
                n += jsU_runelen(*full);
                full = full.offset(1);
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
            }
        } else {
            rune = jsU_toupperrune(rune);
            d = d.offset(jsU_runetochar(d, &mut rune) as isize);
        }
    }
    *d = 0;
    js_pushstring(J, dst);
    js_endtry(J);
    js_free(J, dst as *mut libc::c_void);
}
unsafe extern "C" fn istrim(mut c: i32) -> i32 {
    (c == 0x9 as i32
        || c == 0xb as i32
        || c == 0xc as i32
        || c == 0x20
        || c == 0xa0
        || c == 0xfeff as i32
        || c == 0xa as i32
        || c == 0xd as i32
        || c == 0x2028 as i32
        || c == 0x2029 as i32) as i32
}
unsafe extern "C" fn Sp_trim(J: &mut js_State) {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut e: *const libc::c_char = core::ptr::null::<libc::c_char>();
    s = checkstring(J, 0);
    while istrim(*s as i32) != 0 {
        s = s.offset(1);
    }
    e = s.offset(strlen(s) as isize);
    while e > s && istrim(*e.offset(-(1 as i32) as isize) as i32) != 0 {
        e = e.offset(-1);
    }
    js_pushlstring(J, s, e.offset_from(s) as i32);
}
unsafe extern "C" fn S_fromCharCode(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    let mut s: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut p: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut c: Rune = 0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, s as *mut libc::c_void);
        js_throw(J);
    }
    p = js_malloc(J, (top - 1) * UTFmax as i32 + 1) as *mut libc::c_char;
    ::core::ptr::write_volatile(&mut s as *mut *mut libc::c_char, p);
    i = 1;
    while i < top {
        c = js_touint32(J, i) as Rune;
        p = p.offset(jsU_runetochar(p, &mut c) as isize);
        i += 1;
    }
    *p = 0;
    js_pushstring(J, s);
    js_endtry(J);
    js_free(J, s as *mut libc::c_void);
}
unsafe extern "C" fn Sp_match(J: &mut js_State) {
    let mut re: *mut js_Regexp = core::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut len: i32 = 0;
    let mut a: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut b: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut c: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut e: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    text = checkstring(J, 0);
    if js_isregexp(J, 1) != 0 {
        js_copy(J, 1);
    } else if js_isundefined(J, 1) != 0 {
        js_newregexp(J, b"\0" as *const u8 as *const libc::c_char, 0);
    } else {
        let pat = js_tostring(J, 1);
        js_newregexp(J, pat, 0);
    }
    re = js_toregexp(J, -(1 as i32));
    if (*re).flags as i32 & JS_REGEXP_G as i32 == 0 {
        js_RegExp_prototype_exec(J, re, text);
        return;
    }
    (*re).last = 0;
    js_newarray(J);
    len = 0;
    a = text;
    e = text.offset(strlen(text) as isize);
    while a <= e {
        if js_doregexec(
            J,
            (*re).prog as *mut Reprog,
            a,
            &mut m,
            if a > text { REG_NOTBOL as i32 } else { 0 },
        ) != 0
        {
            break;
        }
        b = m.sub[0].sp;
        c = m.sub[0].ep;
        js_pushlstring(J, b, c.offset_from(b) as i32);
        let fresh89 = len;
        len += 1;
        js_setindex(J, -(2), fresh89);
        a = c;
        if c.offset_from(b) == 0 {
            a = a.offset(1);
        }
    }
    if len == 0 {
        js_pop(J, 1);
        js_pushnull(J);
    }
}
unsafe extern "C" fn Sp_search(J: &mut js_State) {
    let mut re: *mut js_Regexp = core::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    text = checkstring(J, 0);
    if js_isregexp(J, 1) != 0 {
        js_copy(J, 1);
    } else if js_isundefined(J, 1) != 0 {
        js_newregexp(J, b"\0" as *const u8 as *const libc::c_char, 0);
    } else {
        let pat = js_tostring(J, 1);
        js_newregexp(J, pat, 0);
    }
    re = js_toregexp(J, -(1 as i32));
    if js_doregexec(J, (*re).prog as *mut Reprog, text, &mut m, 0) == 0 {
        js_pushnumber(J, js_utfptrtoidx(text, m.sub[0].sp) as f64);
    } else {
        js_pushnumber(J, -(1 as i32) as f64);
    };
}
unsafe extern "C" fn Sp_replace_regexp(J: &mut js_State) {
    let mut re: *mut js_Regexp = core::ptr::null_mut::<js_Regexp>();
    let mut source: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut r: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut n: i32 = 0;
    let mut x: i32 = 0;
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    source = checkstring(J, 0);
    re = js_toregexp(J, 1);
    if js_doregexec(J, (*re).prog as *mut Reprog, source, &mut m, 0) != 0 {
        js_copy(J, 0);
        return;
    }
    (*re).last = 0;
    loop {
        s = m.sub[0].sp;
        n = (m.sub[0].ep).offset_from(m.sub[0].sp) as i32;
        if js_iscallable(J, 2) != 0 {
            js_copy(J, 2);
            js_pushundefined(J);
            x = 0;
            while !(m.sub[x as usize].sp).is_null() {
                js_pushlstring(
                    J,
                    m.sub[x as usize].sp,
                    (m.sub[x as usize].ep).offset_from(m.sub[x as usize].sp) as i32,
                );
                x += 1;
            }
            js_pushnumber(J, s.offset_from(source) as f64);
            js_copy(J, 0);
            js_call(J, 2 + x);
            r = js_tostring(J, -(1 as i32));
            js_putm(J, &mut sb, source, s);
            js_puts(J, &mut sb, r);
            js_pop(J, 1);
        } else {
            r = js_tostring(J, 2);
            js_putm(J, &mut sb, source, s);
            while *r != 0 {
                if *r as i32 == '$' as i32 {
                    let mut current_block_44: u64;
                    r = r.offset(1);
                    match *r as i32 {
                        0 => {
                            r = r.offset(-1);

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
                            x = *r as i32 - '0' as i32;
                            if *r.offset(1 as i32 as isize) as i32 >= '0' as i32
                                && *r.offset(1 as i32 as isize) as i32 <= '9' as i32
                            {
                                r = r.offset(1);
                                x = x * 10 + *r as i32 - '0' as i32;
                            }
                            if x > 0 && x < m.nsub {
                                js_putm(J, &mut sb, m.sub[x as usize].sp, m.sub[x as usize].ep);
                            } else {
                                js_putc(J, &mut sb, '$' as i32);
                                if x > 10 {
                                    js_putc(J, &mut sb, '0' as i32 + x / 10);
                                    js_putc(J, &mut sb, '0' as i32 + x % 10);
                                } else {
                                    js_putc(J, &mut sb, '0' as i32 + x);
                                }
                            }
                            current_block_44 = 9441801433784995173;
                        }
                        _ => {
                            js_putc(J, &mut sb, '$' as i32);
                            js_putc(J, &mut sb, *r as i32);
                            current_block_44 = 9441801433784995173;
                        }
                    }
                    if current_block_44 == 5554089773008369846 {
                        js_putc(J, &mut sb, '$' as i32);
                    }
                    r = r.offset(1);
                } else {
                    let fresh90 = r;
                    r = r.offset(1);
                    js_putc(J, &mut sb, *fresh90 as i32);
                }
            }
        }
        if (*re).flags as i32 & JS_REGEXP_G as i32 == 0 {
            break;
        }
        source = m.sub[0].ep;
        if n == 0 {
            if *source == 0 {
                break;
            }
            let fresh91 = source;
            source = source.offset(1);
            js_putc(J, &mut sb, *fresh91 as i32);
        }
        if js_doregexec(
            J,
            (*re).prog as *mut Reprog,
            source,
            &mut m,
            REG_NOTBOL as i32,
        ) != 0
        {
            break;
        }
    }
    js_puts(J, &mut sb, s.offset(n as isize));
    js_putc(J, &mut sb, 0);
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
unsafe extern "C" fn Sp_replace_string(J: &mut js_State) {
    let mut source: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut needle: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut r: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut n: i32 = 0;
    source = checkstring(J, 0);
    needle = js_tostring(J, 1);
    s = strstr(source, needle);
    if s.is_null() {
        js_copy(J, 0);
        return;
    }
    n = strlen(needle) as i32;
    if js_iscallable(J, 2) != 0 {
        js_copy(J, 2);
        js_pushundefined(J);
        js_pushlstring(J, s, n);
        js_pushnumber(J, s.offset_from(source) as f64);
        js_copy(J, 0);
        js_call(J, 3);
        r = js_tostring(J, -(1 as i32));
        js_putm(J, &mut sb, source, s);
        js_puts(J, &mut sb, r);
        js_puts(J, &mut sb, s.offset(n as isize));
        js_putc(J, &mut sb, 0);
        js_pop(J, 1);
    } else {
        r = js_tostring(J, 2);
        js_putm(J, &mut sb, source, s);
        while *r != 0 {
            if *r as i32 == '$' as i32 {
                let mut current_block_29: u64;
                r = r.offset(1);
                match *r as i32 {
                    0 => {
                        r = r.offset(-1);

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
                        js_putc(J, &mut sb, *r as i32);
                        current_block_29 = 15125582407903384992;
                    }
                }
                if current_block_29 == 6980996644429870198 {
                    js_putc(J, &mut sb, '$' as i32);
                }
                r = r.offset(1);
            } else {
                let fresh92 = r;
                r = r.offset(1);
                js_putc(J, &mut sb, *fresh92 as i32);
            }
        }
        js_puts(J, &mut sb, s.offset(n as isize));
        js_putc(J, &mut sb, 0);
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
unsafe extern "C" fn Sp_replace(J: &mut js_State) {
    if js_isregexp(J, 1) != 0 {
        Sp_replace_regexp(J);
    } else {
        Sp_replace_string(J);
    };
}
unsafe extern "C" fn Sp_split_regexp(J: &mut js_State) {
    let mut re: *mut js_Regexp = core::ptr::null_mut::<js_Regexp>();
    let mut text: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut limit: i32 = 0;
    let mut len: i32 = 0;
    let mut k: i32 = 0;
    let mut p: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut a: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut b: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut c: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut e: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut m: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    text = checkstring(J, 0);
    re = js_toregexp(J, 1);
    limit = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        (1 as i32) << 30
    };
    js_newarray(J);
    len = 0;
    if limit == 0 {
        return;
    }
    e = text.offset(strlen(text) as isize);
    if e == text {
        if js_doregexec(J, (*re).prog as *mut Reprog, text, &mut m, 0) != 0 {
            js_pushliteral(J, b"\0" as *const u8 as *const libc::c_char);
            js_setindex(J, -(2), 0);
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
            if a > text { REG_NOTBOL as i32 } else { 0 },
        ) != 0
        {
            break;
        }
        b = m.sub[0].sp;
        c = m.sub[0].ep;
        if b == c && b == p {
            a = a.offset(1);
        } else {
            if len == limit {
                return;
            }
            js_pushlstring(J, p, b.offset_from(p) as i32);
            let fresh93 = len;
            len += 1;
            js_setindex(J, -(2), fresh93);
            k = 1;
            while k < m.nsub {
                if len == limit {
                    return;
                }
                js_pushlstring(
                    J,
                    m.sub[k as usize].sp,
                    (m.sub[k as usize].ep).offset_from(m.sub[k as usize].sp) as i32,
                );
                let fresh94 = len;
                len += 1;
                js_setindex(J, -(2), fresh94);
                k += 1;
            }
            p = c;
            a = p;
        }
    }
    if len == limit {
        return;
    }
    js_pushstring(J, p);
    js_setindex(J, -(2), len);
}
unsafe extern "C" fn Sp_split_string(J: &mut js_State) {
    let mut str: *const libc::c_char = checkstring(J, 0);
    let mut sep: *const libc::c_char = js_tostring(J, 1);
    let mut limit: i32 = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        (1 as i32) << 30
    };
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    js_newarray(J);
    if limit == 0 {
        return;
    }
    n = strlen(sep) as i32;
    if n == 0 {
        let mut rune: Rune = 0;
        i = 0;
        while *str as i32 != 0 && i < limit {
            n = jsU_chartorune(&mut rune, str);
            js_pushlstring(J, str, n);
            js_setindex(J, -(2), i);
            str = str.offset(n as isize);
            i += 1;
        }
        return;
    }
    i = 0;
    while !str.is_null() && i < limit {
        let mut s: *const libc::c_char = strstr(str, sep);
        if !s.is_null() {
            js_pushlstring(J, str, s.offset_from(str) as i32);
            js_setindex(J, -(2), i);
            str = s.offset(n as isize);
        } else {
            js_pushstring(J, str);
            js_setindex(J, -(2), i);
            str = core::ptr::null::<libc::c_char>();
        }
        i += 1;
    }
}
unsafe extern "C" fn Sp_split(J: &mut js_State) {
    if js_isundefined(J, 1) != 0 {
        js_newarray(J);
        let v = js_tostring(J, 0);
        js_pushstring(J, v);
        js_setindex(J, -(2), 0);
    } else if js_isregexp(J, 1) != 0 {
        Sp_split_regexp(J);
    } else {
        Sp_split_string(J);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initstring(J: &mut js_State) {
    (*(*J).String_prototype).u.s.shrstr[0] = 0;
    (*(*J).String_prototype).u.s.string = ((*(*J).String_prototype).u.s.shrstr).as_mut_ptr();
    (*(*J).String_prototype).u.s.length = 0;
    js_pushobject(J, (*J).String_prototype);
    jsB_propf(
        J,
        b"String.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Sp_toString),
        0,
    );
    jsB_propf(
        J,
        b"String.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Sp_valueOf),
        0,
    );
    jsB_propf(
        J,
        b"String.prototype.charAt\0" as *const u8 as *const libc::c_char,
        Some(Sp_charAt),
        1,
    );
    jsB_propf(
        J,
        b"String.prototype.charCodeAt\0" as *const u8 as *const libc::c_char,
        Some(Sp_charCodeAt),
        1,
    );
    jsB_propf(
        J,
        b"String.prototype.concat\0" as *const u8 as *const libc::c_char,
        Some(Sp_concat),
        0,
    );
    jsB_propf(
        J,
        b"String.prototype.indexOf\0" as *const u8 as *const libc::c_char,
        Some(Sp_indexOf),
        1,
    );
    jsB_propf(
        J,
        b"String.prototype.lastIndexOf\0" as *const u8 as *const libc::c_char,
        Some(Sp_lastIndexOf),
        1,
    );
    jsB_propf(
        J,
        b"String.prototype.localeCompare\0" as *const u8 as *const libc::c_char,
        Some(Sp_localeCompare),
        1,
    );
    jsB_propf(
        J,
        b"String.prototype.match\0" as *const u8 as *const libc::c_char,
        Some(Sp_match),
        1,
    );
    jsB_propf(
        J,
        b"String.prototype.replace\0" as *const u8 as *const libc::c_char,
        Some(Sp_replace),
        2,
    );
    jsB_propf(
        J,
        b"String.prototype.search\0" as *const u8 as *const libc::c_char,
        Some(Sp_search),
        1,
    );
    jsB_propf(
        J,
        b"String.prototype.slice\0" as *const u8 as *const libc::c_char,
        Some(Sp_slice),
        2,
    );
    jsB_propf(
        J,
        b"String.prototype.split\0" as *const u8 as *const libc::c_char,
        Some(Sp_split),
        2,
    );
    jsB_propf(
        J,
        b"String.prototype.substring\0" as *const u8 as *const libc::c_char,
        Some(Sp_substring),
        2,
    );
    jsB_propf(
        J,
        b"String.prototype.toLowerCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toLowerCase),
        0,
    );
    jsB_propf(
        J,
        b"String.prototype.toLocaleLowerCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toLowerCase),
        0,
    );
    jsB_propf(
        J,
        b"String.prototype.toUpperCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toUpperCase),
        0,
    );
    jsB_propf(
        J,
        b"String.prototype.toLocaleUpperCase\0" as *const u8 as *const libc::c_char,
        Some(Sp_toUpperCase),
        0,
    );
    jsB_propf(
        J,
        b"String.prototype.trim\0" as *const u8 as *const libc::c_char,
        Some(Sp_trim),
        0,
    );
    js_newcconstructor(
        J,
        Some(jsB_String),
        Some(jsB_new_String),
        b"String\0" as *const u8 as *const libc::c_char,
        0,
    );
    jsB_propf(
        J,
        b"String.fromCharCode\0" as *const u8 as *const libc::c_char,
        Some(S_fromCharCode),
        0,
    );
    js_defglobal(
        J,
        b"String\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
