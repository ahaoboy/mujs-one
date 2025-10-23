use crate::*;
#[no_mangle]
pub unsafe extern "C" fn jsC_error(
    J: &mut js_State,
    mut node: *mut js_Ast,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> ! {
    let mut ap: ::core::ffi::VaListImpl;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut msgbuf: [libc::c_char; 256] = [0; 256];
    ap = args.clone();
    vsnprintf(msgbuf.as_mut_ptr(), 256 as i32 as u32, fmt, ap.as_va_list());
    snprintf(
        buf.as_mut_ptr(),
        256 as i32 as u32,
        b"%s:%d: \0" as *const u8 as *const libc::c_char,
        (*J).filename,
        (*node).line,
    );
    strcat(buf.as_mut_ptr(), msgbuf.as_mut_ptr());
    js_newsyntaxerror(J, buf.as_mut_ptr());
    js_throw(J);
}
const futurewords: [*const libc::c_char; 7] = [
    b"class\0" as *const u8 as *const libc::c_char,
    b"const\0" as *const u8 as *const libc::c_char,
    b"enum\0" as *const u8 as *const libc::c_char,
    b"export\0" as *const u8 as *const libc::c_char,
    b"extends\0" as *const u8 as *const libc::c_char,
    b"import\0" as *const u8 as *const libc::c_char,
    b"super\0" as *const u8 as *const libc::c_char,
];
const strictfuturewords: [*const libc::c_char; 9] = [
    b"implements\0" as *const u8 as *const libc::c_char,
    b"interface\0" as *const u8 as *const libc::c_char,
    b"let\0" as *const u8 as *const libc::c_char,
    b"package\0" as *const u8 as *const libc::c_char,
    b"private\0" as *const u8 as *const libc::c_char,
    b"protected\0" as *const u8 as *const libc::c_char,
    b"public\0" as *const u8 as *const libc::c_char,
    b"static\0" as *const u8 as *const libc::c_char,
    b"yield\0" as *const u8 as *const libc::c_char,
];
unsafe extern "C" fn checkfutureword(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut exp_0: *mut js_Ast,
) {
    if jsY_findword(
        (*exp_0).string,
        futurewords.as_ptr(),
        (::core::mem::size_of::<[*const libc::c_char; 7]>() as u32)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as u32) as i32,
    ) >= 0
    {
        jsC_error(
            J,
            exp_0,
            b"'%s' is a future reserved word\0" as *const u8 as *const libc::c_char,
            (*exp_0).string,
        );
    }
    if (*F).strict != 0
        && jsY_findword(
            (*exp_0).string,
            strictfuturewords.as_ptr(),
            (::core::mem::size_of::<[*const libc::c_char; 9]>() as u32)
                .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as u32)
                as i32,
        ) >= 0
    {
        jsC_error(
            J,
            exp_0,
            b"'%s' is a strict mode future reserved word\0" as *const u8 as *const libc::c_char,
            (*exp_0).string,
        );
    }
}
unsafe extern "C" fn newfun(
    J: &mut js_State,
    mut line: i32,
    mut name: *mut js_Ast,
    mut params: *mut js_Ast,
    mut body: *mut js_Ast,
    mut script_0: i32,
    mut default_strict: i32,
    mut is_fun_exp: i32,
) -> *mut js_Function {
    let mut F: *mut js_Function =
        js_malloc(J, ::core::mem::size_of::<js_Function>() as u32 as i32) as *mut js_Function;
    memset(
        F as *mut libc::c_void,
        0,
        ::core::mem::size_of::<js_Function>() as u32,
    );
    (*F).gcmark = 0;
    (*F).gcnext = (*J).gcfun;
    (*J).gcfun = F;
    (*J).gccounter = ((*J).gccounter).wrapping_add(1);
    (*J).gccounter;
    (*F).filename = js_intern(J, (*J).filename);
    (*F).line = line;
    (*F).script = script_0;
    (*F).strict = default_strict;
    (*F).name = if !name.is_null() {
        (*name).string
    } else {
        b"\0" as *const u8 as *const libc::c_char
    };
    cfunbody(J, F, name, params, body, is_fun_exp);
    F
}
unsafe extern "C" fn emitraw(J: &mut js_State, mut F: *mut js_Function, mut value: i32) {
    if value != value as js_Instruction as i32 {
        js_syntaxerror(
            J,
            b"integer overflow in instruction coding\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*F).codelen >= (*F).codecap {
        (*F).codecap = if (*F).codecap != 0 {
            (*F).codecap * 2
        } else {
            64 as i32
        };
        (*F).code = js_realloc(
            J,
            (*F).code as *mut libc::c_void,
            ((*F).codecap as u32).wrapping_mul(::core::mem::size_of::<js_Instruction>() as u32)
                as i32,
        ) as *mut js_Instruction;
    }
    let fresh9 = (*F).codelen;
    (*F).codelen += 1;
    *((*F).code).offset(fresh9 as isize) = value as js_Instruction;
}
unsafe extern "C" fn emit(J: &mut js_State, mut F: *mut js_Function, mut value: i32) {
    emitraw(J, F, (*F).lastline);
    emitraw(J, F, value);
}
unsafe extern "C" fn emitarg(J: &mut js_State, mut F: *mut js_Function, mut value: i32) {
    emitraw(J, F, value);
}
unsafe extern "C" fn emitline(J: &mut js_State, mut F: *mut js_Function, mut node: *mut js_Ast) {
    (*F).lastline = (*node).line;
}
unsafe extern "C" fn addfunction(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut value: *mut js_Function,
) -> i32 {
    if (*F).funlen >= (*F).funcap {
        (*F).funcap = if (*F).funcap != 0 {
            (*F).funcap * 2
        } else {
            16 as i32
        };
        (*F).funtab = js_realloc(
            J,
            (*F).funtab as *mut libc::c_void,
            ((*F).funcap as u32).wrapping_mul(::core::mem::size_of::<*mut js_Function>() as u32)
                as i32,
        ) as *mut *mut js_Function;
    }
    let fresh10 = &mut (*((*F).funtab).offset((*F).funlen as isize));
    *fresh10 = value;
    let fresh11 = (*F).funlen;
    (*F).funlen += 1;
    fresh11
}
unsafe extern "C" fn addlocal(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut ident: *mut js_Ast,
    mut reuse: i32,
) -> i32 {
    let mut name: *const libc::c_char = (*ident).string;
    if (*F).strict != 0 {
        if strcmp(name, b"arguments\0" as *const u8 as *const libc::c_char) == 0 {
            jsC_error(
                J,
                ident,
                b"redefining 'arguments' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if strcmp(name, b"eval\0" as *const u8 as *const libc::c_char) == 0 {
            jsC_error(
                J,
                ident,
                b"redefining 'eval' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if strcmp(name, b"eval\0" as *const u8 as *const libc::c_char) == 0 {
        js_evalerror(
            J,
            b"%s:%d: invalid use of 'eval'\0" as *const u8 as *const libc::c_char,
            (*J).filename,
            (*ident).line,
        );
    }
    if reuse != 0 || (*F).strict != 0 {
        let mut i: i32 = 0;
        i = 0;
        while i < (*F).varlen {
            if strcmp(*((*F).vartab).offset(i as isize), name) == 0 {
                if reuse != 0 {
                    return i + 1;
                }
                if (*F).strict != 0 {
                    jsC_error(
                        J,
                        ident,
                        b"duplicate formal parameter '%s'\0" as *const u8 as *const libc::c_char,
                        name,
                    );
                }
            }
            i += 1;
        }
    }
    if (*F).varlen >= (*F).varcap {
        (*F).varcap = if (*F).varcap != 0 {
            (*F).varcap * 2
        } else {
            16 as i32
        };
        (*F).vartab = js_realloc(
            J,
            (*F).vartab as *mut libc::c_void,
            ((*F).varcap as u32).wrapping_mul(::core::mem::size_of::<*const libc::c_char>() as u32)
                as i32,
        ) as *mut *const libc::c_char;
    }
    let fresh12 = &mut (*((*F).vartab).offset((*F).varlen as isize));
    *fresh12 = name;
    (*F).varlen += 1;
    (*F).varlen
}
unsafe extern "C" fn findlocal(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut name: *const libc::c_char,
) -> i32 {
    let mut i: i32 = 0;
    i = (*F).varlen;
    while i > 0 {
        if strcmp(*((*F).vartab).offset((i - 1) as isize), name) == 0 {
            return i;
        }
        i -= 1;
    }
    -(1 as i32)
}
unsafe extern "C" fn emitfunction(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut fun: *mut js_Function,
) {
    (*F).lightweight = 0;
    emit(J, F, OP_CLOSURE as i32);
    let value = addfunction(J, F, fun);
    emitarg(J, F, value);
}
unsafe extern "C" fn emitnumber(J: &mut js_State, mut F: *mut js_Function, mut num: f64) {
    if num == 0.0 {
        emit(J, F, OP_INTEGER as i32);
        emitarg(J, F, 32768 as i32);
        if num.is_sign_negative() as i32 != 0 {
            emit(J, F, OP_NEG as i32);
        }
    } else if num >= (-(32767 as i32) - 1) as f64
        && num <= 32767 as i32 as f64
        && num == num as i32 as f64
    {
        emit(J, F, OP_INTEGER as i32);
        emitarg(J, F, (num + 32768 as i32 as f64) as i32);
    } else {
        let mut x: [js_Instruction; 4] = [0; 4];
        let mut i: size_t = 0;
        emit(J, F, OP_NUMBER as i32);
        memcpy(
            x.as_mut_ptr() as *mut libc::c_void,
            &mut num as *mut f64 as *const libc::c_void,
            ::core::mem::size_of::<f64>() as u32,
        );
        i = 0;
        while i
            < (::core::mem::size_of::<f64>() as u32)
                .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
        {
            emitarg(J, F, x[i as usize] as i32);
            i = i.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn emitstring(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut opcode: i32,
    mut str: *const libc::c_char,
) {
    let mut x: [js_Instruction; 4] = [0; 4];
    let mut i: size_t = 0;
    emit(J, F, opcode);
    memcpy(
        x.as_mut_ptr() as *mut libc::c_void,
        &mut str as *mut *const libc::c_char as *const libc::c_void,
        ::core::mem::size_of::<*const libc::c_char>() as u32,
    );
    i = 0;
    while i
        < (::core::mem::size_of::<*const libc::c_char>() as u32)
            .wrapping_div(::core::mem::size_of::<js_Instruction>() as u32)
    {
        emitarg(J, F, x[i as usize] as i32);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn emitlocal(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut oploc: i32,
    mut opvar: i32,
    mut ident: *mut js_Ast,
) {
    let mut is_arguments: i32 = (strcmp(
        (*ident).string,
        b"arguments\0" as *const u8 as *const libc::c_char,
    ) == 0) as i32;
    let mut is_eval: i32 = (strcmp(
        (*ident).string,
        b"eval\0" as *const u8 as *const libc::c_char,
    ) == 0) as i32;
    let mut i: i32 = 0;
    if is_arguments != 0 {
        (*F).lightweight = 0;
        (*F).arguments = 1;
    }
    checkfutureword(J, F, ident);
    if (*F).strict != 0 && oploc == OP_SETLOCAL as i32 {
        if is_arguments != 0 {
            jsC_error(
                J,
                ident,
                b"'arguments' is read-only in strict mode\0" as *const u8 as *const libc::c_char,
            );
        }
        if is_eval != 0 {
            jsC_error(
                J,
                ident,
                b"'eval' is read-only in strict mode\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if is_eval != 0 {
        js_evalerror(
            J,
            b"%s:%d: invalid use of 'eval'\0" as *const u8 as *const libc::c_char,
            (*J).filename,
            (*ident).line,
        );
    }
    i = findlocal(J, F, (*ident).string);
    if i < 0 {
        emitstring(J, F, opvar, (*ident).string);
    } else {
        emit(J, F, oploc);
        emitarg(J, F, i);
    };
}
unsafe extern "C" fn here(J: &mut js_State, mut F: *mut js_Function) -> i32 {
    (*F).codelen
}
unsafe extern "C" fn emitjump(J: &mut js_State, mut F: *mut js_Function, mut opcode: i32) -> i32 {
    let mut inst: i32 = 0;
    emit(J, F, opcode);
    inst = (*F).codelen;
    emitarg(J, F, 0);
    inst
}
unsafe extern "C" fn emitjumpto(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut opcode: i32,
    mut dest: i32,
) {
    emit(J, F, opcode);
    if dest != dest as js_Instruction as i32 {
        js_syntaxerror(
            J,
            b"jump address integer overflow\0" as *const u8 as *const libc::c_char,
        );
    }
    emitarg(J, F, dest);
}
unsafe extern "C" fn labelto(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut inst: i32,
    mut addr: i32,
) {
    if addr != addr as js_Instruction as i32 {
        js_syntaxerror(
            J,
            b"jump address integer overflow\0" as *const u8 as *const libc::c_char,
        );
    }
    *((*F).code).offset(inst as isize) = addr as js_Instruction;
}
unsafe extern "C" fn label(J: &mut js_State, mut F: *mut js_Function, mut inst: i32) {
    labelto(J, F, inst, (*F).codelen);
}
unsafe extern "C" fn ctypeof(J: &mut js_State, mut F: *mut js_Function, mut exp_0: *mut js_Ast) {
    if (*(*exp_0).a).type_0 as libc::c_uint == EXP_IDENTIFIER as i32 as libc::c_uint {
        emitline(J, F, (*exp_0).a);
        emitlocal(J, F, OP_GETLOCAL as i32, OP_HASVAR as i32, (*exp_0).a);
    } else {
        jsC_cexp(J, F, (*exp_0).a);
    }
    emitline(J, F, exp_0);
    emit(J, F, OP_TYPEOF as i32);
}
unsafe extern "C" fn cunary(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut exp_0: *mut js_Ast,
    mut opcode: i32,
) {
    jsC_cexp(J, F, (*exp_0).a);
    emitline(J, F, exp_0);
    emit(J, F, opcode);
}
unsafe extern "C" fn cbinary(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut exp_0: *mut js_Ast,
    mut opcode: i32,
) {
    jsC_cexp(J, F, (*exp_0).a);
    jsC_cexp(J, F, (*exp_0).b);
    emitline(J, F, exp_0);
    emit(J, F, opcode);
}
unsafe extern "C" fn carray(J: &mut js_State, mut F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        emitline(J, F, (*list).a);
        if (*(*list).a).type_0 as libc::c_uint == EXP_ELISION as i32 as libc::c_uint {
            emit(J, F, OP_SKIPARRAY as i32);
        } else {
            jsC_cexp(J, F, (*list).a);
            emit(J, F, OP_INITARRAY as i32);
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn checkdup(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut list: *mut js_Ast,
    mut end: *mut js_Ast,
) {
    let mut nbuf: [libc::c_char; 32] = [0; 32];
    let mut sbuf: [libc::c_char; 32] = [0; 32];
    let mut needle: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut straw: *const libc::c_char = core::ptr::null::<libc::c_char>();
    if (*(*end).a).type_0 as libc::c_uint == EXP_NUMBER as i32 as libc::c_uint {
        needle = jsV_numbertostring(nbuf.as_mut_ptr(), (*(*end).a).number);
    } else {
        needle = (*(*end).a).string;
    }
    while (*list).a != end {
        if (*(*list).a).type_0 as libc::c_uint == (*end).type_0 as libc::c_uint {
            let mut prop: *mut js_Ast = (*(*list).a).a;
            if (*prop).type_0 as libc::c_uint == EXP_NUMBER as i32 as libc::c_uint {
                straw = jsV_numbertostring(sbuf.as_mut_ptr(), (*prop).number);
            } else {
                straw = (*prop).string;
            }
            if strcmp(needle, straw) == 0 {
                jsC_error(
                    J,
                    list,
                    b"duplicate property '%s' in object literal\0" as *const u8
                        as *const libc::c_char,
                    needle,
                );
            }
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cobject(J: &mut js_State, mut F: *mut js_Function, mut list: *mut js_Ast) {
    let mut head: *mut js_Ast = list;
    while !list.is_null() {
        let mut kv: *mut js_Ast = (*list).a;
        let mut prop: *mut js_Ast = (*kv).a;
        if (*prop).type_0 as libc::c_uint == AST_IDENTIFIER as i32 as libc::c_uint
            || (*prop).type_0 as libc::c_uint == EXP_STRING as i32 as libc::c_uint
        {
            emitline(J, F, prop);
            emitstring(J, F, OP_STRING as i32, (*prop).string);
        } else if (*prop).type_0 as libc::c_uint == EXP_NUMBER as i32 as libc::c_uint {
            emitline(J, F, prop);
            emitnumber(J, F, (*prop).number);
        } else {
            jsC_error(
                J,
                prop,
                b"invalid property name in object initializer\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*F).strict != 0 {
            checkdup(J, F, head, kv);
        }
        match (*kv).type_0 as libc::c_uint {
            14 => {
                jsC_cexp(J, F, (*kv).b);
                emitline(J, F, kv);
                emit(J, F, OP_INITPROP as i32);
            }
            15 => {
                let func = newfun(
                    J,
                    (*prop).line,
                    core::ptr::null_mut::<js_Ast>(),
                    core::ptr::null_mut::<js_Ast>(),
                    (*kv).c,
                    0,
                    (*F).strict,
                    1,
                );
                emitfunction(J, F, func);
                emitline(J, F, kv);
                emit(J, F, OP_INITGETTER as i32);
            }
            16 => {
                let fun = newfun(
                    J,
                    (*prop).line,
                    core::ptr::null_mut::<js_Ast>(),
                    (*kv).b,
                    (*kv).c,
                    0,
                    (*F).strict,
                    1,
                );
                emitfunction(J, F, fun);
                emitline(J, F, kv);
                emit(J, F, OP_INITSETTER as i32);
            }
            _ => {}
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cargs(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut list: *mut js_Ast,
) -> i32 {
    let mut n: i32 = 0;
    while !list.is_null() {
        jsC_cexp(J, F, (*list).a);
        list = (*list).b;
        n += 1;
    }
    n
}
unsafe extern "C" fn cassign(J: &mut js_State, mut F: *mut js_Function, mut exp_0: *mut js_Ast) {
    let mut lhs: *mut js_Ast = (*exp_0).a;
    let mut rhs: *mut js_Ast = (*exp_0).b;
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            jsC_cexp(J, F, rhs);
            emitline(J, F, exp_0);
            emitlocal(J, F, OP_SETLOCAL as i32, OP_SETVAR as i32, lhs);
        }
        18 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, (*lhs).b);
            jsC_cexp(J, F, rhs);
            emitline(J, F, exp_0);
            emit(J, F, OP_SETPROP as i32);
        }
        19 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, rhs);
            emitline(J, F, exp_0);
            emitstring(J, F, OP_SETPROP_S as i32, (*(*lhs).b).string);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignforin(J: &mut js_State, mut F: *mut js_Function, mut stm: *mut js_Ast) {
    let mut lhs: *mut js_Ast = (*stm).a;
    if (*stm).type_0 as libc::c_uint == STM_FOR_IN_VAR as i32 as libc::c_uint {
        if !((*lhs).b).is_null() {
            jsC_error(
                J,
                (*lhs).b,
                b"more than one loop variable in for-in statement\0" as *const u8
                    as *const libc::c_char,
            );
        }
        emitline(J, F, (*lhs).a);
        emitlocal(J, F, OP_SETLOCAL as i32, OP_SETVAR as i32, (*(*lhs).a).a);
        emit(J, F, OP_POP as i32);
        return;
    }
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            emitline(J, F, lhs);
            emitlocal(J, F, OP_SETLOCAL as i32, OP_SETVAR as i32, lhs);
            emit(J, F, OP_POP as i32);
        }
        18 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, (*lhs).b);
            emitline(J, F, lhs);
            emit(J, F, OP_ROT3 as i32);
            emit(J, F, OP_SETPROP as i32);
            emit(J, F, OP_POP as i32);
        }
        19 => {
            jsC_cexp(J, F, (*lhs).a);
            emitline(J, F, lhs);
            emit(J, F, OP_ROT2 as i32);
            emitstring(J, F, OP_SETPROP_S as i32, (*(*lhs).b).string);
            emit(J, F, OP_POP as i32);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in for-in loop assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignop1(J: &mut js_State, mut F: *mut js_Function, mut lhs: *mut js_Ast) {
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            emitline(J, F, lhs);
            emitlocal(J, F, OP_GETLOCAL as i32, OP_GETVAR as i32, lhs);
        }
        18 => {
            jsC_cexp(J, F, (*lhs).a);
            jsC_cexp(J, F, (*lhs).b);
            emitline(J, F, lhs);
            emit(J, F, OP_DUP2 as i32);
            emit(J, F, OP_GETPROP as i32);
        }
        19 => {
            jsC_cexp(J, F, (*lhs).a);
            emitline(J, F, lhs);
            emit(J, F, OP_DUP as i32);
            emitstring(J, F, OP_GETPROP_S as i32, (*(*lhs).b).string);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignop2(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut lhs: *mut js_Ast,
    mut postfix_0: i32,
) {
    match (*lhs).type_0 as libc::c_uint {
        3 => {
            emitline(J, F, lhs);
            if postfix_0 != 0 {
                emit(J, F, OP_ROT2 as i32);
            }
            emitlocal(J, F, OP_SETLOCAL as i32, OP_SETVAR as i32, lhs);
        }
        18 => {
            emitline(J, F, lhs);
            if postfix_0 != 0 {
                emit(J, F, OP_ROT4 as i32);
            }
            emit(J, F, OP_SETPROP as i32);
        }
        19 => {
            emitline(J, F, lhs);
            if postfix_0 != 0 {
                emit(J, F, OP_ROT3 as i32);
            }
            emitstring(J, F, OP_SETPROP_S as i32, (*(*lhs).b).string);
        }
        _ => {
            jsC_error(
                J,
                lhs,
                b"invalid l-value in assignment\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn cassignop(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut exp_0: *mut js_Ast,
    mut opcode: i32,
) {
    let mut lhs: *mut js_Ast = (*exp_0).a;
    let mut rhs: *mut js_Ast = (*exp_0).b;
    cassignop1(J, F, lhs);
    jsC_cexp(J, F, rhs);
    emitline(J, F, exp_0);
    emit(J, F, opcode);
    cassignop2(J, F, lhs, 0);
}
unsafe extern "C" fn cdelete(J: &mut js_State, mut F: *mut js_Function, mut exp_0: *mut js_Ast) {
    let mut arg: *mut js_Ast = (*exp_0).a;
    match (*arg).type_0 as libc::c_uint {
        3 => {
            if (*F).strict != 0 {
                jsC_error(
                    J,
                    exp_0,
                    b"delete on an unqualified name is not allowed in strict mode\0" as *const u8
                        as *const libc::c_char,
                );
            }
            emitline(J, F, exp_0);
            emitlocal(J, F, OP_DELLOCAL as i32, OP_DELVAR as i32, arg);
        }
        18 => {
            jsC_cexp(J, F, (*arg).a);
            jsC_cexp(J, F, (*arg).b);
            emitline(J, F, exp_0);
            emit(J, F, OP_DELPROP as i32);
        }
        19 => {
            jsC_cexp(J, F, (*arg).a);
            emitline(J, F, exp_0);
            emitstring(J, F, OP_DELPROP_S as i32, (*(*arg).b).string);
        }
        _ => {
            jsC_error(
                J,
                exp_0,
                b"invalid l-value in delete expression\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn ceval(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut fun: *mut js_Ast,
    mut args: *mut js_Ast,
) {
    let mut n: i32 = cargs(J, F, args);
    (*F).lightweight = 0;
    (*F).arguments = 1;
    if n == 0 {
        emit(J, F, OP_UNDEF as i32);
    } else {
        loop {
            let fresh13 = n;
            n -= 1;
            if fresh13 <= 1 {
                break;
            }
            emit(J, F, OP_POP as i32);
        }
    }
    emit(J, F, OP_EVAL as i32);
}
unsafe extern "C" fn ccall(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut fun: *mut js_Ast,
    mut args: *mut js_Ast,
) {
    let mut n: i32 = 0;
    let mut current_block_14: u64;
    match (*fun).type_0 as libc::c_uint {
        18 => {
            jsC_cexp(J, F, (*fun).a);
            emit(J, F, OP_DUP as i32);
            jsC_cexp(J, F, (*fun).b);
            emit(J, F, OP_GETPROP as i32);
            emit(J, F, OP_ROT2 as i32);
            current_block_14 = 11050875288958768710;
        }
        19 => {
            jsC_cexp(J, F, (*fun).a);
            emit(J, F, OP_DUP as i32);
            emitstring(J, F, OP_GETPROP_S as i32, (*(*fun).b).string);
            emit(J, F, OP_ROT2 as i32);
            current_block_14 = 11050875288958768710;
        }
        3 => {
            if strcmp((*fun).string, b"eval\0" as *const u8 as *const libc::c_char) == 0 {
                ceval(J, F, fun, args);
                return;
            }
            current_block_14 = 5035918384899282284;
        }
        _ => {
            current_block_14 = 5035918384899282284;
        }
    }
    if current_block_14 == 5035918384899282284 {
        jsC_cexp(J, F, fun);
        emit(J, F, OP_UNDEF as i32);
    }
    n = cargs(J, F, args);
    emit(J, F, OP_CALL as i32);
    emitarg(J, F, n);
}
unsafe extern "C" fn jsC_cexp(J: &mut js_State, mut F: *mut js_Function, mut exp_0: *mut js_Ast) {
    let mut then: i32 = 0;
    let mut end: i32 = 0;
    let mut n: i32 = 0;
    match (*exp_0).type_0 as libc::c_uint {
        5 => {
            emitline(J, F, exp_0);
            emitstring(J, F, OP_STRING as i32, (*exp_0).string);
        }
        4 => {
            emitline(J, F, exp_0);
            emitnumber(J, F, (*exp_0).number);
        }
        7 => {}
        8 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_NULL as i32);
        }
        9 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_TRUE as i32);
        }
        10 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_FALSE as i32);
        }
        11 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_THIS as i32);
        }
        6 => {
            emitline(J, F, exp_0);
            emitstring(J, F, OP_NEWREGEXP as i32, (*exp_0).string);
            emitarg(J, F, (*exp_0).number as i32);
        }
        13 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_NEWOBJECT as i32);
            cobject(J, F, (*exp_0).a);
        }
        12 => {
            emitline(J, F, exp_0);
            emit(J, F, OP_NEWARRAY as i32);
            carray(J, F, (*exp_0).a);
        }
        17 => {
            emitline(J, F, exp_0);
            let fun = newfun(
                J,
                (*exp_0).line,
                (*exp_0).a,
                (*exp_0).b,
                (*exp_0).c,
                0,
                (*F).strict,
                1,
            );
            emitfunction(J, F, fun);
        }
        3 => {
            emitline(J, F, exp_0);
            emitlocal(J, F, OP_GETLOCAL as i32, OP_GETVAR as i32, exp_0);
        }
        18 => {
            jsC_cexp(J, F, (*exp_0).a);
            jsC_cexp(J, F, (*exp_0).b);
            emitline(J, F, exp_0);
            emit(J, F, OP_GETPROP as i32);
        }
        19 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emitstring(J, F, OP_GETPROP_S as i32, (*(*exp_0).b).string);
        }
        20 => {
            ccall(J, F, (*exp_0).a, (*exp_0).b);
        }
        21 => {
            jsC_cexp(J, F, (*exp_0).a);
            n = cargs(J, F, (*exp_0).b);
            emitline(J, F, exp_0);
            emit(J, F, OP_NEW as i32);
            emitarg(J, F, n);
        }
        24 => {
            cdelete(J, F, exp_0);
        }
        27 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_INC as i32);
            cassignop2(J, F, (*exp_0).a, 0);
        }
        28 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_DEC as i32);
            cassignop2(J, F, (*exp_0).a, 0);
        }
        22 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POSTINC as i32);
            cassignop2(J, F, (*exp_0).a, 1);
            emit(J, F, OP_POP as i32);
        }
        23 => {
            cassignop1(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POSTDEC as i32);
            cassignop2(J, F, (*exp_0).a, 1);
            emit(J, F, OP_POP as i32);
        }
        25 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POP as i32);
            emit(J, F, OP_UNDEF as i32);
        }
        26 => {
            ctypeof(J, F, exp_0);
        }
        29 => {
            cunary(J, F, exp_0, OP_POS as i32);
        }
        30 => {
            cunary(J, F, exp_0, OP_NEG as i32);
        }
        31 => {
            cunary(J, F, exp_0, OP_BITNOT as i32);
        }
        32 => {
            cunary(J, F, exp_0, OP_LOGNOT as i32);
        }
        53 => {
            cbinary(J, F, exp_0, OP_BITOR as i32);
        }
        52 => {
            cbinary(J, F, exp_0, OP_BITXOR as i32);
        }
        51 => {
            cbinary(J, F, exp_0, OP_BITAND as i32);
        }
        50 => {
            cbinary(J, F, exp_0, OP_EQ as i32);
        }
        49 => {
            cbinary(J, F, exp_0, OP_NE as i32);
        }
        48 => {
            cbinary(J, F, exp_0, OP_STRICTEQ as i32);
        }
        47 => {
            cbinary(J, F, exp_0, OP_STRICTNE as i32);
        }
        46 => {
            cbinary(J, F, exp_0, OP_LT as i32);
        }
        45 => {
            cbinary(J, F, exp_0, OP_GT as i32);
        }
        44 => {
            cbinary(J, F, exp_0, OP_LE as i32);
        }
        43 => {
            cbinary(J, F, exp_0, OP_GE as i32);
        }
        42 => {
            cbinary(J, F, exp_0, OP_INSTANCEOF as i32);
        }
        41 => {
            cbinary(J, F, exp_0, OP_IN as i32);
        }
        40 => {
            cbinary(J, F, exp_0, OP_SHL as i32);
        }
        39 => {
            cbinary(J, F, exp_0, OP_SHR as i32);
        }
        38 => {
            cbinary(J, F, exp_0, OP_USHR as i32);
        }
        37 => {
            cbinary(J, F, exp_0, OP_ADD as i32);
        }
        36 => {
            cbinary(J, F, exp_0, OP_SUB as i32);
        }
        35 => {
            cbinary(J, F, exp_0, OP_MUL as i32);
        }
        34 => {
            cbinary(J, F, exp_0, OP_DIV as i32);
        }
        33 => {
            cbinary(J, F, exp_0, OP_MOD as i32);
        }
        57 => {
            cassign(J, F, exp_0);
        }
        58 => {
            cassignop(J, F, exp_0, OP_MUL as i32);
        }
        59 => {
            cassignop(J, F, exp_0, OP_DIV as i32);
        }
        60 => {
            cassignop(J, F, exp_0, OP_MOD as i32);
        }
        61 => {
            cassignop(J, F, exp_0, OP_ADD as i32);
        }
        62 => {
            cassignop(J, F, exp_0, OP_SUB as i32);
        }
        63 => {
            cassignop(J, F, exp_0, OP_SHL as i32);
        }
        64 => {
            cassignop(J, F, exp_0, OP_SHR as i32);
        }
        65 => {
            cassignop(J, F, exp_0, OP_USHR as i32);
        }
        66 => {
            cassignop(J, F, exp_0, OP_BITAND as i32);
        }
        67 => {
            cassignop(J, F, exp_0, OP_BITXOR as i32);
        }
        68 => {
            cassignop(J, F, exp_0, OP_BITOR as i32);
        }
        69 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_POP as i32);
            jsC_cexp(J, F, (*exp_0).b);
        }
        55 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_DUP as i32);
            end = emitjump(J, F, OP_JTRUE as i32);
            emit(J, F, OP_POP as i32);
            jsC_cexp(J, F, (*exp_0).b);
            label(J, F, end);
        }
        54 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            emit(J, F, OP_DUP as i32);
            end = emitjump(J, F, OP_JFALSE as i32);
            emit(J, F, OP_POP as i32);
            jsC_cexp(J, F, (*exp_0).b);
            label(J, F, end);
        }
        56 => {
            jsC_cexp(J, F, (*exp_0).a);
            emitline(J, F, exp_0);
            then = emitjump(J, F, OP_JTRUE as i32);
            jsC_cexp(J, F, (*exp_0).c);
            end = emitjump(J, F, OP_JUMP as i32);
            label(J, F, then);
            jsC_cexp(J, F, (*exp_0).b);
            label(J, F, end);
        }
        _ => {
            jsC_error(
                J,
                exp_0,
                b"unknown expression type\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn addjump(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut type_0: js_AstType,
    mut target: *mut js_Ast,
    mut inst: i32,
) {
    let mut jump: *mut js_JumpList =
        js_malloc(J, ::core::mem::size_of::<js_JumpList>() as u32 as i32) as *mut js_JumpList;
    (*jump).type_0 = type_0;
    (*jump).inst = inst;
    (*jump).next = (*target).jumps;
    (*target).jumps = jump;
}
unsafe extern "C" fn labeljumps(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut stm: *mut js_Ast,
    mut baddr: i32,
    mut caddr: i32,
) {
    let mut jump: *mut js_JumpList = (*stm).jumps;
    while !jump.is_null() {
        let mut next_0: *mut js_JumpList = (*jump).next;
        if (*jump).type_0 as libc::c_uint == STM_BREAK as i32 as libc::c_uint {
            labelto(J, F, (*jump).inst, baddr);
        }
        if (*jump).type_0 as libc::c_uint == STM_CONTINUE as i32 as libc::c_uint {
            labelto(J, F, (*jump).inst, caddr);
        }
        js_free(J, jump as *mut libc::c_void);
        jump = next_0;
    }
    (*stm).jumps = core::ptr::null_mut::<js_JumpList>();
}
unsafe extern "C" fn isloop(mut T: js_AstType) -> i32 {
    (T as libc::c_uint == STM_DO as i32 as libc::c_uint
        || T as libc::c_uint == STM_WHILE as i32 as libc::c_uint
        || T as libc::c_uint == STM_FOR as i32 as libc::c_uint
        || T as libc::c_uint == STM_FOR_VAR as i32 as libc::c_uint
        || T as libc::c_uint == STM_FOR_IN as i32 as libc::c_uint
        || T as libc::c_uint == STM_FOR_IN_VAR as i32 as libc::c_uint) as i32
}
unsafe extern "C" fn isfun(mut T: js_AstType) -> i32 {
    (T as libc::c_uint == AST_FUNDEC as i32 as libc::c_uint
        || T as libc::c_uint == EXP_FUN as i32 as libc::c_uint
        || T as libc::c_uint == EXP_PROP_GET as i32 as libc::c_uint
        || T as libc::c_uint == EXP_PROP_SET as i32 as libc::c_uint) as i32
}
unsafe extern "C" fn matchlabel(mut node: *mut js_Ast, mut label_0: *const libc::c_char) -> i32 {
    while !node.is_null() && (*node).type_0 as libc::c_uint == STM_LABEL as i32 as libc::c_uint {
        if strcmp((*(*node).a).string, label_0) == 0 {
            return 1;
        }
        node = (*node).parent;
    }
    0
}
unsafe extern "C" fn breaktarget(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut node: *mut js_Ast,
    mut label_0: *const libc::c_char,
) -> *mut js_Ast {
    while !node.is_null() {
        if isfun((*node).type_0) != 0 {
            break;
        }
        if label_0.is_null() {
            if isloop((*node).type_0) != 0
                || (*node).type_0 as libc::c_uint == STM_SWITCH as i32 as libc::c_uint
            {
                return node;
            }
        } else if matchlabel((*node).parent, label_0) != 0 {
            return node;
        }
        node = (*node).parent;
    }
    core::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn continuetarget(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut node: *mut js_Ast,
    mut label_0: *const libc::c_char,
) -> *mut js_Ast {
    while !node.is_null() {
        if isfun((*node).type_0) != 0 {
            break;
        }
        if isloop((*node).type_0) != 0 {
            if label_0.is_null() {
                return node;
            } else if matchlabel((*node).parent, label_0) != 0 {
                return node;
            }
        }
        node = (*node).parent;
    }
    core::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn returntarget(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut node: *mut js_Ast,
) -> *mut js_Ast {
    while !node.is_null() {
        if isfun((*node).type_0) != 0 {
            return node;
        }
        node = (*node).parent;
    }
    core::ptr::null_mut::<js_Ast>()
}
unsafe extern "C" fn cexit(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut T: js_AstType,
    mut node: *mut js_Ast,
    mut target: *mut js_Ast,
) {
    let mut prev: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    loop {
        prev = node;
        node = (*node).parent;
        match (*node).type_0 as libc::c_uint {
            84 => {
                emitline(J, F, node);
                emit(J, F, OP_ENDWITH as i32);
            }
            79 | 80 => {
                emitline(J, F, node);
                if (*F).script != 0 {
                    if T as libc::c_uint == STM_RETURN as i32 as libc::c_uint
                        || T as libc::c_uint == STM_BREAK as i32 as libc::c_uint
                        || T as libc::c_uint == STM_CONTINUE as i32 as libc::c_uint
                            && target != node
                    {
                        emit(J, F, OP_ROT2 as i32);
                        emit(J, F, OP_POP as i32);
                    }
                    if T as libc::c_uint == STM_CONTINUE as i32 as libc::c_uint {
                        emit(J, F, OP_ROT2 as i32);
                    }
                } else {
                    if T as libc::c_uint == STM_RETURN as i32 as libc::c_uint {
                        emit(J, F, OP_ROT2 as i32);
                        emit(J, F, OP_POP as i32);
                    }
                    if T as libc::c_uint == STM_BREAK as i32 as libc::c_uint
                        || T as libc::c_uint == STM_CONTINUE as i32 as libc::c_uint
                            && target != node
                    {
                        emit(J, F, OP_POP as i32);
                    }
                }
            }
            87 => {
                emitline(J, F, node);
                if prev == (*node).a {
                    emit(J, F, OP_ENDTRY as i32);
                    if !((*node).d).is_null() {
                        cstm(J, F, (*node).d);
                    }
                }
                if prev == (*node).c {
                    if !((*node).d).is_null() {
                        emit(J, F, OP_ENDCATCH as i32);
                        emit(J, F, OP_ENDTRY as i32);
                        cstm(J, F, (*node).d);
                    } else {
                        emit(J, F, OP_ENDCATCH as i32);
                    }
                }
            }
            _ => {}
        }
        if node == target {
            break;
        }
    }
}
unsafe extern "C" fn ctryfinally(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut trystm: *mut js_Ast,
    mut finallystm: *mut js_Ast,
) {
    let mut L1: i32 = 0;
    L1 = emitjump(J, F, OP_TRY as i32);
    cstm(J, F, finallystm);
    emit(J, F, OP_THROW as i32);
    label(J, F, L1);
    cstm(J, F, trystm);
    emit(J, F, OP_ENDTRY as i32);
    cstm(J, F, finallystm);
}
unsafe extern "C" fn ctrycatch(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut trystm: *mut js_Ast,
    mut catchvar: *mut js_Ast,
    mut catchstm: *mut js_Ast,
) {
    let mut L1: i32 = 0;
    let mut L2: i32 = 0;
    L1 = emitjump(J, F, OP_TRY as i32);
    checkfutureword(J, F, catchvar);
    if (*F).strict != 0 {
        if strcmp(
            (*catchvar).string,
            b"arguments\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'arguments' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if strcmp(
            (*catchvar).string,
            b"eval\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'eval' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    emitline(J, F, catchvar);
    emitstring(J, F, OP_CATCH as i32, (*catchvar).string);
    cstm(J, F, catchstm);
    emit(J, F, OP_ENDCATCH as i32);
    L2 = emitjump(J, F, OP_JUMP as i32);
    label(J, F, L1);
    cstm(J, F, trystm);
    emit(J, F, OP_ENDTRY as i32);
    label(J, F, L2);
}
unsafe extern "C" fn ctrycatchfinally(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut trystm: *mut js_Ast,
    mut catchvar: *mut js_Ast,
    mut catchstm: *mut js_Ast,
    mut finallystm: *mut js_Ast,
) {
    let mut L1: i32 = 0;
    let mut L2: i32 = 0;
    let mut L3: i32 = 0;
    L1 = emitjump(J, F, OP_TRY as i32);
    L2 = emitjump(J, F, OP_TRY as i32);
    cstm(J, F, finallystm);
    emit(J, F, OP_THROW as i32);
    label(J, F, L2);
    if (*F).strict != 0 {
        checkfutureword(J, F, catchvar);
        if strcmp(
            (*catchvar).string,
            b"arguments\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'arguments' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if strcmp(
            (*catchvar).string,
            b"eval\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            jsC_error(
                J,
                catchvar,
                b"redefining 'eval' is not allowed in strict mode\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    emitline(J, F, catchvar);
    emitstring(J, F, OP_CATCH as i32, (*catchvar).string);
    cstm(J, F, catchstm);
    emit(J, F, OP_ENDCATCH as i32);
    emit(J, F, OP_ENDTRY as i32);
    L3 = emitjump(J, F, OP_JUMP as i32);
    label(J, F, L1);
    cstm(J, F, trystm);
    emit(J, F, OP_ENDTRY as i32);
    label(J, F, L3);
    cstm(J, F, finallystm);
}
unsafe extern "C" fn cswitch(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut ref_0: *mut js_Ast,
    mut head: *mut js_Ast,
) {
    let mut node: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut clause: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut def: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut end: i32 = 0;
    jsC_cexp(J, F, ref_0);
    node = head;
    while !node.is_null() {
        clause = (*node).a;
        if (*clause).type_0 as libc::c_uint == STM_DEFAULT as i32 as libc::c_uint {
            if !def.is_null() {
                jsC_error(
                    J,
                    clause,
                    b"more than one default label in switch\0" as *const u8 as *const libc::c_char,
                );
            }
            def = clause;
        } else {
            jsC_cexp(J, F, (*clause).a);
            emitline(J, F, clause);
            (*clause).casejump = emitjump(J, F, OP_JCASE as i32);
        }
        node = (*node).b;
    }
    emit(J, F, OP_POP as i32);
    if !def.is_null() {
        emitline(J, F, def);
        (*def).casejump = emitjump(J, F, OP_JUMP as i32);
        end = 0;
    } else {
        end = emitjump(J, F, OP_JUMP as i32);
    }
    node = head;
    while !node.is_null() {
        clause = (*node).a;
        label(J, F, (*clause).casejump);
        if (*clause).type_0 as libc::c_uint == STM_DEFAULT as i32 as libc::c_uint {
            cstmlist(J, F, (*clause).a);
        } else {
            cstmlist(J, F, (*clause).b);
        }
        node = (*node).b;
    }
    if end != 0 {
        label(J, F, end);
    }
}
unsafe extern "C" fn cvarinit(J: &mut js_State, mut F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        let mut var: *mut js_Ast = (*list).a;
        if !((*var).b).is_null() {
            jsC_cexp(J, F, (*var).b);
            emitline(J, F, var);
            emitlocal(J, F, OP_SETLOCAL as i32, OP_SETVAR as i32, (*var).a);
            emit(J, F, OP_POP as i32);
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cstm(J: &mut js_State, mut F: *mut js_Function, mut stm: *mut js_Ast) {
    let mut target: *mut js_Ast = core::ptr::null_mut::<js_Ast>();
    let mut loop_0: i32 = 0;
    let mut cont: i32 = 0;
    let mut then: i32 = 0;
    let mut end: i32 = 0;
    emitline(J, F, stm);
    match (*stm).type_0 as libc::c_uint {
        1 => {}
        71 => {
            cstmlist(J, F, (*stm).a);
        }
        72 => {
            if (*F).script != 0 {
                emitline(J, F, stm);
                emit(J, F, OP_POP as i32);
                emit(J, F, OP_UNDEF as i32);
            }
        }
        73 => {
            cvarinit(J, F, (*stm).a);
        }
        74 => {
            if !((*stm).c).is_null() {
                jsC_cexp(J, F, (*stm).a);
                emitline(J, F, stm);
                then = emitjump(J, F, OP_JTRUE as i32);
                cstm(J, F, (*stm).c);
                emitline(J, F, stm);
                end = emitjump(J, F, OP_JUMP as i32);
                label(J, F, then);
                cstm(J, F, (*stm).b);
                label(J, F, end);
            } else {
                jsC_cexp(J, F, (*stm).a);
                emitline(J, F, stm);
                end = emitjump(J, F, OP_JFALSE as i32);
                cstm(J, F, (*stm).b);
                label(J, F, end);
            }
        }
        75 => {
            loop_0 = here(J, F);
            cstm(J, F, (*stm).a);
            cont = here(J, F);
            jsC_cexp(J, F, (*stm).b);
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JTRUE as i32, loop_0);
            let baddr = here(J, F);
            labeljumps(J, F, stm, baddr, cont);
        }
        76 => {
            loop_0 = here(J, F);
            jsC_cexp(J, F, (*stm).a);
            emitline(J, F, stm);
            end = emitjump(J, F, OP_JFALSE as i32);
            cstm(J, F, (*stm).b);
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JUMP as i32, loop_0);
            label(J, F, end);
            let baddr = here(J, F);
            labeljumps(J, F, stm, baddr, loop_0);
        }
        77 | 78 => {
            if (*stm).type_0 as libc::c_uint == STM_FOR_VAR as i32 as libc::c_uint {
                cvarinit(J, F, (*stm).a);
            } else if !((*stm).a).is_null() {
                jsC_cexp(J, F, (*stm).a);
                emit(J, F, OP_POP as i32);
            }
            loop_0 = here(J, F);
            if !((*stm).b).is_null() {
                jsC_cexp(J, F, (*stm).b);
                emitline(J, F, stm);
                end = emitjump(J, F, OP_JFALSE as i32);
            } else {
                end = 0;
            }
            cstm(J, F, (*stm).d);
            cont = here(J, F);
            if !((*stm).c).is_null() {
                jsC_cexp(J, F, (*stm).c);
                emit(J, F, OP_POP as i32);
            }
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JUMP as i32, loop_0);
            if end != 0 {
                label(J, F, end);
            }
            let baddr = here(J, F);
            labeljumps(J, F, stm, baddr, cont);
        }
        79 | 80 => {
            jsC_cexp(J, F, (*stm).b);
            emitline(J, F, stm);
            emit(J, F, OP_ITERATOR as i32);
            loop_0 = here(J, F);
            emitline(J, F, stm);
            emit(J, F, OP_NEXTITER as i32);
            end = emitjump(J, F, OP_JFALSE as i32);
            cassignforin(J, F, stm);
            if (*F).script != 0 {
                emit(J, F, OP_ROT2 as i32);
                cstm(J, F, (*stm).c);
                emit(J, F, OP_ROT2 as i32);
            } else {
                cstm(J, F, (*stm).c);
            }
            emitline(J, F, stm);
            emitjumpto(J, F, OP_JUMP as i32, loop_0);
            label(J, F, end);
            let baddr = here(J, F);
            labeljumps(J, F, stm, baddr, loop_0);
        }
        85 => {
            cswitch(J, F, (*stm).a, (*stm).b);
            let baddr = here(J, F);
            labeljumps(J, F, stm, baddr, 0);
        }
        89 => {
            cstm(J, F, (*stm).b);
            while (*stm).type_0 as libc::c_uint == STM_LABEL as i32 as libc::c_uint {
                stm = (*stm).b;
            }
            if isloop((*stm).type_0) == 0
                && (*stm).type_0 as libc::c_uint != STM_SWITCH as i32 as libc::c_uint
            {
                let baddr = here(J, F);
                labeljumps(J, F, stm, baddr, 0);
            }
        }
        82 => {
            if !((*stm).a).is_null() {
                checkfutureword(J, F, (*stm).a);
                target = breaktarget(J, F, (*stm).parent, (*(*stm).a).string);
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"break label '%s' not found\0" as *const u8 as *const libc::c_char,
                        (*(*stm).a).string,
                    );
                }
            } else {
                target = breaktarget(J, F, (*stm).parent, core::ptr::null::<libc::c_char>());
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"unlabelled break must be inside loop or switch\0" as *const u8
                            as *const libc::c_char,
                    );
                }
            }
            cexit(J, F, STM_BREAK, stm, target);
            emitline(J, F, stm);
            let inst = emitjump(J, F, OP_JUMP as i32);
            addjump(J, F, STM_BREAK, target, inst);
        }
        81 => {
            if !((*stm).a).is_null() {
                checkfutureword(J, F, (*stm).a);
                target = continuetarget(J, F, (*stm).parent, (*(*stm).a).string);
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"continue label '%s' not found\0" as *const u8 as *const libc::c_char,
                        (*(*stm).a).string,
                    );
                }
            } else {
                target = continuetarget(J, F, (*stm).parent, core::ptr::null::<libc::c_char>());
                if target.is_null() {
                    jsC_error(
                        J,
                        stm,
                        b"continue must be inside loop\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
            cexit(J, F, STM_CONTINUE, stm, target);
            emitline(J, F, stm);
            let inst = emitjump(J, F, OP_JUMP as i32);
            addjump(J, F, STM_CONTINUE, target, inst);
        }
        83 => {
            if !((*stm).a).is_null() {
                jsC_cexp(J, F, (*stm).a);
            } else {
                emit(J, F, OP_UNDEF as i32);
            }
            target = returntarget(J, F, (*stm).parent);
            if target.is_null() {
                jsC_error(
                    J,
                    stm,
                    b"return not in function\0" as *const u8 as *const libc::c_char,
                );
            }
            cexit(J, F, STM_RETURN, stm, target);
            emitline(J, F, stm);
            emit(J, F, OP_RETURN as i32);
        }
        86 => {
            jsC_cexp(J, F, (*stm).a);
            emitline(J, F, stm);
            emit(J, F, OP_THROW as i32);
        }
        84 => {
            (*F).lightweight = 0;
            if (*F).strict != 0 {
                jsC_error(
                    J,
                    (*stm).a,
                    b"'with' statements are not allowed in strict mode\0" as *const u8
                        as *const libc::c_char,
                );
            }
            jsC_cexp(J, F, (*stm).a);
            emitline(J, F, stm);
            emit(J, F, OP_WITH as i32);
            cstm(J, F, (*stm).b);
            emitline(J, F, stm);
            emit(J, F, OP_ENDWITH as i32);
        }
        87 => {
            emitline(J, F, stm);
            if !((*stm).b).is_null() && !((*stm).c).is_null() {
                (*F).lightweight = 0;
                if !((*stm).d).is_null() {
                    ctrycatchfinally(J, F, (*stm).a, (*stm).b, (*stm).c, (*stm).d);
                } else {
                    ctrycatch(J, F, (*stm).a, (*stm).b, (*stm).c);
                }
            } else {
                ctryfinally(J, F, (*stm).a, (*stm).d);
            }
        }
        88 => {
            emitline(J, F, stm);
            emit(J, F, OP_DEBUGGER as i32);
        }
        _ => {
            if (*F).script != 0 {
                emitline(J, F, stm);
                emit(J, F, OP_POP as i32);
                jsC_cexp(J, F, stm);
            } else {
                jsC_cexp(J, F, stm);
                emitline(J, F, stm);
                emit(J, F, OP_POP as i32);
            }
        }
    };
}
unsafe extern "C" fn cstmlist(J: &mut js_State, mut F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        cstm(J, F, (*list).a);
        list = (*list).b;
    }
}
unsafe extern "C" fn listlength(mut list: *mut js_Ast) -> i32 {
    let mut n: i32 = 0;
    while !list.is_null() {
        n += 1;

        list = (*list).b;
    }
    n
}
unsafe extern "C" fn cparams(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut list: *mut js_Ast,
    mut fname: *mut js_Ast,
) {
    (*F).numparams = listlength(list);
    while !list.is_null() {
        checkfutureword(J, F, (*list).a);
        addlocal(J, F, (*list).a, 0);
        list = (*list).b;
    }
}
unsafe extern "C" fn cvardecs(J: &mut js_State, mut F: *mut js_Function, mut node: *mut js_Ast) {
    if (*node).type_0 as libc::c_uint == AST_LIST as i32 as libc::c_uint {
        while !node.is_null() {
            cvardecs(J, F, (*node).a);
            node = (*node).b;
        }
        return;
    }
    if isfun((*node).type_0) != 0 {
        return;
    }
    if (*node).type_0 as libc::c_uint == EXP_VAR as i32 as libc::c_uint {
        checkfutureword(J, F, (*node).a);
        addlocal(J, F, (*node).a, 1);
    }
    if !((*node).a).is_null() {
        cvardecs(J, F, (*node).a);
    }
    if !((*node).b).is_null() {
        cvardecs(J, F, (*node).b);
    }
    if !((*node).c).is_null() {
        cvardecs(J, F, (*node).c);
    }
    if !((*node).d).is_null() {
        cvardecs(J, F, (*node).d);
    }
}
unsafe extern "C" fn cfundecs(J: &mut js_State, mut F: *mut js_Function, mut list: *mut js_Ast) {
    while !list.is_null() {
        let mut stm: *mut js_Ast = (*list).a;
        if (*stm).type_0 as libc::c_uint == AST_FUNDEC as i32 as libc::c_uint {
            emitline(J, F, stm);
            let fun = newfun(
                J,
                (*stm).line,
                (*stm).a,
                (*stm).b,
                (*stm).c,
                0,
                (*F).strict,
                0,
            );
            emitfunction(J, F, fun);
            emitline(J, F, stm);
            emit(J, F, OP_SETLOCAL as i32);
            let value = addlocal(J, F, (*stm).a, 1);
            emitarg(J, F, value);
            emit(J, F, OP_POP as i32);
        }
        list = (*list).b;
    }
}
unsafe extern "C" fn cfunbody(
    J: &mut js_State,
    mut F: *mut js_Function,
    mut name: *mut js_Ast,
    mut params: *mut js_Ast,
    mut body: *mut js_Ast,
    mut is_fun_exp: i32,
) {
    (*F).lightweight = 1;
    (*F).arguments = 0;
    if (*F).script != 0 {
        (*F).lightweight = 0;
    }
    if !body.is_null()
        && (*body).type_0 as libc::c_uint == AST_LIST as i32 as libc::c_uint
        && !((*body).a).is_null()
        && (*(*body).a).type_0 as libc::c_uint == EXP_STRING as i32 as libc::c_uint
        && strcmp(
            (*(*body).a).string,
            b"use strict\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        (*F).strict = 1;
    }
    (*F).lastline = (*F).line;
    cparams(J, F, params, name);
    if !body.is_null() {
        cvardecs(J, F, body);
        cfundecs(J, F, body);
    }
    if !name.is_null() {
        checkfutureword(J, F, name);
        if is_fun_exp != 0 && findlocal(J, F, (*name).string) < 0 {
            emit(J, F, OP_CURRENT as i32);
            emit(J, F, OP_SETLOCAL as i32);
            let value = addlocal(J, F, name, 1);
            emitarg(J, F, value);
            emit(J, F, OP_POP as i32);
        }
    }
    if (*F).script != 0 {
        emit(J, F, OP_UNDEF as i32);
        cstmlist(J, F, body);
        emit(J, F, OP_RETURN as i32);
    } else {
        cstmlist(J, F, body);
        emit(J, F, OP_UNDEF as i32);
        emit(J, F, OP_RETURN as i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn jsC_compilefunction(
    J: &mut js_State,
    mut prog: *mut js_Ast,
) -> *mut js_Function {
    newfun(
        J,
        (*prog).line,
        (*prog).a,
        (*prog).b,
        (*prog).c,
        0,
        (*J).default_strict,
        1,
    )
}
#[no_mangle]
pub unsafe extern "C" fn jsC_compilescript(
    J: &mut js_State,
    mut prog: *mut js_Ast,
    mut default_strict: i32,
) -> *mut js_Function {
    newfun(
        J,
        if !prog.is_null() { (*prog).line } else { 0 },
        core::ptr::null_mut::<js_Ast>(),
        core::ptr::null_mut::<js_Ast>(),
        prog,
        1,
        default_strict,
        0,
    )
}
