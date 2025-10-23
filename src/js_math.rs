use crate::*;

extern "C" fn jsM_round(x: f64) -> f64 {
    if x.is_nan() || x.is_infinite() || x == 0. {
        x
    } else if x > 0. && x < 0.5 {
        0.
    } else if x < 0. && x >= -0.5 {
        -0.
    } else {
        (x + 0.5).floor()
    }
}
unsafe extern "C" fn Math_abs(J: &mut js_State) {
    let v = js_tonumber(J, 1).abs();
    js_pushnumber(J, v);
}
unsafe extern "C" fn Math_acos(J: &mut js_State) {
    let v = acos(js_tonumber(J, 1));
    js_pushnumber(J, v);
}
unsafe extern "C" fn Math_asin(J: &mut js_State) {
    let v = asin(js_tonumber(J, 1));
    js_pushnumber(J, v);
}
unsafe extern "C" fn Math_atan(J: &mut js_State) {
    let v = atan(js_tonumber(J, 1));
    js_pushnumber(J, v);
}
unsafe extern "C" fn Math_atan2(J: &mut js_State) {
    let mut y: f64 = js_tonumber(J, 1);
    let mut x: f64 = js_tonumber(J, 2);
    js_pushnumber(J, atan2(y, x));
}
unsafe extern "C" fn Math_ceil(J: &mut js_State) {
    let v = js_tonumber(J, 1);
    js_pushnumber(J, v.ceil());
}
unsafe extern "C" fn Math_cos(J: &mut js_State) {
    let v = js_tonumber(J, 1);
    js_pushnumber(J, cos(v));
}
unsafe extern "C" fn Math_exp(J: &mut js_State) {
    let v = js_tonumber(J, 1);
    js_pushnumber(J, exp(v));
}
unsafe extern "C" fn Math_floor(J: &mut js_State) {
    let v = js_tonumber(J, 1);
    js_pushnumber(J, v.floor());
}
unsafe extern "C" fn Math_log(J: &mut js_State) {
    let v = js_tonumber(J, 1);
    js_pushnumber(J, log(v));
}
unsafe extern "C" fn Math_pow(J: &mut js_State) {
    let x = js_tonumber(J, 1);
    let y = js_tonumber(J, 2);
    if y.is_finite() && x.abs() == 1.0 {
        js_pushnumber(J, f64::NAN);
    } else {
        js_pushnumber(J, x.powf(y));
    };
}
unsafe extern "C" fn Math_random(J: &mut js_State) {
    (*J).seed = (((*J).seed as u64).wrapping_mul(48271) % 0x7fffffff) as u32;
    js_pushnumber(J, (*J).seed as f64 / 0x7fffffff as f64);
}
unsafe extern "C" fn Math_init_random(J: &mut js_State) {
    let mut seed = (time(core::ptr::null_mut::<time_t>()) + 123) as u32;
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    (*J).seed = seed % 0x7fffffff;
}
unsafe extern "C" fn Math_round(J: &mut js_State) {
    let x = js_tonumber(J, 1);
    js_pushnumber(J, jsM_round(x));
}
unsafe extern "C" fn Math_sin(J: &mut js_State) {
    let v = js_tonumber(J, 1);

    js_pushnumber(J, sin(v));
}
unsafe extern "C" fn Math_sqrt(J: &mut js_State) {
    let v = js_tonumber(J, 1);

    js_pushnumber(J, sqrt(v));
}
unsafe extern "C" fn Math_tan(J: &mut js_State) {
    let v = js_tonumber(J, 1);

    js_pushnumber(J, tan(v));
}
unsafe extern "C" fn Math_max(J: &mut js_State) {
    let mut n = js_gettop(J);
    let mut x = f64::NEG_INFINITY;
    for i in 1..n {
        let mut y = js_tonumber(J, i);
        if y.is_nan() {
            x = y;
            break;
        } else {
            if x.is_sign_negative() == y.is_sign_negative() {
                x = if x > y { x } else { y };
            } else if x.is_sign_negative() {
                x = y;
            }
        }
    }
    js_pushnumber(J, x);
}
unsafe extern "C" fn Math_min(J: &mut js_State) {
    let mut n = js_gettop(J);
    let mut x = f64::NEG_INFINITY;
    for i in 1..n {
        let mut y = js_tonumber(J, i);
        if y.is_nan() as i32 != 0 {
            x = y;
            break;
        } else {
            if x.is_sign_negative() == y.is_sign_negative() {
                x = if x < y { x } else { y };
            } else if y.is_sign_negative() {
                x = y;
            }
        }
    }
    js_pushnumber(J, x);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initmath(J: &mut js_State) {
    Math_init_random(J);
    let v = jsV_newobject(J, JS_CMATH, (*J).Object_prototype);
    js_pushobject(J, v);
    jsB_propn(
        J,
        b"E\0" as *const u8 as *const libc::c_char,
        core::f64::consts::E,
    );
    jsB_propn(
        J,
        b"LN10\0" as *const u8 as *const libc::c_char,
        core::f64::consts::LN_10,
    );
    jsB_propn(
        J,
        b"LN2\0" as *const u8 as *const libc::c_char,
        core::f64::consts::LN_2,
    );
    jsB_propn(
        J,
        b"LOG2E\0" as *const u8 as *const libc::c_char,
        core::f64::consts::LOG2_E,
    );
    jsB_propn(
        J,
        b"LOG10E\0" as *const u8 as *const libc::c_char,
        core::f64::consts::LOG10_E,
    );
    jsB_propn(
        J,
        b"PI\0" as *const u8 as *const libc::c_char,
        core::f64::consts::PI,
    );
    jsB_propn(
        J,
        b"SQRT1_2\0" as *const u8 as *const libc::c_char,
        core::f64::consts::FRAC_1_SQRT_2,
    );
    jsB_propn(
        J,
        b"SQRT2\0" as *const u8 as *const libc::c_char,
        core::f64::consts::SQRT_2,
    );
    jsB_propf(
        J,
        b"Math.abs\0" as *const u8 as *const libc::c_char,
        Some(Math_abs),
        1,
    );
    jsB_propf(
        J,
        b"Math.acos\0" as *const u8 as *const libc::c_char,
        Some(Math_acos),
        1,
    );
    jsB_propf(
        J,
        b"Math.asin\0" as *const u8 as *const libc::c_char,
        Some(Math_asin),
        1,
    );
    jsB_propf(
        J,
        b"Math.atan\0" as *const u8 as *const libc::c_char,
        Some(Math_atan),
        1,
    );
    jsB_propf(
        J,
        b"Math.atan2\0" as *const u8 as *const libc::c_char,
        Some(Math_atan2),
        2,
    );
    jsB_propf(
        J,
        b"Math.ceil\0" as *const u8 as *const libc::c_char,
        Some(Math_ceil),
        1,
    );
    jsB_propf(
        J,
        b"Math.cos\0" as *const u8 as *const libc::c_char,
        Some(Math_cos),
        1,
    );
    jsB_propf(
        J,
        b"Math.exp\0" as *const u8 as *const libc::c_char,
        Some(Math_exp),
        1,
    );
    jsB_propf(
        J,
        b"Math.floor\0" as *const u8 as *const libc::c_char,
        Some(Math_floor),
        1,
    );
    jsB_propf(
        J,
        b"Math.log\0" as *const u8 as *const libc::c_char,
        Some(Math_log),
        1,
    );
    jsB_propf(
        J,
        b"Math.max\0" as *const u8 as *const libc::c_char,
        Some(Math_max),
        0,
    );
    jsB_propf(
        J,
        b"Math.min\0" as *const u8 as *const libc::c_char,
        Some(Math_min),
        0,
    );
    jsB_propf(
        J,
        b"Math.pow\0" as *const u8 as *const libc::c_char,
        Some(Math_pow),
        2,
    );
    jsB_propf(
        J,
        b"Math.random\0" as *const u8 as *const libc::c_char,
        Some(Math_random),
        0,
    );
    jsB_propf(
        J,
        b"Math.round\0" as *const u8 as *const libc::c_char,
        Some(Math_round),
        1,
    );
    jsB_propf(
        J,
        b"Math.sin\0" as *const u8 as *const libc::c_char,
        Some(Math_sin),
        1,
    );
    jsB_propf(
        J,
        b"Math.sqrt\0" as *const u8 as *const libc::c_char,
        Some(Math_sqrt),
        1,
    );
    jsB_propf(
        J,
        b"Math.tan\0" as *const u8 as *const libc::c_char,
        Some(Math_tan),
        1,
    );
    js_defglobal(
        J,
        b"Math\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
