use crate::*;
#[no_mangle]
pub unsafe extern "C" fn js_getlength(J: &mut js_State, idx: i32) -> i32 {
    js_getproperty(J, idx, b"length\0" as *const u8 as *const libc::c_char);
    let len = js_tointeger(J, -(1));
    js_pop(J, 1);
    len
}
#[no_mangle]
pub unsafe extern "C" fn js_setlength(J: &mut js_State, idx: i32, len: i32) {
    js_pushnumber(J, len as f64);
    js_setproperty(J, if idx < 0 { idx - 1 } else { idx }, c"length".as_ptr());
}
unsafe extern "C" fn jsB_new_Array(J: &mut js_State) {
    let mut i = 0;
    let mut top = js_gettop(J);
    js_newarray(J);
    if top == 2 {
        if js_isnumber(J, 1) != 0 {
            js_copy(J, 1);
            js_setproperty(
                J,
                -(2 as i32),
                b"length\0" as *const u8 as *const libc::c_char,
            );
        } else {
            js_copy(J, 1);
            js_setindex(J, -(2 as i32), 0);
        }
    } else {
        i = 1;
        while i < top {
            js_copy(J, i);
            js_setindex(J, -(2 as i32), i - 1);
            i += 1;
        }
    };
}
unsafe extern "C" fn Ap_concat(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    js_newarray(J);
    n = 0;
    i = 0;
    while i < top {
        js_copy(J, i);
        if js_isarray(J, -(1)) != 0 {
            len = js_getlength(J, -(1));
            k = 0;
            while k < len {
                if js_hasindex(J, -(1), k) != 0 {
                    let fresh0 = n;
                    n += 1;
                    js_setindex(J, -(3 as i32), fresh0);
                }
                k += 1;
            }
            js_pop(J, 1);
        } else {
            let fresh1 = n;
            n += 1;
            js_setindex(J, -(2 as i32), fresh1);
        }
        i += 1;
    }
}
unsafe extern "C" fn Ap_join(J: &mut js_State) {
    let mut out: *mut libc::c_char = core::ptr::null_mut::<libc::c_char>();
    let mut r: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut sep: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut seplen: i32 = 0;
    let mut k: i32 = 0;
    let mut n: i32 = 0;
    let mut len: i32 = 0;
    let mut rlen: i32 = 0;
    len = js_getlength(J, 0);
    if js_isdefined(J, 1) != 0 {
        sep = js_tostring(J, 1);
        seplen = strlen(sep) as i32;
    } else {
        sep = b",\0" as *const u8 as *const libc::c_char;
        seplen = 1;
    }
    if len <= 0 {
        js_pushliteral(J, b"\0" as *const u8 as *const libc::c_char);
        return;
    }
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        js_free(J, out as *mut libc::c_void);
        js_throw(J);
    }
    n = 0;
    k = 0;
    while k < len {
        js_getindex(J, 0, k);
        if js_iscoercible(J, -(1)) != 0 {
            ::core::ptr::write_volatile(&mut r as *mut *const libc::c_char, js_tostring(J, -(1)));
            rlen = strlen(r) as i32;
        } else {
            rlen = 0;
        }
        if k == 0 {
            ::core::ptr::write_volatile(
                &mut out as *mut *mut libc::c_char,
                js_malloc(J, rlen + 1) as *mut libc::c_char,
            );
            if rlen > 0 {
                memcpy(
                    out as *mut libc::c_void,
                    r as *const libc::c_void,
                    rlen as u32,
                );
                n += rlen;
            }
        } else {
            if n + seplen + rlen > (1) << 28 as i32 {
                js_rangeerror(
                    J,
                    b"invalid string length\0" as *const u8 as *const libc::c_char,
                );
            }
            ::core::ptr::write_volatile(
                &mut out as *mut *mut libc::c_char,
                js_realloc(J, out as *mut libc::c_void, n + seplen + rlen + 1) as *mut libc::c_char,
            );
            if seplen > 0 {
                memcpy(
                    out.offset(n as isize) as *mut libc::c_void,
                    sep as *const libc::c_void,
                    seplen as u32,
                );
                n += seplen;
            }
            if rlen > 0 {
                memcpy(
                    out.offset(n as isize) as *mut libc::c_void,
                    r as *const libc::c_void,
                    rlen as u32,
                );
                n += rlen;
            }
        }
        js_pop(J, 1);
        k += 1;
    }
    js_pushlstring(J, out, n);
    js_endtry(J);
    js_free(J, out as *mut libc::c_void);
}
unsafe extern "C" fn Ap_pop(J: &mut js_State) {
    let mut n: i32 = 0;
    n = js_getlength(J, 0);
    if n > 0 {
        js_getindex(J, 0, n - 1);
        js_delindex(J, 0, n - 1);
        js_setlength(J, 0, n - 1);
    } else {
        js_setlength(J, 0, 0);
        js_pushundefined(J);
    };
}
unsafe extern "C" fn Ap_push(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    let mut n: i32 = 0;
    n = js_getlength(J, 0);
    i = 1;
    while i < top {
        js_copy(J, i);
        js_setindex(J, 0, n);
        i += 1;

        n += 1;
    }
    js_setlength(J, 0, n);
    js_pushnumber(J, n as f64);
}
unsafe extern "C" fn Ap_reverse(J: &mut js_State) {
    let mut len: i32 = 0;
    let mut middle: i32 = 0;
    let mut lower: i32 = 0;
    len = js_getlength(J, 0);
    middle = len / 2;
    lower = 0;
    while lower != middle {
        let mut upper: i32 = len - lower - 1;
        let mut haslower: i32 = js_hasindex(J, 0, lower);
        let mut hasupper: i32 = js_hasindex(J, 0, upper);
        if haslower != 0 && hasupper != 0 {
            js_setindex(J, 0, lower);
            js_setindex(J, 0, upper);
        } else if hasupper != 0 {
            js_setindex(J, 0, lower);
            js_delindex(J, 0, upper);
        } else if haslower != 0 {
            js_setindex(J, 0, upper);
            js_delindex(J, 0, lower);
        }
        lower += 1;
    }
    js_copy(J, 0);
}
unsafe extern "C" fn Ap_shift(J: &mut js_State) {
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    len = js_getlength(J, 0);
    if len == 0 {
        js_setlength(J, 0, 0);
        js_pushundefined(J);
        return;
    }
    js_getindex(J, 0, 0);
    k = 1;
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            js_setindex(J, 0, k - 1);
        } else {
            js_delindex(J, 0, k - 1);
        }
        k += 1;
    }
    js_delindex(J, 0, len - 1);
    js_setlength(J, 0, len - 1);
}
unsafe extern "C" fn Ap_slice(J: &mut js_State) {
    let mut len: i32 = 0;
    let mut s: i32 = 0;
    let mut e: i32 = 0;
    let mut n: i32 = 0;
    let mut sv: f64 = 0.;
    let mut ev: f64 = 0.;
    js_newarray(J);
    len = js_getlength(J, 0);
    sv = js_tointeger(J, 1) as f64;
    ev = (if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        len
    }) as f64;
    if sv < 0.0 {
        sv += len as f64;
    }
    if ev < 0.0 {
        ev += len as f64;
    }
    s = (if sv < 0.0 {
        0.0
    } else if sv > len as f64 {
        len as f64
    } else {
        sv
    }) as i32;
    e = (if ev < 0.0 {
        0.0
    } else if ev > len as f64 {
        len as f64
    } else {
        ev
    }) as i32;
    n = 0;
    while s < e {
        if js_hasindex(J, 0, s) != 0 {
            js_setindex(J, -(2 as i32), n);
        }
        s += 1;

        n += 1;
    }
}
unsafe extern "C" fn Ap_sort_cmp(J: &mut js_State, mut idx_a: i32, mut idx_b: i32) -> i32 {
    let mut obj: *mut js_Object = (*js_tovalue(J, 0)).u.object;
    if (*obj).u.a.simple != 0 {
        let mut val_a: *mut js_Value =
            &mut *((*obj).u.a.array).offset(idx_a as isize) as *mut js_Value;
        let mut val_b: *mut js_Value =
            &mut *((*obj).u.a.array).offset(idx_b as isize) as *mut js_Value;
        let mut und_a: i32 = ((*val_a).t.type_0 as i32 == JS_TUNDEFINED as i32) as i32;
        let mut und_b: i32 = ((*val_b).t.type_0 as i32 == JS_TUNDEFINED as i32) as i32;
        if und_a != 0 {
            return und_b;
        }
        if und_b != 0 {
            return -(1);
        }
        if js_iscallable(J, 1) != 0 {
            let mut v: f64 = 0.;
            js_copy(J, 1);
            js_pushundefined(J);
            js_pushvalue(J, *val_a);
            js_pushvalue(J, *val_b);
            js_call(J, 2);
            v = js_tonumber(J, -(1));
            js_pop(J, 1);
            if v.is_nan() as i32 != 0 {
                return 0;
            }
            if v == 0.0 {
                return 0;
            }
            if v < 0.0 {
                -(1)
            } else {
                1
            }
        } else {
            let mut str_a: *const libc::c_char = core::ptr::null::<libc::c_char>();
            let mut str_b: *const libc::c_char = core::ptr::null::<libc::c_char>();
            let mut c: i32 = 0;
            js_pushvalue(J, *val_a);
            js_pushvalue(J, *val_b);
            str_a = js_tostring(J, -(2 as i32));
            str_b = js_tostring(J, -(1));
            c = strcmp(str_a, str_b);
            js_pop(J, 2);
            c
        }
    } else {
        let mut und_a_0: i32 = 0;
        let mut und_b_0: i32 = 0;
        let mut has_a: i32 = js_hasindex(J, 0, idx_a);
        let mut has_b: i32 = js_hasindex(J, 0, idx_b);
        if has_a == 0 && has_b == 0 {
            return 0;
        }
        if has_a != 0 && has_b == 0 {
            js_pop(J, 1);
            return -(1);
        }
        if has_a == 0 && has_b != 0 {
            js_pop(J, 1);
            return 1;
        }
        und_a_0 = js_isundefined(J, -(2 as i32));
        und_b_0 = js_isundefined(J, -(1));
        if und_a_0 != 0 {
            js_pop(J, 2);
            return und_b_0;
        }
        if und_b_0 != 0 {
            js_pop(J, 2);
            return -(1);
        }
        if js_iscallable(J, 1) != 0 {
            let mut v_0: f64 = 0.;
            js_copy(J, 1);
            js_pushundefined(J);
            js_copy(J, -(4 as i32));
            js_copy(J, -(4 as i32));
            js_call(J, 2);
            v_0 = js_tonumber(J, -(1));
            js_pop(J, 3);
            if v_0.is_nan() as i32 != 0 {
                return 0;
            }
            if v_0 == 0.0 {
                return 0;
            }
            if v_0 < 0.0 {
                -(1)
            } else {
                1
            }
        } else {
            let mut str_a_0: *const libc::c_char = js_tostring(J, -(2 as i32));
            let mut str_b_0: *const libc::c_char = js_tostring(J, -(1));
            let mut c_0: i32 = strcmp(str_a_0, str_b_0);
            js_pop(J, 2);
            c_0
        }
    }
}
unsafe extern "C" fn Ap_sort_swap(J: &mut js_State, mut idx_a: i32, mut idx_b: i32) {
    let mut obj: *mut js_Object = (*js_tovalue(J, 0)).u.object;
    if (*obj).u.a.simple != 0 {
        let mut tmp: js_Value = *((*obj).u.a.array).offset(idx_a as isize);
        *((*obj).u.a.array).offset(idx_a as isize) = *((*obj).u.a.array).offset(idx_b as isize);
        *((*obj).u.a.array).offset(idx_b as isize) = tmp;
    } else {
        let mut has_a: i32 = js_hasindex(J, 0, idx_a);
        let mut has_b: i32 = js_hasindex(J, 0, idx_b);
        if has_a != 0 && has_b != 0 {
            js_setindex(J, 0, idx_a);
            js_setindex(J, 0, idx_b);
        } else if has_a != 0 && has_b == 0 {
            js_delindex(J, 0, idx_a);
            js_setindex(J, 0, idx_b);
        } else if has_a == 0 && has_b != 0 {
            js_delindex(J, 0, idx_b);
            js_setindex(J, 0, idx_a);
        }
    };
}
unsafe extern "C" fn Ap_sort_leaf(J: &mut js_State, mut i: i32, mut end: i32) -> i32 {
    let mut j: i32 = i;
    let mut lc: i32 = (j << 1) + 1;
    let mut rc: i32 = (j << 1) + 2;
    while rc < end {
        if Ap_sort_cmp(J, rc, lc) > 0 {
            j = rc;
        } else {
            j = lc;
        }
        lc = (j << 1) + 1;
        rc = (j << 1) + 2;
    }
    if lc < end {
        j = lc;
    }
    j
}
unsafe extern "C" fn Ap_sort_sift(J: &mut js_State, mut i: i32, mut end: i32) {
    let mut j: i32 = Ap_sort_leaf(J, i, end);
    while Ap_sort_cmp(J, i, j) > 0 {
        j = (j - 1) >> 1;
    }
    while j > i {
        Ap_sort_swap(J, i, j);
        j = (j - 1) >> 1;
    }
}
unsafe extern "C" fn Ap_sort_heapsort(J: &mut js_State, mut n: i32) {
    let mut i: i32 = 0;
    i = n / 2 - 1;
    while i >= 0 {
        Ap_sort_sift(J, i, n);
        i -= 1;
    }
    i = n - 1;
    while i > 0 {
        Ap_sort_swap(J, 0, i);
        Ap_sort_sift(J, 0, i);
        i -= 1;
    }
}
unsafe extern "C" fn Ap_sort(J: &mut js_State) {
    let mut len: i32 = 0;
    len = js_getlength(J, 0);
    if len <= 1 {
        js_copy(J, 0);
        return;
    }
    if js_iscallable(J, 1) == 0 && js_isundefined(J, 1) == 0 {
        js_typeerror(
            J,
            b"comparison function must be a function or undefined\0" as *const u8
                as *const libc::c_char,
        );
    }
    if len >= 2147483647 as i32 {
        js_rangeerror(
            J,
            b"array is too large to sort\0" as *const u8 as *const libc::c_char,
        );
    }
    Ap_sort_heapsort(J, len);
    js_copy(J, 0);
}
unsafe extern "C" fn Ap_splice(J: &mut js_State) {
    let mut top: i32 = js_gettop(J);
    let mut len: i32 = 0;
    let mut start: i32 = 0;
    let mut del: i32 = 0;
    let mut add: i32 = 0;
    let mut k: i32 = 0;
    len = js_getlength(J, 0);
    start = js_tointeger(J, 1);
    if start < 0 {
        start = if len + start > 0 { len + start } else { 0 };
    } else if start > len {
        start = len;
    }
    if js_isdefined(J, 2) != 0 {
        del = js_tointeger(J, 2);
    } else {
        del = len - start;
    }
    if del > len - start {
        del = len - start;
    }
    if del < 0 {
        del = 0;
    }
    js_newarray(J);
    k = 0;
    while k < del {
        if js_hasindex(J, 0, start + k) != 0 {
            js_setindex(J, -(2 as i32), k);
        }
        k += 1;
    }
    js_setlength(J, -(1), del);
    add = top - 3;
    if add < del {
        k = start;
        while k < len - del {
            if js_hasindex(J, 0, k + del) != 0 {
                js_setindex(J, 0, k + add);
            } else {
                js_delindex(J, 0, k + add);
            }
            k += 1;
        }
        k = len;
        while k > len - del + add {
            js_delindex(J, 0, k - 1);
            k -= 1;
        }
    } else if add > del {
        k = len - del;
        while k > start {
            if js_hasindex(J, 0, k + del - 1) != 0 {
                js_setindex(J, 0, k + add - 1);
            } else {
                js_delindex(J, 0, k + add - 1);
            }
            k -= 1;
        }
    }
    k = 0;
    while k < add {
        js_copy(J, 3 + k);
        js_setindex(J, 0, start + k);
        k += 1;
    }
    js_setlength(J, 0, len - del + add);
}
unsafe extern "C" fn Ap_unshift(J: &mut js_State) {
    let mut i: i32 = 0;
    let mut top: i32 = js_gettop(J);
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    len = js_getlength(J, 0);
    k = len;
    while k > 0 {
        let mut from: i32 = k - 1;
        let mut to: i32 = k + top - 2;
        if js_hasindex(J, 0, from) != 0 {
            js_setindex(J, 0, to);
        } else {
            js_delindex(J, 0, to);
        }
        k -= 1;
    }
    i = 1;
    while i < top {
        js_copy(J, i);
        js_setindex(J, 0, i - 1);
        i += 1;
    }
    js_setlength(J, 0, len + top - 1);
    js_pushnumber(J, (len + top - 1) as f64);
}
unsafe extern "C" fn Ap_toString(J: &mut js_State) {
    if js_iscoercible(J, 0) == 0 {
        js_typeerror(
            J,
            b"'this' is not an object\0" as *const u8 as *const libc::c_char,
        );
    }
    js_getproperty(J, 0, b"join\0" as *const u8 as *const libc::c_char);
    if js_iscallable(J, -(1)) == 0 {
        js_pop(J, 1);
        js_getglobal(J, b"Object\0" as *const u8 as *const libc::c_char);
        js_getproperty(J, -(1), b"prototype\0" as *const u8 as *const libc::c_char);
        js_rot2pop1(J);
        js_getproperty(J, -(1), b"toString\0" as *const u8 as *const libc::c_char);
        js_rot2pop1(J);
    }
    js_copy(J, 0);
    js_call(J, 0);
}
unsafe extern "C" fn Ap_indexOf(J: &mut js_State) {
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut from: i32 = 0;
    len = js_getlength(J, 0);
    from = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        0
    };
    if from < 0 {
        from += len;
    }
    if from < 0 {
        from = 0;
    }
    js_copy(J, 1);
    k = from;
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            if js_strictequal(J) != 0 {
                js_pushnumber(J, k as f64);
                return;
            }
            js_pop(J, 1);
        }
        k += 1;
    }
    js_pushnumber(J, -(1) as f64);
}
unsafe extern "C" fn Ap_lastIndexOf(J: &mut js_State) {
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut from: i32 = 0;
    len = js_getlength(J, 0);
    from = if js_isdefined(J, 2) != 0 {
        js_tointeger(J, 2)
    } else {
        len - 1
    };
    if from > len - 1 {
        from = len - 1;
    }
    if from < 0 {
        from += len;
    }
    js_copy(J, 1);
    k = from;
    while k >= 0 {
        if js_hasindex(J, 0, k) != 0 {
            if js_strictequal(J) != 0 {
                js_pushnumber(J, k as f64);
                return;
            }
            js_pop(J, 1);
        }
        k -= 1;
    }
    js_pushnumber(J, -(1) as f64);
}
unsafe extern "C" fn Ap_every(J: &mut js_State) {
    let mut hasthis: i32 = (js_gettop(J) >= 3) as i32;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    if js_iscallable(J, 1) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0);
    k = 0;
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            js_copy(J, 1);
            if hasthis != 0 {
                js_copy(J, 2);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as i32));
            js_pushnumber(J, k as f64);
            js_copy(J, 0);
            js_call(J, 3);
            if js_toboolean(J, -(1)) == 0 {
                return;
            }
            js_pop(J, 2);
        }
        k += 1;
    }
    js_pushboolean(J, 1);
}
unsafe extern "C" fn Ap_some(J: &mut js_State) {
    let mut hasthis: i32 = (js_gettop(J) >= 3) as i32;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    if js_iscallable(J, 1) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0);
    k = 0;
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            js_copy(J, 1);
            if hasthis != 0 {
                js_copy(J, 2);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as i32));
            js_pushnumber(J, k as f64);
            js_copy(J, 0);
            js_call(J, 3);
            if js_toboolean(J, -(1)) != 0 {
                return;
            }
            js_pop(J, 2);
        }
        k += 1;
    }
    js_pushboolean(J, 0);
}
unsafe extern "C" fn Ap_forEach(J: &mut js_State) {
    let mut hasthis: i32 = (js_gettop(J) >= 3) as i32;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    if js_iscallable(J, 1) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0);
    k = 0;
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            js_copy(J, 1);
            if hasthis != 0 {
                js_copy(J, 2);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as i32));
            js_pushnumber(J, k as f64);
            js_copy(J, 0);
            js_call(J, 3);
            js_pop(J, 2);
        }
        k += 1;
    }
    js_pushundefined(J);
}
unsafe extern "C" fn Ap_map(J: &mut js_State) {
    let mut hasthis: i32 = (js_gettop(J) >= 3) as i32;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    if js_iscallable(J, 1) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    js_newarray(J);
    len = js_getlength(J, 0);
    k = 0;
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            js_copy(J, 1);
            if hasthis != 0 {
                js_copy(J, 2);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as i32));
            js_pushnumber(J, k as f64);
            js_copy(J, 0);
            js_call(J, 3);
            js_setindex(J, -(3 as i32), k);
            js_pop(J, 1);
        }
        k += 1;
    }
    js_setlength(J, -(1), len);
}
unsafe extern "C" fn Ap_filter(J: &mut js_State) {
    let mut hasthis: i32 = (js_gettop(J) >= 3) as i32;
    let mut k: i32 = 0;
    let mut to: i32 = 0;
    let mut len: i32 = 0;
    if js_iscallable(J, 1) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    js_newarray(J);
    to = 0;
    len = js_getlength(J, 0);
    k = 0;
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            js_copy(J, 1);
            if hasthis != 0 {
                js_copy(J, 2);
            } else {
                js_pushundefined(J);
            }
            js_copy(J, -(3 as i32));
            js_pushnumber(J, k as f64);
            js_copy(J, 0);
            js_call(J, 3);
            if js_toboolean(J, -(1)) != 0 {
                js_pop(J, 1);
                let fresh2 = to;
                to += 1;
                js_setindex(J, -(2 as i32), fresh2);
            } else {
                js_pop(J, 2);
            }
        }
        k += 1;
    }
}
unsafe extern "C" fn Ap_reduce(J: &mut js_State) {
    let mut hasinitial: i32 = (js_gettop(J) >= 3) as i32;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    if js_iscallable(J, 1) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0);
    k = 0;
    if len == 0 && hasinitial == 0 {
        js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
    }
    if hasinitial != 0 {
        js_copy(J, 2);
    } else {
        while k < len {
            let fresh3 = k;
            k += 1;
            if js_hasindex(J, 0, fresh3) != 0 {
                break;
            }
        }
        if k == len {
            js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
        }
    }
    while k < len {
        if js_hasindex(J, 0, k) != 0 {
            js_copy(J, 1);
            js_pushundefined(J);
            js_rot(J, 4);
            js_rot(J, 4);
            js_pushnumber(J, k as f64);
            js_copy(J, 0);
            js_call(J, 4);
        }
        k += 1;
    }
}
unsafe extern "C" fn Ap_reduceRight(J: &mut js_State) {
    let mut hasinitial: i32 = (js_gettop(J) >= 3) as i32;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    if js_iscallable(J, 1) == 0 {
        js_typeerror(
            J,
            b"callback is not a function\0" as *const u8 as *const libc::c_char,
        );
    }
    len = js_getlength(J, 0);
    k = len - 1;
    if len == 0 && hasinitial == 0 {
        js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
    }
    if hasinitial != 0 {
        js_copy(J, 2);
    } else {
        while k >= 0 {
            let fresh4 = k;
            k -= 1;
            if js_hasindex(J, 0, fresh4) != 0 {
                break;
            }
        }
        if k < 0 {
            js_typeerror(J, b"no initial value\0" as *const u8 as *const libc::c_char);
        }
    }
    while k >= 0 {
        if js_hasindex(J, 0, k) != 0 {
            js_copy(J, 1);
            js_pushundefined(J);
            js_rot(J, 4);
            js_rot(J, 4);
            js_pushnumber(J, k as f64);
            js_copy(J, 0);
            js_call(J, 4);
        }
        k -= 1;
    }
}
unsafe extern "C" fn A_isArray(J: &mut js_State) {
    if js_isobject(J, 1) != 0 {
        let mut T: *mut js_Object = js_toobject(J, 1);
        js_pushboolean(
            J,
            ((*T).type_0 as i32 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint) as i32,
        );
    } else {
        js_pushboolean(J, 0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initarray(J: &mut js_State) {
    js_pushobject(J, (*J).Array_prototype);
    jsB_propf(
        J,
        b"Array.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Ap_toString),
        0,
    );
    jsB_propf(
        J,
        b"Array.prototype.concat\0" as *const u8 as *const libc::c_char,
        Some(Ap_concat),
        0,
    );
    jsB_propf(
        J,
        b"Array.prototype.join\0" as *const u8 as *const libc::c_char,
        Some(Ap_join),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.pop\0" as *const u8 as *const libc::c_char,
        Some(Ap_pop),
        0,
    );
    jsB_propf(
        J,
        b"Array.prototype.push\0" as *const u8 as *const libc::c_char,
        Some(Ap_push),
        0,
    );
    jsB_propf(
        J,
        b"Array.prototype.reverse\0" as *const u8 as *const libc::c_char,
        Some(Ap_reverse),
        0,
    );
    jsB_propf(
        J,
        b"Array.prototype.shift\0" as *const u8 as *const libc::c_char,
        Some(Ap_shift),
        0,
    );
    jsB_propf(
        J,
        b"Array.prototype.slice\0" as *const u8 as *const libc::c_char,
        Some(Ap_slice),
        2,
    );
    jsB_propf(
        J,
        b"Array.prototype.sort\0" as *const u8 as *const libc::c_char,
        Some(Ap_sort),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.splice\0" as *const u8 as *const libc::c_char,
        Some(Ap_splice),
        2,
    );
    jsB_propf(
        J,
        b"Array.prototype.unshift\0" as *const u8 as *const libc::c_char,
        Some(Ap_unshift),
        0,
    );
    jsB_propf(
        J,
        b"Array.prototype.indexOf\0" as *const u8 as *const libc::c_char,
        Some(Ap_indexOf),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.lastIndexOf\0" as *const u8 as *const libc::c_char,
        Some(Ap_lastIndexOf),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.every\0" as *const u8 as *const libc::c_char,
        Some(Ap_every),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.some\0" as *const u8 as *const libc::c_char,
        Some(Ap_some),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.forEach\0" as *const u8 as *const libc::c_char,
        Some(Ap_forEach),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.map\0" as *const u8 as *const libc::c_char,
        Some(Ap_map),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.filter\0" as *const u8 as *const libc::c_char,
        Some(Ap_filter),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.reduce\0" as *const u8 as *const libc::c_char,
        Some(Ap_reduce),
        1,
    );
    jsB_propf(
        J,
        b"Array.prototype.reduceRight\0" as *const u8 as *const libc::c_char,
        Some(Ap_reduceRight),
        1,
    );
    js_newcconstructor(
        J,
        Some(jsB_new_Array),
        Some(jsB_new_Array),
        b"Array\0" as *const u8 as *const libc::c_char,
        0,
    );
    jsB_propf(
        J,
        b"Array.isArray\0" as *const u8 as *const libc::c_char,
        Some(A_isArray),
        1,
    );
    js_defglobal(
        J,
        b"Array\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
