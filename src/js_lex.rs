use crate::*;
use core::ptr;

unsafe extern "C" fn jsY_error(J: &mut js_State, mut fmt: *const libc::c_char, mut args: ...) -> ! {
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
        (*J).lexline,
    );
    strcat(buf.as_mut_ptr(), msgbuf.as_mut_ptr());
    js_newsyntaxerror(J, buf.as_mut_ptr());
    js_throw(J);
}

#[rustfmt::skip]
const TOKEN_STRING: [*const libc::c_char; 313] = [
    c"(end-of-file)".as_ptr(),
    c"'\\x01'".as_ptr(), c"'\\x02'".as_ptr(), c"'\\x03'".as_ptr(), c"'\\x04'".as_ptr(), c"'\\x05'".as_ptr(), c"'\\x06'".as_ptr(), c"'\\x07'".as_ptr(),
    c"'\\x08'".as_ptr(), c"'\\x09'".as_ptr(), c"'\\x0A'".as_ptr(), c"'\\x0B'".as_ptr(), c"'\\x0C'".as_ptr(), c"'\\x0D'".as_ptr(), c"'\\x0E'".as_ptr(), c"'\\x0F'".as_ptr(),
    c"'\\x10'".as_ptr(), c"'\\x11'".as_ptr(), c"'\\x12'".as_ptr(), c"'\\x13'".as_ptr(), c"'\\x14'".as_ptr(), c"'\\x15'".as_ptr(), c"'\\x16'".as_ptr(), c"'\\x17'".as_ptr(),
    c"'\\x18'".as_ptr(), c"'\\x19'".as_ptr(), c"'\\x1A'".as_ptr(), c"'\\x1B'".as_ptr(), c"'\\x1C'".as_ptr(), c"'\\x1D'".as_ptr(), c"'\\x1E'".as_ptr(), c"'\\x1F'".as_ptr(),
    c"' '".as_ptr(), c"'!'".as_ptr(), c"'\"'".as_ptr(), c"'#'".as_ptr(), c"'$'".as_ptr(), c"'%'".as_ptr(), c"'&'".as_ptr(), c"'\\''".as_ptr(),
    c"'('".as_ptr(), c"')'".as_ptr(), c"'*'".as_ptr(), c"'+'".as_ptr(), c"','".as_ptr(), c"'-'".as_ptr(), c"'.'".as_ptr(), c"'/'".as_ptr(),
    c"'0'".as_ptr(), c"'1'".as_ptr(), c"'2'".as_ptr(), c"'3'".as_ptr(), c"'4'".as_ptr(), c"'5'".as_ptr(), c"'6'".as_ptr(), c"'7'".as_ptr(),
    c"'8'".as_ptr(), c"'9'".as_ptr(), c"':'".as_ptr(), c"';'".as_ptr(), c"'<'".as_ptr(), c"'='".as_ptr(), c"'>'".as_ptr(), c"'?'".as_ptr(),
    c"'@'".as_ptr(), c"'A'".as_ptr(), c"'B'".as_ptr(), c"'C'".as_ptr(), c"'D'".as_ptr(), c"'E'".as_ptr(), c"'F'".as_ptr(), c"'G'".as_ptr(),
    c"'H'".as_ptr(), c"'I'".as_ptr(), c"'J'".as_ptr(), c"'K'".as_ptr(), c"'L'".as_ptr(), c"'M'".as_ptr(), c"'N'".as_ptr(), c"'O'".as_ptr(),
    c"'P'".as_ptr(), c"'Q'".as_ptr(), c"'R'".as_ptr(), c"'S'".as_ptr(), c"'T'".as_ptr(), c"'U'".as_ptr(), c"'V'".as_ptr(), c"'W'".as_ptr(),
    c"'X'".as_ptr(), c"'Y'".as_ptr(), c"'Z'".as_ptr(), c"'['".as_ptr(), c"''".as_ptr(), c"']'".as_ptr(), c"'^'".as_ptr(), c"'_'".as_ptr(),
    c"'`'".as_ptr(), c"'a'".as_ptr(), c"'b'".as_ptr(), c"'c'".as_ptr(), c"'d'".as_ptr(), c"'e'".as_ptr(), c"'f'".as_ptr(), c"'g'".as_ptr(),
    c"'h'".as_ptr(), c"'i'".as_ptr(), c"'j'".as_ptr(), c"'k'".as_ptr(), c"'l'".as_ptr(), c"'m'".as_ptr(), c"'n'".as_ptr(), c"'o'".as_ptr(),
    c"'p'".as_ptr(), c"'q'".as_ptr(), c"'r'".as_ptr(), c"'s'".as_ptr(), c"'t'".as_ptr(), c"'u'".as_ptr(), c"'v'".as_ptr(), c"'w'".as_ptr(),
    c"'x'".as_ptr(), c"'y'".as_ptr(), c"'z'".as_ptr(), c"'{'".as_ptr(), c"'|'".as_ptr(), c"'}'".as_ptr(), c"'~'".as_ptr(), c"'\\x7F'".as_ptr(),
    
    ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
	ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
	ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
	ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
	ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
	ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
	ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
	ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(), ptr::null(),ptr::null(),ptr::null(),ptr::null(),
    
    c"(identifier)".as_ptr(), c"(number)".as_ptr(), c"(string)".as_ptr(), c"(regexp)".as_ptr(),
    
    c"'<='".as_ptr(), c"'>='".as_ptr(), c"'=='".as_ptr(), c"'!='".as_ptr(), c"'==='".as_ptr(), c"'!=='".as_ptr(),
    c"'<<'".as_ptr(), c"'>>'".as_ptr(), c"'>>>'".as_ptr(), c"'&&'".as_ptr(), c"'||'".as_ptr(),
    c"'+='".as_ptr(), c"'-='".as_ptr(), c"'*='".as_ptr(), c"'/='".as_ptr(), c"'%='".as_ptr(),
    c"'<<='".as_ptr(), c"'>>='".as_ptr(), c"'>>>='".as_ptr(), c"'&='".as_ptr(), c"'|='".as_ptr(), c"'^='".as_ptr(),
    c"'++'".as_ptr(), c"'--'".as_ptr(),
    
    c"'break'".as_ptr(), c"'case'".as_ptr(), c"'catch'".as_ptr(), c"'continue'".as_ptr(), c"'debugger'".as_ptr(),
    c"'default'".as_ptr(), c"'delete'".as_ptr(), c"'do'".as_ptr(), c"'else'".as_ptr(), c"'false'".as_ptr(), c"'finally'".as_ptr(), c"'for'".as_ptr(),
    c"'function'".as_ptr(), c"'if'".as_ptr(), c"'in'".as_ptr(), c"'instanceof'".as_ptr(), c"'new'".as_ptr(), c"'null'".as_ptr(), c"'return'".as_ptr(),
    c"'switch'".as_ptr(), c"'this'".as_ptr(), c"'throw'".as_ptr(), c"'true'".as_ptr(), c"'try'".as_ptr(), c"'typeof'".as_ptr(), c"'var'".as_ptr(),
    c"'void'".as_ptr(), c"'while'".as_ptr(), c"'with'".as_ptr(),
];
#[no_mangle]
pub unsafe extern "C" fn jsY_tokenstring(mut token: i32) -> *const libc::c_char {
    if token >= 0
        && token
            < (::core::mem::size_of::<[*const libc::c_char; 313]>() as u32)
                .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as u32)
                as i32
        && !(TOKEN_STRING[token as usize]).is_null()
    {
        return TOKEN_STRING[token as usize];
    }
    b"<unknown>\0" as *const u8 as *const libc::c_char
}
#[rustfmt::skip]
const KEYWORDS: [*const libc::c_char; 29] = [
    c"break".as_ptr(), c"case".as_ptr(), c"catch".as_ptr(), c"continue".as_ptr(), c"debugger".as_ptr(), c"default".as_ptr(), c"delete".as_ptr(),
    c"do".as_ptr(), c"else".as_ptr(), c"false".as_ptr(), c"finally".as_ptr(), c"for".as_ptr(), c"function".as_ptr(), c"if".as_ptr(), c"in".as_ptr(),
    c"instanceof".as_ptr(), c"new".as_ptr(), c"null".as_ptr(), c"return".as_ptr(), c"switch".as_ptr(), c"this".as_ptr(), c"throw".as_ptr(),
    c"true".as_ptr(), c"try".as_ptr(), c"typeof".as_ptr(), c"var".as_ptr(), c"void".as_ptr(), c"while".as_ptr(), c"with".as_ptr(),
];
#[no_mangle]
pub unsafe extern "C" fn jsY_findword(
    mut s: *const libc::c_char,
    mut list: *const *const libc::c_char,
    mut num: i32,
) -> i32 {
    let mut l: i32 = 0;
    let mut r: i32 = num - 1;
    while l <= r {
        let mut m: i32 = (l + r) >> 1;
        let mut c: i32 = strcmp(s, *list.offset(m as isize));
        if c < 0 {
            r = m - 1;
        } else if c > 0 {
            l = m + 1;
        } else {
            return m;
        }
    }
    -(1 as i32)
}
unsafe extern "C" fn jsY_findkeyword(J: &mut js_State, mut s: *const libc::c_char) -> i32 {
    let mut i: i32 = jsY_findword(
        s,
        KEYWORDS.as_ptr(),
        (::core::mem::size_of::<[*const libc::c_char; 29]>() as u32)
            .wrapping_div(::core::mem::size_of::<*const libc::c_char>() as u32) as i32,
    );
    if i >= 0 {
        (*J).text = KEYWORDS[i as usize];
        return TK_BREAK as i32 + i;
    }
    (*J).text = js_intern(J, s);
    TK_IDENTIFIER as i32
}
#[no_mangle]
pub extern "C" fn jsY_iswhite(c: i32) -> bool {
    c == 0x9 || c == 0xb || c == 0xc || c == 0x20 || c == 0xa0 || c == 0xfeff
}
#[no_mangle]
pub extern "C" fn jsY_isnewline(c: i32) -> bool {
    c == 0xa || c == 0xd || c == 0x2028 || c == 0x2029
}
unsafe extern "C" fn jsY_isidentifierstart(mut c: i32) -> bool {
    c >= 'a' as i32 && c <= 'z' as i32
        || c >= 'A' as i32 && c <= 'Z' as i32
        || c == '$' as i32
        || c == '_' as i32
        || jsU_isalpharune(c) != 0
}
unsafe extern "C" fn jsY_isidentifierpart(mut c: i32) -> bool {
    c >= '0' as i32 && c <= '9' as i32
        || (c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32)
        || c == '$' as i32
        || c == '_' as i32
        || jsU_isalpharune(c) != 0
}
extern "C" fn jsY_isdec(mut c: i32) -> bool {
    c >= '0' as i32 && c <= '9' as i32
}
#[no_mangle]
pub extern "C" fn jsY_ishex(mut c: i32) -> bool {
    c >= '0' as i32 && c <= '9' as i32
        || (c >= 'a' as i32 && c <= 'f' as i32 || c >= 'A' as i32 && c <= 'F' as i32)
}
#[no_mangle]
pub extern "C" fn jsY_tohex(mut c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return c - 'a' as i32 + 0xa as i32;
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return c - 'A' as i32 + 0xa as i32;
    }
    0
}
unsafe extern "C" fn jsY_next(J: &mut js_State) {
    let mut c: Rune = 0;
    if *(*J).source as i32 == 0 {
        (*J).lexchar = -(1 as i32);
        return;
    }
    (*J).source = ((*J).source).offset(jsU_chartorune(&mut c, (*J).source) as isize);
    if c == '\r' as i32 && *(*J).source as i32 == '\n' as i32 {
        (*J).source = ((*J).source).offset(1);
        (*J).source;
    }
    if jsY_isnewline(c) {
        (*J).line += 1;
        (*J).line;
        c = '\n' as i32;
    }
    (*J).lexchar = c;
}
unsafe extern "C" fn jsY_unescape(J: &mut js_State) {
    if if (*J).lexchar == '\\' as i32 {
        jsY_next(J);
        1
    } else {
        0
    } != 0
    {
        if if (*J).lexchar == 'u' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            let mut x: i32 = 0;
            if jsY_ishex((*J).lexchar) {
                x |= jsY_tohex((*J).lexchar) << 12 as i32;
                jsY_next(J);
                if jsY_ishex((*J).lexchar) {
                    x |= jsY_tohex((*J).lexchar) << 8;
                    jsY_next(J);
                    if jsY_ishex((*J).lexchar) {
                        x |= jsY_tohex((*J).lexchar) << 4;
                        jsY_next(J);
                        if jsY_ishex((*J).lexchar) {
                            x |= jsY_tohex((*J).lexchar);
                            (*J).lexchar = x;
                            return;
                        }
                    }
                }
            }
        }
        jsY_error(
            J,
            b"unexpected escape sequence\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn textinit(J: &mut js_State) {
    if ((*J).lexbuf.text).is_null() {
        (*J).lexbuf.cap = 4096 as i32;
        (*J).lexbuf.text = js_malloc(J, (*J).lexbuf.cap) as *mut libc::c_char;
    }
    (*J).lexbuf.len = 0;
}
unsafe extern "C" fn textpush(J: &mut js_State, mut c: Rune) {
    let mut n: i32 = 0;
    if c == -(1 as i32) {
        n = 1;
    } else {
        n = jsU_runelen(c);
    }
    if (*J).lexbuf.len + n > (*J).lexbuf.cap {
        (*J).lexbuf.cap *= 2;
        (*J).lexbuf.text = js_realloc(J, (*J).lexbuf.text as *mut libc::c_void, (*J).lexbuf.cap)
            as *mut libc::c_char;
    }
    if c == -(1 as i32) {
        let fresh29 = (*J).lexbuf.len;
        (*J).lexbuf.len += 1;
        *((*J).lexbuf.text).offset(fresh29 as isize) = 0;
    } else {
        (*J).lexbuf.len +=
            jsU_runetochar(((*J).lexbuf.text).offset((*J).lexbuf.len as isize), &mut c);
    };
}
unsafe extern "C" fn textend(J: &mut js_State) -> *mut libc::c_char {
    textpush(J, -(1 as i32));
    (*J).lexbuf.text
}
unsafe extern "C" fn lexlinecomment(J: &mut js_State) {
    while (*J).lexchar != -(1 as i32) && (*J).lexchar != '\n' as i32 {
        jsY_next(J);
    }
}
unsafe extern "C" fn lexcomment(J: &mut js_State) -> i32 {
    while (*J).lexchar != -(1 as i32) {
        if if (*J).lexchar == '*' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            while (*J).lexchar == '*' as i32 {
                jsY_next(J);
            }
            if if (*J).lexchar == '/' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } != 0
            {
                return 0;
            }
        } else {
            jsY_next(J);
        }
    }
    -(1 as i32)
}
unsafe extern "C" fn lexhex(J: &mut js_State) -> f64 {
    let mut n: f64 = 0.0;
    if !jsY_ishex((*J).lexchar) {
        jsY_error(
            J,
            b"malformed hexadecimal number\0" as *const u8 as *const libc::c_char,
        );
    }
    while jsY_ishex((*J).lexchar) {
        n = n * 16 as i32 as f64 + jsY_tohex((*J).lexchar) as f64;
        jsY_next(J);
    }
    n
}
unsafe extern "C" fn lexnumber(J: &mut js_State) -> i32 {
    let mut s: *const libc::c_char = ((*J).source).offset(-(1 as i32 as isize));
    if if (*J).lexchar == '0' as i32 {
        jsY_next(J);
        1
    } else {
        0
    } != 0
    {
        if (if (*J).lexchar == 'x' as i32 {
            jsY_next(J);
            1
        } else {
            0
        }) != 0
            || (if (*J).lexchar == 'X' as i32 {
                jsY_next(J);
                1
            } else {
                0
            }) != 0
        {
            (*J).number = lexhex(J);
            return TK_NUMBER as i32;
        }
        if jsY_isdec((*J).lexchar) {
            jsY_error(
                J,
                b"number with leading zero\0" as *const u8 as *const libc::c_char,
            );
        }
        if if (*J).lexchar == '.' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            while jsY_isdec((*J).lexchar) {
                jsY_next(J);
            }
        }
    } else if if (*J).lexchar == '.' as i32 {
        jsY_next(J);
        1
    } else {
        0
    } != 0
    {
        if !jsY_isdec((*J).lexchar) {
            return '.' as i32;
        }
        while jsY_isdec((*J).lexchar) {
            jsY_next(J);
        }
    } else {
        while jsY_isdec((*J).lexchar) {
            jsY_next(J);
        }
        if if (*J).lexchar == '.' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            while jsY_isdec((*J).lexchar) {
                jsY_next(J);
            }
        }
    }
    if (if (*J).lexchar == 'e' as i32 {
        jsY_next(J);
        1
    } else {
        0
    }) != 0
        || (if (*J).lexchar == 'E' as i32 {
            jsY_next(J);
            1
        } else {
            0
        }) != 0
    {
        if (*J).lexchar == '-' as i32 || (*J).lexchar == '+' as i32 {
            jsY_next(J);
        }
        if jsY_isdec((*J).lexchar) {
            while jsY_isdec((*J).lexchar) {
                jsY_next(J);
            }
        } else {
            jsY_error(J, b"missing exponent\0" as *const u8 as *const libc::c_char);
        }
    }
    if jsY_isidentifierstart((*J).lexchar) {
        jsY_error(
            J,
            b"number with letter suffix\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).number = js_strtod(s, core::ptr::null_mut::<*mut libc::c_char>());
    TK_NUMBER as i32
}
unsafe extern "C" fn lexescape(J: &mut js_State) -> i32 {
    let mut x: i32 = 0;
    if if (*J).lexchar == '\n' as i32 {
        jsY_next(J);
        1
    } else {
        0
    } != 0
    {
        return 0;
    }
    match (*J).lexchar {
        -1 => {
            jsY_error(
                J,
                b"unterminated escape sequence\0" as *const u8 as *const libc::c_char,
            );
        }
        117 => {
            jsY_next(J);
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar) << 12 as i32;
                jsY_next(J);
            }
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar) << 8;
                jsY_next(J);
            }
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar) << 4;
                jsY_next(J);
            }
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar);
                jsY_next(J);
            }
            textpush(J, x);
        }
        120 => {
            jsY_next(J);
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar) << 4;
                jsY_next(J);
            }
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar);
                jsY_next(J);
            }
            textpush(J, x);
        }
        48 => {
            textpush(J, 0);
            jsY_next(J);
        }
        92 => {
            textpush(J, '\\' as i32);
            jsY_next(J);
        }
        39 => {
            textpush(J, '\'' as i32);
            jsY_next(J);
        }
        34 => {
            textpush(J, '"' as i32);
            jsY_next(J);
        }
        98 => {
            textpush(J, '\u{8}' as i32);
            jsY_next(J);
        }
        102 => {
            textpush(J, '\u{c}' as i32);
            jsY_next(J);
        }
        110 => {
            textpush(J, '\n' as i32);
            jsY_next(J);
        }
        114 => {
            textpush(J, '\r' as i32);
            jsY_next(J);
        }
        116 => {
            textpush(J, '\t' as i32);
            jsY_next(J);
        }
        118 => {
            textpush(J, '\u{b}' as i32);
            jsY_next(J);
        }
        _ => {
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    0
}
unsafe extern "C" fn lexstring(J: &mut js_State) -> i32 {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut q: i32 = (*J).lexchar;
    jsY_next(J);
    textinit(J);
    while (*J).lexchar != q {
        if (*J).lexchar == -(1 as i32) || (*J).lexchar == '\n' as i32 {
            jsY_error(
                J,
                b"string not terminated\0" as *const u8 as *const libc::c_char,
            );
        }
        if if (*J).lexchar == '\\' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            if lexescape(J) != 0 {
                jsY_error(
                    J,
                    b"malformed escape sequence\0" as *const u8 as *const libc::c_char,
                );
            }
        } else {
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    if if (*J).lexchar == q {
        jsY_next(J);
        1
    } else {
        0
    } == 0
    {
        jsY_error(J, b"expected '%c'\0" as *const u8 as *const libc::c_char, q);
    }
    s = textend(J);
    (*J).text = js_intern(J, s);
    TK_STRING as i32
}
unsafe extern "C" fn isregexpcontext(mut last: i32) -> i32 {
    match last {
        93 | 41 | 125 | 256 | 257 | 258 | 293 | 301 | 304 | 306 => 0,
        _ => 1,
    }
}
unsafe extern "C" fn lexregexp(J: &mut js_State) -> i32 {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    let mut g: i32 = 0;
    let mut m: i32 = 0;
    let mut i: i32 = 0;
    let mut inclass: i32 = 0;
    textinit(J);
    while (*J).lexchar != '/' as i32 || inclass != 0 {
        if (*J).lexchar == -(1 as i32) || (*J).lexchar == '\n' as i32 {
            jsY_error(
                J,
                b"regular expression not terminated\0" as *const u8 as *const libc::c_char,
            );
        } else if if (*J).lexchar == '\\' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            if if (*J).lexchar == '/' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } != 0
            {
                textpush(J, '/' as i32);
            } else {
                textpush(J, '\\' as i32);
                if (*J).lexchar == -(1 as i32) || (*J).lexchar == '\n' as i32 {
                    jsY_error(
                        J,
                        b"regular expression not terminated\0" as *const u8 as *const libc::c_char,
                    );
                }
                textpush(J, (*J).lexchar);
                jsY_next(J);
            }
        } else {
            if (*J).lexchar == '[' as i32 && inclass == 0 {
                inclass = 1;
            }
            if (*J).lexchar == ']' as i32 && inclass != 0 {
                inclass = 0;
            }
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    if if (*J).lexchar == '/' as i32 {
        jsY_next(J);
        1
    } else {
        0
    } == 0
    {
        jsY_error(
            J,
            b"expected '%c'\0" as *const u8 as *const libc::c_char,
            '/' as i32,
        );
    }
    s = textend(J);
    m = 0;
    i = m;
    g = i;
    while jsY_isidentifierpart((*J).lexchar) {
        if if (*J).lexchar == 'g' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            g += 1;
        } else if if (*J).lexchar == 'i' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            i += 1;
        } else if if (*J).lexchar == 'm' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            m += 1;
        } else {
            jsY_error(
                J,
                b"illegal flag in regular expression: %c\0" as *const u8 as *const libc::c_char,
                (*J).lexchar,
            );
        }
    }
    if g > 1 || i > 1 || m > 1 {
        jsY_error(
            J,
            b"duplicated flag in regular expression\0" as *const u8 as *const libc::c_char,
        );
    }
    (*J).text = js_intern(J, s);
    (*J).number = 0.0;
    if g != 0 {
        (*J).number += JS_REGEXP_G as i32 as f64;
    }
    if i != 0 {
        (*J).number += JS_REGEXP_I as i32 as f64;
    }
    if m != 0 {
        (*J).number += JS_REGEXP_M as i32 as f64;
    }
    TK_REGEXP as i32
}
unsafe extern "C" fn isnlthcontext(mut last: i32) -> i32 {
    match last {
        284 | 287 | 302 | 305 => 1,
        _ => 0,
    }
}
unsafe extern "C" fn jsY_lexx(J: &mut js_State) -> i32 {
    (*J).newline = 0;
    loop {
        (*J).lexline = (*J).line;
        while jsY_iswhite((*J).lexchar) {
            jsY_next(J);
        }
        if if (*J).lexchar == '\n' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            (*J).newline = 1;
            if isnlthcontext((*J).lasttoken) != 0 {
                return ';' as i32;
            }
        } else if if (*J).lexchar == '/' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            if if (*J).lexchar == '/' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } != 0
            {
                lexlinecomment(J);
            } else if if (*J).lexchar == '*' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } != 0
            {
                if lexcomment(J) != 0 {
                    jsY_error(
                        J,
                        b"multi-line comment not terminated\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else if isregexpcontext((*J).lasttoken) != 0 {
                return lexregexp(J);
            } else if if (*J).lexchar == '=' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } != 0
            {
                return TK_DIV_ASS as i32;
            } else {
                return '/' as i32;
            }
        } else {
            if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
                return lexnumber(J);
            }
            match (*J).lexchar {
                40 => {
                    jsY_next(J);
                    return '(' as i32;
                }
                41 => {
                    jsY_next(J);
                    return ')' as i32;
                }
                44 => {
                    jsY_next(J);
                    return ',' as i32;
                }
                58 => {
                    jsY_next(J);
                    return ':' as i32;
                }
                59 => {
                    jsY_next(J);
                    return ';' as i32;
                }
                63 => {
                    jsY_next(J);
                    return '?' as i32;
                }
                91 => {
                    jsY_next(J);
                    return '[' as i32;
                }
                93 => {
                    jsY_next(J);
                    return ']' as i32;
                }
                123 => {
                    jsY_next(J);
                    return '{' as i32;
                }
                125 => {
                    jsY_next(J);
                    return '}' as i32;
                }
                126 => {
                    jsY_next(J);
                    return '~' as i32;
                }
                39 | 34 => return lexstring(J),
                46 => return lexnumber(J),
                60 => {
                    jsY_next(J);
                    if if (*J).lexchar == '<' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1
                        } else {
                            0
                        } != 0
                        {
                            return TK_SHL_ASS as i32;
                        }
                        return TK_SHL as i32;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_LE as i32;
                    }
                    return '<' as i32;
                }
                62 => {
                    jsY_next(J);
                    if if (*J).lexchar == '>' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        if if (*J).lexchar == '>' as i32 {
                            jsY_next(J);
                            1
                        } else {
                            0
                        } != 0
                        {
                            if if (*J).lexchar == '=' as i32 {
                                jsY_next(J);
                                1
                            } else {
                                0
                            } != 0
                            {
                                return TK_USHR_ASS as i32;
                            }
                            return TK_USHR as i32;
                        }
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1
                        } else {
                            0
                        } != 0
                        {
                            return TK_SHR_ASS as i32;
                        }
                        return TK_SHR as i32;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_GE as i32;
                    }
                    return '>' as i32;
                }
                61 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1
                        } else {
                            0
                        } != 0
                        {
                            return TK_STRICTEQ as i32;
                        }
                        return TK_EQ as i32;
                    }
                    return '=' as i32;
                }
                33 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        if if (*J).lexchar == '=' as i32 {
                            jsY_next(J);
                            1
                        } else {
                            0
                        } != 0
                        {
                            return TK_STRICTNE as i32;
                        }
                        return TK_NE as i32;
                    }
                    return '!' as i32;
                }
                43 => {
                    jsY_next(J);
                    if if (*J).lexchar == '+' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_INC as i32;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_ADD_ASS as i32;
                    }
                    return '+' as i32;
                }
                45 => {
                    jsY_next(J);
                    if if (*J).lexchar == '-' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_DEC as i32;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_SUB_ASS as i32;
                    }
                    return '-' as i32;
                }
                42 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_MUL_ASS as i32;
                    }
                    return '*' as i32;
                }
                37 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_MOD_ASS as i32;
                    }
                    return '%' as i32;
                }
                38 => {
                    jsY_next(J);
                    if if (*J).lexchar == '&' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_AND as i32;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_AND_ASS as i32;
                    }
                    return '&' as i32;
                }
                124 => {
                    jsY_next(J);
                    if if (*J).lexchar == '|' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_OR as i32;
                    }
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_OR_ASS as i32;
                    }
                    return '|' as i32;
                }
                94 => {
                    jsY_next(J);
                    if if (*J).lexchar == '=' as i32 {
                        jsY_next(J);
                        1
                    } else {
                        0
                    } != 0
                    {
                        return TK_XOR_ASS as i32;
                    }
                    return '^' as i32;
                }
                -1 => return 0,
                _ => {}
            }
            jsY_unescape(J);
            if jsY_isidentifierstart((*J).lexchar) {
                textinit(J);
                textpush(J, (*J).lexchar);
                jsY_next(J);
                jsY_unescape(J);
                while jsY_isidentifierpart((*J).lexchar) {
                    textpush(J, (*J).lexchar);
                    jsY_next(J);
                    jsY_unescape(J);
                }
                textend(J);
                return jsY_findkeyword(J, (*J).lexbuf.text);
            }
            if (*J).lexchar >= 0x20 && (*J).lexchar <= 0x7e as i32 {
                jsY_error(
                    J,
                    b"unexpected character: '%c'\0" as *const u8 as *const libc::c_char,
                    (*J).lexchar,
                );
            }
            jsY_error(
                J,
                b"unexpected character: \\u%04X\0" as *const u8 as *const libc::c_char,
                (*J).lexchar,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn jsY_initlex(
    J: &mut js_State,
    mut filename: *const libc::c_char,
    mut source: *const libc::c_char,
) {
    (*J).filename = filename;
    (*J).source = source;
    (*J).line = 1;
    (*J).lasttoken = 0;
    jsY_next(J);
}
#[no_mangle]
pub unsafe extern "C" fn jsY_lex(J: &mut js_State) -> i32 {
    (*J).lasttoken = jsY_lexx(J);
    (*J).lasttoken
}
unsafe extern "C" fn lexjsonnumber(J: &mut js_State) -> i32 {
    let mut s: *const libc::c_char = ((*J).source).offset(-(1 as i32 as isize));
    if (*J).lexchar == '-' as i32 {
        jsY_next(J);
    }
    if (*J).lexchar == '0' as i32 {
        jsY_next(J);
    } else if (*J).lexchar >= '1' as i32 && (*J).lexchar <= '9' as i32 {
        while (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
            jsY_next(J);
        }
    } else {
        jsY_error(
            J,
            b"unexpected non-digit\0" as *const u8 as *const libc::c_char,
        );
    }
    if if (*J).lexchar == '.' as i32 {
        jsY_next(J);
        1
    } else {
        0
    } != 0
    {
        if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
            while (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
                jsY_next(J);
            }
        } else {
            jsY_error(
                J,
                b"missing digits after decimal point\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    if (if (*J).lexchar == 'e' as i32 {
        jsY_next(J);
        1
    } else {
        0
    }) != 0
        || (if (*J).lexchar == 'E' as i32 {
            jsY_next(J);
            1
        } else {
            0
        }) != 0
    {
        if (*J).lexchar == '-' as i32 || (*J).lexchar == '+' as i32 {
            jsY_next(J);
        }
        if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
            while (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 {
                jsY_next(J);
            }
        } else {
            jsY_error(
                J,
                b"missing digits after exponent indicator\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    (*J).number = js_strtod(s, core::ptr::null_mut::<*mut libc::c_char>());
    TK_NUMBER as i32
}
unsafe extern "C" fn lexjsonescape(J: &mut js_State) -> i32 {
    let mut x: i32 = 0;
    match (*J).lexchar {
        117 => {
            jsY_next(J);
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar) << 12 as i32;
                jsY_next(J);
            }
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar) << 8;
                jsY_next(J);
            }
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar) << 4;
                jsY_next(J);
            }
            if !jsY_ishex((*J).lexchar) {
                return 1;
            } else {
                x |= jsY_tohex((*J).lexchar);
                jsY_next(J);
            }
            textpush(J, x);
        }
        34 => {
            textpush(J, '"' as i32);
            jsY_next(J);
        }
        92 => {
            textpush(J, '\\' as i32);
            jsY_next(J);
        }
        47 => {
            textpush(J, '/' as i32);
            jsY_next(J);
        }
        98 => {
            textpush(J, '\u{8}' as i32);
            jsY_next(J);
        }
        102 => {
            textpush(J, '\u{c}' as i32);
            jsY_next(J);
        }
        110 => {
            textpush(J, '\n' as i32);
            jsY_next(J);
        }
        114 => {
            textpush(J, '\r' as i32);
            jsY_next(J);
        }
        116 => {
            textpush(J, '\t' as i32);
            jsY_next(J);
        }
        _ => {
            jsY_error(
                J,
                b"invalid escape sequence\0" as *const u8 as *const libc::c_char,
            );
        }
    }
    0
}
unsafe extern "C" fn lexjsonstring(J: &mut js_State) -> i32 {
    let mut s: *const libc::c_char = core::ptr::null::<libc::c_char>();
    textinit(J);
    while (*J).lexchar != '"' as i32 {
        if (*J).lexchar == -(1 as i32) {
            jsY_error(
                J,
                b"unterminated string\0" as *const u8 as *const libc::c_char,
            );
        } else if (*J).lexchar < 32 as i32 {
            jsY_error(
                J,
                b"invalid control character in string\0" as *const u8 as *const libc::c_char,
            );
        } else if if (*J).lexchar == '\\' as i32 {
            jsY_next(J);
            1
        } else {
            0
        } != 0
        {
            lexjsonescape(J);
        } else {
            textpush(J, (*J).lexchar);
            jsY_next(J);
        }
    }
    if if (*J).lexchar == '"' as i32 {
        jsY_next(J);
        1
    } else {
        0
    } == 0
    {
        jsY_error(
            J,
            b"expected '%c'\0" as *const u8 as *const libc::c_char,
            '"' as i32,
        );
    }
    s = textend(J);
    (*J).text = js_intern(J, s);
    TK_STRING as i32
}
#[no_mangle]
pub unsafe extern "C" fn jsY_lexjson(J: &mut js_State) -> i32 {
    (*J).lexline = (*J).line;
    while jsY_iswhite((*J).lexchar) || (*J).lexchar == '\n' as i32 {
        jsY_next(J);
    }
    if (*J).lexchar >= '0' as i32 && (*J).lexchar <= '9' as i32 || (*J).lexchar == '-' as i32 {
        return lexjsonnumber(J);
    }
    match (*J).lexchar {
        44 => {
            jsY_next(J);
            return ',' as i32;
        }
        58 => {
            jsY_next(J);
            return ':' as i32;
        }
        91 => {
            jsY_next(J);
            return '[' as i32;
        }
        93 => {
            jsY_next(J);
            return ']' as i32;
        }
        123 => {
            jsY_next(J);
            return '{' as i32;
        }
        125 => {
            jsY_next(J);
            return '}' as i32;
        }
        34 => {
            jsY_next(J);
            return lexjsonstring(J);
        }
        102 => {
            jsY_next(J);
            if if (*J).lexchar == 'a' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'a' as i32,
                );
            }
            if if (*J).lexchar == 'l' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'l' as i32,
                );
            }
            if if (*J).lexchar == 's' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    's' as i32,
                );
            }
            if if (*J).lexchar == 'e' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'e' as i32,
                );
            }
            return TK_FALSE as i32;
        }
        110 => {
            jsY_next(J);
            if if (*J).lexchar == 'u' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'u' as i32,
                );
            }
            if if (*J).lexchar == 'l' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'l' as i32,
                );
            }
            if if (*J).lexchar == 'l' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'l' as i32,
                );
            }
            return TK_NULL as i32;
        }
        116 => {
            jsY_next(J);
            if if (*J).lexchar == 'r' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'r' as i32,
                );
            }
            if if (*J).lexchar == 'u' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'u' as i32,
                );
            }
            if if (*J).lexchar == 'e' as i32 {
                jsY_next(J);
                1
            } else {
                0
            } == 0
            {
                jsY_error(
                    J,
                    b"expected '%c'\0" as *const u8 as *const libc::c_char,
                    'e' as i32,
                );
            }
            return TK_TRUE as i32;
        }
        -1 => return 0,
        _ => {}
    }
    if (*J).lexchar >= 0x20 && (*J).lexchar <= 0x7e as i32 {
        jsY_error(
            J,
            b"unexpected character: '%c'\0" as *const u8 as *const libc::c_char,
            (*J).lexchar,
        );
    }
    jsY_error(
        J,
        b"unexpected character: \\u%04X\0" as *const u8 as *const libc::c_char,
        (*J).lexchar,
    );
}
