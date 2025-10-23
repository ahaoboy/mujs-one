use crate::*;

unsafe extern "C" fn jsB_new_Object(J: &mut js_State) {
    if js_isundefined(J, 1) != 0 || js_isnull(J, 1) != 0 {
        js_newobject(J);
    } else {
        let v = js_toobject(J, 1);
        js_pushobject(J, v);
    };
}
unsafe extern "C" fn jsB_Object(J: &mut js_State) {
    if js_isundefined(J, 1) != 0 || js_isnull(J, 1) != 0 {
        js_newobject(J);
    } else {
        let v = js_toobject(J, 1);
        js_pushobject(J, v);
    };
}
unsafe extern "C" fn Op_toString(J: &mut js_State) {
    if js_isundefined(J, 0) != 0 {
        js_pushliteral(
            J,
            b"[object Undefined]\0" as *const u8 as *const libc::c_char,
        );
    } else if js_isnull(J, 0) != 0 {
        js_pushliteral(J, b"[object Null]\0" as *const u8 as *const libc::c_char);
    } else {
        let mut self_0: *mut js_Object = js_toobject(J, 0);
        match (*self_0).type_0 as i32 as libc::c_uint {
            0 => {
                js_pushliteral(J, b"[object Object]\0" as *const u8 as *const libc::c_char);
            }
            1 => {
                js_pushliteral(J, b"[object Array]\0" as *const u8 as *const libc::c_char);
            }
            2 => {
                js_pushliteral(
                    J,
                    b"[object Function]\0" as *const u8 as *const libc::c_char,
                );
            }
            3 => {
                js_pushliteral(
                    J,
                    b"[object Function]\0" as *const u8 as *const libc::c_char,
                );
            }
            4 => {
                js_pushliteral(
                    J,
                    b"[object Function]\0" as *const u8 as *const libc::c_char,
                );
            }
            5 => {
                js_pushliteral(J, b"[object Error]\0" as *const u8 as *const libc::c_char);
            }
            6 => {
                js_pushliteral(J, b"[object Boolean]\0" as *const u8 as *const libc::c_char);
            }
            7 => {
                js_pushliteral(J, b"[object Number]\0" as *const u8 as *const libc::c_char);
            }
            8 => {
                js_pushliteral(J, b"[object String]\0" as *const u8 as *const libc::c_char);
            }
            9 => {
                js_pushliteral(J, b"[object RegExp]\0" as *const u8 as *const libc::c_char);
            }
            10 => {
                js_pushliteral(J, b"[object Date]\0" as *const u8 as *const libc::c_char);
            }
            11 => {
                js_pushliteral(J, b"[object Math]\0" as *const u8 as *const libc::c_char);
            }
            12 => {
                js_pushliteral(J, b"[object JSON]\0" as *const u8 as *const libc::c_char);
            }
            13 => {
                js_pushliteral(
                    J,
                    b"[object Arguments]\0" as *const u8 as *const libc::c_char,
                );
            }
            14 => {
                js_pushliteral(
                    J,
                    b"[object Iterator]\0" as *const u8 as *const libc::c_char,
                );
            }
            15 => {
                js_pushliteral(J, b"[object \0" as *const u8 as *const libc::c_char);
                js_pushliteral(J, (*self_0).u.user.tag);
                js_concat(J);
                js_pushliteral(J, b"]\0" as *const u8 as *const libc::c_char);
                js_concat(J);
            }
            _ => {}
        }
    };
}
unsafe extern "C" fn Op_valueOf(J: &mut js_State) {
    js_copy(J, 0);
}
unsafe extern "C" fn Op_hasOwnProperty(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    let mut name: *const libc::c_char = js_tostring(J, 1);
    let mut ref_0: *mut js_Property = core::ptr::null_mut::<js_Property>();
    let mut k: i32 = 0;
    if (*self_0).type_0 as i32 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint
        && js_isarrayindex(J, name, &mut k) != 0
        && k >= 0
        && k < (*self_0).u.s.length
    {
        js_pushboolean(J, 1);
        return;
    }
    if (*self_0).type_0 as i32 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*self_0).u.a.simple != 0
        && js_isarrayindex(J, name, &mut k) != 0
        && k >= 0
        && k < (*self_0).u.a.flat_length
    {
        js_pushboolean(J, 1);
        return;
    }
    ref_0 = jsV_getownproperty(J, self_0, name);
    js_pushboolean(
        J,
        (ref_0 != core::ptr::null_mut::<libc::c_void>() as *mut js_Property) as i32,
    );
}
unsafe extern "C" fn Op_isPrototypeOf(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    if js_isobject(J, 1) != 0 {
        let mut V: *mut js_Object = js_toobject(J, 1);
        loop {
            V = (*V).prototype;
            if V == self_0 {
                js_pushboolean(J, 1);
                return;
            }
            if V.is_null() {
                break;
            }
        }
    }
    js_pushboolean(J, 0);
}
unsafe extern "C" fn Op_propertyIsEnumerable(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    let mut name: *const libc::c_char = js_tostring(J, 1);
    let mut ref_0: *mut js_Property = jsV_getownproperty(J, self_0, name);
    js_pushboolean(
        J,
        (!ref_0.is_null() && (*ref_0).atts & JS_DONTENUM as i32 == 0) as i32,
    );
}
unsafe extern "C" fn O_getPrototypeOf(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    if !((*obj).prototype).is_null() {
        js_pushobject(J, (*obj).prototype);
    } else {
        js_pushnull(J);
    };
}
unsafe extern "C" fn O_getOwnPropertyDescriptor(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut ref_0: *mut js_Property = core::ptr::null_mut::<js_Property>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    let name = js_tostring(J, 2);
    ref_0 = jsV_getproperty(J, obj, name);
    if ref_0.is_null() {
        js_pushundefined(J);
    } else {
        js_newobject(J);
        if ((*ref_0).getter).is_null() && ((*ref_0).setter).is_null() {
            js_pushvalue(J, (*ref_0).value);
            js_defproperty(
                J,
                -(2 as i32),
                b"value\0" as *const u8 as *const libc::c_char,
                0,
            );
            js_pushboolean(J, ((*ref_0).atts & JS_READONLY as i32 == 0) as i32);
            js_defproperty(
                J,
                -(2 as i32),
                b"writable\0" as *const u8 as *const libc::c_char,
                0,
            );
        } else {
            if !((*ref_0).getter).is_null() {
                js_pushobject(J, (*ref_0).getter);
            } else {
                js_pushundefined(J);
            }
            js_defproperty(
                J,
                -(2 as i32),
                b"get\0" as *const u8 as *const libc::c_char,
                0,
            );
            if !((*ref_0).setter).is_null() {
                js_pushobject(J, (*ref_0).setter);
            } else {
                js_pushundefined(J);
            }
            js_defproperty(
                J,
                -(2 as i32),
                b"set\0" as *const u8 as *const libc::c_char,
                0,
            );
        }
        js_pushboolean(J, ((*ref_0).atts & JS_DONTENUM as i32 == 0) as i32);
        js_defproperty(
            J,
            -(2 as i32),
            b"enumerable\0" as *const u8 as *const libc::c_char,
            0,
        );
        js_pushboolean(J, ((*ref_0).atts & JS_DONTCONF as i32 == 0) as i32);
        js_defproperty(
            J,
            -(2 as i32),
            b"configurable\0" as *const u8 as *const libc::c_char,
            0,
        );
    };
}
unsafe extern "C" fn O_getOwnPropertyNames_walk(
    J: &mut js_State,
    mut ref_0: *mut js_Property,
    mut i: i32,
) -> i32 {
    if (*(*ref_0).left).level != 0 {
        i = O_getOwnPropertyNames_walk(J, (*ref_0).left, i);
    }
    js_pushstring(J, ((*ref_0).name).as_mut_ptr());
    let fresh35 = i;
    i += 1;
    js_setindex(J, -(2 as i32), fresh35);
    if (*(*ref_0).right).level != 0 {
        i = O_getOwnPropertyNames_walk(J, (*ref_0).right, i);
    }
    i
}
unsafe extern "C" fn O_getOwnPropertyNames(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut k: i32 = 0;
    let mut i: i32 = 0;
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    js_newarray(J);
    if (*(*obj).properties).level != 0 {
        i = O_getOwnPropertyNames_walk(J, (*obj).properties, 0);
    } else {
        i = 0;
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint {
        js_pushliteral(J, b"length\0" as *const u8 as *const libc::c_char);
        let fresh36 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh36);
        if (*obj).u.a.simple != 0 {
            k = 0;
            while k < (*obj).u.a.flat_length {
                js_itoa(name.as_mut_ptr(), k);
                js_pushstring(J, name.as_mut_ptr());
                let fresh37 = i;
                i += 1;
                js_setindex(J, -(2 as i32), fresh37);
                k += 1;
            }
        }
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint {
        js_pushliteral(J, b"length\0" as *const u8 as *const libc::c_char);
        let fresh38 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh38);
        k = 0;
        while k < (*obj).u.s.length {
            js_itoa(name.as_mut_ptr(), k);
            js_pushstring(J, name.as_mut_ptr());
            let fresh39 = i;
            i += 1;
            js_setindex(J, -(2 as i32), fresh39);
            k += 1;
        }
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint {
        js_pushliteral(J, b"source\0" as *const u8 as *const libc::c_char);
        let fresh40 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh40);
        js_pushliteral(J, b"global\0" as *const u8 as *const libc::c_char);
        let fresh41 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh41);
        js_pushliteral(J, b"ignoreCase\0" as *const u8 as *const libc::c_char);
        let fresh42 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh42);
        js_pushliteral(J, b"multiline\0" as *const u8 as *const libc::c_char);
        let fresh43 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh43);
        js_pushliteral(J, b"lastIndex\0" as *const u8 as *const libc::c_char);
        let fresh44 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh44);
    }
}
unsafe extern "C" fn ToPropertyDescriptor(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
    mut desc: *mut js_Object,
) {
    let mut haswritable: i32 = 0;
    let mut hasvalue: i32 = 0;
    let mut enumerable: i32 = 0;
    let mut configurable: i32 = 0;
    let mut writable: i32 = 0;
    let mut atts: i32 = 0;
    js_pushobject(J, obj);
    js_pushobject(J, desc);
    if js_hasproperty(
        J,
        -(1 as i32),
        b"writable\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        haswritable = 1;
        writable = js_toboolean(J, -(1 as i32));
        js_pop(J, 1);
    }
    if js_hasproperty(
        J,
        -(1 as i32),
        b"enumerable\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        enumerable = js_toboolean(J, -(1 as i32));
        js_pop(J, 1);
    }
    if js_hasproperty(
        J,
        -(1 as i32),
        b"configurable\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        configurable = js_toboolean(J, -(1 as i32));
        js_pop(J, 1);
    }
    if js_hasproperty(
        J,
        -(1 as i32),
        b"value\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        hasvalue = 1;
        js_defproperty(J, -(3 as i32), name, 0);
    }
    if writable == 0 {
        atts |= JS_READONLY as i32;
    }
    if enumerable == 0 {
        atts |= JS_DONTENUM as i32;
    }
    if configurable == 0 {
        atts |= JS_DONTCONF as i32;
    }
    if js_hasproperty(J, -(1 as i32), b"get\0" as *const u8 as *const libc::c_char) != 0 {
        if haswritable != 0 || hasvalue != 0 {
            js_typeerror(
                J,
                b"value/writable and get/set attributes are exclusive\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        js_pushundefined(J);
    }
    if js_hasproperty(J, -(2 as i32), b"set\0" as *const u8 as *const libc::c_char) != 0 {
        if haswritable != 0 || hasvalue != 0 {
            js_typeerror(
                J,
                b"value/writable and get/set attributes are exclusive\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        js_pushundefined(J);
    }
    js_defaccessor(J, -(4 as i32), name, atts);
    js_pop(J, 2);
}
unsafe extern "C" fn O_defineProperty(J: &mut js_State) {
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    if js_isobject(J, 3) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    let obj = js_toobject(J, 1);
    let name = js_tostring(J, 2);
    let desc = js_toobject(J, 3);
    ToPropertyDescriptor(J, obj, name, desc);
    js_copy(J, 1);
}
unsafe extern "C" fn O_defineProperties_walk(J: &mut js_State, mut ref_0: *mut js_Property) {
    if (*(*ref_0).left).level != 0 {
        O_defineProperties_walk(J, (*ref_0).left);
    }
    if (*ref_0).atts & JS_DONTENUM as i32 == 0 {
        js_pushvalue(J, (*ref_0).value);
        let obj = js_toobject(J, 1);
        let desc: *mut js_Object = js_toobject(J, -(1 as i32));
        ToPropertyDescriptor(J, obj, ((*ref_0).name).as_mut_ptr(), desc);
        js_pop(J, 1);
    }
    if (*(*ref_0).right).level != 0 {
        O_defineProperties_walk(J, (*ref_0).right);
    }
}
unsafe extern "C" fn O_defineProperties(J: &mut js_State) {
    let mut props: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    if js_isobject(J, 2) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    props = js_toobject(J, 2);
    if (*(*props).properties).level != 0 {
        O_defineProperties_walk(J, (*props).properties);
    }
    js_copy(J, 1);
}
unsafe extern "C" fn O_create_walk(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut ref_0: *mut js_Property,
) {
    if (*(*ref_0).left).level != 0 {
        O_create_walk(J, obj, (*ref_0).left);
    }
    if (*ref_0).atts & JS_DONTENUM as i32 == 0 {
        if (*ref_0).value.t.type_0 as i32 != JS_TOBJECT as i32 {
            js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
        }
        ToPropertyDescriptor(
            J,
            obj,
            ((*ref_0).name).as_mut_ptr(),
            (*ref_0).value.u.object,
        );
    }
    if (*(*ref_0).right).level != 0 {
        O_create_walk(J, obj, (*ref_0).right);
    }
}
unsafe extern "C" fn O_create(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut proto: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut props: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) != 0 {
        proto = js_toobject(J, 1);
    } else if js_isnull(J, 1) != 0 {
        proto = core::ptr::null_mut::<js_Object>();
    } else {
        js_typeerror(
            J,
            b"not an object or null\0" as *const u8 as *const libc::c_char,
        );
    }
    obj = jsV_newobject(J, JS_COBJECT, proto);
    js_pushobject(J, obj);
    if js_isdefined(J, 2) != 0 {
        if js_isobject(J, 2) == 0 {
            js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
        }
        props = js_toobject(J, 2);
        if (*(*props).properties).level != 0 {
            O_create_walk(J, obj, (*props).properties);
        }
    }
}
unsafe extern "C" fn O_keys_walk(J: &mut js_State, mut ref_0: *mut js_Property, mut i: i32) -> i32 {
    if (*(*ref_0).left).level != 0 {
        i = O_keys_walk(J, (*ref_0).left, i);
    }
    if (*ref_0).atts & JS_DONTENUM as i32 == 0 {
        js_pushstring(J, ((*ref_0).name).as_mut_ptr());
        let fresh45 = i;
        i += 1;
        js_setindex(J, -(2 as i32), fresh45);
    }
    if (*(*ref_0).right).level != 0 {
        i = O_keys_walk(J, (*ref_0).right, i);
    }
    i
}
unsafe extern "C" fn O_keys(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    js_newarray(J);
    if (*(*obj).properties).level != 0 {
        i = O_keys_walk(J, (*obj).properties, 0);
    } else {
        i = 0;
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint {
        k = 0;
        while k < (*obj).u.s.length {
            js_itoa(name.as_mut_ptr(), k);
            js_pushstring(J, name.as_mut_ptr());
            let fresh46 = i;
            i += 1;
            js_setindex(J, -(2 as i32), fresh46);
            k += 1;
        }
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        k = 0;
        while k < (*obj).u.a.flat_length {
            js_itoa(name.as_mut_ptr(), k);
            js_pushstring(J, name.as_mut_ptr());
            let fresh47 = i;
            i += 1;
            js_setindex(J, -(2 as i32), fresh47);
            k += 1;
        }
    }
}
unsafe extern "C" fn O_preventExtensions(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    jsR_unflattenarray(J, obj);
    (*obj).extensible = 0;
    js_copy(J, 1);
}
unsafe extern "C" fn O_isExtensible(J: &mut js_State) {
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    let v = (*js_toobject(J, 1)).extensible;
    js_pushboolean(J, v);
}
unsafe extern "C" fn O_seal_walk(J: &mut js_State, mut ref_0: *mut js_Property) {
    if (*(*ref_0).left).level != 0 {
        O_seal_walk(J, (*ref_0).left);
    }
    (*ref_0).atts |= JS_DONTCONF as i32;
    if (*(*ref_0).right).level != 0 {
        O_seal_walk(J, (*ref_0).right);
    }
}
unsafe extern "C" fn O_seal(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    jsR_unflattenarray(J, obj);
    (*obj).extensible = 0;
    if (*(*obj).properties).level != 0 {
        O_seal_walk(J, (*obj).properties);
    }
    js_copy(J, 1);
}
unsafe extern "C" fn O_isSealed_walk(J: &mut js_State, mut ref_0: *mut js_Property) -> i32 {
    if (*(*ref_0).left).level != 0 && O_isSealed_walk(J, (*ref_0).left) == 0 {
        return 0;
    }
    if (*ref_0).atts & JS_DONTCONF as i32 == 0 {
        return 0;
    }
    if (*(*ref_0).right).level != 0 && O_isSealed_walk(J, (*ref_0).right) == 0 {
        return 0;
    }
    1
}
unsafe extern "C" fn O_isSealed(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    if (*obj).extensible != 0 {
        js_pushboolean(J, 0);
        return;
    }
    if (*(*obj).properties).level != 0 {
        let v = O_isSealed_walk(J, (*obj).properties);
        js_pushboolean(J, v);
    } else {
        js_pushboolean(J, 1);
    };
}
unsafe extern "C" fn O_freeze_walk(J: &mut js_State, mut ref_0: *mut js_Property) {
    if (*(*ref_0).left).level != 0 {
        O_freeze_walk(J, (*ref_0).left);
    }
    (*ref_0).atts |= JS_READONLY as i32 | JS_DONTCONF as i32;
    if (*(*ref_0).right).level != 0 {
        O_freeze_walk(J, (*ref_0).right);
    }
}
unsafe extern "C" fn O_freeze(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    jsR_unflattenarray(J, obj);
    (*obj).extensible = 0;
    if (*(*obj).properties).level != 0 {
        O_freeze_walk(J, (*obj).properties);
    }
    js_copy(J, 1);
}
unsafe extern "C" fn O_isFrozen_walk(J: &mut js_State, mut ref_0: *mut js_Property) -> i32 {
    if (*(*ref_0).left).level != 0 && O_isFrozen_walk(J, (*ref_0).left) == 0 {
        return 0;
    }
    if (*ref_0).atts & JS_READONLY as i32 == 0 {
        return 0;
    }
    if (*ref_0).atts & JS_DONTCONF as i32 == 0 {
        return 0;
    }
    if (*(*ref_0).right).level != 0 && O_isFrozen_walk(J, (*ref_0).right) == 0 {
        return 0;
    }
    1
}
unsafe extern "C" fn O_isFrozen(J: &mut js_State) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_isobject(J, 1) == 0 {
        js_typeerror(J, b"not an object\0" as *const u8 as *const libc::c_char);
    }
    obj = js_toobject(J, 1);
    if (*(*obj).properties).level != 0 && O_isFrozen_walk(J, (*obj).properties) == 0 {
        js_pushboolean(J, 0);
        return;
    }
    js_pushboolean(J, ((*obj).extensible == 0) as i32);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initobject(J: &mut js_State) {
    js_pushobject(J, (*J).Object_prototype);
    jsB_propf(
        J,
        b"Object.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Op_toString),
        0,
    );
    jsB_propf(
        J,
        b"Object.prototype.toLocaleString\0" as *const u8 as *const libc::c_char,
        Some(Op_toString),
        0,
    );
    jsB_propf(
        J,
        b"Object.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Op_valueOf),
        0,
    );
    jsB_propf(
        J,
        b"Object.prototype.hasOwnProperty\0" as *const u8 as *const libc::c_char,
        Some(Op_hasOwnProperty),
        1,
    );
    jsB_propf(
        J,
        b"Object.prototype.isPrototypeOf\0" as *const u8 as *const libc::c_char,
        Some(Op_isPrototypeOf),
        1,
    );
    jsB_propf(
        J,
        b"Object.prototype.propertyIsEnumerable\0" as *const u8 as *const libc::c_char,
        Some(Op_propertyIsEnumerable),
        1,
    );
    js_newcconstructor(
        J,
        Some(jsB_Object),
        Some(jsB_new_Object),
        b"Object\0" as *const u8 as *const libc::c_char,
        1,
    );
    jsB_propf(
        J,
        b"Object.getPrototypeOf\0" as *const u8 as *const libc::c_char,
        Some(O_getPrototypeOf),
        1,
    );
    jsB_propf(
        J,
        b"Object.getOwnPropertyDescriptor\0" as *const u8 as *const libc::c_char,
        Some(O_getOwnPropertyDescriptor),
        2,
    );
    jsB_propf(
        J,
        b"Object.getOwnPropertyNames\0" as *const u8 as *const libc::c_char,
        Some(O_getOwnPropertyNames),
        1,
    );
    jsB_propf(
        J,
        b"Object.create\0" as *const u8 as *const libc::c_char,
        Some(O_create),
        2,
    );
    jsB_propf(
        J,
        b"Object.defineProperty\0" as *const u8 as *const libc::c_char,
        Some(O_defineProperty),
        3,
    );
    jsB_propf(
        J,
        b"Object.defineProperties\0" as *const u8 as *const libc::c_char,
        Some(O_defineProperties),
        2,
    );
    jsB_propf(
        J,
        b"Object.seal\0" as *const u8 as *const libc::c_char,
        Some(O_seal),
        1,
    );
    jsB_propf(
        J,
        b"Object.freeze\0" as *const u8 as *const libc::c_char,
        Some(O_freeze),
        1,
    );
    jsB_propf(
        J,
        b"Object.preventExtensions\0" as *const u8 as *const libc::c_char,
        Some(O_preventExtensions),
        1,
    );
    jsB_propf(
        J,
        b"Object.isSealed\0" as *const u8 as *const libc::c_char,
        Some(O_isSealed),
        1,
    );
    jsB_propf(
        J,
        b"Object.isFrozen\0" as *const u8 as *const libc::c_char,
        Some(O_isFrozen),
        1,
    );
    jsB_propf(
        J,
        b"Object.isExtensible\0" as *const u8 as *const libc::c_char,
        Some(O_isExtensible),
        1,
    );
    jsB_propf(
        J,
        b"Object.keys\0" as *const u8 as *const libc::c_char,
        Some(O_keys),
        1,
    );
    js_defglobal(
        J,
        b"Object\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
