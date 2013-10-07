extern mod std;

use std::from_str::FromStr;
use std::libc::{c_char, c_double, c_int, c_long, c_ulong, c_void, size_t};
use std::num::{IntConvertible, One, Zero};
use std::unstable::intrinsics::uninit;
use std::{cmp, int, str, to_str, uint, vec};

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

#[link_args = "-lmpfr"]
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
    #[fixed_stack_segment]
    fn drop(&mut self) { unsafe { mpfr_clear(&mut self.mpfr)}}
}


impl MPFR {
    #[fixed_stack_segment]
    pub fn new(prec : mpfr_prec_t) -> MPFR {
        unsafe {
            let mut mpfr = uninit();
            mpfr_init2(&mut mpfr, prec);
            MPFR { mpfr: mpfr }
        }
    }

    #[fixed_stack_segment]
    pub fn to_str_internal(&self) -> ~str {
        unsafe {
            let mut len = 128;
            for i in range(0,2) {
                // Allocate the null-terminated string
                let dst = vec::from_elem(len, '0');
                // Get a pointer to it
                let pdst = vec::raw::to_ptr(dst);
                // Try to allocate
                len = mpfr_snprintf(pdst as *c_char, (len + 1) as size_t, vec::raw::to_ptr(bytes!("%.Re\x00")) as *c_char, &self.mpfr) as uint;
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
    #[fixed_stack_segment]
    pub fn set_d(&mut self, op: f64) -> int {
        unsafe {
            mpfr_set_d(&mut self.mpfr, op, 0) as int
        }
    }
    #[fixed_stack_segment]
    pub fn set_si(&mut self, op: int) -> int {
        unsafe {
            mpfr_set_si(&mut self.mpfr, op as c_long, 0) as int
        }
    }
}

impl to_str::ToStr for MPFR {
    #[fixed_stack_segment]
    fn to_str(&self) -> ~str {
        self.to_str_internal()
    }
}



fn main() {
    let x = 12;
    let mut y : MPFR = MPFR::new(256);

    println(format!("{:d}, {:f} and {:?}", x, std::float::NaN, y));
    println(format!("Let's test! {:s}", y.to_str()));

    y.set_d(1.1);
    println(format!("And now? {:s} is the value.", y.to_str()));

    y.set_si(1424);
    println(format!("And now? {:s} is the value.", y.to_str()));
}
