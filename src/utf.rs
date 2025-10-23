use crate::*;

#[no_mangle]
pub unsafe extern "C" fn jsU_chartorune(mut rune: *mut Rune, mut str: *const libc::c_char) -> i32 {
    let mut c: i32 = 0;
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut c3: i32 = 0;
    let mut l: i32 = 0;
    if *str.offset(0 as isize) as uchar as i32 == 0xc0
        && *str.offset(1 as i32 as isize) as uchar as i32 == 0x80
    {
        *rune = 0;
        return 2;
    }
    c = *(str as *mut uchar) as i32;
    if c < Tx as i32 {
        *rune = c;
        return 1;
    }
    c1 = *(str.offset(1 as i32 as isize) as *mut uchar) as i32 ^ Tx as i32;
    if c1 & Testx as i32 == 0 {
        if c < T3 as i32 {
            if c >= T2 as i32 {
                l = (c << Bitx as i32 | c1) & Rune2 as i32;
                if l > Rune1 as i32 {
                    *rune = l;
                    return 2;
                }
            }
        } else {
            c2 = *(str.offset(2 as i32 as isize) as *mut uchar) as i32 ^ Tx as i32;
            if c2 & Testx as i32 == 0 {
                if c < T4 as i32 {
                    l = ((c << Bitx as i32 | c1) << Bitx as i32 | c2) & Rune3 as i32;
                    if l > Rune2 as i32 {
                        *rune = l;
                        return 3;
                    }
                } else if UTFmax as i32 >= 4 {
                    c3 = *(str.offset(3 as i32 as isize) as *mut uchar) as i32 ^ Tx as i32;
                    if c3 & Testx as i32 == 0 && c < T5 as i32 {
                        l = (((c << Bitx as i32 | c1) << Bitx as i32 | c2) << Bitx as i32 | c3)
                            & Rune4 as i32;
                        if l > Rune3 as i32 && l <= Runemax as i32 {
                            *rune = l;
                            return 4;
                        }
                    }
                }
            }
        }
    }
    *rune = Bad as i32;
    1
}
#[no_mangle]
pub unsafe extern "C" fn jsU_runetochar(mut str: *mut libc::c_char, mut rune: *const Rune) -> i32 {
    let mut c: i32 = *rune;
    if c == 0 {
        *str.offset(0 as libc::c_int as isize) = 0xc0 as libc::c_int as libc::c_char;
        *str.offset(1 as libc::c_int as isize) = 0x80 as libc::c_int as libc::c_char;
        return 2;
    }
    if c <= Rune1 as i32 {
        *str.offset(0 as isize) = c as libc::c_char;
        return 1;
    }
    if c <= Rune2 as i32 {
        *str.offset(0 as isize) = (T2 as i32 | c >> (1 as i32 * Bitx as i32)) as libc::c_char;
        *str.offset(1 as i32 as isize) = (Tx as i32 | c & Maskx as i32) as libc::c_char;
        return 2;
    }
    if c > Runemax as i32 {
        c = Runeerror as i32;
    }
    if c <= Rune3 as i32 {
        *str.offset(0 as isize) = (T3 as i32 | c >> (2 as i32 * Bitx as i32)) as libc::c_char;
        *str.offset(1 as i32 as isize) =
            (Tx as i32 | c >> (1 as i32 * Bitx as i32) & Maskx as i32) as libc::c_char;
        *str.offset(2 as i32 as isize) = (Tx as i32 | c & Maskx as i32) as libc::c_char;
        return 3;
    }
    *str.offset(0 as isize) = (T4 as i32 | c >> (3 as i32 * Bitx as i32)) as libc::c_char;
    *str.offset(1 as i32 as isize) =
        (Tx as i32 | c >> (2 as i32 * Bitx as i32) & Maskx as i32) as libc::c_char;
    *str.offset(2 as i32 as isize) =
        (Tx as i32 | c >> (1 as i32 * Bitx as i32) & Maskx as i32) as libc::c_char;
    *str.offset(3 as i32 as isize) = (Tx as i32 | c & Maskx as i32) as libc::c_char;
    4
}
#[no_mangle]
pub unsafe extern "C" fn jsU_runelen(mut c: i32) -> i32 {
    let mut rune: Rune = 0;
    let mut str: [libc::c_char; 10] = [0; 10];
    rune = c;
    jsU_runetochar(str.as_mut_ptr(), &mut rune)
}
unsafe extern "C" fn ucd_bsearch(c: Rune, mut t: *const Rune, mut n: i32, ne: i32) -> *const Rune {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    let mut m: i32 = 0;
    while n > 1 {
        m = n / 2;
        p = t.offset((m * ne) as isize);
        if c >= *p.offset(0 as isize) {
            t = p;
            n -= m;
        } else {
            n = m;
        }
    }
    if n != 0 && c >= *t.offset(0 as isize) {
        return t;
    }
    core::ptr::null::<Rune>()
}
#[no_mangle]
pub unsafe extern "C" fn jsU_tolowerrune(mut c: Rune) -> Rune {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        UCD_TO_LOWER_2.as_ptr(),
        (::core::mem::size_of::<[Rune; 156]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 3,
        3,
    );
    if !p.is_null() && c >= *p.offset(0 as isize) && c <= *p.offset(1 as i32 as isize) {
        return c + *p.offset(2 as i32 as isize);
    }
    p = ucd_bsearch(
        c,
        UCD_TO_LOWER_1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1244]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 2,
        2,
    );
    if !p.is_null() && c == *p.offset(0 as isize) {
        return c + *p.offset(1 as i32 as isize);
    }
    c
}
#[no_mangle]
pub unsafe extern "C" fn jsU_toupperrune(mut c: Rune) -> Rune {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        UCD_TO_UPPER_1.as_ptr(),
        (::core::mem::size_of::<[Rune; 159]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 3,
        3,
    );
    if !p.is_null() && c >= *p.offset(0 as isize) && c <= *p.offset(1 as i32 as isize) {
        return c + *p.offset(2 as i32 as isize);
    }
    p = ucd_bsearch(
        c,
        UCD_TO_UPPER_1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1274]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 2,
        2,
    );
    if !p.is_null() && c == *p.offset(0 as isize) {
        return c + *p.offset(1 as i32 as isize);
    }
    c
}
#[no_mangle]
pub unsafe extern "C" fn jsU_islowerrune(mut c: Rune) -> i32 {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        UCD_TO_UPPER_2.as_ptr(),
        (::core::mem::size_of::<[Rune; 159]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 3,
        3,
    );
    if !p.is_null() && c >= *p.offset(0 as isize) && c <= *p.offset(1 as i32 as isize) {
        return 1;
    }
    p = ucd_bsearch(
        c,
        UCD_TO_UPPER_1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1274]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 2,
        2,
    );
    if !p.is_null() && c == *p.offset(0 as isize) {
        return 1;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn jsU_isupperrune(mut c: Rune) -> i32 {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        UCD_TO_LOWER_2.as_ptr(),
        (::core::mem::size_of::<[Rune; 156]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 3,
        3,
    );
    if !p.is_null() && c >= *p.offset(0 as isize) && c <= *p.offset(1 as i32 as isize) {
        return 1;
    }
    p = ucd_bsearch(
        c,
        UCD_TO_LOWER_1.as_ptr(),
        (::core::mem::size_of::<[Rune; 1244]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 2,
        2,
    );
    if !p.is_null() && c == *p.offset(0 as isize) {
        return 1;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn jsU_isalpharune(mut c: Rune) -> i32 {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        UCD_ALPHA_2.as_ptr(),
        (::core::mem::size_of::<[Rune; 1046]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 2,
        2,
    );
    if !p.is_null() && c >= *p.offset(0 as isize) && c <= *p.offset(1 as i32 as isize) {
        return 1;
    }
    p = ucd_bsearch(
        c,
        UCD_ALPHA_1.as_ptr(),
        (::core::mem::size_of::<[Rune; 167]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32,
        1,
    );
    if !p.is_null() && c == *p.offset(0 as isize) {
        return 1;
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn jsU_tolowerrune_full(mut c: Rune) -> *const Rune {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        UCD_TO_LOWER_FULL.as_ptr(),
        (::core::mem::size_of::<[Rune; 112]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 4,
        4,
    );
    if !p.is_null() && c == *p.offset(0 as isize) {
        return p.offset(1 as i32 as isize);
    }
    core::ptr::null::<Rune>()
}
#[no_mangle]
pub unsafe extern "C" fn jsU_toupperrune_full(mut c: Rune) -> *const Rune {
    let mut p: *const Rune = core::ptr::null::<Rune>();
    p = ucd_bsearch(
        c,
        UCD_TO_UPPER_FULL.as_ptr(),
        (::core::mem::size_of::<[Rune; 510]>() as u32)
            .wrapping_div(::core::mem::size_of::<Rune>() as u32) as i32
            / 5,
        5,
    );
    if !p.is_null() && c == *p.offset(0 as isize) {
        return p.offset(1 as i32 as isize);
    }
    core::ptr::null::<Rune>()
}
