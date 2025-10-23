use crate::*;

#[no_mangle]
pub unsafe extern "C" fn js_putc(J: &mut js_State, mut sbp: *mut *mut js_Buffer, mut c: i32) {
    let mut sb: *mut js_Buffer = *sbp;
    if sb.is_null() {
        sb = Box::into_raw(Box::new(js_Buffer {
            s: CompactString::with_capacity(64),
        }));
        *sbp = sb;
    }
    (*sb).s.push(c as u8 as char);
}

#[no_mangle]
pub unsafe extern "C" fn js_puts(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut s: *const libc::c_char,
) {
    while *s != 0 {
        let fresh27 = s;
        s = s.offset(1);
        js_putc(J, sb, *fresh27 as i32);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_putm(
    J: &mut js_State,
    mut sb: *mut *mut js_Buffer,
    mut s: *const libc::c_char,
    mut e: *const libc::c_char,
) {
    while s < e {
        let fresh28 = s;
        s = s.offset(1);
        js_putc(J, sb, *fresh28 as i32);
    }
}
static mut jsS_sentinel: js_StringNode = js_StringNode {
    left: &raw mut jsS_sentinel,
    right: &raw mut jsS_sentinel,
    level: 0,
    string: [b'0' as i8],
};
unsafe extern "C" fn jsS_newstringnode(
    J: &mut js_State,
    mut string: *const libc::c_char,
    mut result: *mut *const libc::c_char,
) -> *mut js_StringNode {
    let mut n: size_t = strlen(string);
    if n > ((1 as i32) << 28 as i32) as u32 {
        js_rangeerror(
            J,
            b"invalid string length\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut node: *mut js_StringNode = js_malloc(
        J,
        (20 as u32 as i32 as u32)
            .wrapping_add(n)
            .wrapping_add(1 as i32 as u32) as i32,
    ) as *mut js_StringNode;
    (*node).right = &mut jsS_sentinel;
    (*node).left = (*node).right;
    (*node).level = 1;
    memcpy(
        ((*node).string).as_mut_ptr() as *mut libc::c_void,
        string as *const libc::c_void,
        n.wrapping_add(1 as i32 as u32),
    );
    *result = ((*node).string).as_mut_ptr();
    node
}
unsafe extern "C" fn jsS_skew(mut node: *mut js_StringNode) -> *mut js_StringNode {
    if (*(*node).left).level == (*node).level {
        let mut temp: *mut js_StringNode = node;
        node = (*node).left;
        (*temp).left = (*node).right;
        (*node).right = temp;
    }
    node
}
unsafe extern "C" fn jsS_split(mut node: *mut js_StringNode) -> *mut js_StringNode {
    if (*(*(*node).right).right).level == (*node).level {
        let mut temp: *mut js_StringNode = node;
        node = (*node).right;
        (*temp).right = (*node).left;
        (*node).left = temp;
        (*node).level += 1;
        (*node).level;
    }
    node
}
unsafe extern "C" fn jsS_insert(
    J: &mut js_State,
    mut node: *mut js_StringNode,
    mut string: *const libc::c_char,
    mut result: *mut *const libc::c_char,
) -> *mut js_StringNode {
    if node != &mut jsS_sentinel as *mut js_StringNode {
        let mut c: i32 = strcmp(string, ((*node).string).as_mut_ptr());
        if c < 0 {
            (*node).left = jsS_insert(J, (*node).left, string, result);
        } else if c > 0 {
            (*node).right = jsS_insert(J, (*node).right, string, result);
        } else {
            *result = ((*node).string).as_mut_ptr();
            return node;
        }
        node = jsS_skew(node);
        node = jsS_split(node);
        return node;
    }
    jsS_newstringnode(J, string, result)
}
unsafe extern "C" fn dumpstringnode(mut node: *mut js_StringNode, mut level: i32) {
    let mut i: i32 = 0;
    if (*node).left != &mut jsS_sentinel as *mut js_StringNode {
        dumpstringnode((*node).left, level + 1);
    }
    printf(b"%d: \0" as *const u8 as *const libc::c_char, (*node).level);
    i = 0;
    while i < level {
        putchar('\t' as i32);
        i += 1;
    }
    printf(
        b"'%s'\n\0" as *const u8 as *const libc::c_char,
        ((*node).string).as_mut_ptr(),
    );
    if (*node).right != &mut jsS_sentinel as *mut js_StringNode {
        dumpstringnode((*node).right, level + 1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsS_dumpstrings(J: &mut js_State) {
    let mut root: *mut js_StringNode = (*J).strings;
    printf(b"interned strings {\n\0" as *const u8 as *const libc::c_char);
    if !root.is_null() && root != &mut jsS_sentinel as *mut js_StringNode {
        dumpstringnode(root, 1);
    }
    printf(b"}\n\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn jsS_freestringnode(J: &mut js_State, mut node: *mut js_StringNode) {
    if (*node).left != &mut jsS_sentinel as *mut js_StringNode {
        jsS_freestringnode(J, (*node).left);
    }
    if (*node).right != &mut jsS_sentinel as *mut js_StringNode {
        jsS_freestringnode(J, (*node).right);
    }
    js_free(J, node as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn jsS_freestrings(J: &mut js_State) {
    if !((*J).strings).is_null() && (*J).strings != &mut jsS_sentinel as *mut js_StringNode {
        jsS_freestringnode(J, (*J).strings);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_intern(
    J: &mut js_State,
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    let mut result: *const libc::c_char = core::ptr::null::<libc::c_char>();
    if ((*J).strings).is_null() {
        (*J).strings = &mut jsS_sentinel;
    }
    (*J).strings = jsS_insert(J, (*J).strings, s, &mut result);
    result
}
