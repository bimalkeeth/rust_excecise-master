#![allow(dead_code)]
#![allow(unused_variables)]

pub fn string_lit() {
    let rust = "\x52\x75\x73\x74";
}

//  this will explain how param works.
pub fn greatest_common_devi(mut a: u64, mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b = c;
        }
        a = a % b;
    }
    return b;
}
