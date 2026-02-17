use optarith_sys as sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XgcdI128 {
    pub gcd: i128,
    pub s: i128,
    pub t: i128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftXgcdI128 {
    pub gcd: i128,
    pub s: i128,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartialXgcdI128 {
    pub r1: i128,
    pub r0: i128,
    pub c1: i64,
    pub c0: i64,
}

#[inline]
fn to_u128_t(x: u128) -> sys::u128_t {
    sys::u128_t {
        v0: x as u64,
        v1: (x >> 64) as u64,
    }
}

#[inline]
fn from_u128_t(x: sys::u128_t) -> u128 {
    ((x.v1 as u128) << 64) | (x.v0 as u128)
}

#[inline]
fn to_s128_t(x: i128) -> sys::s128_t {
    let ux = x as u128;
    sys::s128_t {
        v0: ux as u64,
        v1: ((ux >> 64) as u64) as i64,
    }
}

#[inline]
fn from_s128_t(x: sys::s128_t) -> i128 {
    let ux = ((x.v1 as u64 as u128) << 64) | (x.v0 as u128);
    ux as i128
}

#[cfg(feature = "gmp")]
#[inline]
pub(crate) fn to_s128_t_for_gmp(x: i128) -> sys::s128_t {
    to_s128_t(x)
}

#[cfg(feature = "gmp")]
#[inline]
pub(crate) fn from_s128_t_for_gmp(x: sys::s128_t) -> i128 {
    from_s128_t(x)
}

pub fn gcd_binary_u128(a: u128, b: u128) -> u128 {
    unsafe {
        let mut out = sys::u128_t { v0: 0, v1: 0 };
        let aa = to_u128_t(a);
        let bb = to_u128_t(b);
        sys::gcd_binary_l2r_u128(&mut out, &aa, &bb);
        from_u128_t(out)
    }
}

pub fn gcd_u128(a: u128, b: u128) -> u128 {
    unsafe {
        let mut out = sys::u128_t { v0: 0, v1: 0 };
        let aa = to_u128_t(a);
        let bb = to_u128_t(b);
        sys::gcd_u128(&mut out, &aa, &bb);
        from_u128_t(out)
    }
}

pub fn gcd_i128(a: i128, b: i128) -> i128 {
    unsafe {
        let mut out = sys::s128_t { v0: 0, v1: 0 };
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        sys::gcd_s128(&mut out, &aa, &bb);
        from_s128_t(out)
    }
}

pub fn sqrt_u128(x: u128) -> u128 {
    unsafe {
        let mut out = sys::u128_t { v0: 0, v1: 0 };
        let xx = to_u128_t(x);
        sys::sqrt_u128_u128(&mut out, &xx);
        from_u128_t(out)
    }
}

pub fn sqrt_i128(x: i128) -> i128 {
    unsafe {
        let mut out = sys::s128_t { v0: 0, v1: 0 };
        let xx = to_s128_t(x);
        sys::sqrt_s128_s128(&mut out, &xx);
        from_s128_t(out)
    }
}

pub fn mul_u128(a: u128, b: u128) -> u128 {
    unsafe {
        let mut out = sys::u128_t { v0: 0, v1: 0 };
        let aa = to_u128_t(a);
        let bb = to_u128_t(b);
        sys::mul_u128_u128_u128(&mut out, &aa, &bb);
        from_u128_t(out)
    }
}

pub fn divrem_u128(n: u128, d: u128) -> (u128, u128) {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut q = sys::u128_t { v0: 0, v1: 0 };
        let mut r = sys::u128_t { v0: 0, v1: 0 };
        let nn = to_u128_t(n);
        let dd = to_u128_t(d);
        sys::divrem_u128_u128_u128_u128(&mut q, &mut r, &nn, &dd);
        (from_u128_t(q), from_u128_t(r))
    }
}

pub fn is_divisible_i128_i128(x: i128, y: i128) -> bool {
    assert!(y != 0, "division by zero");
    unsafe {
        let xx = to_s128_t(x);
        let yy = to_s128_t(y);
        sys::is_divisible_s128_s128(&xx, &yy) != 0
    }
}

pub fn is_divisible_i128_i64(x: i128, y: i64) -> bool {
    assert!(y != 0, "division by zero");
    unsafe {
        let xx = to_s128_t(x);
        sys::is_divisible_s128_s64(&xx, y) != 0
    }
}

pub fn is_divisible_i128_u64(x: i128, y: u64) -> bool {
    assert!(y != 0, "division by zero");
    unsafe {
        let xx = to_s128_t(x);
        sys::is_divisible_s128_u64(&xx, y) != 0
    }
}

pub fn div_s128_i128_i128(n: i128, d: i128) -> i128 {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut q = sys::s128_t { v0: 0, v1: 0 };
        let nn = to_s128_t(n);
        let dd = to_s128_t(d);
        sys::div_s128_s128_s128(&mut q, &nn, &dd);
        from_s128_t(q)
    }
}

pub fn div_s128_i128_i64(n: i128, d: i64) -> i128 {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut q = sys::s128_t { v0: 0, v1: 0 };
        let nn = to_s128_t(n);
        sys::div_s128_s128_s64(&mut q, &nn, d);
        from_s128_t(q)
    }
}

pub fn mod_i128_i128_i128(n: i128, d: i128) -> i128 {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut r = sys::s128_t { v0: 0, v1: 0 };
        let nn = to_s128_t(n);
        let dd = to_s128_t(d);
        sys::mod_s128_s128_s128(&mut r, &nn, &dd);
        from_s128_t(r)
    }
}

pub fn mod_i64_i128_i64(n: i128, d: i64) -> i64 {
    assert!(d != 0, "division by zero");
    unsafe {
        let nn = to_s128_t(n);
        sys::mod_s64_s128_s64(&nn, d)
    }
}

pub fn mod_i64_i128_u64(n: i128, d: u64) -> i64 {
    assert!(d != 0, "division by zero");
    unsafe {
        let nn = to_s128_t(n);
        sys::mod_s64_s128_u64(&nn, d)
    }
}

pub fn xgcd_binary_i128(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_binary_l2r_s128(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_left_binary_i128(a: i128, b: i128) -> LeftXgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_left_binary_l2r_s128(&mut d, &mut s, &aa, &bb);
        LeftXgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
        }
    }
}

pub fn xgcd_divrem_i128(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_divrem_s128(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_stein_i128(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_stein_s128(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_brent_i128(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_brent_s128(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_lehmer_i128(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_lehmer_s128(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_lehmer_i128_s32eea(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_lehmer_s128_s32eea(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_lehmer_i128_s64eea(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_lehmer_s128_s64eea(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_lehmer_i128_s64l2r(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_lehmer_s128_s64l2r(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_mpz_i64(a: i64, b: i64) -> crate::XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_mpz_s64(&mut s, &mut t, a, b);
        crate::XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_mpz_i128(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_mpz_s128(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_shallit_i128(a: i128, b: i128) -> XgcdI128 {
    unsafe {
        let aa = to_s128_t(a);
        let bb = to_s128_t(b);
        let mut d = sys::s128_t { v0: 0, v1: 0 };
        let mut s = sys::s128_t { v0: 0, v1: 0 };
        let mut t = sys::s128_t { v0: 0, v1: 0 };
        sys::xgcd_shallit_s128(&mut d, &mut s, &mut t, &aa, &bb);
        XgcdI128 {
            gcd: from_s128_t(d),
            s: from_s128_t(s),
            t: from_s128_t(t),
        }
    }
}

pub fn xgcd_shortpartial_binary_l2r_i128(
    mut state: PartialXgcdI128,
    bound: i64,
) -> (u64, PartialXgcdI128) {
    unsafe {
        let mut r1 = to_s128_t(state.r1);
        let mut r0 = to_s128_t(state.r0);
        let steps = sys::xgcd_shortpartial_binary_l2r_s128(
            &mut r1,
            &mut r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state.r1 = from_s128_t(r1);
        state.r0 = from_s128_t(r0);
        (steps, state)
    }
}

pub fn xgcd_shortpartial_brent_i128(mut state: PartialXgcdI128, bound: i64) -> PartialXgcdI128 {
    unsafe {
        let mut r1 = to_s128_t(state.r1);
        let mut r0 = to_s128_t(state.r0);
        sys::xgcd_shortpartial_brent_s128(&mut r1, &mut r0, &mut state.c1, &mut state.c0, bound);
        state.r1 = from_s128_t(r1);
        state.r0 = from_s128_t(r0);
        state
    }
}

pub fn xgcd_shortpartial_divrem_i128(mut state: PartialXgcdI128, bound: i64) -> PartialXgcdI128 {
    unsafe {
        let mut r1 = to_s128_t(state.r1);
        let mut r0 = to_s128_t(state.r0);
        sys::xgcd_shortpartial_divrem_s128(&mut r1, &mut r0, &mut state.c1, &mut state.c0, bound);
        state.r1 = from_s128_t(r1);
        state.r0 = from_s128_t(r0);
        state
    }
}

pub fn xgcd_shortpartial_lehmer_i128_eea64(
    mut state: PartialXgcdI128,
    bound: i64,
) -> PartialXgcdI128 {
    unsafe {
        let mut r1 = to_s128_t(state.r1);
        let mut r0 = to_s128_t(state.r0);
        sys::xgcd_shortpartial_lehmer_s128_eea64(
            &mut r1,
            &mut r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state.r1 = from_s128_t(r1);
        state.r0 = from_s128_t(r0);
        state
    }
}

pub fn xgcd_shortpartial_lehmer_i128_brent64(
    mut state: PartialXgcdI128,
    bound: i64,
) -> PartialXgcdI128 {
    unsafe {
        let mut r1 = to_s128_t(state.r1);
        let mut r0 = to_s128_t(state.r0);
        sys::xgcd_shortpartial_lehmer_s128_brent64(
            &mut r1,
            &mut r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state.r1 = from_s128_t(r1);
        state.r0 = from_s128_t(r0);
        state
    }
}

pub fn xgcd_shortpartial_lehmer_i128_l2r64(
    mut state: PartialXgcdI128,
    bound: i64,
) -> PartialXgcdI128 {
    unsafe {
        let mut r1 = to_s128_t(state.r1);
        let mut r0 = to_s128_t(state.r0);
        sys::xgcd_shortpartial_lehmer_s128_l2r64(
            &mut r1,
            &mut r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state.r1 = from_s128_t(r1);
        state.r0 = from_s128_t(r0);
        state
    }
}

pub fn xgcd_shortpartial_mpz_i128(mut state: PartialXgcdI128, bound: i64) -> PartialXgcdI128 {
    unsafe {
        let mut r1 = to_s128_t(state.r1);
        let mut r0 = to_s128_t(state.r0);
        sys::xgcd_shortpartial_mpz_s128(&mut r1, &mut r0, &mut state.c1, &mut state.c0, bound);
        state.r1 = from_s128_t(r1);
        state.r0 = from_s128_t(r0);
        state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u128_ops_work() {
        assert_eq!(gcd_binary_u128(48, 18), 6);
        assert_eq!(gcd_u128(48, 18), 6);
        assert_eq!(gcd_i128(-48, 18), 6);
        assert_eq!(sqrt_u128(10_000), 100);
        assert_eq!(sqrt_i128(-10_000), 100);
        assert_eq!(mul_u128(123, 456), 123_u128.wrapping_mul(456));
        let (q, r) = divrem_u128(1000, 33);
        assert_eq!(q, 30);
        assert_eq!(r, 10);
    }

    #[test]
    fn xgcd_i128_identity_holds() {
        let a = 123_456_789_123_456_789_i128;
        let b = 98_765_432_100_000_003_i128;
        for out in [
            xgcd_binary_i128(a, b),
            xgcd_divrem_i128(a, b),
            xgcd_stein_i128(a, b),
            xgcd_brent_i128(a, b),
            xgcd_lehmer_i128(a, b),
            xgcd_lehmer_i128_s32eea(a, b),
            xgcd_lehmer_i128_s64eea(a, b),
            xgcd_lehmer_i128_s64l2r(a, b),
            xgcd_mpz_i128(a, b),
            xgcd_shallit_i128(a, b),
        ] {
            assert_eq!(out.s * a + out.t * b, out.gcd);
        }
    }

    #[test]
    fn partial_paths_update_state() {
        let state = PartialXgcdI128 {
            r1: 987_654_321,
            r0: 123_456_789,
            c1: 0,
            c0: -1,
        };
        let (_steps, state) = xgcd_shortpartial_binary_l2r_i128(state, 64);
        let state = xgcd_shortpartial_brent_i128(state, 64);
        let state = xgcd_shortpartial_divrem_i128(state, 64);
        let state = xgcd_shortpartial_lehmer_i128_eea64(state, 64);
        let state = xgcd_shortpartial_lehmer_i128_brent64(state, 64);
        let state = xgcd_shortpartial_lehmer_i128_l2r64(state, 64);
        let _ = xgcd_shortpartial_mpz_i128(state, 64);
    }
}
