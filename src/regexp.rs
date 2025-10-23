use crate::*;

unsafe extern "C" fn die(mut g: *mut cstate, mut message: *const libc::c_char) {
    (*g).error = message;
    longjmp(((*g).kaboom).as_mut_ptr(), 1);
}
unsafe extern "C" fn canon(mut c: Rune) -> i32 {
    let mut u: Rune = jsU_toupperrune(c);
    if c >= 128 as i32 && u < 128 as i32 {
        return c;
    }
    u
}
unsafe extern "C" fn hex(mut g: *mut cstate, mut c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 0xa as i32;
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 0xa as i32;
    }
    die(
        g,
        b"invalid escape sequence\0" as *const u8 as *const libc::c_char,
    );
    0
}
unsafe extern "C" fn dec(mut g: *mut cstate, mut c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    die(
        g,
        b"invalid quantifier\0" as *const u8 as *const libc::c_char,
    );
    0
}
unsafe extern "C" fn isunicodeletter(mut c: i32) -> i32 {
    (c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || jsU_isalpharune(c) != 0) as i32
}
unsafe extern "C" fn nextrune(mut g: *mut cstate) -> i32 {
    if *(*g).source == 0 {
        (*g).yychar = -(1 as i32);
        return 0;
    }
    (*g).source = ((*g).source).offset(jsU_chartorune(&mut (*g).yychar, (*g).source) as isize);
    if (*g).yychar == '\\' as i32 {
        if *(*g).source == 0 {
            die(
                g,
                b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
            );
        }
        (*g).source = ((*g).source).offset(jsU_chartorune(&mut (*g).yychar, (*g).source) as isize);
        match (*g).yychar {
            102 => {
                (*g).yychar = '\u{c}' as i32;
                return 0;
            }
            110 => {
                (*g).yychar = '\n' as i32;
                return 0;
            }
            114 => {
                (*g).yychar = '\r' as i32;
                return 0;
            }
            116 => {
                (*g).yychar = '\t' as i32;
                return 0;
            }
            118 => {
                (*g).yychar = '\u{b}' as i32;
                return 0;
            }
            99 => {
                if *((*g).source).offset(0 as isize) == 0 {
                    die(
                        g,
                        b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh126 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = *fresh126 as i32 & 31 as i32;
                return 0;
            }
            120 => {
                if *((*g).source).offset(0 as isize) == 0
                    || *((*g).source).offset(1 as i32 as isize) == 0
                {
                    die(
                        g,
                        b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh127 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = hex(g, *fresh127 as i32) << 4;
                let fresh128 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh128 as i32);
                if (*g).yychar == 0 {
                    (*g).yychar = '0' as i32;
                    return 1;
                }
                return 0;
            }
            117 => {
                if *((*g).source).offset(0 as isize) == 0
                    || *((*g).source).offset(1 as i32 as isize) == 0
                    || *((*g).source).offset(2 as i32 as isize) == 0
                    || *((*g).source).offset(3 as i32 as isize) == 0
                {
                    die(
                        g,
                        b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
                    );
                }
                let fresh129 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = hex(g, *fresh129 as i32) << 12 as i32;
                let fresh130 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh130 as i32) << 8;
                let fresh131 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh131 as i32) << 4;
                let fresh132 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar += hex(g, *fresh132 as i32);
                if (*g).yychar == 0 {
                    (*g).yychar = '0' as i32;
                    return 1;
                }
                return 0;
            }
            0 => {
                (*g).yychar = '0' as i32;
                return 1;
            }
            _ => {}
        }
        if !(strchr(
            b"BbDdSsWw^$\\.*+?()[]{}|-0123456789\0" as *const u8 as *const libc::c_char,
            (*g).yychar,
        ))
        .is_null()
        {
            return 1;
        }
        if isunicodeletter((*g).yychar) != 0 || (*g).yychar == '_' as i32 {
            die(
                g,
                b"invalid escape character\0" as *const u8 as *const libc::c_char,
            );
        }
        return 0;
    }
    0
}
unsafe extern "C" fn lexcount(mut g: *mut cstate) -> i32 {
    let fresh133 = (*g).source;
    (*g).source = ((*g).source).offset(1);
    (*g).yychar = *fresh133 as Rune;
    (*g).yymin = dec(g, (*g).yychar);
    let fresh134 = (*g).source;
    (*g).source = ((*g).source).offset(1);
    (*g).yychar = *fresh134 as Rune;
    while (*g).yychar != ',' as i32 && (*g).yychar != '}' as i32 {
        (*g).yymin = (*g).yymin * 10 + dec(g, (*g).yychar);
        let fresh135 = (*g).source;
        (*g).source = ((*g).source).offset(1);
        (*g).yychar = *fresh135 as Rune;
        if (*g).yymin >= 255 as i32 {
            die(g, b"numeric overflow\0" as *const u8 as *const libc::c_char);
        }
    }
    if (*g).yychar == ',' as i32 {
        let fresh136 = (*g).source;
        (*g).source = ((*g).source).offset(1);
        (*g).yychar = *fresh136 as Rune;
        if (*g).yychar == '}' as i32 {
            (*g).yymax = 255 as i32;
        } else {
            (*g).yymax = dec(g, (*g).yychar);
            let fresh137 = (*g).source;
            (*g).source = ((*g).source).offset(1);
            (*g).yychar = *fresh137 as Rune;
            while (*g).yychar != '}' as i32 {
                (*g).yymax = (*g).yymax * 10 + dec(g, (*g).yychar);
                let fresh138 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = *fresh138 as Rune;
                if (*g).yymax >= 255 as i32 {
                    die(g, b"numeric overflow\0" as *const u8 as *const libc::c_char);
                }
            }
        }
    } else {
        (*g).yymax = (*g).yymin;
    }
    L_COUNT as i32
}
unsafe extern "C" fn newcclass(mut g: *mut cstate) {
    if (*g).ncclass >= 128 as i32 {
        die(
            g,
            b"too many character classes\0" as *const u8 as *const libc::c_char,
        );
    }
    let fresh139 = (*g).ncclass;
    (*g).ncclass += 1;
    (*g).yycc = ((*g).cclass).as_mut_ptr().offset(fresh139 as isize);
    (*(*g).yycc).end = ((*(*g).yycc).spans).as_mut_ptr();
}
unsafe extern "C" fn addrange(mut g: *mut cstate, mut a: Rune, mut b: Rune) {
    let mut cc: *mut Reclass = (*g).yycc;
    let mut p: *mut Rune = core::ptr::null_mut::<Rune>();
    if a > b {
        die(
            g,
            b"invalid character class range\0" as *const u8 as *const libc::c_char,
        );
    }
    p = ((*cc).spans).as_mut_ptr();
    while p < (*cc).end {
        if a >= *p.offset(0 as isize) && b <= *p.offset(1 as i32 as isize) {
            return;
        }
        if a < *p.offset(0 as isize) && b >= *p.offset(1 as i32 as isize) {
            *p.offset(0 as isize) = a;
            *p.offset(1 as i32 as isize) = b;
            return;
        }
        if b >= *p.offset(0 as isize) - 1
            && b <= *p.offset(1 as i32 as isize)
            && a < *p.offset(0 as isize)
        {
            *p.offset(0 as isize) = a;
            return;
        }
        if a >= *p.offset(0 as isize)
            && a <= *p.offset(1 as i32 as isize) + 1
            && b > *p.offset(1 as i32 as isize)
        {
            *p.offset(1 as i32 as isize) = b;
            return;
        }
        p = p.offset(2 as i32 as isize);
    }
    if ((*cc).end).offset(2 as i32 as isize)
        >= ((*cc).spans).as_mut_ptr().offset(
            (::core::mem::size_of::<[Rune; 64]>() as u32)
                .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32 as isize,
        )
    {
        die(
            g,
            b"too many character class ranges\0" as *const u8 as *const libc::c_char,
        );
    }
    let fresh140 = (*cc).end;
    (*cc).end = ((*cc).end).offset(1);
    *fresh140 = a;
    let fresh141 = (*cc).end;
    (*cc).end = ((*cc).end).offset(1);
    *fresh141 = b;
}
unsafe extern "C" fn addranges_d(mut g: *mut cstate) {
    addrange(g, '0' as i32, '9' as i32);
}
unsafe extern "C" fn addranges_D(mut g: *mut cstate) {
    addrange(g, 0, '0' as i32 - 1);
    addrange(g, '9' as i32 + 1, 0xffff as i32);
}
unsafe extern "C" fn addranges_s(mut g: *mut cstate) {
    addrange(g, 0x9 as i32, 0xd as i32);
    addrange(g, 0x20, 0x20);
    addrange(g, 0xa0, 0xa0);
    addrange(g, 0x2028 as i32, 0x2029 as i32);
    addrange(g, 0xfeff as i32, 0xfeff as i32);
}
unsafe extern "C" fn addranges_S(mut g: *mut cstate) {
    addrange(g, 0, 0x9 as i32 - 1);
    addrange(g, 0xd as i32 + 1, 0x20 - 1);
    addrange(g, 0x20 + 1, 0xa0 - 1);
    addrange(g, 0xa0 + 1, 0x2028 as i32 - 1);
    addrange(g, 0x2029 as i32 + 1, 0xfeff as i32 - 1);
    addrange(g, 0xfeff as i32 + 1, 0xffff as i32);
}
unsafe extern "C" fn addranges_w(mut g: *mut cstate) {
    addrange(g, '0' as i32, '9' as i32);
    addrange(g, 'A' as i32, 'Z' as i32);
    addrange(g, '_' as i32, '_' as i32);
    addrange(g, 'a' as i32, 'z' as i32);
}
unsafe extern "C" fn addranges_W(mut g: *mut cstate) {
    addrange(g, 0, '0' as i32 - 1);
    addrange(g, '9' as i32 + 1, 'A' as i32 - 1);
    addrange(g, 'Z' as i32 + 1, '_' as i32 - 1);
    addrange(g, '_' as i32 + 1, 'a' as i32 - 1);
    addrange(g, 'z' as i32 + 1, 0xffff as i32);
}
unsafe extern "C" fn lexclass(mut g: *mut cstate) -> i32 {
    let mut type_0: i32 = L_CCLASS as i32;
    let mut quoted: i32 = 0;
    let mut havesave: i32 = 0;
    let mut havedash: i32 = 0;
    let mut save: Rune = 0;
    newcclass(g);
    quoted = nextrune(g);
    if quoted == 0 && (*g).yychar == '^' as i32 {
        type_0 = L_NCCLASS as i32;
        quoted = nextrune(g);
    }
    havedash = 0;
    havesave = havedash;
    loop {
        if (*g).yychar == -(1 as i32) {
            die(
                g,
                b"unterminated character class\0" as *const u8 as *const libc::c_char,
            );
        }
        if quoted == 0 && (*g).yychar == ']' as i32 {
            break;
        }
        if quoted == 0 && (*g).yychar == '-' as i32 {
            if havesave != 0 {
                if havedash != 0 {
                    addrange(g, save, '-' as i32);
                    havedash = 0;
                    havesave = havedash;
                } else {
                    havedash = 1;
                }
            } else {
                save = '-' as i32;
                havesave = 1;
            }
        } else if quoted != 0
            && !(strchr(b"DSWdsw\0" as *const u8 as *const libc::c_char, (*g).yychar)).is_null()
        {
            if havesave != 0 {
                addrange(g, save, save);
                if havedash != 0 {
                    addrange(g, '-' as i32, '-' as i32);
                }
            }
            match (*g).yychar {
                100 => {
                    addranges_d(g);
                }
                115 => {
                    addranges_s(g);
                }
                119 => {
                    addranges_w(g);
                }
                68 => {
                    addranges_D(g);
                }
                83 => {
                    addranges_S(g);
                }
                87 => {
                    addranges_W(g);
                }
                _ => {}
            }
            havedash = 0;
            havesave = havedash;
        } else {
            if quoted != 0 {
                if (*g).yychar == 'b' as i32 {
                    (*g).yychar = '\u{8}' as i32;
                } else if (*g).yychar == '0' as i32 {
                    (*g).yychar = 0;
                }
            }
            if havesave != 0 {
                if havedash != 0 {
                    addrange(g, save, (*g).yychar);
                    havedash = 0;
                    havesave = havedash;
                } else {
                    addrange(g, save, save);
                    save = (*g).yychar;
                }
            } else {
                save = (*g).yychar;
                havesave = 1;
            }
        }
        quoted = nextrune(g);
    }
    if havesave != 0 {
        addrange(g, save, save);
        if havedash != 0 {
            addrange(g, '-' as i32, '-' as i32);
        }
    }
    type_0
}
unsafe extern "C" fn lex(mut g: *mut cstate) -> i32 {
    let mut quoted: i32 = nextrune(g);
    if quoted != 0 {
        match (*g).yychar {
            98 => return L_WORD as i32,
            66 => return L_NWORD as i32,
            100 => {
                newcclass(g);
                addranges_d(g);
                return L_CCLASS as i32;
            }
            115 => {
                newcclass(g);
                addranges_s(g);
                return L_CCLASS as i32;
            }
            119 => {
                newcclass(g);
                addranges_w(g);
                return L_CCLASS as i32;
            }
            68 => {
                newcclass(g);
                addranges_d(g);
                return L_NCCLASS as i32;
            }
            83 => {
                newcclass(g);
                addranges_s(g);
                return L_NCCLASS as i32;
            }
            87 => {
                newcclass(g);
                addranges_w(g);
                return L_NCCLASS as i32;
            }
            48 => {
                (*g).yychar = 0;
                return L_CHAR as i32;
            }
            _ => {}
        }
        if (*g).yychar >= '0' as i32 && (*g).yychar <= '9' as i32 {
            (*g).yychar -= '0' as i32;
            if *(*g).source as i32 >= '0' as i32 && *(*g).source as i32 <= '9' as i32 {
                let fresh142 = (*g).source;
                (*g).source = ((*g).source).offset(1);
                (*g).yychar = (*g).yychar * 10 + *fresh142 as i32 - '0' as i32;
            }
            return L_REF as i32;
        }
        return L_CHAR as i32;
    }
    match (*g).yychar {
        -1 | 36 | 41 | 42 | 43 | 46 | 63 | 94 | 124 => return (*g).yychar,
        _ => {}
    }
    if (*g).yychar == '{' as i32 {
        return lexcount(g);
    }
    if (*g).yychar == '[' as i32 {
        return lexclass(g);
    }
    if (*g).yychar == '(' as i32 {
        if *((*g).source).offset(0 as isize) as i32 == '?' as i32 {
            if *((*g).source).offset(1 as i32 as isize) as i32 == ':' as i32 {
                (*g).source = ((*g).source).offset(2 as i32 as isize);
                return L_NC as i32;
            }
            if *((*g).source).offset(1 as i32 as isize) as i32 == '=' as i32 {
                (*g).source = ((*g).source).offset(2 as i32 as isize);
                return L_PLA as i32;
            }
            if *((*g).source).offset(1 as i32 as isize) as i32 == '!' as i32 {
                (*g).source = ((*g).source).offset(2 as i32 as isize);
                return L_NLA as i32;
            }
        }
        return '(' as i32;
    }
    L_CHAR as i32
}
unsafe extern "C" fn newnode(mut g: *mut cstate, mut type_0: i32) -> *mut Renode {
    let fresh143 = (*g).pend;
    (*g).pend = ((*g).pend).offset(1);
    let mut node: *mut Renode = fresh143;
    (*node).type_0 = type_0 as libc::c_uchar;
    (*node).cc = -(1 as i32);
    (*node).c = 0;
    (*node).ng = 0;
    (*node).m = 0;
    (*node).n = 0;
    (*node).y = core::ptr::null_mut::<Renode>();
    (*node).x = (*node).y;
    node
}
unsafe extern "C" fn empty(mut node: *mut Renode) -> i32 {
    if node.is_null() {
        return 1;
    }
    match (*node).type_0 {
        0 => (empty((*node).x) != 0 && empty((*node).y) != 0) as i32,
        1 => (empty((*node).x) != 0 || empty((*node).y) != 0) as i32,
        2 => (empty((*node).x) != 0 || (*node).m as i32 == 0) as i32,
        7 => empty((*node).x),
        14 => empty((*node).x),
        10..=13 => 0,
        _ => 1,
    }
}
unsafe extern "C" fn newrep(
    mut g: *mut cstate,
    mut atom: *mut Renode,
    mut ng: i32,
    mut min: i32,
    mut max: i32,
) -> *mut Renode {
    let mut rep: *mut Renode = newnode(g, P_REP as i32);
    if max == 255 as i32 && empty(atom) != 0 {
        die(
            g,
            b"infinite loop matching the empty string\0" as *const u8 as *const libc::c_char,
        );
    }
    (*rep).ng = ng as libc::c_uchar;
    (*rep).m = min as libc::c_uchar;
    (*rep).n = max as libc::c_uchar;
    (*rep).x = atom;
    rep
}
unsafe extern "C" fn regnext(mut g: *mut cstate) {
    (*g).lookahead = lex(g);
}
unsafe extern "C" fn regaccept(mut g: *mut cstate, mut t: i32) -> i32 {
    if (*g).lookahead == t {
        regnext(g);
        return 1;
    }
    0
}
unsafe extern "C" fn parseatom(mut g: *mut cstate) -> *mut Renode {
    let mut atom: *mut Renode = core::ptr::null_mut::<Renode>();
    if (*g).lookahead == L_CHAR as i32 {
        atom = newnode(g, P_CHAR as i32);
        (*atom).c = (*g).yychar;
        regnext(g);
        return atom;
    }
    if (*g).lookahead == L_CCLASS as i32 {
        atom = newnode(g, P_CCLASS as i32);
        (*atom).cc = ((*g).yycc).offset_from(((*g).cclass).as_mut_ptr()) as i32;
        regnext(g);
        return atom;
    }
    if (*g).lookahead == L_NCCLASS as i32 {
        atom = newnode(g, P_NCCLASS as i32);
        (*atom).cc = ((*g).yycc).offset_from(((*g).cclass).as_mut_ptr()) as i32;
        regnext(g);
        return atom;
    }
    if (*g).lookahead == L_REF as i32 {
        atom = newnode(g, P_REF as i32);
        if (*g).yychar == 0
            || (*g).yychar >= (*g).nsub
            || ((*g).sub[(*g).yychar as usize]).is_null()
        {
            die(
                g,
                b"invalid back-reference\0" as *const u8 as *const libc::c_char,
            );
        }
        (*atom).n = (*g).yychar as libc::c_uchar;
        (*atom).x = (*g).sub[(*g).yychar as usize];
        regnext(g);
        return atom;
    }
    if regaccept(g, '.' as i32) != 0 {
        return newnode(g, P_ANY as i32);
    }
    if regaccept(g, '(' as i32) != 0 {
        atom = newnode(g, P_PAR as i32);
        if (*g).nsub == 16 as i32 {
            die(
                g,
                b"too many captures\0" as *const u8 as *const libc::c_char,
            );
        }
        let fresh144 = (*g).nsub;
        (*g).nsub += 1;
        (*atom).n = fresh144 as libc::c_uchar;
        (*atom).x = parsealt(g);
        (*g).sub[(*atom).n as usize] = atom;
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    if regaccept(g, L_NC as i32) != 0 {
        atom = parsealt(g);
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    if regaccept(g, L_PLA as i32) != 0 {
        atom = newnode(g, P_PLA as i32);
        (*atom).x = parsealt(g);
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    if regaccept(g, L_NLA as i32) != 0 {
        atom = newnode(g, P_NLA as i32);
        (*atom).x = parsealt(g);
        if regaccept(g, ')' as i32) == 0 {
            die(g, b"unmatched '('\0" as *const u8 as *const libc::c_char);
        }
        return atom;
    }
    die(g, b"syntax error\0" as *const u8 as *const libc::c_char);
    core::ptr::null_mut::<Renode>()
}
unsafe extern "C" fn parserep(mut g: *mut cstate) -> *mut Renode {
    let mut atom: *mut Renode = core::ptr::null_mut::<Renode>();
    if regaccept(g, '^' as i32) != 0 {
        return newnode(g, P_BOL as i32);
    }
    if regaccept(g, '$' as i32) != 0 {
        return newnode(g, P_EOL as i32);
    }
    if regaccept(g, L_WORD as i32) != 0 {
        return newnode(g, P_WORD as i32);
    }
    if regaccept(g, L_NWORD as i32) != 0 {
        return newnode(g, P_NWORD as i32);
    }
    atom = parseatom(g);
    if (*g).lookahead == L_COUNT as i32 {
        let mut min: i32 = (*g).yymin;
        let mut max: i32 = (*g).yymax;
        regnext(g);
        if max < min {
            die(
                g,
                b"invalid quantifier\0" as *const u8 as *const libc::c_char,
            );
        }
        return newrep(g, atom, regaccept(g, '?' as i32), min, max);
    }
    if regaccept(g, '*' as i32) != 0 {
        return newrep(g, atom, regaccept(g, '?' as i32), 0, 255 as i32);
    }
    if regaccept(g, '+' as i32) != 0 {
        return newrep(g, atom, regaccept(g, '?' as i32), 1, 255 as i32);
    }
    if regaccept(g, '?' as i32) != 0 {
        return newrep(g, atom, regaccept(g, '?' as i32), 0, 1);
    }
    atom
}
unsafe extern "C" fn parsecat(mut g: *mut cstate) -> *mut Renode {
    let mut cat: *mut Renode = core::ptr::null_mut::<Renode>();
    let mut head: *mut Renode = core::ptr::null_mut::<Renode>();
    let mut tail: *mut *mut Renode = core::ptr::null_mut::<*mut Renode>();
    let g = &mut *g;
    if g.lookahead != -1 && g.lookahead != '|' as i32 && g.lookahead != ')' as i32 {
        head = parserep(g);
        tail = &mut head;
        while g.lookahead != -1 && g.lookahead != '|' as i32 && g.lookahead != ')' as i32 {
            cat = newnode(g, P_CAT as i32);
            (*cat).x = *tail;
            (*cat).y = parserep(g);
            *tail = cat;
            tail = &mut (*cat).y;
        }
        return head;
    }
    core::ptr::null_mut::<Renode>()
}
unsafe extern "C" fn parsealt(mut g: *mut cstate) -> *mut Renode {
    let mut alt: *mut Renode = core::ptr::null_mut::<Renode>();
    let mut x: *mut Renode = core::ptr::null_mut::<Renode>();
    alt = parsecat(g);
    while regaccept(g, '|' as i32) != 0 {
        x = alt;
        alt = newnode(g, P_ALT as i32);
        (*alt).x = x;
        (*alt).y = parsecat(g);
    }
    alt
}
unsafe extern "C" fn count(mut g: *mut cstate, mut node: *mut Renode, mut depth: i32) -> i32 {
    let mut min: i32 = 0;
    let mut max: i32 = 0;
    let mut n: i32 = 0;
    if node.is_null() {
        return 0;
    }
    depth += 1;
    if depth > 1024 as i32 {
        die(g, b"stack overflow\0" as *const u8 as *const libc::c_char);
    }
    match (*node).type_0 {
        0 => count(g, (*node).x, depth) + count(g, (*node).y, depth),
        1 => count(g, (*node).x, depth) + count(g, (*node).y, depth) + 2,
        2 => {
            min = (*node).m as i32;
            max = (*node).n as i32;
            if min == max {
                n = count(g, (*node).x, depth) * min;
            } else if max < 255 as i32 {
                n = count(g, (*node).x, depth) * max + (max - min);
            } else {
                n = count(g, (*node).x, depth) * (min + 1) + 2;
            }
            if n < 0 || n > (32 as i32) << 10 {
                die(
                    g,
                    b"program too large\0" as *const u8 as *const libc::c_char,
                );
            }
            n
        }
        7 => count(g, (*node).x, depth) + 2,
        8 => count(g, (*node).x, depth) + 2,
        9 => count(g, (*node).x, depth) + 2,
        _ => 1,
    }
}
unsafe extern "C" fn regemit(mut prog: *mut Reprog, mut opcode: i32) -> *mut Reinst {
    let fresh145 = (*prog).end;
    (*prog).end = ((*prog).end).offset(1);
    let mut inst: *mut Reinst = fresh145;
    (*inst).opcode = opcode as libc::c_uchar;
    (*inst).n = 0;
    (*inst).c = 0;
    (*inst).cc = core::ptr::null_mut::<Reclass>();
    (*inst).y = core::ptr::null_mut::<Reinst>();
    (*inst).x = (*inst).y;
    inst
}
unsafe extern "C" fn compile(mut prog: *mut Reprog, mut node: *mut Renode) {
    let mut current_block: u64;
    let mut inst: *mut Reinst = core::ptr::null_mut::<Reinst>();
    let mut split_0: *mut Reinst = core::ptr::null_mut::<Reinst>();
    let mut jump: *mut Reinst = core::ptr::null_mut::<Reinst>();
    let mut i: i32 = 0;
    loop {
        if node.is_null() {
            return;
        }
        match (*node).type_0 {
            0 => {
                compile(prog, (*node).x);
                node = (*node).y;
            }
            1 => {
                split_0 = regemit(prog, I_SPLIT as i32);
                compile(prog, (*node).x);
                jump = regemit(prog, I_JUMP as i32);
                compile(prog, (*node).y);
                (*split_0).x = split_0.offset(1 as i32 as isize);
                (*split_0).y = jump.offset(1 as i32 as isize);
                (*jump).x = (*prog).end;
                current_block = 13853033528615664019;
                break;
            }
            2 => {
                inst = core::ptr::null_mut::<Reinst>();
                i = 0;
                while i < (*node).m as i32 {
                    inst = (*prog).end;
                    compile(prog, (*node).x);
                    i += 1;
                }
                if (*node).m as i32 == (*node).n as i32 {
                    current_block = 13853033528615664019;
                    break;
                } else {
                    current_block = 8831408221741692167;
                    break;
                }
            }
            3 => {
                regemit(prog, I_BOL as i32);
                current_block = 13853033528615664019;
                break;
            }
            4 => {
                regemit(prog, I_EOL as i32);
                current_block = 13853033528615664019;
                break;
            }
            5 => {
                regemit(prog, I_WORD as i32);
                current_block = 13853033528615664019;
                break;
            }
            6 => {
                regemit(prog, I_NWORD as i32);
                current_block = 13853033528615664019;
                break;
            }
            7 => {
                inst = regemit(prog, I_LPAR as i32);
                (*inst).n = (*node).n;
                compile(prog, (*node).x);
                inst = regemit(prog, I_RPAR as i32);
                (*inst).n = (*node).n;
                current_block = 13853033528615664019;
                break;
            }
            8 => {
                split_0 = regemit(prog, I_PLA as i32);
                compile(prog, (*node).x);
                regemit(prog, I_END as i32);
                (*split_0).x = split_0.offset(1 as i32 as isize);
                (*split_0).y = (*prog).end;
                current_block = 13853033528615664019;
                break;
            }
            9 => {
                split_0 = regemit(prog, I_NLA as i32);
                compile(prog, (*node).x);
                regemit(prog, I_END as i32);
                (*split_0).x = split_0.offset(1 as i32 as isize);
                (*split_0).y = (*prog).end;
                current_block = 13853033528615664019;
                break;
            }
            10 => {
                regemit(prog, I_ANY as i32);
                current_block = 13853033528615664019;
                break;
            }
            11 => {
                inst = regemit(prog, I_CHAR as i32);
                (*inst).c = if (*prog).flags & REG_ICASE as i32 != 0 {
                    canon((*node).c)
                } else {
                    (*node).c
                };
                current_block = 13853033528615664019;
                break;
            }
            12 => {
                inst = regemit(prog, I_CCLASS as i32);
                (*inst).cc = ((*prog).cclass).offset((*node).cc as isize);
                current_block = 13853033528615664019;
                break;
            }
            13 => {
                inst = regemit(prog, I_NCCLASS as i32);
                (*inst).cc = ((*prog).cclass).offset((*node).cc as isize);
                current_block = 13853033528615664019;
                break;
            }
            14 => {
                inst = regemit(prog, I_REF as i32);
                (*inst).n = (*node).n;
                current_block = 13853033528615664019;
                break;
            }
            _ => {
                current_block = 13853033528615664019;
                break;
            }
        }
    }
    if current_block == 8831408221741692167 {
        if ((*node).n as i32) < 255 as i32 {
            i = (*node).m as i32;
            while i < (*node).n as i32 {
                split_0 = regemit(prog, I_SPLIT as i32);
                compile(prog, (*node).x);
                if (*node).ng != 0 {
                    (*split_0).y = split_0.offset(1 as i32 as isize);
                    (*split_0).x = (*prog).end;
                } else {
                    (*split_0).x = split_0.offset(1 as i32 as isize);
                    (*split_0).y = (*prog).end;
                }
                i += 1;
            }
        } else if (*node).m as i32 == 0 {
            split_0 = regemit(prog, I_SPLIT as i32);
            compile(prog, (*node).x);
            jump = regemit(prog, I_JUMP as i32);
            if (*node).ng != 0 {
                (*split_0).y = split_0.offset(1 as i32 as isize);
                (*split_0).x = (*prog).end;
            } else {
                (*split_0).x = split_0.offset(1 as i32 as isize);
                (*split_0).y = (*prog).end;
            }
            (*jump).x = split_0;
        } else {
            split_0 = regemit(prog, I_SPLIT as i32);
            if (*node).ng != 0 {
                (*split_0).y = inst;
                (*split_0).x = (*prog).end;
            } else {
                (*split_0).x = inst;
                (*split_0).y = (*prog).end;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn js_regcompx(
    mut alloc: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, i32) -> *mut libc::c_void,
    >,
    mut ctx: *mut libc::c_void,
    mut pattern: *const libc::c_char,
    mut cflags: i32,
    mut errorp: *mut *const libc::c_char,
) -> *mut Reprog {
    let mut g: cstate = cstate {
        prog: core::ptr::null_mut::<Reprog>(),
        pstart: core::ptr::null_mut::<Renode>(),
        pend: core::ptr::null_mut::<Renode>(),
        source: core::ptr::null::<libc::c_char>(),
        ncclass: 0,
        nsub: 0,
        sub: [core::ptr::null_mut::<Renode>(); 16],
        lookahead: 0,
        yychar: 0,
        yycc: core::ptr::null_mut::<Reclass>(),
        yymin: 0,
        yymax: 0,
        error: core::ptr::null::<libc::c_char>(),
        kaboom: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        cclass: [Reclass {
            end: core::ptr::null_mut::<Rune>(),
            spans: [0; 64],
        }; 128],
    };
    let mut node: *mut Renode = core::ptr::null_mut::<Renode>();
    let mut split_0: *mut Reinst = core::ptr::null_mut::<Reinst>();
    let mut jump: *mut Reinst = core::ptr::null_mut::<Reinst>();
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    g.pstart = core::ptr::null_mut::<Renode>();
    g.prog = core::ptr::null_mut::<Reprog>();
    if _setjmp((g.kaboom).as_mut_ptr()) != 0 {
        if !errorp.is_null() {
            *errorp = g.error;
        }
        alloc.expect("non-null function pointer")(ctx, g.pstart as *mut libc::c_void, 0);
        if !(g.prog).is_null() {
            alloc.expect("non-null function pointer")(
                ctx,
                (*g.prog).cclass as *mut libc::c_void,
                0,
            );
            alloc.expect("non-null function pointer")(ctx, (*g.prog).start as *mut libc::c_void, 0);
            alloc.expect("non-null function pointer")(ctx, g.prog as *mut libc::c_void, 0);
        }
        return core::ptr::null_mut::<Reprog>();
    }
    g.prog = alloc.expect("non-null function pointer")(
        ctx,
        core::ptr::null_mut::<libc::c_void>(),
        ::core::mem::size_of::<Reprog>() as u32 as i32,
    ) as *mut Reprog;
    if (g.prog).is_null() {
        die(
            &mut g,
            b"cannot allocate regular expression\0" as *const u8 as *const libc::c_char,
        );
    }
    (*g.prog).start = core::ptr::null_mut::<Reinst>();
    (*g.prog).cclass = core::ptr::null_mut::<Reclass>();
    n = (strlen(pattern)).wrapping_mul(2 as i32 as u32) as i32;
    if n > (32 as i32) << 10 {
        die(
            &mut g,
            b"program too large\0" as *const u8 as *const libc::c_char,
        );
    }
    if n > 0 {
        g.pend = alloc.expect("non-null function pointer")(
            ctx,
            core::ptr::null_mut::<libc::c_void>(),
            (::core::mem::size_of::<Renode>() as u32).wrapping_mul(n as u32) as i32,
        ) as *mut Renode;
        g.pstart = g.pend;
        if (g.pstart).is_null() {
            die(
                &mut g,
                b"cannot allocate regular expression parse list\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    g.source = pattern;
    g.ncclass = 0;
    g.nsub = 1;
    i = 0;
    while i < 16 as i32 {
        g.sub[i as usize] = core::ptr::null_mut::<Renode>();
        i += 1;
    }
    (*g.prog).flags = cflags;
    regnext(&mut g);
    node = parsealt(&mut g);
    if g.lookahead == ')' as i32 {
        die(
            &mut g,
            b"unmatched ')'\0" as *const u8 as *const libc::c_char,
        );
    }
    if g.lookahead != -(1 as i32) {
        die(
            &mut g,
            b"syntax error\0" as *const u8 as *const libc::c_char,
        );
    }
    n = 6 + count(&mut g, node, 0);
    if n < 0 || n > (32 as i32) << 10 {
        die(
            &mut g,
            b"program too large\0" as *const u8 as *const libc::c_char,
        );
    }
    (*g.prog).nsub = g.nsub;
    (*g.prog).end = alloc.expect("non-null function pointer")(
        ctx,
        core::ptr::null_mut::<libc::c_void>(),
        (n as u32).wrapping_mul(::core::mem::size_of::<Reinst>() as u32) as i32,
    ) as *mut Reinst;
    (*g.prog).start = (*g.prog).end;
    if ((*g.prog).start).is_null() {
        die(
            &mut g,
            b"cannot allocate regular expression instruction list\0" as *const u8
                as *const libc::c_char,
        );
    }
    if g.ncclass > 0 {
        (*g.prog).cclass = alloc.expect("non-null function pointer")(
            ctx,
            core::ptr::null_mut::<libc::c_void>(),
            (g.ncclass as u32).wrapping_mul(::core::mem::size_of::<Reclass>() as u32) as i32,
        ) as *mut Reclass;
        if ((*g.prog).cclass).is_null() {
            die(
                &mut g,
                b"cannot allocate regular expression character class list\0" as *const u8
                    as *const libc::c_char,
            );
        }
        memcpy(
            (*g.prog).cclass as *mut libc::c_void,
            (g.cclass).as_mut_ptr() as *const libc::c_void,
            (g.ncclass as u32).wrapping_mul(::core::mem::size_of::<Reclass>() as u32),
        );
        i = 0;
        while i < g.ncclass {
            let fresh146 = &mut (*((*g.prog).cclass).offset(i as isize)).end;
            *fresh146 = ((*((*g.prog).cclass).offset(i as isize)).spans)
                .as_mut_ptr()
                .offset(
                    (g.cclass[i as usize].end)
                        .offset_from((g.cclass[i as usize].spans).as_mut_ptr())
                        as isize,
                );
            i += 1;
        }
    }
    split_0 = regemit(g.prog, I_SPLIT as i32);
    (*split_0).x = split_0.offset(3 as i32 as isize);
    (*split_0).y = split_0.offset(1 as i32 as isize);
    regemit(g.prog, I_ANYNL as i32);
    jump = regemit(g.prog, I_JUMP as i32);
    (*jump).x = split_0;
    regemit(g.prog, I_LPAR as i32);
    compile(g.prog, node);
    regemit(g.prog, I_RPAR as i32);
    regemit(g.prog, I_END as i32);
    alloc.expect("non-null function pointer")(ctx, g.pstart as *mut libc::c_void, 0);
    if !errorp.is_null() {
        *errorp = core::ptr::null::<libc::c_char>();
    }
    g.prog
}
#[no_mangle]
pub unsafe extern "C" fn js_regfreex(
    mut alloc: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void, i32) -> *mut libc::c_void,
    >,
    mut ctx: *mut libc::c_void,
    mut prog: *mut Reprog,
) {
    if !prog.is_null() {
        if !((*prog).cclass).is_null() {
            alloc.expect("non-null function pointer")(ctx, (*prog).cclass as *mut libc::c_void, 0);
        }
        alloc.expect("non-null function pointer")(ctx, (*prog).start as *mut libc::c_void, 0);
        alloc.expect("non-null function pointer")(ctx, prog as *mut libc::c_void, 0);
    }
}
unsafe extern "C" fn default_alloc(
    mut ctx: *mut libc::c_void,
    mut p: *mut libc::c_void,
    mut n: i32,
) -> *mut libc::c_void {
    if n == 0 {
        free(p);
        return core::ptr::null_mut::<libc::c_void>();
    }
    realloc(p, n as size_t)
}
#[no_mangle]
pub unsafe extern "C" fn js_regcomp(
    mut pattern: *const libc::c_char,
    mut cflags: i32,
    mut errorp: *mut *const libc::c_char,
) -> *mut Reprog {
    js_regcompx(
        Some(
            default_alloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    i32,
                ) -> *mut libc::c_void,
        ),
        core::ptr::null_mut::<libc::c_void>(),
        pattern,
        cflags,
        errorp,
    )
}
#[no_mangle]
pub unsafe extern "C" fn js_regfree(mut prog: *mut Reprog) {
    js_regfreex(
        Some(
            default_alloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                    i32,
                ) -> *mut libc::c_void,
        ),
        core::ptr::null_mut::<libc::c_void>(),
        prog,
    );
}
unsafe extern "C" fn isnewline(mut c: i32) -> i32 {
    (c == 0xa as i32 || c == 0xd as i32 || c == 0x2028 as i32 || c == 0x2029 as i32) as i32
}
unsafe extern "C" fn iswordchar(mut c: i32) -> i32 {
    (c == '_' as i32
        || c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c >= '0' as i32 && c <= '9' as i32) as i32
}
unsafe extern "C" fn incclass(mut cc: *mut Reclass, mut c: Rune) -> i32 {
    let mut p: *mut Rune = core::ptr::null_mut::<Rune>();
    p = ((*cc).spans).as_mut_ptr();
    while p < (*cc).end {
        if *p.offset(0 as isize) <= c && c <= *p.offset(1 as i32 as isize) {
            return 1;
        }
        p = p.offset(2 as i32 as isize);
    }
    0
}
unsafe extern "C" fn incclasscanon(mut cc: *mut Reclass, mut c: Rune) -> i32 {
    let mut p: *mut Rune = core::ptr::null_mut::<Rune>();
    let mut r: Rune = 0;
    p = ((*cc).spans).as_mut_ptr();
    while p < (*cc).end {
        r = *p.offset(0 as isize);
        while r <= *p.offset(1 as i32 as isize) {
            if c == canon(r) {
                return 1;
            }
            r += 1;
        }
        p = p.offset(2 as i32 as isize);
    }
    0
}
unsafe extern "C" fn strncmpcanon(
    mut a: *const libc::c_char,
    mut b: *const libc::c_char,
    mut n: i32,
) -> i32 {
    let mut ra: Rune = 0;
    let mut rb: Rune = 0;
    let mut c: i32 = 0;
    loop {
        let fresh147 = n;
        n -= 1;
        if fresh147 == 0 {
            break;
        }
        if *a == 0 {
            return -(1 as i32);
        }
        if *b == 0 {
            return 1;
        }
        a = a.offset(jsU_chartorune(&mut ra, a) as isize);
        b = b.offset(jsU_chartorune(&mut rb, b) as isize);
        c = canon(ra) - canon(rb);
        if c != 0 {
            return c;
        }
    }
    0
}
unsafe extern "C" fn match_0(
    mut pc: *mut Reinst,
    mut sp: *const libc::c_char,
    mut bol: *const libc::c_char,
    mut flags: i32,
    mut out: *mut Resub,
    mut depth: i32,
) -> i32 {
    let mut scratch: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    let mut result: i32 = 0;
    let mut i: i32 = 0;
    let mut c: Rune = 0;
    if depth > 1024 as i32 {
        return -(1 as i32);
    }
    loop {
        let mut current_block_97: u64;
        match (*pc).opcode as i32 {
            0 => return 0,
            1 => {
                pc = (*pc).x;
            }
            2 => {
                scratch = *out;
                result = match_0((*pc).x, sp, bol, flags, &mut scratch, depth + 1);
                if result == -(1 as i32) {
                    return -(1 as i32);
                }
                if result == 0 {
                    *out = scratch;
                    return 0;
                }
                pc = (*pc).y;
            }
            3 => {
                result = match_0((*pc).x, sp, bol, flags, out, depth + 1);
                if result == -(1 as i32) {
                    return -(1 as i32);
                }
                if result == 1 {
                    return 1;
                }
                pc = (*pc).y;
            }
            4 => {
                scratch = *out;
                result = match_0((*pc).x, sp, bol, flags, &mut scratch, depth + 1);
                if result == -(1 as i32) {
                    return -(1 as i32);
                }
                if result == 0 {
                    return 1;
                }
                pc = (*pc).y;
            }
            5 => {
                if *sp == 0 {
                    return 1;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                pc = pc.offset(1 as i32 as isize);
            }
            6 => {
                if *sp == 0 {
                    return 1;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if isnewline(c) != 0 {
                    return 1;
                }
                pc = pc.offset(1 as i32 as isize);
            }
            7 => {
                if *sp == 0 {
                    return 1;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if flags & REG_ICASE as i32 != 0 {
                    c = canon(c);
                }
                if c != (*pc).c {
                    return 1;
                }
                pc = pc.offset(1 as i32 as isize);
            }
            8 => {
                if *sp == 0 {
                    return 1;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if flags & REG_ICASE as i32 != 0 {
                    if incclasscanon((*pc).cc, canon(c)) == 0 {
                        return 1;
                    }
                } else if incclass((*pc).cc, c) == 0 {
                    return 1;
                }
                pc = pc.offset(1 as i32 as isize);
            }
            9 => {
                if *sp == 0 {
                    return 1;
                }
                sp = sp.offset(jsU_chartorune(&mut c, sp) as isize);
                if flags & REG_ICASE as i32 != 0 {
                    if incclasscanon((*pc).cc, canon(c)) != 0 {
                        return 1;
                    }
                } else if incclass((*pc).cc, c) != 0 {
                    return 1;
                }
                pc = pc.offset(1 as i32 as isize);
            }
            10 => {
                i = ((*out).sub[(*pc).n as usize].ep).offset_from((*out).sub[(*pc).n as usize].sp)
                    as i32;
                if flags & REG_ICASE as i32 != 0 {
                    if strncmpcanon(sp, (*out).sub[(*pc).n as usize].sp, i) != 0 {
                        return 1;
                    }
                } else if strncmp(sp, (*out).sub[(*pc).n as usize].sp, i as u32) != 0 {
                    return 1;
                }
                if i > 0 {
                    sp = sp.offset(i as isize);
                }
                pc = pc.offset(1 as i32 as isize);
            }
            11 => {
                if sp == bol && flags & REG_NOTBOL as i32 == 0 {
                    pc = pc.offset(1 as i32 as isize);
                } else {
                    if flags & REG_NEWLINE as i32 != 0 {
                        if sp > bol && isnewline(*sp.offset(-(1 as i32) as isize) as i32) != 0 {
                            pc = pc.offset(1 as i32 as isize);
                            current_block_97 = 6471821049853688503;
                        } else {
                            current_block_97 = 15462640364611497761;
                        }
                    } else {
                        current_block_97 = 15462640364611497761;
                    }
                    match current_block_97 {
                        6471821049853688503 => {}
                        _ => return 1,
                    }
                }
            }
            12 => {
                if *sp as i32 == 0 {
                    pc = pc.offset(1 as i32 as isize);
                } else {
                    if flags & REG_NEWLINE as i32 != 0 {
                        if isnewline(*sp as i32) != 0 {
                            pc = pc.offset(1 as i32 as isize);
                            current_block_97 = 6471821049853688503;
                        } else {
                            current_block_97 = 5793491756164225964;
                        }
                    } else {
                        current_block_97 = 5793491756164225964;
                    }
                    match current_block_97 {
                        6471821049853688503 => {}
                        _ => return 1,
                    }
                }
            }
            13 => {
                i = (sp > bol && iswordchar(*sp.offset(-(1 as i32) as isize) as i32) != 0) as i32;
                i ^= iswordchar(*sp.offset(0 as isize) as i32);
                if i == 0 {
                    return 1;
                }
                pc = pc.offset(1 as i32 as isize);
            }
            14 => {
                i = (sp > bol && iswordchar(*sp.offset(-(1 as i32) as isize) as i32) != 0) as i32;
                i ^= iswordchar(*sp.offset(0 as isize) as i32);
                if i != 0 {
                    return 1;
                }
                pc = pc.offset(1 as i32 as isize);
            }
            15 => {
                (*out).sub[(*pc).n as usize].sp = sp;
                pc = pc.offset(1 as i32 as isize);
            }
            16 => {
                (*out).sub[(*pc).n as usize].ep = sp;
                pc = pc.offset(1 as i32 as isize);
            }
            _ => return 1,
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn js_regexec(
    mut prog: *mut Reprog,
    mut sp: *const libc::c_char,
    mut sub: *mut Resub,
    mut eflags: i32,
) -> i32 {
    let mut scratch: Resub = Resub {
        nsub: 0,
        sub: [C2RustUnnamed_9 {
            sp: core::ptr::null::<libc::c_char>(),
            ep: core::ptr::null::<libc::c_char>(),
        }; 16],
    };
    let mut i: i32 = 0;
    if sub.is_null() {
        sub = &mut scratch;
    }
    (*sub).nsub = (*prog).nsub;
    i = 0;
    while i < 16 as i32 {
        (*sub).sub[i as usize].ep = core::ptr::null::<libc::c_char>();
        (*sub).sub[i as usize].sp = (*sub).sub[i as usize].ep;
        i += 1;
    }
    match_0((*prog).start, sp, sp, (*prog).flags | eflags, sub, 0)
}
