use crate::*;

unsafe extern "C" fn jsB_new_Boolean(J: &mut js_State) {
    let b = js_toboolean(J, 1);
    js_newboolean(J, b);
}
unsafe extern "C" fn jsB_Boolean(J: &mut js_State) {
    let b = js_toboolean(J, 1);
    js_pushboolean(J, b);
}
unsafe extern "C" fn Bp_toString(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    if (*self_0).type_0 as libc::c_uint != JS_CBOOLEAN as i32 as libc::c_uint {
        js_typeerror(J, b"not a boolean\0" as *const u8 as *const libc::c_char);
    }
    js_pushliteral(
        J,
        if (*self_0).u.boolean != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn Bp_valueOf(J: &mut js_State) {
    let mut self_0: *mut js_Object = js_toobject(J, 0);
    if (*self_0).type_0 as libc::c_uint != JS_CBOOLEAN as i32 as libc::c_uint {
        js_typeerror(J, b"not a boolean\0" as *const u8 as *const libc::c_char);
    }
    js_pushboolean(J, (*self_0).u.boolean);
}
#[no_mangle]
pub unsafe extern "C" fn jsB_initboolean(J: &mut js_State) {
    (*(*J).Boolean_prototype).u.boolean = 0;
    js_pushobject(J, (*J).Boolean_prototype);
    jsB_propf(
        J,
        b"Boolean.prototype.toString\0" as *const u8 as *const libc::c_char,
        Some(Bp_toString),
        0,
    );
    jsB_propf(
        J,
        b"Boolean.prototype.valueOf\0" as *const u8 as *const libc::c_char,
        Some(Bp_valueOf),
        0,
    );
    js_newcconstructor(
        J,
        Some(jsB_Boolean),
        Some(jsB_new_Boolean),
        b"Boolean\0" as *const u8 as *const libc::c_char,
        1,
    );
    js_defglobal(
        J,
        b"Boolean\0" as *const u8 as *const libc::c_char,
        JS_DONTENUM as i32,
    );
}
