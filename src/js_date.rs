use crate::*;

unsafe extern "C" fn Now() -> f64 {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut tv, core::ptr::null_mut::<libc::c_void>());
    floor(tv.tv_sec as f64 * 1000.0f64 + tv.tv_usec as f64 / 1000.0f64)
}
unsafe extern "C" fn LocalTZA() -> f64 {
    static mut once: i32 = 1;
    static mut tza: f64 = 0.0;
    if once != 0 {
        let mut now = time(core::ptr::null_mut());
        let mut utc = mktime(gmtime(&mut now));
        let mut loc = mktime(localtime(&mut now));
        tza = ((loc - utc) * 1000) as f64;
        once = 0;
    }
    tza
}
unsafe extern "C" fn DaylightSavingTA(mut t: f64) -> f64 {
    0.0
}
unsafe extern "C" fn pmod(mut x: f64, mut y: f64) -> f64 {
    x = fmod(x, y);
    if x < 0.0 {
        x += y;
    }
    x
}
unsafe extern "C" fn Day(mut t: f64) -> i32 {
    floor(t / (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64)) as i32
}
unsafe extern "C" fn TimeWithinDay(mut t: f64) -> f64 {
    pmod(t, 24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64)
}
unsafe extern "C" fn DaysInYear(mut y: i32) -> i32 {
    if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
        366 as i32
    } else {
        365 as i32
    }
}
unsafe extern "C" fn DayFromYear(mut y: i32) -> i32 {
    ((365 as i32 * (y - 1970)) as f64 + floor((y - 1969 as i32) as f64 / 4.0f64)
        - floor((y - 1901 as i32) as f64 / 100.0f64)
        + floor((y - 1601 as i32) as f64 / 400.0f64)) as i32
}
unsafe extern "C" fn TimeFromYear(mut y: i32) -> f64 {
    DayFromYear(y) as f64 * (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64)
}
unsafe extern "C" fn YearFromTime(mut t: f64) -> i32 {
    let mut y: i32 =
        (floor(t / (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64 * 365.2425f64)) + 1970.0) as i32;
    let mut t2: f64 = TimeFromYear(y);
    if t2 > t {
        y -= 1;
    } else if t2 + 24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64 * DaysInYear(y) as f64 <= t {
        y += 1;
    }
    y
}
unsafe extern "C" fn InLeapYear(mut t: f64) -> i32 {
    (DaysInYear(YearFromTime(t)) == 366 as i32) as i32
}
unsafe extern "C" fn DayWithinYear(mut t: f64) -> i32 {
    Day(t) - DayFromYear(YearFromTime(t))
}
unsafe extern "C" fn MonthFromTime(mut t: f64) -> i32 {
    let mut day: i32 = DayWithinYear(t);
    let mut leap: i32 = InLeapYear(t);
    if day < 31 as i32 {
        return 0;
    }
    if day < 59 as i32 + leap {
        return 1;
    }
    if day < 90 + leap {
        return 2;
    }
    if day < 120 + leap {
        return 3;
    }
    if day < 151 as i32 + leap {
        return 4;
    }
    if day < 181 as i32 + leap {
        return 5;
    }
    if day < 212 as i32 + leap {
        return 6;
    }
    if day < 243 as i32 + leap {
        return 7;
    }
    if day < 273 as i32 + leap {
        return 8;
    }
    if day < 304 as i32 + leap {
        return 9;
    }
    if day < 334 as i32 + leap {
        return 10;
    }
    11 as i32
}
unsafe extern "C" fn DateFromTime(mut t: f64) -> i32 {
    let mut day: i32 = DayWithinYear(t);
    let mut leap: i32 = InLeapYear(t);
    match MonthFromTime(t) {
        0 => day + 1,
        1 => day - 30,
        2 => day - 58 as i32 - leap,
        3 => day - 89 as i32 - leap,
        4 => day - 119 as i32 - leap,
        5 => day - 150 - leap,
        6 => day - 180 - leap,
        7 => day - 211 as i32 - leap,
        8 => day - 242 as i32 - leap,
        9 => day - 272 as i32 - leap,
        10 => day - 303 as i32 - leap,
        _ => day - 333 as i32 - leap,
    }
}
unsafe extern "C" fn WeekDay(mut t: f64) -> i32 {
    pmod((Day(t) + 4) as f64, 7.0) as i32
}
unsafe extern "C" fn LocalTime(mut utc: f64) -> f64 {
    utc + LocalTZA() + DaylightSavingTA(utc)
}
unsafe extern "C" fn UTC(mut loc: f64) -> f64 {
    loc - LocalTZA() - DaylightSavingTA(loc - LocalTZA())
}
unsafe extern "C" fn HourFromTime(mut t: f64) -> i32 {
    pmod(floor(t / (60.0f64 * 60.0f64 * 1000.0f64)), 24.0f64) as i32
}
unsafe extern "C" fn MinFromTime(mut t: f64) -> i32 {
    pmod(floor(t / (60.0f64 * 1000.0f64)), 60.0f64) as i32
}
unsafe extern "C" fn SecFromTime(mut t: f64) -> i32 {
    pmod(floor(t / 1000.0f64), 60.0f64) as i32
}
unsafe extern "C" fn msFromTime(mut t: f64) -> i32 {
    pmod(t, 1000.0f64) as i32
}
unsafe extern "C" fn MakeTime(mut hour: f64, mut min: f64, mut sec: f64, mut ms: f64) -> f64 {
    ((hour * 60.0f64 + min) * 60.0f64 + sec) * 1000.0f64 + ms
}
unsafe extern "C" fn MakeDay(mut y: f64, mut m: f64, mut date: f64) -> f64 {
    #[rustfmt::skip]
    const firstDayOfMonth: [[usize; 12]; 2] = [
        [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334],
        [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335],
    ];
    let mut yd: f64 = 0.;
    let mut md: f64 = 0.;
    let mut im: i32 = 0;
    y += floor(m / 12 as i32 as f64);
    m = pmod(m, 12 as i32 as f64);
    im = m as i32;
    if im < 0 || im >= 12 as i32 {
        return ::core::f32::NAN as f64;
    }
    yd = floor(TimeFromYear(y as i32) / (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64));
    md = firstDayOfMonth[(DaysInYear(y as i32) == 366) as usize][im as usize] as f64;
    yd + md + date - 1.0
}
unsafe extern "C" fn MakeDate(mut day: f64, mut time_0: f64) -> f64 {
    day * (24.0f64 * 60.0f64 * 60.0f64 * 1000.0f64) + time_0
}
unsafe extern "C" fn TimeClip(mut t: f64) -> f64 {
    if t.is_finite() as i32 == 0 {
        return ::core::f32::NAN as f64;
    }
    if fabs(t) > 8.64e15f64 {
        return ::core::f32::NAN as f64;
    }
    if t < 0.0 {
        -floor(-t)
    } else {
        floor(t)
    }
}
unsafe extern "C" fn toint(mut sp: *mut *const libc::c_char, mut w: i32, mut v: *mut i32) -> i32 {
    let mut s: *const libc::c_char = *sp;
    *v = 0;
    loop {
        let fresh14 = w;
        w -= 1;
        if fresh14 == 0 {
            break;
        }
        if (*s as i32) < '0' as i32 || *s as i32 > '9' as i32 {
            return 0;
        }
        let fresh15 = s;
        s = s.offset(1);
        *v = *v * 10 + (*fresh15 as i32 - '0' as i32);
    }
    *sp = s;
    1
}
unsafe extern "C" fn parseDateTime(mut s: *const libc::c_char) -> f64 {
    let mut y: i32 = 1970;
    let mut m: i32 = 1;
    let mut d: i32 = 1;
    let mut H: i32 = 0;
    let mut M: i32 = 0;
    let mut S: i32 = 0;
    let mut ms: i32 = 0;
    let mut tza: i32 = 0;
    let mut t: f64 = 0.;
    if toint(&mut s, 4, &mut y) == 0 {
        return ::core::f32::NAN as f64;
    }
    if *s as i32 == '-' as i32 {
        s = s.offset(1 as i32 as isize);
        if toint(&mut s, 2, &mut m) == 0 {
            return ::core::f32::NAN as f64;
        }
        if *s as i32 == '-' as i32 {
            s = s.offset(1 as i32 as isize);
            if toint(&mut s, 2, &mut d) == 0 {
                return ::core::f32::NAN as f64;
            }
        }
    }
    if *s as i32 == 'T' as i32 {
        s = s.offset(1 as i32 as isize);
        if toint(&mut s, 2, &mut H) == 0 {
            return ::core::f32::NAN as f64;
        }
        if *s as i32 != ':' as i32 {
            return ::core::f32::NAN as f64;
        }
        s = s.offset(1 as i32 as isize);
        if toint(&mut s, 2, &mut M) == 0 {
            return ::core::f32::NAN as f64;
        }
        if *s as i32 == ':' as i32 {
            s = s.offset(1 as i32 as isize);
            if toint(&mut s, 2, &mut S) == 0 {
                return ::core::f32::NAN as f64;
            }
            if *s as i32 == '.' as i32 {
                s = s.offset(1 as i32 as isize);
                if toint(&mut s, 3, &mut ms) == 0 {
                    return ::core::f32::NAN as f64;
                }
            }
        }
        if *s as i32 == 'Z' as i32 {
            s = s.offset(1 as i32 as isize);
            tza = 0;
        } else if *s as i32 == '+' as i32 || *s as i32 == '-' as i32 {
            let mut tzh: i32 = 0;
            let mut tzm: i32 = 0;
            let mut tzs: i32 = if *s as i32 == '+' as i32 {
                1
            } else {
                -(1 as i32)
            };
            s = s.offset(1 as i32 as isize);
            if toint(&mut s, 2, &mut tzh) == 0 {
                return ::core::f32::NAN as f64;
            }
            if *s as i32 == ':' as i32 {
                s = s.offset(1 as i32 as isize);
                if toint(&mut s, 2, &mut tzm) == 0 {
                    return ::core::f32::NAN as f64;
                }
            }
            if tzh > 23 as i32 || tzm > 59 as i32 {
                return ::core::f32::NAN as f64;
            }
            tza = (tzs as f64
                * (tzh as f64 * (60.0f64 * 60.0f64 * 1000.0f64)
                    + tzm as f64 * (60.0f64 * 1000.0f64))) as i32;
        } else {
            tza = LocalTZA() as i32;
        }
    }
    if *s != 0 {
        return ::core::f32::NAN as f64;
    }
    if m < 1 || m > 12 as i32 {
        return ::core::f32::NAN as f64;
    }
    if d < 1 || d > 31 as i32 {
        return ::core::f32::NAN as f64;
    }
    if H < 0 || H > 24 as i32 {
        return ::core::f32::NAN as f64;
    }
    if M < 0 || M > 59 as i32 {
        return ::core::f32::NAN as f64;
    }
    if S < 0 || S > 59 as i32 {
        return ::core::f32::NAN as f64;
    }
    if ms < 0 || ms > 999 as i32 {
        return ::core::f32::NAN as f64;
    }
    if H == 24 as i32 && (M != 0 || S != 0 || ms != 0) {
        return ::core::f32::NAN as f64;
    }
    t = MakeDate(
        MakeDay(y as f64, (m - 1) as f64, d as f64),
        MakeTime(H as f64, M as f64, S as f64, ms as f64),
    );
    t - tza as f64
}
unsafe extern "C" fn fmtdate(mut buf: *mut libc::c_char, mut t: f64) -> *mut libc::c_char {
    let mut y: i32 = YearFromTime(t);
    let mut m: i32 = MonthFromTime(t);
    let mut d: i32 = DateFromTime(t);
    if t.is_finite() as i32 == 0 {
        return b"Invalid Date\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    sprintf(
        buf,
        b"%04d-%02d-%02d\0" as *const u8 as *const libc::c_char,
        y,
        m + 1,
        d,
    );
    buf
}
unsafe extern "C" fn fmttime(
    mut buf: *mut libc::c_char,
    mut t: f64,
    mut tza: f64,
) -> *mut libc::c_char {
    let mut H: i32 = HourFromTime(t);
    let mut M: i32 = MinFromTime(t);
    let mut S: i32 = SecFromTime(t);
    let mut ms: i32 = msFromTime(t);
    let mut tzh: i32 = HourFromTime(fabs(tza));
    let mut tzm: i32 = MinFromTime(fabs(tza));
    if t.is_finite() as i32 == 0 {
        return b"Invalid Date\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if tza == 0.0 {
        sprintf(
            buf,
            b"%02d:%02d:%02d.%03dZ\0" as *const u8 as *const libc::c_char,
            H,
            M,
            S,
            ms,
        );
    } else if tza < 0.0 {
        sprintf(
            buf,
            b"%02d:%02d:%02d.%03d-%02d:%02d\0" as *const u8 as *const libc::c_char,
            H,
            M,
            S,
            ms,
            tzh,
            tzm,
        );
    } else {
        sprintf(
            buf,
            b"%02d:%02d:%02d.%03d+%02d:%02d\0" as *const u8 as *const libc::c_char,
            H,
            M,
            S,
            ms,
            tzh,
            tzm,
        );
    }
    buf
}
unsafe extern "C" fn fmtdatetime(
    mut buf: *mut libc::c_char,
    mut t: f64,
    mut tza: f64,
) -> *mut libc::c_char {
    let mut dbuf: [libc::c_char; 20] = [0; 20];
    let mut tbuf: [libc::c_char; 20] = [0; 20];
    if t.is_finite() as i32 == 0 {
        return b"Invalid Date\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    fmtdate(dbuf.as_mut_ptr(), t);
    fmttime(tbuf.as_mut_ptr(), t, tza);
    sprintf(
        buf,
        b"%sT%s\0" as *const u8 as *const libc::c_char,
        dbuf.as_mut_ptr(),
        tbuf.as_mut_ptr(),
    );
    buf
}
unsafe extern "C" fn js_todate(J: &mut js_State, mut idx: i32) -> f64 {
    let mut self_0: *mut js_Object = js_toobject(J, idx);
    if (*self_0).type_0 as libc::c_uint != JS_CDATE as i32 as libc::c_uint {
        js_typeerror(J, b"not a date\0" as *const u8 as *const libc::c_char);
    }
    (*self_0).u.number
}
unsafe extern "C" fn js_setdate(J: &mut js_State, mut idx: i32, mut t: f64) {
    let mut self_0: *mut js_Object = js_toobject(J, idx);
    if (*self_0).type_0 as libc::c_uint != JS_CDATE as i32 as libc::c_uint {
        js_typeerror(J, b"not a date\0" as *const u8 as *const libc::c_char);
    }
    (*self_0).u.number = TimeClip(t);
    js_pushnumber(J, (*self_0).u.number);
}
unsafe extern "C" fn D_parse(J: &mut js_State) {
    let mut t: f64 = parseDateTime(js_tostring(J, 1));
    js_pushnumber(J, t);
}
unsafe extern "C" fn D_UTC(J: &mut js_State) {
    let mut y: f64 = 0.;
    let mut m: f64 = 0.;
    let mut d: f64 = 0.;
    let mut H: f64 = 0.;
    let mut M: f64 = 0.;
    let mut S: f64 = 0.;
    let mut ms: f64 = 0.;
    let mut t: f64 = 0.;
    y = js_tonumber(J, 1);
    if y < 100.0 {
        y += 1900.0;
    }
    m = js_tonumber(J, 2);
    d = if js_isdefined(J, 3) != 0 {
        js_tonumber(J, 3)
    } else {
        1.0
    };
    H = if js_isdefined(J, 4) != 0 {
        js_tonumber(J, 4)
    } else {
        0.0
    };
    M = if js_isdefined(J, 5) != 0 {
        js_tonumber(J, 5)
    } else {
        0.0
    };
    S = if js_isdefined(J, 6) != 0 {
        js_tonumber(J, 6)
    } else {
        0.0
    };
    ms = if js_isdefined(J, 7) != 0 {
        js_tonumber(J, 7)
    } else {
        0.0
    };
    t = MakeDate(MakeDay(y, m, d), MakeTime(H, M, S, ms));
    t = TimeClip(t);
    js_pushnumber(J, t);
}
unsafe extern "C" fn D_now(J: &mut js_State) {
    js_pushnumber(J, Now());
}
unsafe extern "C" fn jsB_Date(J: &mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    js_pushstring(
        J,
        fmtdatetime(buf.as_mut_ptr(), LocalTime(Now()), LocalTZA()),
    );
}
unsafe extern "C" fn jsB_new_Date(J: &mut js_State) {
    let mut top: i32 = js_gettop(J);
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut t: f64 = 0.;
    if top == 1 {
        t = Now();
    } else if top == 2 {
        js_toprimitive(J, 1, JS_HNONE as i32);
        if js_isstring(J, 1) != 0 {
            t = parseDateTime(js_tostring(J, 1));
        } else {
            t = TimeClip(js_tonumber(J, 1));
        }
    } else {
        let mut y: f64 = 0.;
        let mut m: f64 = 0.;
        let mut d: f64 = 0.;
        let mut H: f64 = 0.;
        let mut M: f64 = 0.;
        let mut S: f64 = 0.;
        let mut ms: f64 = 0.;
        y = js_tonumber(J, 1);
        if y < 100.0 {
            y += 1900.0;
        }
        m = js_tonumber(J, 2);
        d = if js_isdefined(J, 3) != 0 {
            js_tonumber(J, 3)
        } else {
            1.0
        };
        H = if js_isdefined(J, 4) != 0 {
            js_tonumber(J, 4)
        } else {
            0.0
        };
        M = if js_isdefined(J, 5) != 0 {
            js_tonumber(J, 5)
        } else {
            0.0
        };
        S = if js_isdefined(J, 6) != 0 {
            js_tonumber(J, 6)
        } else {
            0.0
        };
        ms = if js_isdefined(J, 7) != 0 {
            js_tonumber(J, 7)
        } else {
            0.0
        };
        t = MakeDate(MakeDay(y, m, d), MakeTime(H, M, S, ms));
        t = TimeClip(UTC(t));
    }
    obj = jsV_newobject(J, JS_CDATE, (*J).Date_prototype);
    (*obj).u.number = t;
    js_pushobject(J, obj);
}
unsafe extern "C" fn Dp_valueOf(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    js_pushnumber(J, t);
}
unsafe extern "C" fn Dp_toString(J: &mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut t: f64 = js_todate(J, 0);
    js_pushstring(J, fmtdatetime(buf.as_mut_ptr(), LocalTime(t), LocalTZA()));
}
unsafe extern "C" fn Dp_toDateString(J: &mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut t: f64 = js_todate(J, 0);
    js_pushstring(J, fmtdate(buf.as_mut_ptr(), LocalTime(t)));
}
unsafe extern "C" fn Dp_toTimeString(J: &mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut t: f64 = js_todate(J, 0);
    js_pushstring(J, fmttime(buf.as_mut_ptr(), LocalTime(t), LocalTZA()));
}
unsafe extern "C" fn Dp_toUTCString(J: &mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut t: f64 = js_todate(J, 0);
    js_pushstring(J, fmtdatetime(buf.as_mut_ptr(), t, 0.0));
}
unsafe extern "C" fn Dp_toISOString(J: &mut js_State) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut t: f64 = js_todate(J, 0);
    if t.is_finite() as i32 == 0 {
        js_rangeerror(J, b"invalid date\0" as *const u8 as *const libc::c_char);
    }
    js_pushstring(J, fmtdatetime(buf.as_mut_ptr(), t, 0.0));
}
unsafe extern "C" fn Dp_getFullYear(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, YearFromTime(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getMonth(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, MonthFromTime(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getDate(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, DateFromTime(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getDay(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, WeekDay(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getHours(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, HourFromTime(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getMinutes(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, MinFromTime(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getSeconds(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, SecFromTime(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getMilliseconds(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, msFromTime(LocalTime(t)) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCFullYear(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, YearFromTime(t) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCMonth(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, MonthFromTime(t) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCDate(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, DateFromTime(t) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCDay(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, WeekDay(t) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCHours(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, HourFromTime(t) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCMinutes(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, MinFromTime(t) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCSeconds(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, SecFromTime(t) as f64);
    };
}
unsafe extern "C" fn Dp_getUTCMilliseconds(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, msFromTime(t) as f64);
    };
}
unsafe extern "C" fn Dp_getTimezoneOffset(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    if t.is_nan() as i32 != 0 {
        js_pushnumber(J, ::core::f32::NAN as f64);
    } else {
        js_pushnumber(J, (t - LocalTime(t)) / (60.0f64 * 1000.0f64));
    };
}
unsafe extern "C" fn Dp_setTime(J: &mut js_State) {
    let t = js_tonumber(J, 1);
    js_setdate(J, 0, t);
}
unsafe extern "C" fn Dp_setMilliseconds(J: &mut js_State) {
    let mut t: f64 = LocalTime(js_todate(J, 0));
    let mut h: f64 = HourFromTime(t) as f64;
    let mut m: f64 = MinFromTime(t) as f64;
    let mut s: f64 = SecFromTime(t) as f64;
    let mut ms: f64 = js_tonumber(J, 1);
    js_setdate(J, 0, UTC(MakeDate(Day(t) as f64, MakeTime(h, m, s, ms))));
}
unsafe extern "C" fn Dp_setSeconds(J: &mut js_State) {
    let mut t: f64 = LocalTime(js_todate(J, 0));
    let mut h: f64 = HourFromTime(t) as f64;
    let mut m: f64 = MinFromTime(t) as f64;
    let mut s: f64 = js_tonumber(J, 1);
    let mut ms: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        msFromTime(t) as f64
    };
    js_setdate(J, 0, UTC(MakeDate(Day(t) as f64, MakeTime(h, m, s, ms))));
}
unsafe extern "C" fn Dp_setMinutes(J: &mut js_State) {
    let mut t: f64 = LocalTime(js_todate(J, 0));
    let mut h: f64 = HourFromTime(t) as f64;
    let mut m: f64 = js_tonumber(J, 1);
    let mut s: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        SecFromTime(t) as f64
    };
    let mut ms: f64 = if js_isdefined(J, 3) != 0 {
        js_tonumber(J, 3)
    } else {
        msFromTime(t) as f64
    };
    js_setdate(J, 0, UTC(MakeDate(Day(t) as f64, MakeTime(h, m, s, ms))));
}
unsafe extern "C" fn Dp_setHours(J: &mut js_State) {
    let mut t: f64 = LocalTime(js_todate(J, 0));
    let mut h: f64 = js_tonumber(J, 1);
    let mut m: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        MinFromTime(t) as f64
    };
    let mut s: f64 = if js_isdefined(J, 3) != 0 {
        js_tonumber(J, 3)
    } else {
        SecFromTime(t) as f64
    };
    let mut ms: f64 = if js_isdefined(J, 4) != 0 {
        js_tonumber(J, 4)
    } else {
        msFromTime(t) as f64
    };
    js_setdate(J, 0, UTC(MakeDate(Day(t) as f64, MakeTime(h, m, s, ms))));
}
unsafe extern "C" fn Dp_setDate(J: &mut js_State) {
    let mut t: f64 = LocalTime(js_todate(J, 0));
    let mut y: f64 = YearFromTime(t) as f64;
    let mut m: f64 = MonthFromTime(t) as f64;
    let mut d: f64 = js_tonumber(J, 1);
    js_setdate(J, 0, UTC(MakeDate(MakeDay(y, m, d), TimeWithinDay(t))));
}
unsafe extern "C" fn Dp_setMonth(J: &mut js_State) {
    let mut t: f64 = LocalTime(js_todate(J, 0));
    let mut y: f64 = YearFromTime(t) as f64;
    let mut m: f64 = js_tonumber(J, 1);
    let mut d: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        DateFromTime(t) as f64
    };
    js_setdate(J, 0, UTC(MakeDate(MakeDay(y, m, d), TimeWithinDay(t))));
}
unsafe extern "C" fn Dp_setFullYear(J: &mut js_State) {
    let mut t: f64 = LocalTime(js_todate(J, 0));
    let mut y: f64 = js_tonumber(J, 1);
    let mut m: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        MonthFromTime(t) as f64
    };
    let mut d: f64 = if js_isdefined(J, 3) != 0 {
        js_tonumber(J, 3)
    } else {
        DateFromTime(t) as f64
    };
    js_setdate(J, 0, UTC(MakeDate(MakeDay(y, m, d), TimeWithinDay(t))));
}
unsafe extern "C" fn Dp_setUTCMilliseconds(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    let mut h: f64 = HourFromTime(t) as f64;
    let mut m: f64 = MinFromTime(t) as f64;
    let mut s: f64 = SecFromTime(t) as f64;
    let mut ms: f64 = js_tonumber(J, 1);
    js_setdate(J, 0, MakeDate(Day(t) as f64, MakeTime(h, m, s, ms)));
}
unsafe extern "C" fn Dp_setUTCSeconds(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    let mut h: f64 = HourFromTime(t) as f64;
    let mut m: f64 = MinFromTime(t) as f64;
    let mut s: f64 = js_tonumber(J, 1);
    let mut ms: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        msFromTime(t) as f64
    };
    js_setdate(J, 0, MakeDate(Day(t) as f64, MakeTime(h, m, s, ms)));
}
unsafe extern "C" fn Dp_setUTCMinutes(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    let mut h: f64 = HourFromTime(t) as f64;
    let mut m: f64 = js_tonumber(J, 1);
    let mut s: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        SecFromTime(t) as f64
    };
    let mut ms: f64 = if js_isdefined(J, 3) != 0 {
        js_tonumber(J, 3)
    } else {
        msFromTime(t) as f64
    };
    js_setdate(J, 0, MakeDate(Day(t) as f64, MakeTime(h, m, s, ms)));
}
unsafe extern "C" fn Dp_setUTCHours(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    let mut h: f64 = js_tonumber(J, 1);
    let mut m: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        HourFromTime(t) as f64
    };
    let mut s: f64 = if js_isdefined(J, 3) != 0 {
        js_tonumber(J, 3)
    } else {
        SecFromTime(t) as f64
    };
    let mut ms: f64 = if js_isdefined(J, 4) != 0 {
        js_tonumber(J, 4)
    } else {
        msFromTime(t) as f64
    };
    js_setdate(J, 0, MakeDate(Day(t) as f64, MakeTime(h, m, s, ms)));
}
unsafe extern "C" fn Dp_setUTCDate(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    let mut y: f64 = YearFromTime(t) as f64;
    let mut m: f64 = MonthFromTime(t) as f64;
    let mut d: f64 = js_tonumber(J, 1);
    js_setdate(J, 0, MakeDate(MakeDay(y, m, d), TimeWithinDay(t)));
}
unsafe extern "C" fn Dp_setUTCMonth(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    let mut y: f64 = YearFromTime(t) as f64;
    let mut m: f64 = js_tonumber(J, 1);
    let mut d: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        DateFromTime(t) as f64
    };
    js_setdate(J, 0, MakeDate(MakeDay(y, m, d), TimeWithinDay(t)));
}
unsafe extern "C" fn Dp_setUTCFullYear(J: &mut js_State) {
    let mut t: f64 = js_todate(J, 0);
    let mut y: f64 = js_tonumber(J, 1);
    let mut m: f64 = if js_isdefined(J, 2) != 0 {
        js_tonumber(J, 2)
    } else {
        MonthFromTime(t) as f64
    };
    let mut d: f64 = if js_isdefined(J, 3) != 0 {
        js_tonumber(J, 3)
    } else {
        DateFromTime(t) as f64
    };
    js_setdate(J, 0, MakeDate(MakeDay(y, m, d), TimeWithinDay(t)));
}
unsafe extern "C" fn Dp_toJSON(J: &mut js_State) {
    js_copy(J, 0);
    js_toprimitive(J, -(1 as i32), JS_HNUMBER as i32);
    if js_isnumber(J, -(1 as i32)) != 0 && (js_tonumber(J, -(1 as i32))).is_finite() as i32 == 0 {
        js_pushnull(J);
        return;
    }
    js_pop(J, 1);
    js_getproperty(J, 0, b"toISOString\0" as *const u8 as *const libc::c_char);
    if js_iscallable(J, -(1 as i32)) == 0 {
        js_typeerror(
            J,
            b"this.toISOString is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    js_copy(J, 0);
    js_call(J, 0);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initdate(J: &mut js_State) {
    (*(*J).Date_prototype).u.number = 0.0;
    js_pushobject(J, (*J).Date_prototype);
    jsB_propf(
        J,
        b"Date.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Dp_valueOf),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toDateString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toDateString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toTimeString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toTimeString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toLocaleString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toLocaleDateString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toDateString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toLocaleTimeString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toTimeString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toUTCString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toUTCString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getTime\0" as *const u8 as *const libc::c_char,
        Some(Dp_valueOf),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_getFullYear),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCFullYear),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_getMonth),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCMonth),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_getDate),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCDate),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getDay\0" as *const u8 as *const libc::c_char,
        Some(Dp_getDay),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCDay\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCDay),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_getHours),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCHours),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_getMinutes),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCMinutes),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getSeconds),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCSeconds),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getMilliseconds),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getUTCMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_getUTCMilliseconds),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.getTimezoneOffset\0" as *const u8 as *const libc::c_char,
        Some(Dp_getTimezoneOffset),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.setTime\0" as *const u8 as *const libc::c_char,
        Some(Dp_setTime),
        1,
    );
    jsB_propf(
        J,
        b"Date.prototype.setMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setMilliseconds),
        1,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCMilliseconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCMilliseconds),
        1,
    );
    jsB_propf(
        J,
        b"Date.prototype.setSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setSeconds),
        2,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCSeconds\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCSeconds),
        2,
    );
    jsB_propf(
        J,
        b"Date.prototype.setMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_setMinutes),
        3,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCMinutes\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCMinutes),
        3,
    );
    jsB_propf(
        J,
        b"Date.prototype.setHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_setHours),
        4,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCHours\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCHours),
        4,
    );
    jsB_propf(
        J,
        b"Date.prototype.setDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_setDate),
        1,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCDate\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCDate),
        1,
    );
    jsB_propf(
        J,
        b"Date.prototype.setMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_setMonth),
        2,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCMonth\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCMonth),
        2,
    );
    jsB_propf(
        J,
        b"Date.prototype.setFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_setFullYear),
        3,
    );
    jsB_propf(
        J,
        b"Date.prototype.setUTCFullYear\0" as *const u8 as *const libc::c_char,
        Some(Dp_setUTCFullYear),
        3,
    );
    jsB_propf(
        J,
        b"Date.prototype.toISOString\0" as *const u8 as *const libc::c_char,
        Some(Dp_toISOString),
        0,
    );
    jsB_propf(
        J,
        b"Date.prototype.toJSON\0" as *const u8 as *const libc::c_char,
        Some(Dp_toJSON),
        1,
    );
    js_newcconstructor(
        J,
        Some(jsB_Date),
        Some(jsB_new_Date),
        b"Date\0" as *const u8 as *const libc::c_char,
        0,
    );
    jsB_propf(
        J,
        b"Date.parse\0" as *const u8 as *const libc::c_char,
        Some(D_parse),
        1,
    );
    jsB_propf(
        J,
        b"Date.UTC\0" as *const u8 as *const libc::c_char,
        Some(D_UTC),
        7,
    );
    jsB_propf(
        J,
        b"Date.now\0" as *const u8 as *const libc::c_char,
        Some(D_now),
        0,
    );
    js_defglobal(
        J,
        b"Date\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
