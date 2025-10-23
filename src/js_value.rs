use crate::*;

#[no_mangle]
pub unsafe extern "C" fn js_strtol(
    mut s: *const libc::c_char,
    mut p: *mut *mut libc::c_char,
    mut base: i32,
) -> f64 {
    #[rustfmt::skip]
    const TABLE: [libc::c_uchar; 256] = [
        80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 80, 80, 80, 80, 80, 80,
		80, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
		25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 80, 80, 80, 80, 80,
		80, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
		25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
		80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80
    ];
    let mut x: f64 = 0.;
    let mut c: libc::c_uchar = 0;
    if base == 10 {
        x = 0.0;
        let fresh95 = s;
        s = s.offset(1);
        c = *fresh95 as libc::c_uchar;
        while 0 <= c as i32 - '0' as i32 && (c as i32 - '0' as i32) < 10 {
            x = x * 10.0 + (c as i32 - '0' as i32) as f64;
            let fresh96 = s;
            s = s.offset(1);
            c = *fresh96 as libc::c_uchar;
        }
    } else {
        x = 0.0;
        let fresh97 = s;
        s = s.offset(1);
        c = *fresh97 as libc::c_uchar;
        while (TABLE[c as usize] as i32) < base {
            x = x * base as f64 + TABLE[c as usize] as i32 as f64;
            let fresh98 = s;
            s = s.offset(1);
            c = *fresh98 as libc::c_uchar;
        }
    }
    if !p.is_null() {
        *p = (s as *mut libc::c_char).offset(-(1 as i32 as isize));
    }
    x
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertointeger(mut n: f64) -> i32 {
    if n == 0.0 {
        return 0;
    }
    if n.is_nan() as i32 != 0 {
        return 0;
    }
    n = if n < 0.0 { -floor(-n) } else { floor(n) };
    if n < (-(2147483647 as i32) - 1) as f64 {
        return -(2147483647 as i32) - 1;
    }
    if n > 2147483647 as i32 as f64 {
        return 2147483647 as i32;
    }
    n as i32
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertoint32(mut n: f64) -> i32 {
    let mut two32: f64 = 4294967296.0f64;
    let mut two31: f64 = 2147483648.0f64;
    if n.is_finite() as i32 == 0 || n == 0.0 {
        return 0;
    }
    n = fmod(n, two32);
    n = if n >= 0.0 { floor(n) } else { ceil(n) + two32 };
    if n >= two31 {
        (n - two32) as i32
    } else {
        n as i32
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertouint32(mut n: f64) -> libc::c_uint {
    jsV_numbertoint32(n) as libc::c_uint
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertoint16(mut n: f64) -> libc::c_short {
    jsV_numbertoint32(n) as libc::c_short
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertouint16(mut n: f64) -> libc::c_ushort {
    jsV_numbertoint32(n) as libc::c_ushort
}
unsafe extern "C" fn jsV_toString(J: &mut js_State, mut obj: *mut js_Object) -> i32 {
    js_pushobject(J, obj);
    js_getproperty(
        J,
        -(1 as i32),
        b"toString\0" as *const u8 as *const libc::c_char,
    );
    if js_iscallable(J, -(1 as i32)) != 0 {
        js_rot2(J);
        js_call(J, 0);
        if js_isprimitive(J, -(1 as i32)) != 0 {
            return 1;
        }
        js_pop(J, 1);
        return 0;
    }
    js_pop(J, 2);
    0
}
unsafe extern "C" fn jsV_valueOf(J: &mut js_State, mut obj: *mut js_Object) -> i32 {
    js_pushobject(J, obj);
    js_getproperty(
        J,
        -(1 as i32),
        b"valueOf\0" as *const u8 as *const libc::c_char,
    );
    if js_iscallable(J, -(1 as i32)) != 0 {
        js_rot2(J);
        js_call(J, 0);
        if js_isprimitive(J, -(1 as i32)) != 0 {
            return 1;
        }
        js_pop(J, 1);
        return 0;
    }
    js_pop(J, 2);
    0
}
#[no_mangle]
pub unsafe extern "C" fn jsV_toprimitive(
    J: &mut js_State,
    mut v: *mut js_Value,
    mut preferred: i32,
) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if (*v).t.type_0 as i32 != JS_TOBJECT as i32 {
        return;
    }
    obj = (*v).u.object;
    if preferred == JS_HNONE as i32 {
        preferred = if (*obj).type_0 as i32 as libc::c_uint == JS_CDATE as i32 as libc::c_uint {
            JS_HSTRING as i32
        } else {
            JS_HNUMBER as i32
        };
    }
    if preferred == JS_HSTRING as i32 {
        if jsV_toString(J, obj) != 0 || jsV_valueOf(J, obj) != 0 {
            *v = *js_tovalue(J, -(1 as i32));
            js_pop(J, 1);
            return;
        }
    } else if jsV_valueOf(J, obj) != 0 || jsV_toString(J, obj) != 0 {
        *v = *js_tovalue(J, -(1 as i32));
        js_pop(J, 1);
        return;
    }
    if (*J).strict != 0 {
        js_typeerror(
            J,
            b"cannot convert object to primitive\0" as *const u8 as *const libc::c_char,
        );
    }
    (*v).t.type_0 = JS_TLITSTR as i32 as libc::c_char;
    (*v).u.litstr = b"[object]\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn jsV_toboolean(J: &mut js_State, mut v: *mut js_Value) -> i32 {
    match (*v).t.type_0 as i32 {
        1 => 0,
        2 => 0,
        3 => (*v).u.boolean,
        4 => ((*v).u.number != 0.0 && ((*v).u.number).is_nan() as i32 == 0) as i32,
        5 => (*((*v).u.litstr).offset(0 as isize) as i32 != 0) as i32,
        6 => (*((*(*v).u.memstr).p).as_mut_ptr().offset(0 as isize) as i32 != 0) as i32,
        7 => 1,
        0 | _ => ((*v).u.shrstr[0 as usize] as i32 != 0) as i32,
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_itoa(mut out: *mut libc::c_char, mut v: i32) -> *const libc::c_char {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut s: *mut libc::c_char = out;
    let mut a: libc::c_uint = 0;
    let mut i: i32 = 0;
    if v < 0 {
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
            a.wrapping_rem(10 as libc::c_uint)
                .wrapping_add('0' as i32 as libc::c_uint) as libc::c_char;
        a = a.wrapping_div(10 as libc::c_uint);
    }
    if i == 0 {
        let fresh101 = i;
        i += 1;
        buf[fresh101 as usize] = '0' as i32 as libc::c_char;
    }
    while i > 0 {
        i -= 1;
        let fresh102 = s;
        s = s.offset(1);
        *fresh102 = buf[i as usize];
    }
    *s = 0;
    out
}
#[no_mangle]
pub unsafe extern "C" fn js_stringtofloat(
    mut s: *const libc::c_char,
    mut ep: *mut *mut libc::c_char,
) -> f64 {
    let mut end: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut n: f64 = 0.;
    let mut e: *const libc::c_char = s;
    let mut isflt: i32 = 0;
    if *e as i32 == '+' as i32 || *e as i32 == '-' as i32 {
        e = e.offset(1);
    }
    while *e as i32 >= '0' as i32 && *e as i32 <= '9' as i32 {
        e = e.offset(1);
    }
    if *e as i32 == '.' as i32 {
        e = e.offset(1);

        isflt = 1;
    }
    while *e as i32 >= '0' as i32 && *e as i32 <= '9' as i32 {
        e = e.offset(1);
    }
    if *e as i32 == 'e' as i32 || *e as i32 == 'E' as i32 {
        e = e.offset(1);

        if *e as i32 == '+' as i32 || *e as i32 == '-' as i32 {
            e = e.offset(1);
        }
        while *e as i32 >= '0' as i32 && *e as i32 <= '9' as i32 {
            e = e.offset(1);
        }
        isflt = 1;
    }
    if isflt != 0 {
        n = js_strtod(s, &mut end);
    } else if *s as i32 == '-' as i32 {
        n = -js_strtol(s.offset(1 as i32 as isize), &mut end, 10);
    } else if *s as i32 == '+' as i32 {
        n = js_strtol(s.offset(1 as i32 as isize), &mut end, 10);
    } else {
        n = js_strtol(s, &mut end, 10);
    }
    if end == e as *mut libc::c_char {
        *ep = e as *mut libc::c_char;
        return n;
    }
    *ep = s as *mut libc::c_char;
    0.0
}
#[no_mangle]
pub unsafe extern "C" fn jsV_stringtonumber(J: &mut js_State, mut s: *const libc::c_char) -> f64 {
    let mut e: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut n: f64 = 0.;
    while jsY_iswhite(*s as i32) || jsY_isnewline(*s as i32) {
        s = s.offset(1);
    }
    if *s.offset(0 as isize) as i32 == '0' as i32
        && (*s.offset(1 as i32 as isize) as i32 == 'x' as i32
            || *s.offset(1 as i32 as isize) as i32 == 'X' as i32)
        && *s.offset(2 as i32 as isize) as i32 != 0
    {
        n = js_strtol(s.offset(2 as i32 as isize), &mut e, 16 as i32);
    } else if strncmp(s, b"Infinity\0" as *const u8 as *const libc::c_char, 8) == 0 {
        n = ::core::f32::INFINITY as f64;
        e = (s as *mut libc::c_char).offset(8 as i32 as isize);
    } else if strncmp(s, b"+Infinity\0" as *const u8 as *const libc::c_char, 9) == 0 {
        n = ::core::f32::INFINITY as f64;
        e = (s as *mut libc::c_char).offset(9 as i32 as isize);
    } else if strncmp(s, b"-Infinity\0" as *const u8 as *const libc::c_char, 9) == 0 {
        n = -::core::f32::INFINITY as f64;
        e = (s as *mut libc::c_char).offset(9 as i32 as isize);
    } else {
        n = js_stringtofloat(s, &mut e);
    }
    while jsY_iswhite(*e as i32) || jsY_isnewline(*e as i32) {
        e = e.offset(1);
    }
    if *e != 0 {
        return ::core::f32::NAN as f64;
    }
    n
}
#[no_mangle]
pub unsafe extern "C" fn jsV_tonumber(J: &mut js_State, mut v: *mut js_Value) -> f64 {
    match (*v).t.type_0 as i32 {
        1 => ::core::f32::NAN as f64,
        2 => 0.0,
        3 => (*v).u.boolean as f64,
        4 => (*v).u.number,
        5 => jsV_stringtonumber(J, (*v).u.litstr),
        6 => jsV_stringtonumber(J, ((*(*v).u.memstr).p).as_mut_ptr()),
        7 => {
            jsV_toprimitive(J, v, JS_HNUMBER as i32);
            jsV_tonumber(J, v)
        }
        0 | _ => jsV_stringtonumber(J, ((*v).u.shrstr).as_mut_ptr()),
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsV_tointeger(J: &mut js_State, mut v: *mut js_Value) -> f64 {
    jsV_numbertointeger(jsV_tonumber(J, v)) as f64
}
#[no_mangle]
pub unsafe extern "C" fn jsV_numbertostring(
    mut buf: *mut libc::c_char,
    mut f: f64,
) -> *const libc::c_char {
    let mut digits: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = buf;
    let mut s: *mut libc::c_char = digits.as_mut_ptr();
    let mut exp_0: i32 = 0;
    let mut ndigits: i32 = 0;
    let mut point: i32 = 0;
    if f == 0.0 {
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
        return if f < 0.0 {
            b"-Infinity\0" as *const u8 as *const libc::c_char
        } else {
            b"Infinity\0" as *const u8 as *const libc::c_char
        };
    }
    if f >= (-(2147483647 as i32) - 1) as f64 && f <= 2147483647 as i32 as f64 {
        let mut i: i32 = f as i32;
        if i as f64 == f {
            return js_itoa(buf, i);
        }
    }
    ndigits = js_grisu2(f, digits.as_mut_ptr(), &mut exp_0);
    point = ndigits + exp_0;
    if f.is_sign_negative() as i32 != 0 {
        let fresh103 = p;
        p = p.offset(1);
        *fresh103 = '-' as i32 as libc::c_char;
    }
    if point < -(5 as i32) || point > 21 as i32 {
        let fresh104 = s;
        s = s.offset(1);
        let fresh105 = p;
        p = p.offset(1);
        *fresh105 = *fresh104;
        if ndigits > 1 {
            let mut n: i32 = ndigits - 1;
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
        js_fmtexp(p, point - 1);
    } else if point <= 0 {
        let fresh110 = p;
        p = p.offset(1);
        *fresh110 = '0' as i32 as libc::c_char;
        let fresh111 = p;
        p = p.offset(1);
        *fresh111 = '.' as i32 as libc::c_char;
        loop {
            let fresh112 = point;
            point += 1;
            if fresh112 >= 0 {
                break;
            }
            let fresh113 = p;
            p = p.offset(1);
            *fresh113 = '0' as i32 as libc::c_char;
        }
        loop {
            let fresh114 = ndigits;
            ndigits -= 1;
            if fresh114 <= 0 {
                break;
            }
            let fresh115 = s;
            s = s.offset(1);
            let fresh116 = p;
            p = p.offset(1);
            *fresh116 = *fresh115;
        }
        *p = 0;
    } else {
        loop {
            let fresh117 = ndigits;
            ndigits -= 1;
            if fresh117 <= 0 {
                break;
            }
            let fresh118 = s;
            s = s.offset(1);
            let fresh119 = p;
            p = p.offset(1);
            *fresh119 = *fresh118;
            point -= 1;
            if point == 0 && ndigits > 0 {
                let fresh120 = p;
                p = p.offset(1);
                *fresh120 = '.' as i32 as libc::c_char;
            }
        }
        loop {
            let fresh121 = point;
            point -= 1;
            if fresh121 <= 0 {
                break;
            }
            let fresh122 = p;
            p = p.offset(1);
            *fresh122 = '0' as i32 as libc::c_char;
        }
        *p = 0;
    }
    buf as *const libc::c_char
}
#[no_mangle]
pub unsafe extern "C" fn jsV_tostring(
    J: &mut js_State,
    mut v: *mut js_Value,
) -> *const libc::c_char {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut p: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let v = &mut *v;
    match v.t.type_0 as i32 {
        1 => c"undefined".as_ptr(),
        2 => c"null".as_ptr(),
        3 => {
            if v.u.boolean != 0 {
                c"true".as_ptr()
            } else {
                c"false".as_ptr()
            }
        }
        5 => v.u.litstr,
        6 => ((*v.u.memstr).p).as_mut_ptr(),
        4 => {
            p = jsV_numbertostring(buf.as_mut_ptr(), v.u.number);
            if p == buf.as_mut_ptr() as *const libc::c_char {
                let mut n: i32 = strlen(p) as i32;
                if n <= 15 as u32 as i32 {
                    let mut s: *mut libc::c_char = (v.u.shrstr).as_mut_ptr();
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
                    *s = 0;
                    v.t.type_0 = JS_TSHRSTR as i32 as libc::c_char;
                    return ((*v).u.shrstr).as_mut_ptr();
                } else {
                    v.u.memstr = jsV_newmemstring(J, p, n);
                    v.t.type_0 = JS_TMEMSTR as i32 as libc::c_char;
                    return ((*(*v).u.memstr).p).as_mut_ptr();
                }
            }
            p
        }
        7 => {
            jsV_toprimitive(J, v, JS_HSTRING as i32);
            jsV_tostring(J, v)
        }
        0 | _ => (v.u.shrstr).as_mut_ptr(),
    }
}
unsafe extern "C" fn jsV_newboolean(J: &mut js_State, mut v: i32) -> *mut js_Object {
    let mut obj: *mut js_Object = jsV_newobject(J, JS_CBOOLEAN, (*J).Boolean_prototype);
    (*obj).u.boolean = v;
    obj
}
unsafe extern "C" fn jsV_newnumber(J: &mut js_State, mut v: f64) -> *mut js_Object {
    let mut obj: *mut js_Object = jsV_newobject(J, JS_CNUMBER, (*J).Number_prototype);
    (*obj).u.number = v;
    obj
}
unsafe extern "C" fn jsV_newstring(J: &mut js_State, mut v: *const libc::c_char) -> *mut js_Object {
    let mut obj: *mut js_Object = jsV_newobject(J, JS_CSTRING, (*J).String_prototype);
    let mut n: size_t = strlen(v);
    if n < ::core::mem::size_of::<[libc::c_char; 16]>() as u32 {
        (*obj).u.s.string = ((*obj).u.s.shrstr).as_mut_ptr();
        memcpy(
            ((*obj).u.s.shrstr).as_mut_ptr() as *mut libc::c_void,
            v as *const libc::c_void,
            n.wrapping_add(1 as i32 as u32),
        );
    } else {
        (*obj).u.s.string = js_strdup(J, v);
    }
    (*obj).u.s.length = js_utflen(v);
    obj
}
#[no_mangle]
pub unsafe extern "C" fn jsV_toobject(J: &mut js_State, mut v: *mut js_Value) -> *mut js_Object {
    let mut o: *mut js_Object = core::ptr::null_mut::<js_Object>();
    match (*v).t.type_0 as i32 {
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
    (*v).t.type_0 = JS_TOBJECT as i32 as libc::c_char;
    (*v).u.object = o;
    o
}
#[no_mangle]
pub unsafe extern "C" fn js_newobjectx(J: &mut js_State) {
    let mut prototype: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, -(1 as i32)) != 0 {
        prototype = js_toobject(J, -(1 as i32));
    }
    js_pop(J, 1);
    let v = jsV_newobject(J, JS_COBJECT, prototype);
    js_pushobject(J, v);
}
#[no_mangle]
pub unsafe extern "C" fn js_newobject(J: &mut js_State) {
    let v = jsV_newobject(J, JS_COBJECT, (*J).Object_prototype);
    js_pushobject(J, v);
}
#[no_mangle]
pub unsafe extern "C" fn js_newarguments(J: &mut js_State) {
    let v = jsV_newobject(J, JS_CARGUMENTS, (*J).Object_prototype);
    js_pushobject(J, v);
}
#[no_mangle]
pub unsafe extern "C" fn js_newarray(J: &mut js_State) {
    let mut obj: *mut js_Object = jsV_newobject(J, JS_CARRAY, (*J).Array_prototype);
    (*obj).u.a.simple = 1;
    js_pushobject(J, obj);
}
#[no_mangle]
pub unsafe extern "C" fn js_newboolean(J: &mut js_State, mut v: i32) {
    let v = jsV_newboolean(J, v);
    js_pushobject(J, v);
}
#[no_mangle]
pub unsafe extern "C" fn js_newnumber(J: &mut js_State, mut v: f64) {
    let v = jsV_newnumber(J, v);
    js_pushobject(J, v);
}
#[no_mangle]
pub unsafe extern "C" fn js_newstring(J: &mut js_State, mut v: *const libc::c_char) {
    let v = jsV_newstring(J, v);
    js_pushobject(J, v);
}
#[no_mangle]
pub unsafe extern "C" fn js_newfunction(
    J: &mut js_State,
    mut fun: *mut js_Function,
    mut scope: *mut js_Environment,
) {
    let mut obj: *mut js_Object = jsV_newobject(J, JS_CFUNCTION, (*J).Function_prototype);
    (*obj).u.f.function = fun;
    (*obj).u.f.scope = scope;
    js_pushobject(J, obj);
    js_pushnumber(J, (*fun).numparams as f64);
    js_defproperty(
        J,
        -(2 as i32),
        b"length\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    js_newobject(J);
    js_copy(J, -(2 as i32));
    js_defproperty(
        J,
        -(2 as i32),
        b"constructor\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_defproperty(
        J,
        -(2 as i32),
        b"prototype\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newscript(
    J: &mut js_State,
    mut fun: *mut js_Function,
    mut scope: *mut js_Environment,
) {
    let mut obj: *mut js_Object = jsV_newobject(J, JS_CSCRIPT, core::ptr::null_mut::<js_Object>());
    (*obj).u.f.function = fun;
    (*obj).u.f.scope = scope;
    js_pushobject(J, obj);
}
#[no_mangle]
pub unsafe extern "C" fn js_newcfunctionx(
    J: &mut js_State,
    mut cfun: js_CFunction,
    mut name: *const libc::c_char,
    mut length: i32,
    mut data: *mut libc::c_void,
    mut finalize: js_Finalize,
) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
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
    js_pushnumber(J, length as f64);
    js_defproperty(
        J,
        -(2 as i32),
        b"length\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    js_newobject(J);
    js_copy(J, -(2 as i32));
    js_defproperty(
        J,
        -(2 as i32),
        b"constructor\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_defproperty(
        J,
        -(2 as i32),
        b"prototype\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newcfunction(
    J: &mut js_State,
    mut cfun: js_CFunction,
    mut name: *const libc::c_char,
    mut length: i32,
) {
    js_newcfunctionx(
        J,
        cfun,
        name,
        length,
        core::ptr::null_mut::<libc::c_void>(),
        None,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newcconstructor(
    J: &mut js_State,
    mut cfun: js_CFunction,
    mut ccon: js_CFunction,
    mut name: *const libc::c_char,
    mut length: i32,
) {
    let mut obj: *mut js_Object = jsV_newobject(J, JS_CCFUNCTION, (*J).Function_prototype);
    (*obj).u.c.name = name;
    (*obj).u.c.function = cfun;
    (*obj).u.c.constructor = ccon;
    (*obj).u.c.length = length;
    js_pushobject(J, obj);
    js_pushnumber(J, length as f64);
    js_defproperty(
        J,
        -(2 as i32),
        b"length\0" as *const u8 as *const libc::c_char,
        JS_READONLY as i32 | JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
    js_rot2(J);
    js_copy(J, -(2 as i32));
    js_defproperty(
        J,
        -(2 as i32),
        b"constructor\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
    js_defproperty(
        J,
        -(2 as i32),
        b"prototype\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32 | JS_DONTCONF as i32,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_newuserdatax(
    J: &mut js_State,
    mut tag: *const libc::c_char,
    mut data: *mut libc::c_void,
    mut has: js_HasProperty,
    mut put: js_Put,
    mut delete: js_Delete,
    mut finalize: js_Finalize,
) {
    let mut prototype: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, -(1 as i32)) != 0 {
        prototype = js_toobject(J, -(1 as i32));
    }
    js_pop(J, 1);
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
    J: &mut js_State,
    mut tag: *const libc::c_char,
    mut data: *mut libc::c_void,
    mut finalize: js_Finalize,
) {
    js_newuserdatax(J, tag, data, None, None, None, finalize);
}
#[no_mangle]
pub unsafe extern "C" fn js_instanceof(J: &mut js_State) -> i32 {
    let mut O: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut V: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_iscallable(J, -(1 as i32)) == 0 {
        js_typeerror(
            J,
            b"instanceof: invalid operand\0" as *const u8 as *const libc::c_char,
        );
    }
    if js_isobject(J, -(2 as i32)) == 0 {
        return 0;
    }
    js_getproperty(
        J,
        -(1 as i32),
        b"prototype\0" as *const u8 as *const libc::c_char,
    );
    if js_isobject(J, -(1 as i32)) == 0 {
        js_typeerror(
            J,
            b"instanceof: 'prototype' property is not an object\0" as *const u8
                as *const libc::c_char,
        );
    }
    O = js_toobject(J, -(1 as i32));
    js_pop(J, 1);
    V = js_toobject(J, -(2 as i32));
    while !V.is_null() {
        V = (*V).prototype;
        if O == V {
            return 1;
        }
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn js_concat(J: &mut js_State) {
    js_toprimitive(J, -(2 as i32), JS_HNONE as i32);
    js_toprimitive(J, -(1 as i32), JS_HNONE as i32);
    if js_isstring(J, -(2 as i32)) != 0 || js_isstring(J, -(1 as i32)) != 0 {
        let mut sa: *const libc::c_char = js_tostring(J, -(2 as i32));
        let mut sb: *const libc::c_char = js_tostring(J, -(1 as i32));
        let mut sab: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
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
                    .wrapping_add(1 as i32 as u32) as i32,
            ) as *mut libc::c_char,
        );
        strcpy(sab, sa);
        strcat(sab, sb);
        js_pop(J, 2);
        js_pushstring(J, sab);
        js_endtry(J);
        js_free(J, sab as *mut libc::c_void);
    } else {
        let mut x: f64 = js_tonumber(J, -(2 as i32));
        let mut y: f64 = js_tonumber(J, -(1 as i32));
        js_pop(J, 2);
        js_pushnumber(J, x + y);
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_compare(J: &mut js_State, mut okay: *mut i32) -> i32 {
    js_toprimitive(J, -(2 as i32), JS_HNUMBER as i32);
    js_toprimitive(J, -(1 as i32), JS_HNUMBER as i32);
    *okay = 1;
    if js_isstring(J, -(2 as i32)) != 0 && js_isstring(J, -(1 as i32)) != 0 {
        strcmp(js_tostring(J, -(2 as i32)), js_tostring(J, -(1 as i32)))
    } else {
        let mut x: f64 = js_tonumber(J, -(2 as i32));
        let mut y: f64 = js_tonumber(J, -(1 as i32));
        if x.is_nan() as i32 != 0 || y.is_nan() as i32 != 0 {
            *okay = 0;
        }
        if x < y {
            -(1 as i32)
        } else if x > y {
            1
        } else {
            0
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_equal(J: &mut js_State) -> i32 {
    let mut x: *mut js_Value = js_tovalue(J, -(2 as i32));
    let mut y: *mut js_Value = js_tovalue(J, -(1 as i32));
    loop {
        if ((*x).t.type_0 as i32 == JS_TSHRSTR as i32
            || (*x).t.type_0 as i32 == JS_TMEMSTR as i32
            || (*x).t.type_0 as i32 == JS_TLITSTR as i32)
            && ((*y).t.type_0 as i32 == JS_TSHRSTR as i32
                || (*y).t.type_0 as i32 == JS_TMEMSTR as i32
                || (*y).t.type_0 as i32 == JS_TLITSTR as i32)
        {
            return (strcmp(
                if (*x).t.type_0 as i32 == JS_TSHRSTR as i32 {
                    ((*x).u.shrstr).as_mut_ptr() as *const libc::c_char
                } else if (*x).t.type_0 as i32 == JS_TLITSTR as i32 {
                    (*x).u.litstr
                } else if (*x).t.type_0 as i32 == JS_TMEMSTR as i32 {
                    ((*(*x).u.memstr).p).as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if (*y).t.type_0 as i32 == JS_TSHRSTR as i32 {
                    ((*y).u.shrstr).as_mut_ptr() as *const libc::c_char
                } else if (*y).t.type_0 as i32 == JS_TLITSTR as i32 {
                    (*y).u.litstr
                } else if (*y).t.type_0 as i32 == JS_TMEMSTR as i32 {
                    ((*(*y).u.memstr).p).as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            ) == 0) as i32;
        }
        if (*x).t.type_0 as i32 == (*y).t.type_0 as i32 {
            if (*x).t.type_0 as i32 == JS_TUNDEFINED as i32 {
                return 1;
            }
            if (*x).t.type_0 as i32 == JS_TNULL as i32 {
                return 1;
            }
            if (*x).t.type_0 as i32 == JS_TNUMBER as i32 {
                return ((*x).u.number == (*y).u.number) as i32;
            }
            if (*x).t.type_0 as i32 == JS_TBOOLEAN as i32 {
                return ((*x).u.boolean == (*y).u.boolean) as i32;
            }
            if (*x).t.type_0 as i32 == JS_TOBJECT as i32 {
                return ((*x).u.object == (*y).u.object) as i32;
            }
            return 0;
        }
        if (*x).t.type_0 as i32 == JS_TNULL as i32 && (*y).t.type_0 as i32 == JS_TUNDEFINED as i32 {
            return 1;
        }
        if (*x).t.type_0 as i32 == JS_TUNDEFINED as i32 && (*y).t.type_0 as i32 == JS_TNULL as i32 {
            return 1;
        }
        if (*x).t.type_0 as i32 == JS_TNUMBER as i32
            && ((*y).t.type_0 as i32 == JS_TSHRSTR as i32
                || (*y).t.type_0 as i32 == JS_TMEMSTR as i32
                || (*y).t.type_0 as i32 == JS_TLITSTR as i32)
        {
            return ((*x).u.number == jsV_tonumber(J, y)) as i32;
        }
        if ((*x).t.type_0 as i32 == JS_TSHRSTR as i32
            || (*x).t.type_0 as i32 == JS_TMEMSTR as i32
            || (*x).t.type_0 as i32 == JS_TLITSTR as i32)
            && (*y).t.type_0 as i32 == JS_TNUMBER as i32
        {
            return (jsV_tonumber(J, x) == (*y).u.number) as i32;
        }
        if (*x).t.type_0 as i32 == JS_TBOOLEAN as i32 {
            (*x).t.type_0 = JS_TNUMBER as i32 as libc::c_char;
            (*x).u.number = (if (*x).u.boolean != 0 { 1 } else { 0 }) as f64;
        } else if (*y).t.type_0 as i32 == JS_TBOOLEAN as i32 {
            (*y).t.type_0 = JS_TNUMBER as i32 as libc::c_char;
            (*y).u.number = (if (*y).u.boolean != 0 { 1 } else { 0 }) as f64;
        } else if ((*x).t.type_0 as i32 == JS_TSHRSTR as i32
            || (*x).t.type_0 as i32 == JS_TMEMSTR as i32
            || (*x).t.type_0 as i32 == JS_TLITSTR as i32
            || (*x).t.type_0 as i32 == JS_TNUMBER as i32)
            && (*y).t.type_0 as i32 == JS_TOBJECT as i32
        {
            jsV_toprimitive(J, y, JS_HNONE as i32);
        } else {
            if !((*x).t.type_0 as i32 == JS_TOBJECT as i32
                && ((*y).t.type_0 as i32 == JS_TSHRSTR as i32
                    || (*y).t.type_0 as i32 == JS_TMEMSTR as i32
                    || (*y).t.type_0 as i32 == JS_TLITSTR as i32
                    || (*y).t.type_0 as i32 == JS_TNUMBER as i32))
            {
                break;
            }
            jsV_toprimitive(J, x, JS_HNONE as i32);
        }
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn js_strictequal(J: &mut js_State) -> i32 {
    let mut x: *mut js_Value = js_tovalue(J, -(2 as i32));
    let mut y: *mut js_Value = js_tovalue(J, -(1 as i32));
    if ((*x).t.type_0 as i32 == JS_TSHRSTR as i32
        || (*x).t.type_0 as i32 == JS_TMEMSTR as i32
        || (*x).t.type_0 as i32 == JS_TLITSTR as i32)
        && ((*y).t.type_0 as i32 == JS_TSHRSTR as i32
            || (*y).t.type_0 as i32 == JS_TMEMSTR as i32
            || (*y).t.type_0 as i32 == JS_TLITSTR as i32)
    {
        return (strcmp(
            if (*x).t.type_0 as i32 == JS_TSHRSTR as i32 {
                ((*x).u.shrstr).as_mut_ptr() as *const libc::c_char
            } else if (*x).t.type_0 as i32 == JS_TLITSTR as i32 {
                (*x).u.litstr
            } else if (*x).t.type_0 as i32 == JS_TMEMSTR as i32 {
                ((*(*x).u.memstr).p).as_mut_ptr() as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if (*y).t.type_0 as i32 == JS_TSHRSTR as i32 {
                ((*y).u.shrstr).as_mut_ptr() as *const libc::c_char
            } else if (*y).t.type_0 as i32 == JS_TLITSTR as i32 {
                (*y).u.litstr
            } else if (*y).t.type_0 as i32 == JS_TMEMSTR as i32 {
                ((*(*y).u.memstr).p).as_mut_ptr() as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        ) == 0) as i32;
    }
    if (*x).t.type_0 as i32 != (*y).t.type_0 as i32 {
        return 0;
    }
    if (*x).t.type_0 as i32 == JS_TUNDEFINED as i32 {
        return 1;
    }
    if (*x).t.type_0 as i32 == JS_TNULL as i32 {
        return 1;
    }
    if (*x).t.type_0 as i32 == JS_TNUMBER as i32 {
        return ((*x).u.number == (*y).u.number) as i32;
    }
    if (*x).t.type_0 as i32 == JS_TBOOLEAN as i32 {
        return ((*x).u.boolean == (*y).u.boolean) as i32;
    }
    if (*x).t.type_0 as i32 == JS_TOBJECT as i32 {
        return ((*x).u.object == (*y).u.object) as i32;
    }
    0
}
