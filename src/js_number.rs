use crate::*;

unsafe extern "C" fn jsB_new_Number(J: &mut js_State) {
    let v = if js_gettop(J) > 1 {
        js_tonumber(J, 1)
    } else {
        0.0
    };
    js_newnumber(J, v);
}
unsafe extern "C" fn jsB_Number(J: &mut js_State) {
    let v = if js_gettop(J) > 1 {
        js_tonumber(J, 1)
    } else {
        0.0
    };
    js_pushnumber(J, v);
}
unsafe extern "C" fn Np_valueOf(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as i32 as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    js_pushnumber(J, (*self_0).u.number);
}
unsafe extern "C" fn Np_toString(J: &mut js_State) {
    let mut buf: [libc::c_char; 100] = [0; 100];
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    let mut radix: i32 = if js_isundefined(J, 1) != 0 {
        10
    } else {
        js_tointeger(J, 1)
    };
    let mut x: f64 = 0.0;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as i32 as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    x = (*self_0).u.number;
    if radix == 10 {
        js_pushstring(J, jsV_numbertostring(buf.as_mut_ptr(), x));
        return;
    }
    if radix < 2 || radix > 36 as i32 {
        js_rangeerror(J, b"invalid radix\0" as *const u8 as *const libc::c_char);
    }
    const DIGITS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut number: f64 = x;
    let mut sign: i32 = (x < 0.0) as i32;
    let mut sb: *mut js_Buffer = core::ptr::null_mut::<js_Buffer>();
    let mut u: uint64_t = 0;
    let limit: u64 = 1 << 52;
    let mut ndigits: i32 = 0;
    let mut exp_0: i32 = 0;
    let mut point: i32 = 0;
    if number == 0.0 {
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
    exp_0 = 0;
    while number * pow(radix as f64, exp_0 as f64) > limit as f64 {
        exp_0 -= 1;
    }
    while number * pow(radix as f64, (exp_0 + 1) as f64) < limit as f64 {
        exp_0 += 1;
    }
    u = (number * pow(radix as f64, exp_0 as f64) + 0.5f64) as uint64_t;
    while u > 0 && u.wrapping_rem(radix as u32) == 0 {
        u = (u as u32).wrapping_div(radix as u32) as uint64_t as uint64_t;
        exp_0 -= 1;
    }
    ndigits = 0;
    while u > 0 {
        let fresh30 = ndigits;
        ndigits += 1;
        buf[fresh30 as usize] = DIGITS[u.wrapping_rem(radix as u32) as usize] as i8;
        u = (u as u32).wrapping_div(radix as u32) as uint64_t as uint64_t;
    }
    point = ndigits - exp_0;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, sb as *mut libc::c_void);
        js_throw(J);
    }
    if sign != 0 {
        js_putc(J, &mut sb, '-' as i32);
    }
    if point <= 0 {
        js_putc(J, &mut sb, '0' as i32);
        js_putc(J, &mut sb, '.' as i32);
        loop {
            let fresh31 = point;
            point += 1;
            if fresh31 >= 0 {
                break;
            }
            js_putc(J, &mut sb, '0' as i32);
        }
        loop {
            let fresh32 = ndigits;
            ndigits -= 1;
            if fresh32 <= 0 {
                break;
            }
            js_putc(J, &mut sb, buf[ndigits as usize] as i32);
        }
    } else {
        loop {
            let fresh33 = ndigits;
            ndigits -= 1;
            if fresh33 <= 0 {
                break;
            }
            js_putc(J, &mut sb, buf[ndigits as usize] as i32);
            point -= 1;
            if point == 0 && ndigits > 0 {
                js_putc(J, &mut sb, '.' as i32);
            }
        }
        loop {
            let fresh34 = point;
            point -= 1;
            if fresh34 <= 0 {
                break;
            }
            js_putc(J, &mut sb, '0' as i32);
        }
    }
    js_putc(J, &mut sb, 0);
    js_pushstring(J, ((*sb).s).as_mut_ptr() as *const libc::c_char);
    js_endtry(J);
    js_free(J, sb as *mut libc::c_void);
}
unsafe extern "C" fn numtostr(
    J: &mut js_State,
    mut fmt: *const libc::c_char,
    mut w: i32,
    mut n: f64,
) {
    let mut buf: [libc::c_char; 50] = [0; 50];
    let mut e: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    sprintf(buf.as_mut_ptr(), fmt, w, n);
    e = strchr(buf.as_mut_ptr(), 'e' as i32);
    if !e.is_null() {
        let mut exp_0: i32 = atoi(e.offset(1 as i32 as isize));
        sprintf(e, b"e%+d\0" as *const u8 as *const libc::c_char, exp_0);
    }
    js_pushstring(J, buf.as_mut_ptr());
}
unsafe extern "C" fn Np_toFixed(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    let mut width: i32 = js_tointeger(J, 1);
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut x: f64 = 0.;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as i32 as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    if width < 0 {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    if width > 20 {
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
        js_pushstring(J, jsV_numbertostring(buf.as_mut_ptr(), x));
    } else {
        numtostr(J, b"%.*f\0" as *const u8 as *const libc::c_char, width, x);
    };
}
unsafe extern "C" fn Np_toExponential(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    let mut width: i32 = js_tointeger(J, 1);
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut x: f64 = 0.;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as i32 as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    if width < 0 {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    if width > 20 {
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
        js_pushstring(J, jsV_numbertostring(buf.as_mut_ptr(), x));
    } else {
        numtostr(J, b"%.*e\0" as *const u8 as *const libc::c_char, width, x);
    };
}
unsafe extern "C" fn Np_toPrecision(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    let mut width: i32 = js_tointeger(J, 1);
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut x: f64 = 0.;
    if (*self_0).type_0 as libc::c_uint != JS_CNUMBER as i32 as libc::c_uint {
        js_typeerror(J, b"not a number\0" as *const u8 as *const libc::c_char);
    }
    if width < 1 {
        js_rangeerror(
            J,
            b"precision %d out of range\0" as *const u8 as *const libc::c_char,
            width,
        );
    }
    if width > 21 as i32 {
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
        js_pushstring(J, jsV_numbertostring(buf.as_mut_ptr(), x));
    } else {
        numtostr(J, b"%.*g\0" as *const u8 as *const libc::c_char, width, x);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initnumber(J: &mut js_State) {
    (*(*J).Number_prototype).u.number = 0.0;
    js_pushobject(J, (*J).Number_prototype);
    jsB_propf(
        J,
        b"Number.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Np_valueOf),
        0,
    );
    jsB_propf(
        J,
        b"Number.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Np_toString),
        1,
    );
    jsB_propf(
        J,
        b"Number.prototype.toLocaleString\0" as *const u8 as *const libc::c_char,
        Some(Np_toString),
        0,
    );
    jsB_propf(
        J,
        b"Number.prototype.toFixed\0" as *const u8 as *const libc::c_char,
        Some(Np_toFixed),
        1,
    );
    jsB_propf(
        J,
        b"Number.prototype.toExponential\0" as *const u8 as *const libc::c_char,
        Some(Np_toExponential),
        1,
    );
    jsB_propf(
        J,
        b"Number.prototype.toPrecision\0" as *const u8 as *const libc::c_char,
        Some(Np_toPrecision),
        1,
    );
    js_newcconstructor(
        J,
        Some(jsB_Number),
        Some(jsB_new_Number),
        b"Number\0" as *const u8 as *const libc::c_char,
        0,
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
        ::core::f32::NAN as f64,
    );
    jsB_propn(
        J,
        b"NEGATIVE_INFINITY\0" as *const u8 as *const libc::c_char,
        -::core::f32::INFINITY as f64,
    );
    jsB_propn(
        J,
        b"POSITIVE_INFINITY\0" as *const u8 as *const libc::c_char,
        ::core::f32::INFINITY as f64,
    );
    js_defglobal(
        J,
        b"Number\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
