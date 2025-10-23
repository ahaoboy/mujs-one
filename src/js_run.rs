use crate::*;

unsafe extern "C" fn js_trystackoverflow(J: &mut js_State) {
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as i32 as libc::c_char;
    let fresh53 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh53 = b"exception stack overflow\0" as *const u8 as *const libc::c_char;
    (*J).top += 1;
    (*J).top;
    js_throw(J);
}
unsafe extern "C" fn js_stackoverflow(J: &mut js_State) {
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as i32 as libc::c_char;
    let fresh54 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh54 = b"stack overflow\0" as *const u8 as *const libc::c_char;
    (*J).top += 1;
    (*J).top;
    js_throw(J);
}
unsafe extern "C" fn js_outofmemory(J: &mut js_State) {
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as i32 as libc::c_char;
    let fresh55 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh55 = b"out of memory\0" as *const u8 as *const libc::c_char;
    (*J).top += 1;
    (*J).top;
    js_throw(J);
}
#[no_mangle]
pub unsafe extern "C" fn js_malloc(J: &mut js_State, mut size: i32) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ((*J).alloc).expect("non-null function pointer")(
        (*J).actx,
        core::ptr::null_mut::<libc::c_void>(),
        size,
    );
    if ptr.is_null() {
        js_outofmemory(J);
    }
    ptr
}
#[no_mangle]
pub unsafe extern "C" fn js_realloc(
    J: &mut js_State,
    mut ptr: *mut libc::c_void,
    mut size: i32,
) -> *mut libc::c_void {
    ptr = ((*J).alloc).expect("non-null function pointer")((*J).actx, ptr, size);
    if ptr.is_null() {
        js_outofmemory(J);
    }
    ptr
}
#[no_mangle]
pub unsafe extern "C" fn js_strdup(
    J: &mut js_State,
    mut s: *const libc::c_char,
) -> *mut libc::c_char {
    let mut n: i32 =
        (strlen(s)).wrapping_add(1 as i32 as u32) as i32;
    let mut p: *mut libc::c_char = js_malloc(J, n) as *mut libc::c_char;
    memcpy(
        p as *mut libc::c_void,
        s as *const libc::c_void,
        n as u32,
    );
    p
}
#[no_mangle]
pub unsafe extern "C" fn js_free(J: &mut js_State, mut ptr: *mut libc::c_void) {
    ((*J).alloc).expect("non-null function pointer")((*J).actx, ptr, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn jsV_newmemstring(
    J: &mut js_State,
    mut s: *const libc::c_char,
    mut n: i32,
) -> *mut js_String {
    let mut v: *mut js_String =
        js_malloc(J, 9 as u32 as i32 + n + 1 as i32) as *mut js_String;
    memcpy(
        ((*v).p).as_mut_ptr() as *mut libc::c_void,
        s as *const libc::c_void,
        n as u32,
    );
    *((*v).p).as_mut_ptr().offset(n as isize) = 0 as i32 as libc::c_char;
    (*v).gcmark = 0 as i32 as libc::c_char;
    (*v).gcnext = (*J).gcstr;
    (*J).gcstr = v;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    v
}
#[no_mangle]
pub unsafe extern "C" fn js_pushvalue(J: &mut js_State, mut v: js_Value) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushundefined(J: &mut js_State) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 =
        JS_TUNDEFINED as i32 as libc::c_char;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushnull(J: &mut js_State) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TNULL as i32 as libc::c_char;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushboolean(J: &mut js_State, mut v: i32) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TBOOLEAN as i32 as libc::c_char;
    (*((*J).stack).offset((*J).top as isize)).u.boolean = (v != 0) as i32;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushnumber(J: &mut js_State, mut v: f64) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TNUMBER as i32 as libc::c_char;
    (*((*J).stack).offset((*J).top as isize)).u.number = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushstring(J: &mut js_State, mut v: *const libc::c_char) {
    let mut n: size_t = strlen(v);
    if n > ((1 as i32) << 28 as i32) as u32 {
        js_rangeerror(
            J,
            b"invalid string length\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    if n <= 15 as u32 as i32 as u32 {
        let mut s: *mut libc::c_char =
            ((*((*J).stack).offset((*J).top as isize)).u.shrstr).as_mut_ptr();
        loop {
            let fresh56 = n;
            n = n.wrapping_sub(1);
            if fresh56 == 0 {
                break;
            }
            let fresh57 = v;
            v = v.offset(1);
            let fresh58 = s;
            s = s.offset(1);
            *fresh58 = *fresh57;
        }
        *s = 0 as i32 as libc::c_char;
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TSHRSTR as i32 as libc::c_char;
    } else {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TMEMSTR as i32 as libc::c_char;
        let fresh59 = &mut (*((*J).stack).offset((*J).top as isize)).u.memstr;
        *fresh59 = jsV_newmemstring(J, v, n as i32);
    }
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushlstring(
    J: &mut js_State,
    mut v: *const libc::c_char,
    mut n: i32,
) {
    if n > (1 as i32) << 28 as i32 {
        js_rangeerror(
            J,
            b"invalid string length\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    if n <= 15 as u32 as i32 {
        let mut s: *mut libc::c_char =
            ((*((*J).stack).offset((*J).top as isize)).u.shrstr).as_mut_ptr();
        loop {
            let fresh60 = n;
            n -= 1;
            if fresh60 == 0 {
                break;
            }
            let fresh61 = v;
            v = v.offset(1);
            let fresh62 = s;
            s = s.offset(1);
            *fresh62 = *fresh61;
        }
        *s = 0 as i32 as libc::c_char;
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TSHRSTR as i32 as libc::c_char;
    } else {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TMEMSTR as i32 as libc::c_char;
        let fresh63 = &mut (*((*J).stack).offset((*J).top as isize)).u.memstr;
        *fresh63 = jsV_newmemstring(J, v, n);
    }
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushliteral(J: &mut js_State, mut v: *const libc::c_char) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TLITSTR as i32 as libc::c_char;
    let fresh64 = &mut (*((*J).stack).offset((*J).top as isize)).u.litstr;
    *fresh64 = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushobject(J: &mut js_State, mut v: *mut js_Object) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    (*((*J).stack).offset((*J).top as isize)).t.type_0 = JS_TOBJECT as i32 as libc::c_char;
    let fresh65 = &mut (*((*J).stack).offset((*J).top as isize)).u.object;
    *fresh65 = v;
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_pushglobal(J: &mut js_State) {
    js_pushobject(J, (*J).G);
}
#[no_mangle]
pub unsafe extern "C" fn js_currentfunction(J: &mut js_State) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    if (*J).bot > 0 as i32 {
        *((*J).stack).offset((*J).top as isize) =
            *((*J).stack).offset(((*J).bot - 1 as i32) as isize);
    } else {
        (*((*J).stack).offset((*J).top as isize)).t.type_0 =
            JS_TUNDEFINED as i32 as libc::c_char;
    }
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_currentfunctiondata(J: &mut js_State) -> *mut libc::c_void {
    if (*J).bot > 0 as i32 {
        return (*(*((*J).stack).offset(((*J).bot - 1 as i32) as isize))
            .u
            .object)
            .u
            .c
            .data;
    }
    core::ptr::null_mut::<libc::c_void>()
}
unsafe extern "C" fn stackidx(J: &mut js_State, mut idx: i32) -> *mut js_Value {
    static mut undefined: js_Value = js_Value {
        t: {
            C2RustUnnamed_6 {
                pad: [
                    0 as i32 as libc::c_char,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                ],
                type_0: JS_TUNDEFINED as i32 as libc::c_char,
            }
        },
    };
    idx = if idx < 0 as i32 {
        (*J).top + idx
    } else {
        (*J).bot + idx
    };
    if idx < 0 as i32 || idx >= (*J).top {
        return &mut undefined;
    }
    ((*J).stack).offset(idx as isize)
}
#[no_mangle]
pub unsafe extern "C" fn js_tovalue(J: &mut js_State, mut idx: i32) -> *mut js_Value {
    stackidx(J, idx)
}
#[no_mangle]
pub unsafe extern "C" fn js_isdefined(J: &mut js_State, mut idx: i32) -> i32 {
    ((*stackidx(J, idx)).t.type_0 as i32 != JS_TUNDEFINED as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isundefined(J: &mut js_State, mut idx: i32) -> i32 {
    ((*stackidx(J, idx)).t.type_0 as i32 == JS_TUNDEFINED as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isnull(J: &mut js_State, mut idx: i32) -> i32 {
    ((*stackidx(J, idx)).t.type_0 as i32 == JS_TNULL as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isboolean(J: &mut js_State, mut idx: i32) -> i32 {
    ((*stackidx(J, idx)).t.type_0 as i32 == JS_TBOOLEAN as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isnumber(J: &mut js_State, mut idx: i32) -> i32 {
    ((*stackidx(J, idx)).t.type_0 as i32 == JS_TNUMBER as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isstring(J: &mut js_State, mut idx: i32) -> i32 {
    let mut t: js_Type = (*stackidx(J, idx)).t.type_0 as js_Type;
    (t as libc::c_uint == JS_TSHRSTR as i32 as libc::c_uint
        || t as libc::c_uint == JS_TLITSTR as i32 as libc::c_uint
        || t as libc::c_uint == JS_TMEMSTR as i32 as libc::c_uint) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isprimitive(J: &mut js_State, mut idx: i32) -> i32 {
    ((*stackidx(J, idx)).t.type_0 as i32 != JS_TOBJECT as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isobject(J: &mut js_State, mut idx: i32) -> i32 {
    ((*stackidx(J, idx)).t.type_0 as i32 == JS_TOBJECT as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_iscoercible(J: &mut js_State, mut idx: i32) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as i32 != JS_TUNDEFINED as i32
        && (*v).t.type_0 as i32 != JS_TNULL as i32) as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_iscallable(J: &mut js_State, mut idx: i32) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as i32 == JS_TOBJECT as i32 {
        return ((*(*v).u.object).type_0 as libc::c_uint
            == JS_CFUNCTION as i32 as libc::c_uint
            || (*(*v).u.object).type_0 as libc::c_uint == JS_CSCRIPT as i32 as libc::c_uint
            || (*(*v).u.object).type_0 as libc::c_uint
                == JS_CCFUNCTION as i32 as libc::c_uint) as i32;
    }
    0 as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isarray(J: &mut js_State, mut idx: i32) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as i32 == JS_TOBJECT as i32
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint)
        as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isregexp(J: &mut js_State, mut idx: i32) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as i32 == JS_TOBJECT as i32
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint)
        as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_isuserdata(
    J: &mut js_State,
    mut idx: i32,
    mut tag: *const libc::c_char,
) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as i32 == JS_TOBJECT as i32
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CUSERDATA as i32 as libc::c_uint
    {
        return (strcmp(tag, (*(*v).u.object).u.user.tag) == 0) as i32;
    }
    0 as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_iserror(J: &mut js_State, mut idx: i32) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    ((*v).t.type_0 as i32 == JS_TOBJECT as i32
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CERROR as i32 as libc::c_uint)
        as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_typeof(J: &mut js_State, mut idx: i32) -> *const libc::c_char {
    let mut v: *mut js_Value = stackidx(J, idx);
    match (*v).t.type_0 as i32 {
        1 => b"undefined\0" as *const u8 as *const libc::c_char,
        2 => b"object\0" as *const u8 as *const libc::c_char,
        3 => b"boolean\0" as *const u8 as *const libc::c_char,
        4 => b"number\0" as *const u8 as *const libc::c_char,
        5 => b"string\0" as *const u8 as *const libc::c_char,
        6 => b"string\0" as *const u8 as *const libc::c_char,
        7 => {
            if (*(*v).u.object).type_0 as libc::c_uint
                == JS_CFUNCTION as i32 as libc::c_uint
                || (*(*v).u.object).type_0 as libc::c_uint
                    == JS_CCFUNCTION as i32 as libc::c_uint
            {
                return b"function\0" as *const u8 as *const libc::c_char;
            }
            b"object\0" as *const u8 as *const libc::c_char
        }
        0 | _ => b"string\0" as *const u8 as *const libc::c_char,
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_type(J: &mut js_State, mut idx: i32) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    match (*v).t.type_0 as i32 {
        1 => JS_ISUNDEFINED as i32,
        2 => JS_ISNULL as i32,
        3 => JS_ISBOOLEAN as i32,
        4 => JS_ISNUMBER as i32,
        5 => JS_ISSTRING as i32,
        6 => JS_ISSTRING as i32,
        7 => {
            if (*(*v).u.object).type_0 as libc::c_uint
                == JS_CFUNCTION as i32 as libc::c_uint
                || (*(*v).u.object).type_0 as libc::c_uint
                    == JS_CCFUNCTION as i32 as libc::c_uint
            {
                return JS_ISFUNCTION as i32;
            }
            JS_ISOBJECT as i32
        }
        0 | _ => JS_ISSTRING as i32,
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_toboolean(J: &mut js_State, mut idx: i32) -> i32 {
    let v = stackidx(J, idx);
    jsV_toboolean(J, v)
}
#[no_mangle]
pub unsafe extern "C" fn js_tonumber(J: &mut js_State, mut idx: i32) -> f64 {
    let v = stackidx(J, idx);
    jsV_tonumber(J, v)
}
#[no_mangle]
pub unsafe extern "C" fn js_tointeger(J: &mut js_State, mut idx: i32) -> i32 {
    let v = stackidx(J, idx);
    jsV_numbertointeger(jsV_tonumber(J, v))
}
#[no_mangle]
pub unsafe extern "C" fn js_toint32(J: &mut js_State, mut idx: i32) -> i32 {
    let v = stackidx(J, idx);
    jsV_numbertoint32(jsV_tonumber(J, v))
}
#[no_mangle]
pub unsafe extern "C" fn js_touint32(J: &mut js_State, mut idx: i32) -> libc::c_uint {
    let v = stackidx(J, idx);
    jsV_numbertouint32(jsV_tonumber(J, v))
}
#[no_mangle]
pub unsafe extern "C" fn js_toint16(J: &mut js_State, mut idx: i32) -> libc::c_short {
    let v = stackidx(J, idx);
    jsV_numbertoint16(jsV_tonumber(J, v))
}
#[no_mangle]
pub unsafe extern "C" fn js_touint16(J: &mut js_State, mut idx: i32) -> libc::c_ushort {
    let v = stackidx(J, idx);
    jsV_numbertouint16(jsV_tonumber(J, v))
}
#[no_mangle]
pub unsafe extern "C" fn js_tostring(
    J: &mut js_State,
    mut idx: i32,
) -> *const libc::c_char {
    let v = stackidx(J, idx);
    jsV_tostring(J, v)
}
#[no_mangle]
pub unsafe extern "C" fn js_toobject(J: &mut js_State, mut idx: i32) -> *mut js_Object {
    let v = stackidx(J, idx);
    jsV_toobject(J, v)
}
#[no_mangle]
pub unsafe extern "C" fn js_toprimitive(
    J: &mut js_State,
    mut idx: i32,
    mut hint: i32,
) {
    let v = stackidx(J, idx);
    jsV_toprimitive(J, v, hint);
}
#[no_mangle]
pub unsafe extern "C" fn js_toregexp(J: &mut js_State, mut idx: i32) -> *mut js_Regexp {
    let mut v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as i32 == JS_TOBJECT as i32
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint
    {
        return &mut (*(*v).u.object).u.r;
    }
    js_typeerror(J, b"not a regexp\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn js_touserdata(
    J: &mut js_State,
    mut idx: i32,
    mut tag: *const libc::c_char,
) -> *mut libc::c_void {
    let mut v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as i32 == JS_TOBJECT as i32
        && (*(*v).u.object).type_0 as libc::c_uint == JS_CUSERDATA as i32 as libc::c_uint
        && strcmp(tag, (*(*v).u.object).u.user.tag) == 0
    {
        return (*(*v).u.object).u.user.data;
    }
    js_typeerror(J, b"not a %s\0" as *const u8 as *const libc::c_char, tag);
}
unsafe extern "C" fn jsR_tofunction(J: &mut js_State, mut idx: i32) -> *mut js_Object {
    let mut v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as i32 == JS_TUNDEFINED as i32
        || (*v).t.type_0 as i32 == JS_TNULL as i32
    {
        return core::ptr::null_mut::<js_Object>();
    }
    if (*v).t.type_0 as i32 == JS_TOBJECT as i32
        && ((*(*v).u.object).type_0 as libc::c_uint == JS_CFUNCTION as i32 as libc::c_uint
            || (*(*v).u.object).type_0 as libc::c_uint
                == JS_CCFUNCTION as i32 as libc::c_uint)
    {
        return (*v).u.object;
    }
    js_typeerror(J, b"not a function\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub extern "C" fn js_gettop(J: &mut js_State) -> i32 {
    J.top - J.bot
}
#[no_mangle]
pub unsafe extern "C" fn js_pop(J: &mut js_State, mut n: i32) {
    (*J).top -= n;
    if (*J).top < (*J).bot {
        (*J).top = (*J).bot;
        js_error(J, b"stack underflow!\0" as *const u8 as *const libc::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_remove(J: &mut js_State, mut idx: i32) {
    idx = if idx < 0 as i32 {
        (*J).top + idx
    } else {
        (*J).bot + idx
    };
    if idx < (*J).bot || idx >= (*J).top {
        js_error(J, b"stack error!\0" as *const u8 as *const libc::c_char);
    }
    while idx < (*J).top - 1 as i32 {
        *((*J).stack).offset(idx as isize) =
            *((*J).stack).offset((idx + 1 as i32) as isize);
        idx += 1;
    }
    (*J).top -= 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_insert(J: &mut js_State, mut idx: i32) {
    js_error(
        J,
        b"not implemented yet\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn js_replace(J: &mut js_State, mut idx: i32) {
    idx = if idx < 0 as i32 {
        (*J).top + idx
    } else {
        (*J).bot + idx
    };
    if idx < (*J).bot || idx >= (*J).top {
        js_error(J, b"stack error!\0" as *const u8 as *const libc::c_char);
    }
    (*J).top -= 1;
    *((*J).stack).offset(idx as isize) = *((*J).stack).offset((*J).top as isize);
}
#[no_mangle]
pub unsafe extern "C" fn js_copy(J: &mut js_State, mut idx: i32) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) = *stackidx(J, idx);
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_dup(J: &mut js_State) {
    if (*J).top + 1 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) =
        *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    (*J).top += 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_dup2(J: &mut js_State) {
    if (*J).top + 2 as i32 >= 4096 as i32 {
        js_stackoverflow(J);
    }
    *((*J).stack).offset((*J).top as isize) =
        *((*J).stack).offset(((*J).top - 2 as i32) as isize);
    *((*J).stack).offset(((*J).top + 1 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    (*J).top += 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot2(J: &mut js_State) {
    let mut tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    *((*J).stack).offset(((*J).top - 1 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 2 as i32) as isize);
    *((*J).stack).offset(((*J).top - 2 as i32) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot3(J: &mut js_State) {
    let mut tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    *((*J).stack).offset(((*J).top - 1 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 2 as i32) as isize);
    *((*J).stack).offset(((*J).top - 2 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 3 as i32) as isize);
    *((*J).stack).offset(((*J).top - 3 as i32) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot4(J: &mut js_State) {
    let mut tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    *((*J).stack).offset(((*J).top - 1 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 2 as i32) as isize);
    *((*J).stack).offset(((*J).top - 2 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 3 as i32) as isize);
    *((*J).stack).offset(((*J).top - 3 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 4 as i32) as isize);
    *((*J).stack).offset(((*J).top - 4 as i32) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot2pop1(J: &mut js_State) {
    *((*J).stack).offset(((*J).top - 2 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    (*J).top -= 1;
    (*J).top;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot3pop2(J: &mut js_State) {
    *((*J).stack).offset(((*J).top - 3 as i32) as isize) =
        *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    (*J).top -= 2 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn js_rot(J: &mut js_State, mut n: i32) {
    let mut i: i32 = 0;
    let mut tmp: js_Value = *((*J).stack).offset(((*J).top - 1 as i32) as isize);
    i = 1 as i32;
    while i < n {
        *((*J).stack).offset(((*J).top - i) as isize) =
            *((*J).stack).offset(((*J).top - i - 1 as i32) as isize);
        i += 1;
    }
    *((*J).stack).offset(((*J).top - i) as isize) = tmp;
}
#[no_mangle]
pub unsafe extern "C" fn js_isarrayindex(
    J: &mut js_State,
    mut p: *const libc::c_char,
    mut idx: *mut i32,
) -> i32 {
    let mut n: i32 = 0 as i32;
    if *p.offset(0 as i32 as isize) as i32 == 0 as i32 {
        return 0 as i32;
    }
    if *p.offset(0 as i32 as isize) as i32 == '0' as i32 {
        return if *p.offset(1 as i32 as isize) as i32 == 0 as i32 {
            *idx = 0 as i32;
            1 as i32
        } else {
            0 as i32
        };
    }
    while *p != 0 {
        let fresh66 = p;
        p = p.offset(1);
        let mut c: i32 = *fresh66 as i32;
        if c >= '0' as i32 && c <= '9' as i32 {
            if n >= 2147483647 as i32 / 10 as i32 {
                return 0 as i32;
            }
            n = n * 10 as i32 + (c - '0' as i32);
        } else {
            return 0 as i32;
        }
    }
    *idx = n;
    1 as i32
}
unsafe extern "C" fn js_pushrune(J: &mut js_State, mut rune: Rune) {
    let mut buf: [libc::c_char; 5] = [0; 5];
    if rune >= 0 as i32 {
        buf[jsU_runetochar(buf.as_mut_ptr(), &mut rune) as usize] =
            0 as i32 as libc::c_char;
        js_pushstring(J, buf.as_mut_ptr());
    } else {
        js_pushundefined(J);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsR_unflattenarray(J: &mut js_State, mut obj: *mut js_Object) {
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        let mut ref_0: *mut js_Property = core::ptr::null_mut::<js_Property>();
        let mut i: i32 = 0;
        let mut name: [libc::c_char; 32] = [0; 32];
        if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
            (*obj).properties = core::ptr::null_mut::<js_Property>();
            js_throw(J);
        }
        i = 0 as i32;
        while i < (*obj).u.a.flat_length {
            js_itoa(name.as_mut_ptr(), i);
            ref_0 = jsV_setproperty(J, obj, name.as_mut_ptr());
            (*ref_0).value = *((*obj).u.a.array).offset(i as isize);
            i += 1;
        }
        js_free(J, (*obj).u.a.array as *mut libc::c_void);
        (*obj).u.a.simple = 0 as i32;
        (*obj).u.a.flat_length = 0 as i32;
        (*obj).u.a.flat_capacity = 0 as i32;
        (*obj).u.a.array = core::ptr::null_mut::<js_Value>();
        js_endtry(J);
    }
}
unsafe extern "C" fn jsR_hasproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) -> i32 {
    let mut ref_0: *mut js_Property = core::ptr::null_mut::<js_Property>();
    let mut k: i32 = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushnumber(J, (*obj).u.a.length as f64);
            return 1 as i32;
        }
        if (*obj).u.a.simple != 0 && js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as i32 && k < (*obj).u.a.flat_length {
                js_pushvalue(J, *((*obj).u.a.array).offset(k as isize));
                return 1 as i32;
            }
            return 0 as i32;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushnumber(J, (*obj).u.s.length as f64);
            return 1 as i32;
        }
        if js_isarrayindex(J, name, &mut k) != 0 && k >= 0 as i32 && k < (*obj).u.s.length {
            let v = js_runeat(J, (*obj).u.s.string, k);
            js_pushrune(J, v);
            return 1 as i32;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushstring(J, (*obj).u.r.source);
            return 1 as i32;
        }
        if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushboolean(
                J,
                (*obj).u.r.flags as i32 & JS_REGEXP_G as i32,
            );
            return 1 as i32;
        }
        if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushboolean(
                J,
                (*obj).u.r.flags as i32 & JS_REGEXP_I as i32,
            );
            return 1 as i32;
        }
        if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushboolean(
                J,
                (*obj).u.r.flags as i32 & JS_REGEXP_M as i32,
            );
            return 1 as i32;
        }
        if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
            js_pushnumber(J, (*obj).u.r.last as f64);
            return 1 as i32;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as i32 as libc::c_uint
        && ((*obj).u.user.has).is_some()
        && ((*obj).u.user.has).expect("non-null function pointer")(J, (*obj).u.user.data, name) != 0
    {
        return 1 as i32;
    }
    ref_0 = jsV_getproperty(J, obj, name);
    if !ref_0.is_null() {
        if !((*ref_0).getter).is_null() {
            js_pushobject(J, (*ref_0).getter);
            js_pushobject(J, obj);
            js_call(J, 0 as i32);
        } else {
            js_pushvalue(J, (*ref_0).value);
        }
        return 1 as i32;
    }
    0 as i32
}
unsafe extern "C" fn jsR_getproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) {
    if jsR_hasproperty(J, obj, name) == 0 {
        js_pushundefined(J);
    }
}
unsafe extern "C" fn jsR_hasindex(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut k: i32,
) -> i32 {
    let mut buf: [libc::c_char; 32] = [0; 32];
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        if k >= 0 as i32 && k < (*obj).u.a.flat_length {
            js_pushvalue(J, *((*obj).u.a.array).offset(k as isize));
            return 1 as i32;
        }
        return 0 as i32;
    }
    jsR_hasproperty(J, obj, js_itoa(buf.as_mut_ptr(), k))
}
unsafe extern "C" fn jsR_getindex(J: &mut js_State, mut obj: *mut js_Object, mut k: i32) {
    if jsR_hasindex(J, obj, k) == 0 {
        js_pushundefined(J);
    }
}
unsafe extern "C" fn jsR_setarrayindex(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut k: i32,
    mut value: *mut js_Value,
) {
    let mut newlen: i32 = k + 1 as i32;
    if (*obj).u.a.simple != 0 {
    } else {
        __assert_fail(
            b"obj->u.a.simple\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            11018 as i32 as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
            ))
            .as_ptr(),
        );
    }
    {
        if (*obj).u.a.simple != 0 {
        } else {
            __assert_fail(
                b"obj->u.a.simple\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                11018 as i32 as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if k >= 0 as i32 {
    } else {
        __assert_fail(
            b"k >= 0\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            11019 as i32 as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
            ))
            .as_ptr(),
        );
    }
    {
        if k >= 0 as i32 {
        } else {
            __assert_fail(
                b"k >= 0\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                11019 as i32 as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if newlen > (1 as i32) << 26 as i32 {
        js_rangeerror(J, b"array too large\0" as *const u8 as *const libc::c_char);
    }
    if newlen > (*obj).u.a.flat_length {
        if newlen == (*obj).u.a.flat_length + 1 as i32 {
        } else {
            __assert_fail(
                b"newlen == obj->u.a.flat_length + 1\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                11023 as i32 as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                    b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                ))
                .as_ptr(),
            );
        }
        {
            if newlen == (*obj).u.a.flat_length + 1 as i32 {
            } else {
                __assert_fail(
                    b"newlen == obj->u.a.flat_length + 1\0" as *const u8 as *const libc::c_char,
                    b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                    11023 as i32 as libc::c_uint,
                    (*::core::mem::transmute::<&[u8; 65], &[libc::c_char; 65]>(
                        b"void jsR_setarrayindex(js_State *, js_Object *, int, js_Value *)\0",
                    ))
                    .as_ptr(),
                );
            }
        };
        if newlen > (*obj).u.a.flat_capacity {
            let mut newcap: i32 = (*obj).u.a.flat_capacity;
            if newcap == 0 as i32 {
                newcap = 8 as i32;
            }
            while newcap < newlen {
                newcap <<= 1 as i32;
            }
            (*obj).u.a.array = js_realloc(
                J,
                (*obj).u.a.array as *mut libc::c_void,
                (newcap as u32)
                    .wrapping_mul(::core::mem::size_of::<js_Value>() as u32)
                    as i32,
            ) as *mut js_Value;
            (*obj).u.a.flat_capacity = newcap;
        }
        (*obj).u.a.flat_length = newlen;
    }
    if newlen > (*obj).u.a.length {
        (*obj).u.a.length = newlen;
    }
    *((*obj).u.a.array).offset(k as isize) = *value;
}
unsafe extern "C" fn jsR_setproperty(
    J: &mut js_State,
    obj: *mut js_Object,
    name: *const libc::c_char,
    transient: i32,
) {
    let mut current_block: u64;
    let mut value: *mut js_Value = stackidx(J, -(1 as i32));
    let mut ref_0: *mut js_Property = core::ptr::null_mut::<js_Property>();
    let mut k: i32 = 0;
    let mut own: i32 = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            let mut rawlen: f64 = jsV_tonumber(J, value);
            let mut newlen: i32 = jsV_numbertointeger(rawlen);
            if newlen as f64 != rawlen || newlen < 0 as i32 {
                js_rangeerror(
                    J,
                    b"invalid array length\0" as *const u8 as *const libc::c_char,
                );
            }
            if newlen > (1 as i32) << 26 as i32 {
                js_rangeerror(J, b"array too large\0" as *const u8 as *const libc::c_char);
            }
            if (*obj).u.a.simple != 0 {
                (*obj).u.a.length = newlen;
                if newlen <= (*obj).u.a.flat_length {
                    (*obj).u.a.flat_length = newlen;
                }
            } else {
                jsV_resizearray(J, obj, newlen);
            }
            return;
        }
        if js_isarrayindex(J, name, &mut k) != 0 {
            if (*obj).u.a.simple != 0 {
                if k >= 0 as i32 && k <= (*obj).u.a.flat_length {
                    jsR_setarrayindex(J, obj, k, value);
                } else {
                    jsR_unflattenarray(J, obj);
                    if (*obj).u.a.length < k + 1 as i32 {
                        (*obj).u.a.length = k + 1 as i32;
                    }
                }
            } else if (*obj).u.a.length < k + 1 as i32 {
                (*obj).u.a.length = k + 1 as i32;
            }
        }
        current_block = 4567019141635105728;
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as i32 && k < (*obj).u.s.length {
                current_block = 5947843046271591061;
            } else {
                current_block = 4567019141635105728;
            }
        } else {
            current_block = 4567019141635105728;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 5947843046271591061;
        } else {
            if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
                (*obj).u.r.last = jsV_tointeger(J, value) as libc::c_ushort;
                return;
            }
            current_block = 4567019141635105728;
        }
    } else {
        if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as i32 as libc::c_uint
            && ((*obj).u.user.put).is_some()
            && ((*obj).u.user.put).expect("non-null function pointer")(J, (*obj).u.user.data, name)
                != 0
        {
            return;
        }
        current_block = 4567019141635105728;
    }
    if current_block == 4567019141635105728 {
        ref_0 = jsV_getpropertyx(J, obj, name, &mut own);
        if !ref_0.is_null() {
            if !((*ref_0).setter).is_null() {
                js_pushobject(J, (*ref_0).setter);
                js_pushobject(J, obj);
                js_pushvalue(J, *value);
                js_call(J, 1 as i32);
                js_pop(J, 1 as i32);
                return;
            } else {
                if (*J).strict != 0 && !((*ref_0).getter).is_null() {
                    js_typeerror(
                        J,
                        b"setting property '%s' that only has a getter\0" as *const u8
                            as *const libc::c_char,
                        name,
                    );
                }
                if (*ref_0).atts & JS_READONLY as i32 != 0 {
                    current_block = 5947843046271591061;
                } else {
                    current_block = 13826291924415791078;
                }
            }
        } else {
            current_block = 13826291924415791078;
        }
        match current_block {
            5947843046271591061 => {}
            _ => {
                if ref_0.is_null() || own == 0 {
                    if transient != 0 {
                        if (*J).strict != 0 {
                            js_typeerror(
                                J,
                                b"cannot create property '%s' on transient object\0" as *const u8
                                    as *const libc::c_char,
                                name,
                            );
                        }
                        return;
                    }
                    ref_0 = jsV_setproperty(J, obj, name);
                }
                if !ref_0.is_null() {
                    if (*ref_0).atts & JS_READONLY as i32 == 0 {
                        (*ref_0).value = *value;
                        current_block = 1623252117315916725;
                    } else {
                        current_block = 5947843046271591061;
                    }
                } else {
                    current_block = 1623252117315916725;
                }
                match current_block {
                    5947843046271591061 => {}
                    _ => return,
                }
            }
        }
    }
    if (*J).strict != 0 {
        js_typeerror(
            J,
            b"'%s' is read-only\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
}
unsafe extern "C" fn jsR_setindex(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut k: i32,
    mut transient: i32,
) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*obj).u.a.simple != 0
        && k >= 0 as i32
        && k <= (*obj).u.a.flat_length
    {
        let v = stackidx(J, -(1 as i32));
        jsR_setarrayindex(J, obj, k, v);
    } else {
        jsR_setproperty(J, obj, js_itoa(buf.as_mut_ptr(), k), transient);
    };
}
unsafe extern "C" fn jsR_defproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
    mut atts: i32,
    mut value: *mut js_Value,
    mut getter: *mut js_Object,
    mut setter: *mut js_Object,
    mut throw: i32,
) {
    let mut current_block: u64;
    let mut ref_0: *mut js_Property = core::ptr::null_mut::<js_Property>();
    let mut k: i32 = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else {
            if (*obj).u.a.simple != 0 {
                jsR_unflattenarray(J, obj);
            }
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as i32 && k < (*obj).u.s.length {
                current_block = 8261918236977660230;
            } else {
                current_block = 2370887241019905314;
            }
        } else {
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 8261918236977660230;
        } else {
            current_block = 2370887241019905314;
        }
    } else {
        if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as i32 as libc::c_uint
            && ((*obj).u.user.put).is_some()
            && ((*obj).u.user.put).expect("non-null function pointer")(J, (*obj).u.user.data, name)
                != 0
        {
            return;
        }
        current_block = 2370887241019905314;
    }
    match current_block {
        8261918236977660230 => {
            if (*J).strict != 0 || throw != 0 {
                js_typeerror(
                    J,
                    b"'%s' is read-only or non-configurable\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
        }
        _ => {
            ref_0 = jsV_setproperty(J, obj, name);
            if !ref_0.is_null() {
                if !value.is_null() {
                    if (*ref_0).atts & JS_READONLY as i32 == 0 {
                        (*ref_0).value = *value;
                    } else if (*J).strict != 0 {
                        js_typeerror(
                            J,
                            b"'%s' is read-only\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                }
                if !getter.is_null() {
                    if (*ref_0).atts & JS_DONTCONF as i32 == 0 {
                        (*ref_0).getter = getter;
                    } else if (*J).strict != 0 {
                        js_typeerror(
                            J,
                            b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                }
                if !setter.is_null() {
                    if (*ref_0).atts & JS_DONTCONF as i32 == 0 {
                        (*ref_0).setter = setter;
                    } else if (*J).strict != 0 {
                        js_typeerror(
                            J,
                            b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
                            name,
                        );
                    }
                }
                (*ref_0).atts |= atts;
            }
        }
    }
}
unsafe extern "C" fn jsR_delproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) -> i32 {
    let mut current_block: u64;
    let mut ref_0: *mut js_Property = core::ptr::null_mut::<js_Property>();
    let mut k: i32 = 0;
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else {
            if (*obj).u.a.simple != 0 {
                jsR_unflattenarray(J, obj);
            }
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint {
        if strcmp(name, b"length\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if js_isarrayindex(J, name, &mut k) != 0 {
            if k >= 0 as i32 && k < (*obj).u.s.length {
                current_block = 15302371838142339756;
            } else {
                current_block = 2370887241019905314;
            }
        } else {
            current_block = 2370887241019905314;
        }
    } else if (*obj).type_0 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint {
        if strcmp(name, b"source\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"global\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"ignoreCase\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"multiline\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else if strcmp(name, b"lastIndex\0" as *const u8 as *const libc::c_char) == 0 {
            current_block = 15302371838142339756;
        } else {
            current_block = 2370887241019905314;
        }
    } else {
        if (*obj).type_0 as libc::c_uint == JS_CUSERDATA as i32 as libc::c_uint
            && ((*obj).u.user.delete).is_some()
            && ((*obj).u.user.delete).expect("non-null function pointer")(
                J,
                (*obj).u.user.data,
                name,
            ) != 0
        {
            return 1 as i32;
        }
        current_block = 2370887241019905314;
    }
    if current_block == 2370887241019905314 {
        ref_0 = jsV_getownproperty(J, obj, name);
        if !ref_0.is_null() {
            if (*ref_0).atts & JS_DONTCONF as i32 != 0 {
                current_block = 15302371838142339756;
            } else {
                jsV_delproperty(J, obj, name);
                current_block = 14576567515993809846;
            }
        } else {
            current_block = 14576567515993809846;
        }
        match current_block {
            15302371838142339756 => {}
            _ => return 1 as i32,
        }
    }
    if (*J).strict != 0 {
        js_typeerror(
            J,
            b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    0 as i32
}
unsafe extern "C" fn jsR_delindex(J: &mut js_State, mut obj: *mut js_Object, mut k: i32) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*obj).u.a.simple != 0
        && k == (*obj).u.a.flat_length - 1 as i32
    {
        (*obj).u.a.flat_length = k;
    } else {
        jsR_delproperty(J, obj, js_itoa(buf.as_mut_ptr(), k));
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_ref(J: &mut js_State) -> *const libc::c_char {
    let mut v: *mut js_Value = stackidx(J, -(1 as i32));
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut buf: [libc::c_char; 32] = [0; 32];
    match (*v).t.type_0 as i32 {
        1 => {
            s = b"_Undefined\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            s = b"_Null\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            s = if (*v).u.boolean != 0 {
                b"_True\0" as *const u8 as *const libc::c_char
            } else {
                b"_False\0" as *const u8 as *const libc::c_char
            };
        }
        7 => {
            sprintf(
                buf.as_mut_ptr(),
                b"%p\0" as *const u8 as *const libc::c_char,
                (*v).u.object as *mut libc::c_void,
            );
            s = js_intern(J, buf.as_mut_ptr());
        }
        _ => {
            let fresh67 = (*J).nextref;
            (*J).nextref += 1;
            sprintf(
                buf.as_mut_ptr(),
                b"%d\0" as *const u8 as *const libc::c_char,
                fresh67,
            );
            s = js_intern(J, buf.as_mut_ptr());
        }
    }
    js_setregistry(J, s);
    s
}
#[no_mangle]
pub unsafe extern "C" fn js_unref(J: &mut js_State, mut ref_0: *const libc::c_char) {
    js_delregistry(J, ref_0);
}
#[no_mangle]
pub unsafe extern "C" fn js_getregistry(J: &mut js_State, mut name: *const libc::c_char) {
    jsR_getproperty(J, (*J).R, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_setregistry(J: &mut js_State, mut name: *const libc::c_char) {
    jsR_setproperty(J, (*J).R, name, 0 as i32);
    js_pop(J, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_delregistry(J: &mut js_State, mut name: *const libc::c_char) {
    jsR_delproperty(J, (*J).R, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_getglobal(J: &mut js_State, mut name: *const libc::c_char) {
    jsR_getproperty(J, (*J).G, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_setglobal(J: &mut js_State, mut name: *const libc::c_char) {
    jsR_setproperty(J, (*J).G, name, 0 as i32);
    js_pop(J, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_defglobal(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut atts: i32,
) {
    let v = stackidx(J, -(1 as i32));
    jsR_defproperty(
        J,
        (*J).G,
        name,
        atts,
        v,
        core::ptr::null_mut::<js_Object>(),
        core::ptr::null_mut::<js_Object>(),
        0 as i32,
    );
    js_pop(J, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_delglobal(J: &mut js_State, mut name: *const libc::c_char) {
    jsR_delproperty(J, (*J).G, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_getproperty(
    J: &mut js_State,
    mut idx: i32,
    mut name: *const libc::c_char,
) {
    let obj = js_toobject(J, idx);
    jsR_getproperty(J, obj, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_setproperty(
    J: &mut js_State,
    idx: i32,
    name: *const libc::c_char,
) {
    let obj = js_toobject(J, idx);
    let transient = (js_isobject(J, idx) == 0) as i32;
    jsR_setproperty(J, obj, name, transient);
    js_pop(J, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_defproperty(
    J: &mut js_State,
    mut idx: i32,
    mut name: *const libc::c_char,
    mut atts: i32,
) {
    let obj = js_toobject(J, idx);
    let attr = stackidx(J, -(1 as i32));
    jsR_defproperty(
        J,
        obj,
        name,
        atts,
        attr,
        core::ptr::null_mut::<js_Object>(),
        core::ptr::null_mut::<js_Object>(),
        1 as i32,
    );
    js_pop(J, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_delproperty(
    J: &mut js_State,
    mut idx: i32,
    mut name: *const libc::c_char,
) {
    let obj = js_toobject(J, idx);
    jsR_delproperty(J, obj, name);
}
#[no_mangle]
pub unsafe extern "C" fn js_defaccessor(
    J: &mut js_State,
    mut idx: i32,
    mut name: *const libc::c_char,
    mut atts: i32,
) {
    let obj = js_toobject(J, idx);
    let getter = jsR_tofunction(J, -(2 as i32));
    let setter = jsR_tofunction(J, -(1 as i32));
    jsR_defproperty(
        J,
        obj,
        name,
        atts,
        core::ptr::null_mut::<js_Value>(),
        getter,
        setter,
        1 as i32,
    );
    js_pop(J, 2 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_hasproperty(
    J: &mut js_State,
    mut idx: i32,
    mut name: *const libc::c_char,
) -> i32 {
    let obj = js_toobject(J, idx);
    jsR_hasproperty(J, obj, name)
}
#[no_mangle]
pub unsafe extern "C" fn js_getindex(J: &mut js_State, mut idx: i32, mut i: i32) {
    let obj = js_toobject(J, idx);
    jsR_getindex(J, obj, i);
}
#[no_mangle]
pub unsafe extern "C" fn js_hasindex(
    J: &mut js_State,
    mut idx: i32,
    mut i: i32,
) -> i32 {
    let obj = js_toobject(J, idx);
    jsR_hasindex(J, obj, i)
}
#[no_mangle]
pub unsafe extern "C" fn js_setindex(J: &mut js_State, mut idx: i32, mut i: i32) {
    let obj = js_toobject(J, idx);
    let transient = js_isobject(J, idx);
    jsR_setindex(J, obj, i, (transient == 0) as i32);
    js_pop(J, 1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_delindex(J: &mut js_State, mut idx: i32, mut i: i32) {
    let obj = js_toobject(J, idx);
    jsR_delindex(J, obj, i);
}
#[no_mangle]
pub unsafe extern "C" fn js_pushiterator(
    J: &mut js_State,
    mut idx: i32,
    mut own: i32,
) {
    let obj = js_toobject(J, idx);
    let v = jsV_newiterator(J, obj, own);
    js_pushobject(J, v);
}
#[no_mangle]
pub unsafe extern "C" fn js_nextiterator(
    J: &mut js_State,
    mut idx: i32,
) -> *const libc::c_char {
    let io: *mut js_Object = js_toobject(J, idx);
    jsV_nextiterator(J, io)
}
#[no_mangle]
pub unsafe extern "C" fn jsR_newenvironment(
    J: &mut js_State,
    mut vars: *mut js_Object,
    mut outer: *mut js_Environment,
) -> *mut js_Environment {
    let mut E: *mut js_Environment = js_malloc(
        J,
        ::core::mem::size_of::<js_Environment>() as u32 as i32,
    ) as *mut js_Environment;
    (*E).gcmark = 0 as i32;
    (*E).gcnext = (*J).gcenv;
    (*J).gcenv = E;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    (*E).outer = outer;
    (*E).variables = vars;
    E
}
unsafe extern "C" fn js_initvar(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut idx: i32,
) {
    let value = stackidx(J, idx);
    jsR_defproperty(
        J,
        (*(*J).E).variables,
        name,
        JS_DONTENUM as i32 | JS_DONTCONF as i32,
        value,
        core::ptr::null_mut::<js_Object>(),
        core::ptr::null_mut::<js_Object>(),
        0 as i32,
    );
}
unsafe extern "C" fn js_hasvar(J: &mut js_State, mut name: *const libc::c_char) -> i32 {
    let mut E: *mut js_Environment = (*J).E;
    loop {
        let mut ref_0: *mut js_Property = jsV_getproperty(J, (*E).variables, name);
        if !ref_0.is_null() {
            if !((*ref_0).getter).is_null() {
                js_pushobject(J, (*ref_0).getter);
                js_pushobject(J, (*E).variables);
                js_call(J, 0 as i32);
            } else {
                js_pushvalue(J, (*ref_0).value);
            }
            return 1 as i32;
        }
        E = (*E).outer;
        if E.is_null() {
            break;
        }
    }
    0 as i32
}
unsafe extern "C" fn js_setvar(J: &mut js_State, mut name: *const libc::c_char) {
    let mut E: *mut js_Environment = (*J).E;
    loop {
        let mut ref_0: *mut js_Property = jsV_getproperty(J, (*E).variables, name);
        if !ref_0.is_null() {
            if !((*ref_0).setter).is_null() {
                js_pushobject(J, (*ref_0).setter);
                js_pushobject(J, (*E).variables);
                js_copy(J, -(3 as i32));
                js_call(J, 1 as i32);
                js_pop(J, 1 as i32);
                return;
            }
            if (*ref_0).atts & JS_READONLY as i32 == 0 {
                (*ref_0).value = *stackidx(J, -(1 as i32));
            } else if (*J).strict != 0 {
                js_typeerror(
                    J,
                    b"'%s' is read-only\0" as *const u8 as *const libc::c_char,
                    name,
                );
            }
            return;
        }
        E = (*E).outer;
        if E.is_null() {
            break;
        }
    }
    if (*J).strict != 0 {
        js_referenceerror(
            J,
            b"assignment to undeclared variable '%s'\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    jsR_setproperty(J, (*J).G, name, 0 as i32);
}
unsafe extern "C" fn js_delvar(J: &mut js_State, mut name: *const libc::c_char) -> i32 {
    let mut E: *mut js_Environment = (*J).E;
    loop {
        let mut ref_0: *mut js_Property = jsV_getownproperty(J, (*E).variables, name);
        if !ref_0.is_null() {
            if (*ref_0).atts & JS_DONTCONF as i32 != 0 {
                if (*J).strict != 0 {
                    js_typeerror(
                        J,
                        b"'%s' is non-configurable\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
                return 0 as i32;
            }
            jsV_delproperty(J, (*E).variables, name);
            return 1 as i32;
        }
        E = (*E).outer;
        if E.is_null() {
            break;
        }
    }
    jsR_delproperty(J, (*J).G, name)
}
unsafe extern "C" fn jsR_savescope(J: &mut js_State, mut newE: *mut js_Environment) {
    if (*J).envtop + 1 as i32 >= 1024 as i32 {
        js_stackoverflow(J);
    }
    let fresh68 = (*J).envtop;
    (*J).envtop += 1;
    (*J).envstack[fresh68 as usize] = (*J).E;
    (*J).E = newE;
}
unsafe extern "C" fn jsR_restorescope(J: &mut js_State) {
    (*J).envtop -= 1;
    (*J).E = (*J).envstack[(*J).envtop as usize];
}
unsafe extern "C" fn jsR_calllwfunction(
    J: &mut js_State,
    mut n: i32,
    mut F: *mut js_Function,
    mut scope: *mut js_Environment,
) {
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    let mut i: i32 = 0;
    jsR_savescope(J, scope);
    if n > (*F).numparams {
        js_pop(J, n - (*F).numparams);
        n = (*F).numparams;
    }
    i = n;
    while i < (*F).varlen {
        js_pushundefined(J);
        i += 1;
    }
    jsR_run(J, F);
    v = *stackidx(J, -(1 as i32));
    (*J).bot -= 1;
    (*J).top = (*J).bot;
    js_pushvalue(J, v);
    jsR_restorescope(J);
}
unsafe extern "C" fn jsR_callfunction(
    J: &mut js_State,
    mut n: i32,
    mut F: *mut js_Function,
    mut scope: *mut js_Environment,
) {
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    let mut i: i32 = 0;
    let vars = jsV_newobject(J, JS_COBJECT, core::ptr::null_mut::<js_Object>());
    scope = jsR_newenvironment(J, vars, scope);
    jsR_savescope(J, scope);
    if (*F).arguments != 0 {
        js_newarguments(J);
        if (*J).strict == 0 {
            js_currentfunction(J);
            js_defproperty(
                J,
                -(2 as i32),
                b"callee\0" as *const u8 as *const libc::c_char,
                JS_DONTENUM as i32,
            );
        }
        js_pushnumber(J, n as f64);
        js_defproperty(
            J,
            -(2 as i32),
            b"length\0" as *const u8 as *const libc::c_char,
            JS_DONTENUM as i32,
        );
        i = 0 as i32;
        while i < n {
            js_copy(J, i + 1 as i32);
            js_setindex(J, -(2 as i32), i);
            i += 1;
        }
        js_initvar(
            J,
            b"arguments\0" as *const u8 as *const libc::c_char,
            -(1 as i32),
        );
        js_pop(J, 1 as i32);
    }
    i = 0 as i32;
    while i < n && i < (*F).numparams {
        js_initvar(J, *((*F).vartab).offset(i as isize), i + 1 as i32);
        i += 1;
    }
    js_pop(J, n);
    while i < (*F).varlen {
        js_pushundefined(J);
        js_initvar(J, *((*F).vartab).offset(i as isize), -(1 as i32));
        js_pop(J, 1 as i32);
        i += 1;
    }
    jsR_run(J, F);
    v = *stackidx(J, -(1 as i32));
    (*J).bot -= 1;
    (*J).top = (*J).bot;
    js_pushvalue(J, v);
    jsR_restorescope(J);
}
unsafe extern "C" fn jsR_callscript(
    J: &mut js_State,
    mut n: i32,
    mut F: *mut js_Function,
    mut scope: *mut js_Environment,
) {
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    let mut i: i32 = 0;
    if !scope.is_null() {
        jsR_savescope(J, scope);
    }
    js_pop(J, n);
    i = 0 as i32;
    while i < (*F).varlen {
        if js_hasvar(J, *((*F).vartab).offset(i as isize)) == 0 {
            js_pushundefined(J);
            js_initvar(J, *((*F).vartab).offset(i as isize), -(1 as i32));
            js_pop(J, 1 as i32);
        }
        i += 1;
    }
    jsR_run(J, F);
    v = *stackidx(J, -(1 as i32));
    (*J).bot -= 1;
    (*J).top = (*J).bot;
    js_pushvalue(J, v);
    if !scope.is_null() {
        jsR_restorescope(J);
    }
}
unsafe extern "C" fn jsR_callcfunction(
    J: &mut js_State,
    mut n: i32,
    mut min: i32,
    mut F: js_CFunction,
) {
    let mut save_top: i32 = 0;
    let mut i: i32 = 0;
    let mut v: js_Value = js_Value {
        t: C2RustUnnamed_6 {
            pad: [0; 15],
            type_0: 0,
        },
    };
    i = n;
    while i < min {
        js_pushundefined(J);
        i += 1;
    }
    save_top = (*J).top;
    F.expect("non-null function pointer")(J);
    if (*J).top > save_top {
        v = *stackidx(J, -(1 as i32));
        (*J).bot -= 1;
        (*J).top = (*J).bot;
        js_pushvalue(J, v);
    } else {
        (*J).bot -= 1;
        (*J).top = (*J).bot;
        js_pushundefined(J);
    };
}
unsafe extern "C" fn jsR_pushtrace(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
) {
    if (*J).tracetop + 1 as i32 == 1024 as i32 {
        js_error(
            J,
            b"call stack overflow\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).tracetop += 1;
    (*J).tracetop;
    (*J).trace[(*J).tracetop as usize].name = name;
    (*J).trace[(*J).tracetop as usize].file = file;
    (*J).trace[(*J).tracetop as usize].line = line;
}
#[no_mangle]
pub unsafe extern "C" fn js_call(J: &mut js_State, mut n: i32) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut savebot: i32 = 0;
    if n < 0 as i32 {
        js_rangeerror(
            J,
            b"number of arguments cannot be negative\0" as *const u8 as *const libc::c_char,
        );
    }
    if js_iscallable(J, -n - 2 as i32) == 0 {
        let t = js_typeof(J, -n - 2 as i32);
        js_typeerror(
            J,
            b"%s is not callable\0" as *const u8 as *const libc::c_char,
            t,
        );
    }
    obj = js_toobject(J, -n - 2 as i32);
    savebot = (*J).bot;
    (*J).bot = (*J).top - n - 1 as i32;
    if (*obj).type_0 as libc::c_uint == JS_CFUNCTION as i32 as libc::c_uint {
        jsR_pushtrace(
            J,
            (*(*obj).u.f.function).name,
            (*(*obj).u.f.function).filename,
            (*(*obj).u.f.function).line,
        );
        if (*(*obj).u.f.function).lightweight != 0 {
            jsR_calllwfunction(J, n, (*obj).u.f.function, (*obj).u.f.scope);
        } else {
            jsR_callfunction(J, n, (*obj).u.f.function, (*obj).u.f.scope);
        }
        (*J).tracetop -= 1;
        (*J).tracetop;
    } else if (*obj).type_0 as libc::c_uint == JS_CSCRIPT as i32 as libc::c_uint {
        jsR_pushtrace(
            J,
            (*(*obj).u.f.function).name,
            (*(*obj).u.f.function).filename,
            (*(*obj).u.f.function).line,
        );
        jsR_callscript(J, n, (*obj).u.f.function, (*obj).u.f.scope);
        (*J).tracetop -= 1;
        (*J).tracetop;
    } else if (*obj).type_0 as libc::c_uint == JS_CCFUNCTION as i32 as libc::c_uint {
        jsR_pushtrace(
            J,
            (*obj).u.c.name,
            b"native\0" as *const u8 as *const libc::c_char,
            0 as i32,
        );
        jsR_callcfunction(J, n, (*obj).u.c.length, (*obj).u.c.function);
        (*J).tracetop -= 1;
        (*J).tracetop;
    }
    (*J).bot = savebot;
}
#[no_mangle]
pub unsafe extern "C" fn js_construct(J: &mut js_State, mut n: i32) {
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut prototype: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut newobj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    if js_iscallable(J, -n - 1 as i32) == 0 {
        let t = js_typeof(J, -n - 1 as i32);
        js_typeerror(
            J,
            b"%s is not callable\0" as *const u8 as *const libc::c_char,
            t,
        );
    }
    obj = js_toobject(J, -n - 1 as i32);
    if (*obj).type_0 as libc::c_uint == JS_CCFUNCTION as i32 as libc::c_uint
        && ((*obj).u.c.constructor).is_some()
    {
        let mut savebot: i32 = (*J).bot;
        js_pushnull(J);
        if n > 0 as i32 {
            js_rot(J, n + 1 as i32);
        }
        (*J).bot = (*J).top - n - 1 as i32;
        jsR_pushtrace(
            J,
            (*obj).u.c.name,
            b"native\0" as *const u8 as *const libc::c_char,
            0 as i32,
        );
        jsR_callcfunction(J, n, (*obj).u.c.length, (*obj).u.c.constructor);
        (*J).tracetop -= 1;
        (*J).tracetop;
        (*J).bot = savebot;
        return;
    }
    js_getproperty(
        J,
        -n - 1 as i32,
        b"prototype\0" as *const u8 as *const libc::c_char,
    );
    if js_isobject(J, -(1 as i32)) != 0 {
        prototype = js_toobject(J, -(1 as i32));
    } else {
        prototype = (*J).Object_prototype;
    }
    js_pop(J, 1 as i32);
    newobj = jsV_newobject(J, JS_COBJECT, prototype);
    js_pushobject(J, newobj);
    if n > 0 as i32 {
        js_rot(J, n + 1 as i32);
    }
    js_pushobject(J, newobj);
    js_rot(J, n + 3 as i32);
    js_call(J, n);
    if js_isobject(J, -(1 as i32)) == 0 {
        js_pop(J, 1 as i32);
    } else {
        js_rot2pop1(J);
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_eval(J: &mut js_State) {
    if js_isstring(J, -(1 as i32)) == 0 {
        return;
    }
    let src = js_tostring(J, -(1 as i32));
    js_loadeval(J, b"(eval)\0" as *const u8 as *const libc::c_char, src);
    js_rot2pop1(J);
    js_copy(J, 0 as i32);
    js_call(J, 0 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn js_pconstruct(J: &mut js_State, mut n: i32) -> i32 {
    let mut savetop: i32 = (*J).top - n - 2 as i32;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        *((*J).stack).offset(savetop as isize) =
            *((*J).stack).offset(((*J).top - 1 as i32) as isize);
        (*J).top = savetop + 1 as i32;
        return 1 as i32;
    }
    js_construct(J, n);
    js_endtry(J);
    0 as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_pcall(J: &mut js_State, mut n: i32) -> i32 {
    let mut savetop: i32 = (*J).top - n - 2 as i32;
    if _setjmp(js_savetry(J) as *mut __jmp_buf_tag) != 0 {
        *((*J).stack).offset(savetop as isize) =
            *((*J).stack).offset(((*J).top - 1 as i32) as isize);
        (*J).top = savetop + 1 as i32;
        return 1 as i32;
    }
    js_call(J, n);
    js_endtry(J);
    0 as i32
}
#[no_mangle]
pub unsafe extern "C" fn js_savetrypc(
    J: &mut js_State,
    mut pc: *mut js_Instruction,
) -> *mut libc::c_void {
    if (*J).trytop == 64 as i32 {
        js_trystackoverflow(J);
    }
    (*J).trybuf[(*J).trytop as usize].E = (*J).E;
    (*J).trybuf[(*J).trytop as usize].envtop = (*J).envtop;
    (*J).trybuf[(*J).trytop as usize].tracetop = (*J).tracetop;
    (*J).trybuf[(*J).trytop as usize].top = (*J).top;
    (*J).trybuf[(*J).trytop as usize].bot = (*J).bot;
    (*J).trybuf[(*J).trytop as usize].strict = (*J).strict;
    (*J).trybuf[(*J).trytop as usize].pc = pc;
    let fresh69 = (*J).trytop;
    (*J).trytop += 1;
    ((*J).trybuf[fresh69 as usize].buf).as_mut_ptr() as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn js_savetry(J: &mut js_State) -> *mut libc::c_void {
    if (*J).trytop == 64 as i32 {
        js_trystackoverflow(J);
    }
    (*J).trybuf[(*J).trytop as usize].E = (*J).E;
    (*J).trybuf[(*J).trytop as usize].envtop = (*J).envtop;
    (*J).trybuf[(*J).trytop as usize].tracetop = (*J).tracetop;
    (*J).trybuf[(*J).trytop as usize].top = (*J).top;
    (*J).trybuf[(*J).trytop as usize].bot = (*J).bot;
    (*J).trybuf[(*J).trytop as usize].strict = (*J).strict;
    (*J).trybuf[(*J).trytop as usize].pc = core::ptr::null_mut::<js_Instruction>();
    let fresh70 = (*J).trytop;
    (*J).trytop += 1;
    ((*J).trybuf[fresh70 as usize].buf).as_mut_ptr() as *mut libc::c_void
}
#[no_mangle]
pub unsafe extern "C" fn js_endtry(J: &mut js_State) {
    if (*J).trytop == 0 as i32 {
        js_error(
            J,
            b"endtry: exception stack underflow\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).trytop -= 1;
    (*J).trytop;
}
#[no_mangle]
pub unsafe extern "C" fn js_throw(J: &mut js_State) -> ! {
    if (*J).trytop > 0 as i32 {
        let mut v: js_Value = *stackidx(J, -(1 as i32));
        (*J).trytop -= 1;
        (*J).trytop;
        (*J).E = (*J).trybuf[(*J).trytop as usize].E;
        (*J).envtop = (*J).trybuf[(*J).trytop as usize].envtop;
        (*J).tracetop = (*J).trybuf[(*J).trytop as usize].tracetop;
        (*J).top = (*J).trybuf[(*J).trytop as usize].top;
        (*J).bot = (*J).trybuf[(*J).trytop as usize].bot;
        (*J).strict = (*J).trybuf[(*J).trytop as usize].strict;
        js_pushvalue(J, v);
        longjmp(
            ((*J).trybuf[(*J).trytop as usize].buf).as_mut_ptr(),
            1 as i32,
        );
    }
    if ((*J).panic).is_some() {
        ((*J).panic).expect("non-null function pointer")(J);
    }
    abort();
}
unsafe extern "C" fn js_dumpvalue(J: &mut js_State, mut v: js_Value) {
    match v.t.type_0 as i32 {
        1 => {
            printf(b"undefined\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            printf(b"null\0" as *const u8 as *const libc::c_char);
        }
        3 => {
            printf(if v.u.boolean != 0 {
                b"true\0" as *const u8 as *const libc::c_char
            } else {
                b"false\0" as *const u8 as *const libc::c_char
            });
        }
        4 => {
            printf(b"%.9g\0" as *const u8 as *const libc::c_char, v.u.number);
        }
        0 => {
            printf(
                b"'%s'\0" as *const u8 as *const libc::c_char,
                (v.u.shrstr).as_mut_ptr(),
            );
        }
        5 => {
            printf(b"'%s'\0" as *const u8 as *const libc::c_char, v.u.litstr);
        }
        6 => {
            printf(
                b"'%s'\0" as *const u8 as *const libc::c_char,
                ((*v.u.memstr).p).as_mut_ptr(),
            );
        }
        7 => {
            if v.u.object == (*J).G {
                printf(b"[Global]\0" as *const u8 as *const libc::c_char);
            } else {
                match (*v.u.object).type_0 as libc::c_uint {
                    0 => {
                        printf(
                            b"[Object %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    1 => {
                        printf(
                            b"[Array %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    2 => {
                        printf(
                            b"[Function %p, %s, %s:%d]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                            (*(*v.u.object).u.f.function).name,
                            (*(*v.u.object).u.f.function).filename,
                            (*(*v.u.object).u.f.function).line,
                        );
                    }
                    3 => {
                        printf(
                            b"[Script %s]\0" as *const u8 as *const libc::c_char,
                            (*(*v.u.object).u.f.function).filename,
                        );
                    }
                    4 => {
                        printf(
                            b"[CFunction %s]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.c.name,
                        );
                    }
                    6 => {
                        printf(
                            b"[Boolean %d]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.boolean,
                        );
                    }
                    7 => {
                        printf(
                            b"[Number %g]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.number,
                        );
                    }
                    8 => {
                        printf(
                            b"[String'%s']\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.s.string,
                        );
                    }
                    5 => {
                        printf(b"[Error]\0" as *const u8 as *const libc::c_char);
                    }
                    13 => {
                        printf(
                            b"[Arguments %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    14 => {
                        printf(
                            b"[Iterator %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                    15 => {
                        printf(
                            b"[Userdata %s %p]\0" as *const u8 as *const libc::c_char,
                            (*v.u.object).u.user.tag,
                            (*v.u.object).u.user.data,
                        );
                    }
                    _ => {
                        printf(
                            b"[Object %p]\0" as *const u8 as *const libc::c_char,
                            v.u.object as *mut libc::c_void,
                        );
                    }
                }
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn js_stacktrace(J: &mut js_State) {
    let mut n: i32 = 0;
    printf(b"stack trace:\n\0" as *const u8 as *const libc::c_char);
    n = (*J).tracetop;
    while n >= 0 as i32 {
        let mut name: *const libc::c_char = (*J).trace[n as usize].name;
        let mut file: *const libc::c_char = (*J).trace[n as usize].file;
        let mut line: i32 = (*J).trace[n as usize].line;
        if line > 0 as i32 {
            if *name.offset(0 as i32 as isize) != 0 {
                printf(
                    b"\tat %s (%s:%d)\n\0" as *const u8 as *const libc::c_char,
                    name,
                    file,
                    line,
                );
            } else {
                printf(
                    b"\tat %s:%d\n\0" as *const u8 as *const libc::c_char,
                    file,
                    line,
                );
            }
        } else {
            printf(
                b"\tat %s (%s)\n\0" as *const u8 as *const libc::c_char,
                name,
                file,
            );
        }
        n -= 1;
    }
}
unsafe extern "C" fn js_dumpstack(J: &mut js_State) {
    let mut i: i32 = 0;
    printf(b"stack {\n\0" as *const u8 as *const libc::c_char);
    i = 0 as i32;
    while i < (*J).top {
        putchar(if i == (*J).bot {
            '>' as i32
        } else {
            ' ' as i32
        });
        printf(b"%4d: \0" as *const u8 as *const libc::c_char, i);
        js_dumpvalue(J, *((*J).stack).offset(i as isize));
        putchar('\n' as i32);
        i += 1;
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn js_trap(J: &mut js_State, mut pc: i32) {
    js_dumpstack(J);
    js_stacktrace(J);
}
unsafe extern "C" fn jsR_isindex(
    J: &mut js_State,
    mut idx: i32,
    mut k: *mut i32,
) -> i32 {
    let mut v: *mut js_Value = stackidx(J, idx);
    if (*v).t.type_0 as i32 == JS_TNUMBER as i32 {
        *k = (*v).u.number as i32;
        return (*k as f64 == (*v).u.number && *k >= 0 as i32) as i32;
    }
    0 as i32
}
unsafe extern "C" fn jsR_run(J: &mut js_State, mut F: *mut js_Function) {
    let mut FT: *mut *mut js_Function = (*F).funtab;
    let mut VT: *mut *const libc::c_char = ((*F).vartab).offset(-(1 as i32 as isize));
    let mut lightweight: i32 = (*F).lightweight;
    let mut pcstart: *mut js_Instruction = (*F).code;
    let mut pc: *mut js_Instruction = (*F).code;
    let mut opcode: js_OpCode = OP_POP;
    let mut offset: i32 = 0;
    let mut savestrict: i32 = 0;
    let mut str: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let mut ux: libc::c_uint = 0;
    let mut uy: libc::c_uint = 0;
    let mut ix: i32 = 0;
    let mut iy: i32 = 0;
    let mut okay: i32 = 0;
    let mut b: i32 = 0;
    let mut transient: i32 = 0;
    savestrict = (*J).strict;
    (*J).strict = (*F).strict;
    loop {
        if (*J).gccounter > (*J).gcthresh {
            js_gc(J, 0 as i32);
        }
        let fresh71 = pc;
        pc = pc.offset(1);
        (*J).trace[(*J).tracetop as usize].line = *fresh71 as i32;
        let fresh72 = pc;
        pc = pc.offset(1);
        opcode = *fresh72 as js_OpCode;
        match opcode as libc::c_uint {
            0 => {
                js_pop(J, 1 as i32);
            }
            1 => {
                js_dup(J);
            }
            2 => {
                js_dup2(J);
            }
            3 => {
                js_rot2(J);
            }
            4 => {
                js_rot3(J);
            }
            5 => {
                js_rot4(J);
            }
            6 => {
                let fresh73 = pc;
                pc = pc.offset(1);
                js_pushnumber(
                    J,
                    (*fresh73 as i32 - 32768 as i32) as f64,
                );
            }
            7 => {
                memcpy(
                    &mut x as *mut f64 as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<f64>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<f64>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                js_pushnumber(J, x);
            }
            8 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                js_pushliteral(J, str);
            }
            9 => {
                let fresh74 = pc;
                pc = pc.offset(1);
                js_newfunction(J, *FT.offset(*fresh74 as isize), (*J).E);
            }
            11 => {
                js_newobject(J);
            }
            10 => {
                js_newarray(J);
            }
            12 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                let fresh75 = pc;
                pc = pc.offset(1);
                js_newregexp(J, str, *fresh75 as i32);
            }
            13 => {
                js_pushundefined(J);
            }
            14 => {
                js_pushnull(J);
            }
            15 => {
                js_pushboolean(J, 1 as i32);
            }
            16 => {
                js_pushboolean(J, 0 as i32);
            }
            17 => {
                if (*J).strict != 0 {
                    js_copy(J, 0 as i32);
                } else if js_iscoercible(J, 0 as i32) != 0 {
                    js_copy(J, 0 as i32);
                } else {
                    js_pushglobal(J);
                }
            }
            18 => {
                js_currentfunction(J);
            }
            19 => {
                if lightweight != 0 {
                    if (*J).top + 1 as i32 >= 4096 as i32 {
                        js_stackoverflow(J);
                    }
                    let fresh76 = pc;
                    pc = pc.offset(1);
                    let fresh77 = (*J).top;
                    (*J).top += 1;
                    *((*J).stack).offset(fresh77 as isize) =
                        *((*J).stack).offset(((*J).bot + *fresh76 as i32) as isize);
                } else {
                    let fresh78 = pc;
                    pc = pc.offset(1);
                    str = *VT.offset(*fresh78 as isize);
                    if js_hasvar(J, str) == 0 {
                        js_referenceerror(
                            J,
                            b"'%s' is not defined\0" as *const u8 as *const libc::c_char,
                            str,
                        );
                    }
                }
            }
            20 => {
                if lightweight != 0 {
                    let fresh79 = pc;
                    pc = pc.offset(1);
                    *((*J).stack).offset(((*J).bot + *fresh79 as i32) as isize) =
                        *((*J).stack).offset(((*J).top - 1 as i32) as isize);
                } else {
                    let fresh80 = pc;
                    pc = pc.offset(1);
                    js_setvar(J, *VT.offset(*fresh80 as isize));
                }
            }
            21 => {
                if lightweight != 0 {
                    pc = pc.offset(1);
                    js_pushboolean(J, 0 as i32);
                } else {
                    let fresh81 = pc;
                    pc = pc.offset(1);
                    b = js_delvar(J, *VT.offset(*fresh81 as isize));
                    js_pushboolean(J, b);
                }
            }
            23 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                if js_hasvar(J, str) == 0 {
                    js_referenceerror(
                        J,
                        b"'%s' is not defined\0" as *const u8 as *const libc::c_char,
                        str,
                    );
                }
            }
            22 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                if js_hasvar(J, str) == 0 {
                    js_pushundefined(J);
                }
            }
            24 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                js_setvar(J, str);
            }
            25 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                b = js_delvar(J, str);
                js_pushboolean(J, b);
            }
            26 => {
                str = js_tostring(J, -(2 as i32));
                if js_isobject(J, -(1 as i32)) == 0 {
                    js_typeerror(
                        J,
                        b"operand to 'in' is not an object\0" as *const u8 as *const libc::c_char,
                    );
                }
                b = js_hasproperty(J, -(1 as i32), str);
                js_pop(J, 2 as i32 + b);
                js_pushboolean(J, b);
            }
            27 => {
                let len = js_getlength(J, -(1 as i32)) + 1 as i32;
                js_setlength(J, -(1 as i32), len);
            }
            28 => {
                let i = js_getlength(J, -(2 as i32));
                js_setindex(J, -(2 as i32), i);
            }
            29 => {
                obj = js_toobject(J, -(3 as i32));
                str = js_tostring(J, -(2 as i32));
                jsR_setproperty(J, obj, str, 0 as i32);
                js_pop(J, 2 as i32);
            }
            30 => {
                obj = js_toobject(J, -(3 as i32));
                str = js_tostring(J, -(2 as i32));
                let getter = jsR_tofunction(J, -(1 as i32));
                jsR_defproperty(
                    J,
                    obj,
                    str,
                    0 as i32,
                    core::ptr::null_mut::<js_Value>(),
                    getter,
                    core::ptr::null_mut::<js_Object>(),
                    0 as i32,
                );
                js_pop(J, 2 as i32);
            }
            31 => {
                obj = js_toobject(J, -(3 as i32));
                str = js_tostring(J, -(2 as i32));
                let getter = jsR_tofunction(J, -(1 as i32));
                jsR_defproperty(
                    J,
                    obj,
                    str,
                    0 as i32,
                    core::ptr::null_mut::<js_Value>(),
                    core::ptr::null_mut::<js_Object>(),
                    getter,
                    0 as i32,
                );
                js_pop(J, 2 as i32);
            }
            32 => {
                if jsR_isindex(J, -(1 as i32), &mut ix) != 0 {
                    obj = js_toobject(J, -(2 as i32));
                    jsR_getindex(J, obj, ix);
                } else {
                    str = js_tostring(J, -(1 as i32));
                    obj = js_toobject(J, -(2 as i32));
                    jsR_getproperty(J, obj, str);
                }
                js_rot3pop2(J);
            }
            33 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                obj = js_toobject(J, -(1 as i32));
                jsR_getproperty(J, obj, str);
                js_rot2pop1(J);
            }
            34 => {
                if jsR_isindex(J, -(2 as i32), &mut ix) != 0 {
                    obj = js_toobject(J, -(3 as i32));
                    transient = (js_isobject(J, -(3 as i32)) == 0) as i32;
                    jsR_setindex(J, obj, ix, transient);
                } else {
                    str = js_tostring(J, -(2 as i32));
                    obj = js_toobject(J, -(3 as i32));
                    transient = (js_isobject(J, -(3 as i32)) == 0) as i32;
                    jsR_setproperty(J, obj, str, transient);
                }
                js_rot3pop2(J);
            }
            35 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                obj = js_toobject(J, -(2 as i32));
                transient = (js_isobject(J, -(2 as i32)) == 0) as i32;
                jsR_setproperty(J, obj, str, transient);
                js_rot2pop1(J);
            }
            36 => {
                str = js_tostring(J, -(1 as i32));
                obj = js_toobject(J, -(2 as i32));
                b = jsR_delproperty(J, obj, str);
                js_pop(J, 2 as i32);
                js_pushboolean(J, b);
            }
            37 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                obj = js_toobject(J, -(1 as i32));
                b = jsR_delproperty(J, obj, str);
                js_pop(J, 1 as i32);
                js_pushboolean(J, b);
            }
            38 => {
                if js_iscoercible(J, -(1 as i32)) != 0 {
                    let o = js_toobject(J, -(1 as i32));
                    obj = jsV_newiterator(J, o, 0 as i32);
                    js_pop(J, 1 as i32);
                    js_pushobject(J, obj);
                }
            }
            39 => {
                if js_isobject(J, -(1 as i32)) != 0 {
                    obj = js_toobject(J, -(1 as i32));
                    str = jsV_nextiterator(J, obj);
                    if !str.is_null() {
                        js_pushstring(J, str);
                        js_pushboolean(J, 1 as i32);
                    } else {
                        js_pop(J, 1 as i32);
                        js_pushboolean(J, 0 as i32);
                    }
                } else {
                    js_pop(J, 1 as i32);
                    js_pushboolean(J, 0 as i32);
                }
            }
            40 => {
                js_eval(J);
            }
            41 => {
                let fresh82 = pc;
                pc = pc.offset(1);
                js_call(J, *fresh82 as i32);
            }
            42 => {
                let fresh83 = pc;
                pc = pc.offset(1);
                js_construct(J, *fresh83 as i32);
            }
            43 => {
                str = js_typeof(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushliteral(J, str);
            }
            44 => {
                x = js_tonumber(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushnumber(J, x);
            }
            45 => {
                x = js_tonumber(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushnumber(J, -x);
            }
            46 => {
                ix = js_toint32(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushnumber(J, !ix as f64);
            }
            47 => {
                b = js_toboolean(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushboolean(J, (b == 0) as i32);
            }
            48 => {
                x = js_tonumber(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushnumber(J, x + 1 as i32 as f64);
            }
            49 => {
                x = js_tonumber(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushnumber(J, x - 1 as i32 as f64);
            }
            50 => {
                x = js_tonumber(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushnumber(J, x + 1 as i32 as f64);
                js_pushnumber(J, x);
            }
            51 => {
                x = js_tonumber(J, -(1 as i32));
                js_pop(J, 1 as i32);
                js_pushnumber(J, x - 1 as i32 as f64);
                js_pushnumber(J, x);
            }
            52 => {
                x = js_tonumber(J, -(2 as i32));
                y = js_tonumber(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(J, x * y);
            }
            53 => {
                x = js_tonumber(J, -(2 as i32));
                y = js_tonumber(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(J, x / y);
            }
            54 => {
                x = js_tonumber(J, -(2 as i32));
                y = js_tonumber(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(J, fmod(x, y));
            }
            55 => {
                js_concat(J);
            }
            56 => {
                x = js_tonumber(J, -(2 as i32));
                y = js_tonumber(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(J, x - y);
            }
            57 => {
                ix = js_toint32(J, -(2 as i32));
                uy = js_touint32(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(
                    J,
                    (ix << (uy & 0x1f as i32 as libc::c_uint)) as f64,
                );
            }
            58 => {
                ix = js_toint32(J, -(2 as i32));
                uy = js_touint32(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(
                    J,
                    (ix >> (uy & 0x1f as i32 as libc::c_uint)) as f64,
                );
            }
            59 => {
                ux = js_touint32(J, -(2 as i32));
                uy = js_touint32(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(
                    J,
                    (ux >> (uy & 0x1f as i32 as libc::c_uint)) as f64,
                );
            }
            60 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as i32);
                js_pushboolean(J, (okay != 0 && b < 0 as i32) as i32);
            }
            61 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as i32);
                js_pushboolean(J, (okay != 0 && b > 0 as i32) as i32);
            }
            62 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as i32);
                js_pushboolean(J, (okay != 0 && b <= 0 as i32) as i32);
            }
            63 => {
                b = js_compare(J, &mut okay);
                js_pop(J, 2 as i32);
                js_pushboolean(J, (okay != 0 && b >= 0 as i32) as i32);
            }
            72 => {
                b = js_instanceof(J);
                js_pop(J, 2 as i32);
                js_pushboolean(J, b);
            }
            64 => {
                b = js_equal(J);
                js_pop(J, 2 as i32);
                js_pushboolean(J, b);
            }
            65 => {
                b = js_equal(J);
                js_pop(J, 2 as i32);
                js_pushboolean(J, (b == 0) as i32);
            }
            66 => {
                b = js_strictequal(J);
                js_pop(J, 2 as i32);
                js_pushboolean(J, b);
            }
            67 => {
                b = js_strictequal(J);
                js_pop(J, 2 as i32);
                js_pushboolean(J, (b == 0) as i32);
            }
            68 => {
                let fresh84 = pc;
                pc = pc.offset(1);
                offset = *fresh84 as i32;
                b = js_strictequal(J);
                if b != 0 {
                    js_pop(J, 2 as i32);
                    pc = pcstart.offset(offset as isize);
                } else {
                    js_pop(J, 1 as i32);
                }
            }
            69 => {
                ix = js_toint32(J, -(2 as i32));
                iy = js_toint32(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(J, (ix & iy) as f64);
            }
            70 => {
                ix = js_toint32(J, -(2 as i32));
                iy = js_toint32(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(J, (ix ^ iy) as f64);
            }
            71 => {
                ix = js_toint32(J, -(2 as i32));
                iy = js_toint32(J, -(1 as i32));
                js_pop(J, 2 as i32);
                js_pushnumber(J, (ix | iy) as f64);
            }
            73 => {
                js_throw(J);
            }
            74 => {
                let fresh85 = pc;
                pc = pc.offset(1);
                offset = *fresh85 as i32;
                if _setjmp(js_savetrypc(J, pc) as *mut __jmp_buf_tag) != 0 {
                    pc = (*J).trybuf[(*J).trytop as usize].pc;
                } else {
                    pc = pcstart.offset(offset as isize);
                }
            }
            75 => {
                js_endtry(J);
            }
            76 => {
                memcpy(
                    &mut str as *mut *const libc::c_char as *mut libc::c_void,
                    pc as *const libc::c_void,
                    ::core::mem::size_of::<*const libc::c_char>() as u32,
                );
                pc = pc.offset(
                    (::core::mem::size_of::<*const libc::c_char>() as u32)
                        .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
                        as isize,
                );
                obj = jsV_newobject(J, JS_COBJECT, core::ptr::null_mut::<js_Object>());
                js_pushobject(J, obj);
                js_rot2(J);
                js_setproperty(J, -(2 as i32), str);
                (*J).E = jsR_newenvironment(J, obj, (*J).E);
                js_pop(J, 1 as i32);
            }
            77 => {
                (*J).E = (*(*J).E).outer;
            }
            78 => {
                obj = js_toobject(J, -(1 as i32));
                (*J).E = jsR_newenvironment(J, obj, (*J).E);
                js_pop(J, 1 as i32);
            }
            79 => {
                (*J).E = (*(*J).E).outer;
            }
            80 => {
                js_trap(
                    J,
                    pc.offset_from(pcstart) as libc::c_long as i32 - 1 as i32,
                );
            }
            81 => {
                pc = pcstart.offset(*pc as i32 as isize);
            }
            82 => {
                let fresh86 = pc;
                pc = pc.offset(1);
                offset = *fresh86 as i32;
                b = js_toboolean(J, -(1 as i32));
                js_pop(J, 1 as i32);
                if b != 0 {
                    pc = pcstart.offset(offset as isize);
                }
            }
            83 => {
                let fresh87 = pc;
                pc = pc.offset(1);
                offset = *fresh87 as i32;
                b = js_toboolean(J, -(1 as i32));
                js_pop(J, 1 as i32);
                if b == 0 {
                    pc = pcstart.offset(offset as isize);
                }
            }
            84 => {
                (*J).strict = savestrict;
                return;
            }
            _ => {}
        }
    }
}
