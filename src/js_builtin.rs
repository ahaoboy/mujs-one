use crate::*;

unsafe extern "C" fn jsB_globalf(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut cfun: js_CFunction,
    mut n: i32,
) {
    js_newcfunction(J, cfun, name, n);
    js_defglobal(J, name, JS_DONTENUM as i32);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_propf(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut cfun: js_CFunction,
    mut n: i32,
) {
    let mut pname: *const libc::c_char = strrchr(name, '.' as i32);
    pname = if !pname.is_null() {
        pname.offset(1 as isize)
    } else {
        name
    };
    js_newcfunction(J, cfun, name, n);
    js_defproperty(J, -(2 as i32), pname, JS_DONTENUM as i32);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_propn(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut number: f64,
) {
    js_pushnumber(J, number);
    js_defproperty(
        J,
        -(2 as i32),
        name,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsB_props(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut string: *const libc::c_char,
) {
    js_pushliteral(J, string);
    js_defproperty(J, -(2 as i32), name, JS_DONTENUM as i32);
}
unsafe extern "C" fn jsB_parseInt(J: &mut js_State) {
    let mut s: *const libc::c_char = js_tostring(J, 1);
    let mut radix: i32 = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        0
    };
    let mut sign: f64 = 1.0;
    let mut n: f64 = 0.;
    let mut e: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    while jsY_iswhite(*s as i32) || jsY_isnewline(*s as i32) {
        s = s.offset(1);
    }
    if *s as i32 == '-' as i32 {
        s = s.offset(1);
        sign = -(1) as f64;
    } else if *s as i32 == '+' as i32 {
        s = s.offset(1);
    }
    if radix == 0 {
        radix = 10;
        if *s.offset(0 as isize) as i32 == '0' as i32
            && (*s.offset(1 as isize) as i32 == 'x' as i32
                || *s.offset(1 as isize) as i32 == 'X' as i32)
        {
            s = s.offset(2 as i32 as isize);
            radix = 16 as i32;
        }
    } else if radix < 2 || radix > 36 as i32 {
        js_pushnumber(J, ::core::f32::NAN as f64);
        return;
    }
    n = js_strtol(s, &mut e, radix);
    if s == e as *const libc::c_char {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, n * sign);
    };
}
unsafe extern "C" fn jsB_parseFloat(J: &mut js_State) {
    let mut s: *const libc::c_char = js_tostring(J, 1);
    let mut e: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut n: f64 = 0.;
    while jsY_iswhite(*s as i32) || jsY_isnewline(*s as i32) {
        s = s.offset(1);
    }
    if strncmp(s, b"Infinity\0" as *const u8 as *const libc::c_char, 8) == 0 {
        js_pushnumber(J, ::core::f32::INFINITY as f64);
    } else if strncmp(
        s,
        b"+Infinity\0" as *const u8 as *const libc::c_char,
        9,
    ) == 0
    {
        js_pushnumber(J, ::core::f32::INFINITY as f64);
    } else if strncmp(
        s,
        b"-Infinity\0" as *const u8 as *const libc::c_char,
        9,
    ) == 0
    {
        js_pushnumber(J, -::core::f32::INFINITY as f64);
    } else {
        n = js_stringtofloat(s, &mut e);
        if e == s as *mut libc::c_char {
            js_pushnumber(J, ::core::f32::NAN as f64);
        } else {
            js_pushnumber(J, n);
        }
    };
}
unsafe extern "C" fn jsB_isNaN(J: &mut js_State) {
    let mut n: f64 = js_tonumber(J, 1);
    js_pushboolean(J, n.is_nan() as i32);
}
unsafe extern "C" fn jsB_isFinite(J: &mut js_State) {
    let mut n: f64 = js_tonumber(J, 1);
    js_pushboolean(J, n.is_finite() as i32);
}
unsafe extern "C" fn Encode(
    J: &mut js_State,
    mut str_: *const libc::c_char,
    mut unescaped: *const libc::c_char,
) {
    let mut str: *const libc::c_char = str_;
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    const HEX: *const libc::c_char = c"0123456789ABCDEF".as_ptr();
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
        let mut c: i32 = *fresh5 as libc::c_uchar as i32;
        if !(strchr(unescaped, c)).is_null() {
            js_putc(J, &mut sb, c);
        } else {
            js_putc(J, &mut sb, '%' as i32);
            js_putc(
                J,
                &mut sb,
                *HEX.offset((c >> 4 & 0xf as i32) as isize) as i32,
            );
            js_putc(J, &mut sb, *HEX.offset((c & 0xf as i32) as isize) as i32);
        }
    }
    js_putc(J, &mut sb, 0);
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
    J: &mut js_State,
    mut str_: *const libc::c_char,
    mut reserved: *const libc::c_char,
) {
    let mut str: *const libc::c_char = str_;
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut a: i32 = 0;
    let mut b: i32 = 0;
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
        let mut c: i32 = *fresh6 as libc::c_uchar as i32;
        if c != '%' as i32 {
            js_putc(J, &mut sb, c);
        } else {
            if *str.offset(0 as isize) == 0 || *str.offset(1 as isize) == 0 {
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
            a = *fresh7 as i32;
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
            b = *fresh8 as i32;
            if !jsY_ishex(a) || !jsY_ishex(b) {
                js_urierror(
                    J,
                    b"invalid escape sequence\0" as *const u8 as *const libc::c_char,
                );
            }
            c = jsY_tohex(a) << 4 | jsY_tohex(b);
            if (strchr(reserved, c)).is_null() {
                js_putc(J, &mut sb, c);
            } else {
                js_putc(J, &mut sb, '%' as i32);
                js_putc(J, &mut sb, a);
                js_putc(J, &mut sb, b);
            }
        }
    }
    js_putc(J, &mut sb, 0);
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
unsafe extern "C" fn jsB_decodeURI(J: &mut js_State) {
    let s = js_tostring(J, 1);
    Decode(J, s, b";/?:@&=+$,#\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn jsB_decodeURIComponent(J: &mut js_State) {
    let s = js_tostring(J, 1);
    Decode(J, s, b"\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn jsB_encodeURI(J: &mut js_State) {
    let s = js_tostring(J, 1);
    Encode(
        J,
        s,
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_.!~*'();/?:@&=+$,#\0"
            as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn jsB_encodeURIComponent(J: &mut js_State) {
    let s = js_tostring(J, 1);
    Encode(
        J,
        s,
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_.!~*'()\0" as *const u8
            as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jsB_init(J: &mut js_State) {
    (*J).Object_prototype = jsV_newobject(J, JS_COBJECT, core::ptr::null_mut::<js_Object>());
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
        0,
        core::ptr::null_mut::<*const libc::c_char>(),
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
    let J = &mut *J;
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
    js_pushnumber(J, ::core::f32::NAN as f64);
    js_defglobal(
        J,
        b"NaN\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    js_pushnumber(J, ::core::f32::INFINITY as f64);
    js_defglobal(
        J,
        b"Infinity\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    js_pushundefined(J);
    js_defglobal(
        J,
        b"undefined\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    jsB_globalf(
        J,
        b"parseInt\0" as *const u8 as *const libc::c_char,
        Some(jsB_parseInt),
        1,
    );
    jsB_globalf(
        J,
        b"parseFloat\0" as *const u8 as *const libc::c_char,
        Some(jsB_parseFloat),
        1,
    );
    jsB_globalf(
        J,
        b"isNaN\0" as *const u8 as *const libc::c_char,
        Some(jsB_isNaN),
        1,
    );
    jsB_globalf(
        J,
        b"isFinite\0" as *const u8 as *const libc::c_char,
        Some(jsB_isFinite),
        1,
    );
    jsB_globalf(
        J,
        b"decodeURI\0" as *const u8 as *const libc::c_char,
        Some(jsB_decodeURI),
        1,
    );
    jsB_globalf(
        J,
        b"decodeURIComponent\0" as *const u8 as *const libc::c_char,
        Some(jsB_decodeURIComponent),
        1,
    );
    jsB_globalf(
        J,
        b"encodeURI\0" as *const u8 as *const libc::c_char,
        Some(jsB_encodeURI),
        1,
    );
    jsB_globalf(
        J,
        b"encodeURIComponent\0" as *const u8 as *const libc::c_char,
        Some(jsB_encodeURIComponent),
        1,
    );
}
