use crate::*;
use core::ptr;
static mut sentinel: js_Property = js_Property {
    left: &raw mut sentinel,
    right: &raw mut sentinel,
    level: 0,
    atts: 0,
    value: js_Value {
        t: {
            C2RustUnnamed_6 {
                pad: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                type_0: JS_TUNDEFINED as i32 as libc::c_char,
            }
        },
    },
    getter: ptr::null_mut(),
    setter: ptr::null_mut(),
    name: [b'0' as i8],
};
unsafe extern "C" fn newproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) -> *mut js_Property {
    let mut n: i32 = (strlen(name)).wrapping_add(1 as i32 as u32) as i32;
    let mut node: *mut js_Property =
        js_malloc(J, (56 as u32).wrapping_add(n as u32) as i32) as *mut js_Property;
    (*node).right = &mut sentinel;
    (*node).left = (*node).right;
    (*node).level = 1;
    (*node).atts = 0;
    (*node).value.t.type_0 = JS_TUNDEFINED as i32 as libc::c_char;
    (*node).value.u.number = 0.0;
    (*node).getter = core::ptr::null_mut::<js_Object>();
    (*node).setter = core::ptr::null_mut::<js_Object>();
    memcpy(
        ((*node).name).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        n as u32,
    );
    (*obj).count += 1;
    (*obj).count;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    node
}
unsafe extern "C" fn lookup(
    mut node: *mut js_Property,
    mut name: *const libc::c_char,
) -> *mut js_Property {
    while node != &mut sentinel as *mut js_Property {
        let mut c: i32 = strcmp(name, ((*node).name).as_mut_ptr());
        if c == 0 {
            return node;
        } else if c < 0 {
            node = (*node).left;
        } else {
            node = (*node).right;
        }
    }
    core::ptr::null_mut::<js_Property>()
}
unsafe extern "C" fn skew(mut node: *mut js_Property) -> *mut js_Property {
    if (*(*node).left).level == (*node).level {
        let mut temp: *mut js_Property = node;
        node = (*node).left;
        (*temp).left = (*node).right;
        (*node).right = temp;
    }
    node
}
unsafe extern "C" fn split(mut node: *mut js_Property) -> *mut js_Property {
    if (*(*(*node).right).right).level == (*node).level {
        let mut temp: *mut js_Property = node;
        node = (*node).right;
        (*temp).right = (*node).left;
        (*node).left = temp;
        (*node).level += 1;
        (*node).level;
    }
    node
}
unsafe extern "C" fn insert(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut node: *mut js_Property,
    mut name: *const libc::c_char,
    mut result: *mut *mut js_Property,
) -> *mut js_Property {
    if node != &mut sentinel as *mut js_Property {
        let mut c: i32 = strcmp(name, ((*node).name).as_mut_ptr());
        if c < 0 {
            (*node).left = insert(J, obj, (*node).left, name, result);
        } else if c > 0 {
            (*node).right = insert(J, obj, (*node).right, name, result);
        } else {
            *result = node;
            return *result;
        }
        node = skew(node);
        node = split(node);
        return node;
    }
    *result = newproperty(J, obj, name);
    *result
}
unsafe extern "C" fn freeproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut node: *mut js_Property,
) {
    js_free(J, node as *mut libc::c_void);
    (*obj).count -= 1;
    (*obj).count;
}
unsafe extern "C" fn unlinkproperty(
    mut node: *mut js_Property,
    mut name: *const libc::c_char,
    mut garbage: *mut *mut js_Property,
) -> *mut js_Property {
    let mut temp: *mut js_Property = core::ptr::null_mut::<js_Property>();
    let mut a: *mut js_Property = core::ptr::null_mut::<js_Property>();
    let mut b: *mut js_Property = core::ptr::null_mut::<js_Property>();
    if node != &mut sentinel as *mut js_Property {
        let mut c: i32 = strcmp(name, ((*node).name).as_mut_ptr());
        if c < 0 {
            (*node).left = unlinkproperty((*node).left, name, garbage);
        } else if c > 0 {
            (*node).right = unlinkproperty((*node).right, name, garbage);
        } else {
            *garbage = node;
            if (*node).left == &mut sentinel as *mut js_Property
                && (*node).right == &mut sentinel as *mut js_Property
            {
                return &mut sentinel;
            } else if (*node).left == &mut sentinel as *mut js_Property {
                a = (*node).right;
                while (*a).left != &mut sentinel as *mut js_Property {
                    a = (*a).left;
                }
                b = unlinkproperty((*node).right, ((*a).name).as_mut_ptr(), &mut temp);
                (*temp).level = (*node).level;
                (*temp).left = (*node).left;
                (*temp).right = b;
                node = temp;
            } else {
                a = (*node).left;
                while (*a).right != &mut sentinel as *mut js_Property {
                    a = (*a).right;
                }
                b = unlinkproperty((*node).left, ((*a).name).as_mut_ptr(), &mut temp);
                (*temp).level = (*node).level;
                (*temp).left = b;
                (*temp).right = (*node).right;
                node = temp;
            }
        }
        if (*(*node).left).level < (*node).level - 1 || (*(*node).right).level < (*node).level - 1 {
            (*node).level -= 1;
            if (*(*node).right).level > (*node).level {
                (*(*node).right).level = (*node).level;
            }
            node = skew(node);
            (*node).right = skew((*node).right);
            (*(*node).right).right = skew((*(*node).right).right);
            node = split(node);
            (*node).right = split((*node).right);
        }
    }
    node
}
unsafe extern "C" fn deleteproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut tree: *mut js_Property,
    mut name: *const libc::c_char,
) -> *mut js_Property {
    let mut garbage: *mut js_Property = &mut sentinel;
    tree = unlinkproperty(tree, name, &mut garbage);
    if garbage != &mut sentinel as *mut js_Property {
        freeproperty(J, obj, garbage);
    }
    tree
}
#[no_mangle]
pub unsafe extern "C" fn jsV_newobject(
    J: &mut js_State,
    mut type_0: js_Class,
    mut prototype: *mut js_Object,
) -> *mut js_Object {
    let mut obj: *mut js_Object =
        js_malloc(J, ::core::mem::size_of::<js_Object>() as u32 as i32) as *mut js_Object;
    memset(
        obj as *mut libc::c_void,
        0,
        ::core::mem::size_of::<js_Object>() as u32,
    );
    (*obj).gcmark = 0;
    (*obj).gcnext = (*J).gcobj;
    (*J).gcobj = obj;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    (*obj).type_0 = type_0;
    (*obj).properties = &mut sentinel;
    (*obj).prototype = prototype;
    (*obj).extensible = 1;
    obj
}
#[no_mangle]
pub unsafe extern "C" fn jsV_getownproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) -> *mut js_Property {
    lookup((*obj).properties, name)
}
#[no_mangle]
pub unsafe extern "C" fn jsV_getpropertyx(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
    mut own: *mut i32,
) -> *mut js_Property {
    *own = 1;
    loop {
        let mut ref_0: *mut js_Property = lookup((*obj).properties, name);
        if !ref_0.is_null() {
            return ref_0;
        }
        obj = (*obj).prototype;
        *own = 0;
        if obj.is_null() {
            break;
        }
    }
    core::ptr::null_mut::<js_Property>()
}
#[no_mangle]
pub unsafe extern "C" fn jsV_getproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) -> *mut js_Property {
    loop {
        let mut ref_0: *mut js_Property = lookup((*obj).properties, name);
        if !ref_0.is_null() {
            return ref_0;
        }
        obj = (*obj).prototype;
        if obj.is_null() {
            break;
        }
    }
    core::ptr::null_mut::<js_Property>()
}
unsafe extern "C" fn jsV_getenumproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) -> *mut js_Property {
    loop {
        let mut ref_0: *mut js_Property = lookup((*obj).properties, name);
        if !ref_0.is_null() && (*ref_0).atts & JS_DONTENUM as i32 == 0 {
            return ref_0;
        }
        obj = (*obj).prototype;
        if obj.is_null() {
            break;
        }
    }
    core::ptr::null_mut::<js_Property>()
}
#[no_mangle]
pub unsafe extern "C" fn jsV_setproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) -> *mut js_Property {
    let mut result: *mut js_Property = core::ptr::null_mut::<js_Property>();
    if (*obj).extensible == 0 {
        result = lookup((*obj).properties, name);
        if (*J).strict != 0 && result.is_null() {
            js_typeerror(
                J,
                b"object is non-extensible\0" as *const u8 as *const libc::c_char,
            );
        }
        return result;
    }
    (*obj).properties = insert(J, obj, (*obj).properties, name, &mut result);
    result
}
#[no_mangle]
pub unsafe extern "C" fn jsV_delproperty(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut name: *const libc::c_char,
) {
    (*obj).properties = deleteproperty(J, obj, (*obj).properties, name);
}
unsafe extern "C" fn itnewnode(
    J: &mut js_State,
    mut name: *const libc::c_char,
    mut next_0: *mut js_Iterator,
) -> *mut js_Iterator {
    let mut n: i32 = (strlen(name)).wrapping_add(1 as i32 as u32) as i32;
    let mut node: *mut js_Iterator =
        js_malloc(J, (8 as u32).wrapping_add(n as u32) as i32) as *mut js_Iterator;
    (*node).next = next_0;
    memcpy(
        ((*node).name).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        n as u32,
    );
    node
}
unsafe extern "C" fn itwalk(
    J: &mut js_State,
    mut iter: *mut js_Iterator,
    mut prop: *mut js_Property,
    mut seen: *mut js_Object,
) -> *mut js_Iterator {
    if (*prop).right != &raw mut sentinel {
        iter = itwalk(J, iter, (*prop).right, seen);
    }
    if (*prop).atts & JS_DONTENUM as i32 == 0
        && (seen.is_null() || (jsV_getenumproperty(J, seen, ((*prop).name).as_mut_ptr())).is_null())
    {
        iter = itnewnode(J, ((*prop).name).as_mut_ptr(), iter);
    }
    if (*prop).left != &raw mut sentinel {
        iter = itwalk(J, iter, (*prop).left, seen);
    }
    iter
}
unsafe extern "C" fn itflatten(J: &mut js_State, mut obj: *mut js_Object) -> *mut js_Iterator {
    let mut iter: *mut js_Iterator = core::ptr::null_mut::<js_Iterator>();
    if !((*obj).prototype).is_null() {
        iter = itflatten(J, (*obj).prototype);
    }
    if (*obj).properties != &mut sentinel as *mut js_Property {
        iter = itwalk(J, iter, (*obj).properties, (*obj).prototype);
    }
    iter
}
#[no_mangle]
pub unsafe extern "C" fn jsV_newiterator(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut own: i32,
) -> *mut js_Object {
    let mut io: *mut js_Object = jsV_newobject(J, JS_CITERATOR, core::ptr::null_mut::<js_Object>());
    (*io).u.iter.target = obj;
    (*io).u.iter.i = 0;
    (*io).u.iter.n = 0;
    if own != 0 {
        (*io).u.iter.head = core::ptr::null_mut::<js_Iterator>();
        if (*obj).properties != &mut sentinel as *mut js_Property {
            (*io).u.iter.head = itwalk(
                J,
                (*io).u.iter.head,
                (*obj).properties,
                core::ptr::null_mut::<js_Object>(),
            );
        }
    } else {
        (*io).u.iter.head = itflatten(J, obj);
    }
    (*io).u.iter.current = (*io).u.iter.head;
    if (*obj).type_0 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint {
        (*io).u.iter.n = (*obj).u.s.length;
    }
    if (*obj).type_0 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint && (*obj).u.a.simple != 0 {
        (*io).u.iter.n = (*obj).u.a.flat_length;
    }
    io
}
#[no_mangle]
pub unsafe extern "C" fn jsV_nextiterator(
    J: &mut js_State,
    mut io: *mut js_Object,
) -> *const libc::c_char {
    if (*io).type_0 as libc::c_uint != JS_CITERATOR as i32 as libc::c_uint {
        js_typeerror(J, b"not an iterator\0" as *const u8 as *const libc::c_char);
    }
    if (*io).u.iter.i < (*io).u.iter.n {
        js_itoa(((*J).scratch).as_mut_ptr(), (*io).u.iter.i);
        (*io).u.iter.i += 1;
        (*io).u.iter.i;
        return ((*J).scratch).as_mut_ptr();
    }
    while !((*io).u.iter.current).is_null() {
        let mut name: *const libc::c_char = ((*(*io).u.iter.current).name).as_mut_ptr();
        (*io).u.iter.current = (*(*io).u.iter.current).next;
        if !(jsV_getproperty(J, (*io).u.iter.target, name)).is_null() {
            return name;
        }
    }
    core::ptr::null::<libc::c_char>()
}
#[no_mangle]
pub unsafe extern "C" fn jsV_resizearray(
    J: &mut js_State,
    mut obj: *mut js_Object,
    mut newlen: i32,
) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut k: i32 = 0;
    if (*obj).u.a.simple == 0 {
    } else {
        __assert_fail(
            b"!obj->u.a.simple\0" as *const u8 as *const libc::c_char,
            b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
            9836 as i32 as libc::c_uint,
            (*::core::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                b"void jsV_resizearray(js_State *, js_Object *, int)\0",
            ))
            .as_ptr(),
        );
    }
    {
        if (*obj).u.a.simple == 0 {
        } else {
            __assert_fail(
                b"!obj->u.a.simple\0" as *const u8 as *const libc::c_char,
                b"/root/mujs-all/mujs_all.c\0" as *const u8 as *const libc::c_char,
                9836 as i32 as libc::c_uint,
                (*::core::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"void jsV_resizearray(js_State *, js_Object *, int)\0",
                ))
                .as_ptr(),
            );
        }
    };
    if newlen < (*obj).u.a.length {
        if (*obj).u.a.length > (*obj).count * 2 {
            let mut it: *mut js_Object = jsV_newiterator(J, obj, 1);
            loop {
                s = jsV_nextiterator(J, it);
                if s.is_null() {
                    break;
                }
                k = jsV_numbertointeger(jsV_stringtonumber(J, s));
                if k >= newlen && strcmp(s, jsV_numbertostring(buf.as_mut_ptr(), k as f64)) == 0 {
                    jsV_delproperty(J, obj, s);
                }
            }
        } else {
            k = newlen;
            while k < (*obj).u.a.length {
                jsV_delproperty(J, obj, js_itoa(buf.as_mut_ptr(), k));
                k += 1;
            }
        }
    }
    (*obj).u.a.length = newlen;
}
