use crate::*;

unsafe extern "C" fn jsG_freeenvironment(J: &mut js_State, mut env: *mut js_Environment) {
    js_free(J, env as *mut libc::c_void);
}
unsafe extern "C" fn jsG_freefunction(J: &mut js_State, mut fun: *mut js_Function) {
    js_free(J, (*fun).funtab as *mut libc::c_void);
    js_free(J, (*fun).vartab as *mut libc::c_void);
    js_free(J, (*fun).code as *mut libc::c_void);
    js_free(J, fun as *mut libc::c_void);
}
unsafe extern "C" fn jsG_freeproperty(J: &mut js_State, mut node: *mut js_Property) {
    if (*(*node).left).level != 0 {
        jsG_freeproperty(J, (*node).left);
    }
    if (*(*node).right).level != 0 {
        jsG_freeproperty(J, (*node).right);
    }
    js_free(J, node as *mut libc::c_void);
}
unsafe extern "C" fn jsG_freeiterator(J: &mut js_State, mut node: *mut js_Iterator) {
    while !node.is_null() {
        let mut next_0: *mut js_Iterator = (*node).next;
        js_free(J, node as *mut libc::c_void);
        node = next_0;
    }
}
unsafe extern "C" fn jsG_freeobject(J: &mut js_State, mut obj: *mut js_Object) {
    if (*(*obj).properties).level != 0 {
        jsG_freeproperty(J, (*obj).properties);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CREGEXP as i32 as libc::c_uint {
        js_free(J, (*obj).u.r.source as *mut libc::c_void);
        js_regfreex((*J).alloc, (*J).actx, (*obj).u.r.prog as *mut Reprog);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CSTRING as i32 as libc::c_uint
        && (*obj).u.s.string != ((*obj).u.s.shrstr).as_mut_ptr()
    {
        js_free(J, (*obj).u.s.string as *mut libc::c_void);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        js_free(J, (*obj).u.a.array as *mut libc::c_void);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CITERATOR as i32 as libc::c_uint {
        jsG_freeiterator(J, (*obj).u.iter.head);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CUSERDATA as i32 as libc::c_uint
        && ((*obj).u.user.finalize).is_some()
    {
        ((*obj).u.user.finalize).expect("non-null function pointer")(J, (*obj).u.user.data);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CCFUNCTION as i32 as libc::c_uint
        && ((*obj).u.c.finalize).is_some()
    {
        ((*obj).u.c.finalize).expect("non-null function pointer")(J, (*obj).u.c.data);
    }
    js_free(J, obj as *mut libc::c_void);
}
unsafe extern "C" fn jsG_markobject(J: &mut js_State, mut mark: i32, mut obj: *mut js_Object) {
    (*obj).gcmark = mark;
    (*obj).gcroot = (*J).gcroot;
    (*J).gcroot = obj;
}
unsafe extern "C" fn jsG_markfunction(J: &mut js_State, mut mark: i32, mut fun: *mut js_Function) {
    let mut i: i32 = 0;
    (*fun).gcmark = mark;
    i = 0;
    while i < (*fun).funlen {
        if (**((*fun).funtab).offset(i as isize)).gcmark != mark {
            jsG_markfunction(J, mark, *((*fun).funtab).offset(i as isize));
        }
        i += 1;
    }
}
unsafe extern "C" fn jsG_markenvironment(
    J: &mut js_State,
    mut mark: i32,
    mut env: *mut js_Environment,
) {
    loop {
        (*env).gcmark = mark;
        if (*(*env).variables).gcmark != mark {
            jsG_markobject(J, mark, (*env).variables);
        }
        env = (*env).outer;
        if !(!env.is_null() && (*env).gcmark != mark) {
            break;
        }
    }
}
unsafe extern "C" fn jsG_markproperty(J: &mut js_State, mut mark: i32, mut node: *mut js_Property) {
    if (*(*node).left).level != 0 {
        jsG_markproperty(J, mark, (*node).left);
    }
    if (*(*node).right).level != 0 {
        jsG_markproperty(J, mark, (*node).right);
    }
    if (*node).value.t.type_0 as i32 == JS_TMEMSTR as i32
        && (*(*node).value.u.memstr).gcmark as i32 != mark
    {
        (*(*node).value.u.memstr).gcmark = mark as libc::c_char;
    }
    if (*node).value.t.type_0 as i32 == JS_TOBJECT as i32
        && (*(*node).value.u.object).gcmark != mark
    {
        jsG_markobject(J, mark, (*node).value.u.object);
    }
    if !((*node).getter).is_null() && (*(*node).getter).gcmark != mark {
        jsG_markobject(J, mark, (*node).getter);
    }
    if !((*node).setter).is_null() && (*(*node).setter).gcmark != mark {
        jsG_markobject(J, mark, (*node).setter);
    }
}
unsafe extern "C" fn jsG_scanobject(J: &mut js_State, mut mark: i32, mut obj: *mut js_Object) {
    if (*(*obj).properties).level != 0 {
        jsG_markproperty(J, mark, (*obj).properties);
    }
    if !((*obj).prototype).is_null() && (*(*obj).prototype).gcmark != mark {
        jsG_markobject(J, mark, (*obj).prototype);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CARRAY as i32 as libc::c_uint
        && (*obj).u.a.simple != 0
    {
        let mut i: i32 = 0;
        i = 0;
        while i < (*obj).u.a.flat_length {
            let mut v: *mut js_Value = &mut *((*obj).u.a.array).offset(i as isize) as *mut js_Value;
            if (*v).t.type_0 as i32 == JS_TMEMSTR as i32 && (*(*v).u.memstr).gcmark as i32 != mark {
                (*(*v).u.memstr).gcmark = mark as libc::c_char;
            }
            if (*v).t.type_0 as i32 == JS_TOBJECT as i32 && (*(*v).u.object).gcmark != mark {
                jsG_markobject(J, mark, (*v).u.object);
            }
            i += 1;
        }
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CITERATOR as i32 as libc::c_uint
        && (*(*obj).u.iter.target).gcmark != mark
    {
        jsG_markobject(J, mark, (*obj).u.iter.target);
    }
    if (*obj).type_0 as i32 as libc::c_uint == JS_CFUNCTION as i32 as libc::c_uint
        || (*obj).type_0 as i32 as libc::c_uint == JS_CSCRIPT as i32 as libc::c_uint
    {
        if !((*obj).u.f.scope).is_null() && (*(*obj).u.f.scope).gcmark != mark {
            jsG_markenvironment(J, mark, (*obj).u.f.scope);
        }
        if !((*obj).u.f.function).is_null() && (*(*obj).u.f.function).gcmark != mark {
            jsG_markfunction(J, mark, (*obj).u.f.function);
        }
    }
}
unsafe extern "C" fn jsG_markstack(J: &mut js_State, mut mark: i32) {
    let mut v: *mut js_Value = (*J).stack;
    let mut n: i32 = (*J).top;
    loop {
        let fresh25 = n;
        n -= 1;
        if fresh25 == 0 {
            break;
        }
        if (*v).t.type_0 as i32 == JS_TMEMSTR as i32 && (*(*v).u.memstr).gcmark as i32 != mark {
            (*(*v).u.memstr).gcmark = mark as libc::c_char;
        }
        if (*v).t.type_0 as i32 == JS_TOBJECT as i32 && (*(*v).u.object).gcmark != mark {
            jsG_markobject(J, mark, (*v).u.object);
        }
        v = v.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_gc(J: &mut js_State, mut report: i32) {
    let mut fun: *mut js_Function = core::ptr::null_mut::<js_Function>();
    let mut nextfun: *mut js_Function = core::ptr::null_mut::<js_Function>();
    let mut prevnextfun: *mut *mut js_Function = core::ptr::null_mut::<*mut js_Function>();
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut nextobj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut prevnextobj: *mut *mut js_Object = core::ptr::null_mut::<*mut js_Object>();
    let mut str: *mut js_String = core::ptr::null_mut::<js_String>();
    let mut nextstr: *mut js_String = core::ptr::null_mut::<js_String>();
    let mut prevnextstr: *mut *mut js_String = core::ptr::null_mut::<*mut js_String>();
    let mut env: *mut js_Environment = core::ptr::null_mut::<js_Environment>();
    let mut nextenv: *mut js_Environment = core::ptr::null_mut::<js_Environment>();
    let mut prevnextenv: *mut *mut js_Environment = core::ptr::null_mut::<*mut js_Environment>();
    let mut nenv: libc::c_uint = 0;
    let mut nfun: libc::c_uint = 0;
    let mut nobj: libc::c_uint = 0;
    let mut nstr: libc::c_uint = 0;
    let mut nprop: libc::c_uint = 0;
    let mut genv: libc::c_uint = 0;
    let mut gfun: libc::c_uint = 0;
    let mut gobj: libc::c_uint = 0;
    let mut gstr: libc::c_uint = 0;
    let mut gprop: libc::c_uint = 0;
    let mut mark: i32 = 0;
    let mut i: i32 = 0;
    (*J).gcmark = if (*J).gcmark == 1 { 2 } else { 1 };
    mark = (*J).gcmark;
    jsG_markobject(J, mark, (*J).Object_prototype);
    jsG_markobject(J, mark, (*J).Array_prototype);
    jsG_markobject(J, mark, (*J).Function_prototype);
    jsG_markobject(J, mark, (*J).Boolean_prototype);
    jsG_markobject(J, mark, (*J).Number_prototype);
    jsG_markobject(J, mark, (*J).String_prototype);
    jsG_markobject(J, mark, (*J).RegExp_prototype);
    jsG_markobject(J, mark, (*J).Date_prototype);
    jsG_markobject(J, mark, (*J).Error_prototype);
    jsG_markobject(J, mark, (*J).EvalError_prototype);
    jsG_markobject(J, mark, (*J).RangeError_prototype);
    jsG_markobject(J, mark, (*J).ReferenceError_prototype);
    jsG_markobject(J, mark, (*J).SyntaxError_prototype);
    jsG_markobject(J, mark, (*J).TypeError_prototype);
    jsG_markobject(J, mark, (*J).URIError_prototype);
    jsG_markobject(J, mark, (*J).R);
    jsG_markobject(J, mark, (*J).G);
    jsG_markstack(J, mark);
    jsG_markenvironment(J, mark, (*J).E);
    jsG_markenvironment(J, mark, (*J).GE);
    i = 0;
    while i < (*J).envtop {
        jsG_markenvironment(J, mark, (*J).envstack[i as usize]);
        i += 1;
    }
    loop {
        obj = (*J).gcroot;
        if obj.is_null() {
            break;
        }
        (*J).gcroot = (*obj).gcroot;
        (*obj).gcroot = core::ptr::null_mut::<js_Object>();
        jsG_scanobject(J, mark, obj);
    }
    prevnextenv = &mut (*J).gcenv;
    env = (*J).gcenv;
    while !env.is_null() {
        nextenv = (*env).gcnext;
        if (*env).gcmark != mark {
            *prevnextenv = nextenv;
            jsG_freeenvironment(J, env);
            genv = genv.wrapping_add(1);
        } else {
            prevnextenv = &mut (*env).gcnext;
        }
        nenv = nenv.wrapping_add(1);

        env = nextenv;
    }
    prevnextfun = &mut (*J).gcfun;
    fun = (*J).gcfun;
    while !fun.is_null() {
        nextfun = (*fun).gcnext;
        if (*fun).gcmark != mark {
            *prevnextfun = nextfun;
            jsG_freefunction(J, fun);
            gfun = gfun.wrapping_add(1);
        } else {
            prevnextfun = &mut (*fun).gcnext;
        }
        nfun = nfun.wrapping_add(1);

        fun = nextfun;
    }
    prevnextobj = &mut (*J).gcobj;
    obj = (*J).gcobj;
    while !obj.is_null() {
        nprop = nprop.wrapping_add((*obj).count as libc::c_uint);
        nextobj = (*obj).gcnext;
        if (*obj).gcmark != mark {
            gprop = gprop.wrapping_add((*obj).count as libc::c_uint);
            *prevnextobj = nextobj;
            jsG_freeobject(J, obj);
            gobj = gobj.wrapping_add(1);
        } else {
            prevnextobj = &mut (*obj).gcnext;
        }
        nobj = nobj.wrapping_add(1);

        obj = nextobj;
    }
    prevnextstr = &mut (*J).gcstr;
    str = (*J).gcstr;
    while !str.is_null() {
        nextstr = (*str).gcnext;
        if (*str).gcmark as i32 != mark {
            *prevnextstr = nextstr;
            js_free(J, str as *mut libc::c_void);
            gstr = gstr.wrapping_add(1);
        } else {
            prevnextstr = &mut (*str).gcnext;
        }
        nstr = nstr.wrapping_add(1);

        str = nextstr;
    }
    let mut ntot: libc::c_uint = nenv
        .wrapping_add(nfun)
        .wrapping_add(nobj)
        .wrapping_add(nstr)
        .wrapping_add(nprop);
    let mut gtot: libc::c_uint = genv
        .wrapping_add(gfun)
        .wrapping_add(gobj)
        .wrapping_add(gstr)
        .wrapping_add(gprop);
    let mut remaining: libc::c_uint = ntot.wrapping_sub(gtot);
    (*J).gccounter = remaining;
    (*J).gcthresh = (remaining as f64 * 5.0f64) as libc::c_uint;
    if report != 0 {
        let mut buf: [libc::c_char; 256] = [0; 256];
        snprintf(
            buf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 256]>() as u32,
            b"garbage collected (%d%%): %d/%d envs, %d/%d funs, %d/%d objs, %d/%d props, %d/%d strs\0"
                as *const u8 as *const libc::c_char,
            (100 as libc::c_uint).wrapping_mul(gtot).wrapping_div(ntot),
            genv,
            nenv,
            gfun,
            nfun,
            gobj,
            nobj,
            gprop,
            nprop,
            gstr,
            nstr,
        );
        js_report(J, buf.as_mut_ptr());
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_freestate(J: *mut js_State) {
    let mut fun: *mut js_Function = core::ptr::null_mut::<js_Function>();
    let mut nextfun: *mut js_Function = core::ptr::null_mut::<js_Function>();
    let mut obj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut nextobj: *mut js_Object = core::ptr::null_mut::<js_Object>();
    let mut env: *mut js_Environment = core::ptr::null_mut::<js_Environment>();
    let mut nextenv: *mut js_Environment = core::ptr::null_mut::<js_Environment>();
    let mut str: *mut js_String = core::ptr::null_mut::<js_String>();
    let mut nextstr: *mut js_String = core::ptr::null_mut::<js_String>();
    if J.is_null() {
        return;
    }
    let J = &mut *J;
    env = (*J).gcenv;
    while !env.is_null() {
        nextenv = (*env).gcnext;
        jsG_freeenvironment(J, env);
        env = nextenv;
    }
    fun = (*J).gcfun;
    while !fun.is_null() {
        nextfun = (*fun).gcnext;
        jsG_freefunction(J, fun);
        fun = nextfun;
    }
    obj = (*J).gcobj;
    while !obj.is_null() {
        nextobj = (*obj).gcnext;
        jsG_freeobject(J, obj);
        obj = nextobj;
    }
    str = (*J).gcstr;
    while !str.is_null() {
        nextstr = (*str).gcnext;
        js_free(J, str as *mut libc::c_void);
        str = nextstr;
    }
    jsS_freestrings(J);
    js_free(J, (*J).lexbuf.text as *mut libc::c_void);
    ((*J).alloc).expect("non-null function pointer")((*J).actx, (*J).stack as *mut libc::c_void, 0);
    ((*J).alloc).expect("non-null function pointer")(
        (*J).actx,
        J as *mut js_State as *mut libc::c_void,
        0,
    );
}
