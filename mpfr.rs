#![crate_id = "mpfr#0.0.0"]

#![comment = "MPFR bindings for Rust"]
#![license = "MIT"]

#![allow(non_camel_case_types)]

use std::libc::{c_char, c_double, c_int, c_long, c_ulong, c_void, size_t};
use std::num::Float;
use std::intrinsics::uninit;
use std::{str, fmt};
use std::io::println;

struct mpfr_struct {
    _mpfr_prec: mpfr_prec_t,
    _mpfr_sign: mpfr_sign_t,
    _mpfr_exp: mpfr_exp_t,
    _mpfr_d: *c_void
}

type mp_bitcnt_t = c_ulong;
type mpfr_prec_t = c_long;
type mpfr_exp_t = c_long;
type mpfr_sign_t = c_int;
type mpfr_rnd_t = c_int;

type mpfr_srcptr = *mpfr_struct;
type mpfr_ptr = *mut mpfr_struct;

#[link(name = "mpfr")]
extern {
    fn mpfr_clear(x: mpfr_ptr);
    fn mpfr_init2(x: mpfr_ptr, p: mpfr_prec_t);
    fn mpfr_set_d(rop: mpfr_ptr, op: c_double, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_set_si(rop: mpfr_ptr, op: c_long, rnd: mpfr_rnd_t) -> c_int;
    fn mpfr_snprintf(buf: *c_char, n: size_t, template: *c_char, ptr: mpfr_srcptr) -> c_int;
}

pub struct MPFR {
    priv mpfr: mpfr_struct,
}

impl Drop for MPFR {
    fn drop(&mut self) { unsafe { mpfr_clear(&mut self.mpfr)}}
}


impl MPFR {
    pub fn new(prec : c_long) -> MPFR {
        unsafe {
            let mut mpfr = uninit();
            mpfr_init2(&mut mpfr, prec);
            MPFR { mpfr: mpfr }
        }
    }

    pub fn to_str_internal(&self) -> ~str {
        unsafe {
            let mut len = 128;
            for i in range(0,2) {
                // Allocate the null-terminated string
                let dst = Vec::from_elem(len, '0');
                // Get a pointer to it
                let pdst = dst.as_ptr();
                // Try to allocate
                len = mpfr_snprintf(pdst as *c_char, (len + 1) as size_t, (bytes!("%.Re\x00").as_ptr()) as *c_char, &self.mpfr) as uint;
                if len < 128 || i == 1 {
                    return str::raw::from_c_str(pdst as *c_char);
                }
            }
            // Technically, this should never be reached
            // But it's not like this function is exactly well written...
            return ~"Error";
        }
    }

    // TODO: Allow rounding modes (using an enum?)
    pub fn set_d(&mut self, op: f64) -> int {
        unsafe {
            mpfr_set_d(&mut self.mpfr, op, 0) as int
        }
    }
    pub fn set_si(&mut self, op: int) -> int {
        unsafe {
            mpfr_set_si(&mut self.mpfr, op as c_long, 0) as int
        }
    }
}

// impl ToStrRadix for MPFR {
//     fn to_str_radix(&self, base: uint) -> ~str {
//         self.to_str_internal()
//     }
// }

// impl ToStr for MPFR {
//     fn to_str(&self) -> ~str {
//         self.to_str_internal()
//     }
// }


impl fmt::Show for MPFR {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f.buf, "{}", self.to_str_internal())
    }
}

fn main() {
    let x = 12;
    let nan : f64 = Float::nan();
    let mut y : MPFR = MPFR::new(256);

    println(format!("{:d}, {:f} and {:?}", x, nan, y));
    println(format!("Let's test! {:s}", y.to_str()));

    y.set_d(1.1);
    println(format!("And now? {:s} is the value.", y.to_str()));

    y.set_si(1424);
    println(format!("And now? {:s} is the value.", y.to_str()));
}
