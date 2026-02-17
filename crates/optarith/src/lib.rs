#[cfg(feature = "gmp")]
pub mod gmp;
pub mod int128;

pub mod raw {
    pub use optarith_sys::*;
}

pub use int128::{
    LeftXgcdI128, PartialXgcdI128, XgcdI128, div_s128_i128_i64, div_s128_i128_i128, divrem_u128,
    gcd_binary_u128, gcd_i128, gcd_u128, is_divisible_i128_i64, is_divisible_i128_i128,
    is_divisible_i128_u64, mod_i64_i128_i64, mod_i64_i128_u64, mod_i128_i128_i128, mul_u128,
    sqrt_i128, sqrt_u128, xgcd_binary_i128, xgcd_brent_i128, xgcd_divrem_i128,
    xgcd_left_binary_i128, xgcd_lehmer_i128, xgcd_lehmer_i128_s32eea, xgcd_lehmer_i128_s64eea,
    xgcd_lehmer_i128_s64l2r, xgcd_mpz_i64, xgcd_mpz_i128, xgcd_shallit_i128,
    xgcd_shortpartial_binary_l2r_i128, xgcd_shortpartial_brent_i128, xgcd_shortpartial_divrem_i128,
    xgcd_shortpartial_lehmer_i128_brent64, xgcd_shortpartial_lehmer_i128_eea64,
    xgcd_shortpartial_lehmer_i128_l2r64, xgcd_shortpartial_mpz_i128, xgcd_stein_i128,
};

use optarith_sys as sys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XgcdU32 {
    pub gcd: u32,
    pub s: i32,
    pub t: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XgcdI32 {
    pub gcd: i32,
    pub s: i32,
    pub t: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XgcdI64 {
    pub gcd: i64,
    pub s: i64,
    pub t: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftXgcdI32 {
    pub gcd: i32,
    pub s: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeftXgcdI64 {
    pub gcd: i64,
    pub s: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartialXgcdI32 {
    pub r1: i32,
    pub r0: i32,
    pub c1: i32,
    pub c0: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartialXgcdI64 {
    pub r1: i64,
    pub r0: i64,
    pub c1: i64,
    pub c0: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmallQCase {
    Q0,
    Q1,
    Q2,
    Q3,
    Q4,
    Q5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XgcdCoreI64 {
    BinaryL2r,
    BinaryL2rBranching,
    Stein,
    Divrem,
    Brent,
    Lehmer,
    Shallit,
    Flint,
    BlockStein2,
    BlockStein3,
    BlockStein4,
    BlockStein5,
}

pub type QuadI32 = (i32, i32, i32, i32);

#[inline]
fn c_int_to_bool(x: i32) -> bool {
    x != 0
}

pub fn rand_u8() -> u8 {
    unsafe { sys::rand_u8() }
}

pub fn rand_u16() -> u16 {
    unsafe { sys::rand_u16() }
}

pub fn rand_u32() -> u32 {
    unsafe { sys::rand_u32() }
}

pub fn rand_u64() -> u64 {
    unsafe { sys::rand_u64() }
}

pub fn ceil_pow2_u32(x: u32) -> u32 {
    unsafe { sys::ceil_pow2_u32(x) }
}

pub fn sub_with_mask_u32(a: u32, b: u32) -> (u32, u32) {
    unsafe {
        let mut m = 0;
        let r = sys::sub_with_mask_u32(&mut m, a, b);
        (m, r)
    }
}

pub fn sub_with_mask_i32(a: i32, b: i32) -> (u32, i32) {
    unsafe {
        let mut m = 0;
        let r = sys::sub_with_mask_s32(&mut m, a, b);
        (m, r)
    }
}

pub fn sub_with_mask_u64(a: u64, b: u64) -> (u64, i64) {
    unsafe {
        let mut m = 0;
        let r = sys::sub_with_mask_u64(&mut m, a, b);
        (m, r)
    }
}

pub fn sub_with_mask_i64(a: i64, b: i64) -> (u64, i64) {
    unsafe {
        let mut m = 0;
        let r = sys::sub_with_mask_s64(&mut m, a, b);
        (m, r)
    }
}

pub fn swap_u32(a: u32, b: u32) -> (u32, u32) {
    unsafe {
        let mut x = a;
        let mut y = b;
        sys::swap_u32(&mut x, &mut y);
        (x, y)
    }
}

pub fn swap_i32(a: i32, b: i32) -> (i32, i32) {
    unsafe {
        let mut x = a;
        let mut y = b;
        sys::swap_s32(&mut x, &mut y);
        (x, y)
    }
}

pub fn swap_i64(a: i64, b: i64) -> (i64, i64) {
    unsafe {
        let mut x = a;
        let mut y = b;
        sys::swap_s64(&mut x, &mut y);
        (x, y)
    }
}

pub fn cond_swap_i32(a: i32, b: i32) -> (i32, i32) {
    unsafe {
        let mut x = a;
        let mut y = b;
        sys::cond_swap_s32(&mut x, &mut y);
        (x, y)
    }
}

pub fn cond_swap_i64(a: i64, b: i64) -> (i64, i64) {
    unsafe {
        let mut x = a;
        let mut y = b;
        sys::cond_swap_s64(&mut x, &mut y);
        (x, y)
    }
}

pub fn cond_swap2_i32(u1: i32, u2: i32, v1: i32, v2: i32) -> ((i32, i32), (i32, i32)) {
    unsafe {
        let mut a1 = u1;
        let mut a2 = u2;
        let mut b1 = v1;
        let mut b2 = v2;
        sys::cond_swap2_s32(&mut a1, &mut a2, &mut b1, &mut b2);
        ((a1, a2), (b1, b2))
    }
}

pub fn cond_swap2_i64(u1: i64, u2: i64, v1: i64, v2: i64) -> ((i64, i64), (i64, i64)) {
    unsafe {
        let mut a1 = u1;
        let mut a2 = u2;
        let mut b1 = v1;
        let mut b2 = v2;
        sys::cond_swap2_s64(&mut a1, &mut a2, &mut b1, &mut b2);
        ((a1, a2), (b1, b2))
    }
}

pub fn cond_swap3_i32(
    u1: i32,
    u2: i32,
    u3: i32,
    v1: i32,
    v2: i32,
    v3: i32,
) -> (u32, (i32, i32, i32), (i32, i32, i32)) {
    unsafe {
        let mut a1 = u1;
        let mut a2 = u2;
        let mut a3 = u3;
        let mut b1 = v1;
        let mut b2 = v2;
        let mut b3 = v3;
        let m = sys::cond_swap3_s32(&mut a1, &mut a2, &mut a3, &mut b1, &mut b2, &mut b3);
        (m, (a1, a2, a3), (b1, b2, b3))
    }
}

pub fn cond_swap3_i64(
    u1: i64,
    u2: i64,
    u3: i64,
    v1: i64,
    v2: i64,
    v3: i64,
) -> (u64, (i64, i64, i64), (i64, i64, i64)) {
    unsafe {
        let mut a1 = u1;
        let mut a2 = u2;
        let mut a3 = u3;
        let mut b1 = v1;
        let mut b2 = v2;
        let mut b3 = v3;
        let m = sys::cond_swap3_s64(&mut a1, &mut a2, &mut a3, &mut b1, &mut b2, &mut b3);
        (m, (a1, a2, a3), (b1, b2, b3))
    }
}

pub fn cond_swap3_i64_mixed(
    u1: i64,
    u2: i64,
    u3: u64,
    v1: i64,
    v2: i64,
    v3: u64,
) -> ((i64, i64, u64), (i64, i64, u64)) {
    unsafe {
        let mut a1 = u1;
        let mut a2 = u2;
        let mut a3 = u3;
        let mut b1 = v1;
        let mut b2 = v2;
        let mut b3 = v3;
        sys::cond_swap3_s64_mixed(&mut a1, &mut a2, &mut a3, &mut b1, &mut b2, &mut b3);
        ((a1, a2, a3), (b1, b2, b3))
    }
}

pub fn cond_swap4_i32(u: QuadI32, v: QuadI32) -> (QuadI32, QuadI32) {
    unsafe {
        let (u1, u2, u3, u4) = u;
        let (v1, v2, v3, v4) = v;
        let mut a1 = u1;
        let mut a2 = u2;
        let mut a3 = u3;
        let mut a4 = u4;
        let mut b1 = v1;
        let mut b2 = v2;
        let mut b3 = v3;
        let mut b4 = v4;
        sys::cond_swap4_s32(
            &mut a1, &mut a2, &mut a3, &mut a4, &mut b1, &mut b2, &mut b3, &mut b4,
        );
        ((a1, a2, a3, a4), (b1, b2, b3, b4))
    }
}

pub fn negate_using_mask_i32(mask: u32, x: i32) -> i32 {
    unsafe { sys::negate_using_mask_s32(mask, x) }
}

pub fn negate_using_mask_i64(mask: u64, x: i64) -> i64 {
    unsafe { sys::negate_using_mask_s64(mask, x) }
}

pub fn cond_negate_i32(cond: i32, x: i32) -> i32 {
    unsafe { sys::cond_negate_s32(cond, x) }
}

pub fn cond_negate_i64(cond: i64, x: i64) -> i64 {
    unsafe { sys::cond_negate_s64(cond, x) }
}

pub fn abs_i32(x: i32) -> u32 {
    unsafe { sys::abs_s32(x) }
}

pub fn abs_i64(x: i64) -> u64 {
    unsafe { sys::abs_s64(x) }
}

pub fn s64_fits_i32(x: i64) -> bool {
    unsafe { c_int_to_bool(sys::s64_is_s32(x)) }
}

pub fn msb_u32(x: u32) -> i32 {
    unsafe { sys::msb_u32(x) }
}

pub fn msb_u64(x: u64) -> i32 {
    unsafe { sys::msb_u64(x) }
}

pub fn lsb_u32(x: u32) -> i32 {
    unsafe { sys::lsb_u32(x) }
}

pub fn lsb_u64(x: u64) -> i32 {
    unsafe { sys::lsb_u64(x) }
}

pub fn lsb_i32(x: i32) -> i32 {
    unsafe { sys::lsb_s32(x) }
}

pub fn lsb_i64(x: i64) -> i32 {
    unsafe { sys::lsb_s64(x) }
}

pub fn setbit_u32(x: u32, i: i32) -> u32 {
    unsafe { sys::setbit_u32(x, i) }
}

pub fn setbit_u64(x: u64, i: i32) -> u64 {
    unsafe { sys::setbit_u64(x, i) }
}

pub fn clrbit_u32(x: u32, i: i32) -> u32 {
    unsafe { sys::clrbit_u32(x, i) }
}

pub fn clrbit_u64(x: u64, i: i32) -> u64 {
    unsafe { sys::clrbit_u64(x, i) }
}

pub fn numbits_u32(x: u32) -> i32 {
    unsafe { sys::numbits_u32(x) }
}

pub fn numbits_i32(x: i32) -> i32 {
    unsafe { sys::numbits_s32(x) }
}

pub fn numbits_i64(x: i64) -> i32 {
    unsafe { sys::numbits_s64(x) }
}

pub fn addmod_i32(x: i32, y: i32, m: i32) -> i32 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::addmod_s32(x, y, m) }
}

pub fn addmod_i64(x: i64, y: i64, m: i64) -> i64 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::addmod_s64(x, y, m) }
}

pub fn submod_i32(x: i32, y: i32, m: i32) -> i32 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::submod_s32(x, y, m) }
}

pub fn submod_i64(x: i64, y: i64, m: i64) -> i64 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::submod_s64(x, y, m) }
}

pub fn mulmod_u32(x: u32, y: u32, m: u32) -> u32 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::mulmod_u32(x, y, m) }
}

pub fn mulmod_u64(x: u64, y: u64, m: u64) -> u64 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::mulmod_u64(x, y, m) }
}

pub fn mulmod_i32(x: i32, y: i32, m: i32) -> i32 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::mulmod_s32(x, y, m) }
}

pub fn mulmod_i64(x: i64, y: i64, m: i64) -> i64 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::mulmod_s64(x, y, m) }
}

pub fn divrem_u32(n: u32, d: u32) -> (u32, u32) {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut q = 0;
        let mut r = 0;
        sys::divrem_u32(&mut q, &mut r, n, d);
        (q, r)
    }
}

pub fn divrem_i32(n: i32, d: i32) -> (i32, i32) {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut q = 0;
        let mut r = 0;
        sys::divrem_s32(&mut q, &mut r, n, d);
        (q, r)
    }
}

pub fn divrem_u64(n: u64, d: u64) -> (u64, u64) {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut q = 0;
        let mut r = 0;
        sys::divrem_u64(&mut q, &mut r, n, d);
        (q, r)
    }
}

pub fn divrem_i64(n: i64, d: i64) -> (i64, i64) {
    assert!(d != 0, "division by zero");
    unsafe {
        let mut q = 0;
        let mut r = 0;
        sys::divrem_s64(&mut q, &mut r, n, d);
        (q, r)
    }
}

pub fn sqrt_u32(x: u32) -> u32 {
    unsafe { sys::sqrt_u32(x) }
}

pub fn sqrt_u64(x: u64) -> u64 {
    unsafe { sys::sqrt_u64(x) }
}

pub fn is_square_u64(x: u64) -> bool {
    unsafe { c_int_to_bool(sys::is_square_u64(x)) }
}

pub fn is_square_i64(x: i64) -> bool {
    unsafe { c_int_to_bool(sys::is_square_s64(x)) }
}

pub fn expmod_u64(a: u64, e: u64, m: u64) -> u64 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::expmod_u64(a, e, m) }
}

pub fn mod_u32_u64_u32(n: u64, m: u32) -> u32 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::mod_u32_u64_u32(n, m) }
}

pub fn mod_i32_i64_u32(n: i64, m: u32) -> i32 {
    assert!(m != 0, "modulus must be non-zero");
    unsafe { sys::mod_s32_s64_u32(n, m) }
}

pub fn muladdmul_i64_4i32(f1: i32, f2: i32, f3: i32, f4: i32) -> i64 {
    unsafe { sys::muladdmul_s64_4s32(f1, f2, f3, f4) }
}

pub fn muladdmuldiv_i64(f1: i64, f2: i64, f3: i64, f4: i64, d: i64) -> i64 {
    assert!(d != 0, "division by zero");
    unsafe { sys::muladdmuldiv_s64(f1, f2, f3, f4, d) }
}

pub fn gcd_binary_u32(a: u32, b: u32) -> u32 {
    unsafe { sys::gcd_binary_l2r_u32(a, b) }
}

pub fn gcd_binary_u64(a: u64, b: u64) -> u64 {
    unsafe { sys::gcd_binary_l2r_u64(a, b) }
}

pub fn gcd_stein_i32(a: i32, b: i32) -> i32 {
    unsafe { sys::gcd_stein_s32(a, b) }
}

pub fn gcd_stein_i64(a: i64, b: i64) -> i64 {
    unsafe { sys::gcd_stein_s64(a, b) }
}

pub fn xgcd_binary_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_binary_l2r_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_binary_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_binary_l2r_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_binary_i64_branching(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_binary_l2rbranching_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_left_binary_i32(a: i32, b: i32) -> LeftXgcdI32 {
    unsafe {
        let mut s = 0;
        let gcd = sys::xgcd_left_binary_l2r_s32(&mut s, a, b);
        LeftXgcdI32 { gcd, s }
    }
}

pub fn xgcd_left_binary_i64(a: i64, b: i64) -> LeftXgcdI64 {
    unsafe {
        let mut s = 0;
        let gcd = sys::xgcd_left_binary_l2r_s64(&mut s, a, b);
        LeftXgcdI64 { gcd, s }
    }
}

pub fn xgcd_stein_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_stein_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_stein_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_stein_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_blockstein2_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein2_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_blockstein3_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein3_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_blockstein4_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein4_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_blockstein5_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein5_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_blockstein2_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein2_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_blockstein3_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein3_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_blockstein4_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein4_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_blockstein5_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_blockstein5_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_divrem_u32(m: u32, n: u32) -> XgcdU32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_divrem_u32(&mut s, &mut t, m, n);
        XgcdU32 { gcd, s, t }
    }
}

pub fn xgcd_divrem_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_divrem_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_divrem_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_divrem_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_left_divrem_i32(a: i32, b: i32) -> LeftXgcdI32 {
    unsafe {
        let mut s = 0;
        let gcd = sys::xgcd_left_divrem_s32(&mut s, a, b);
        LeftXgcdI32 { gcd, s }
    }
}

pub fn xgcd_left_divrem_i64(a: i64, b: i64) -> LeftXgcdI64 {
    unsafe {
        let mut s = 0;
        let gcd = sys::xgcd_left_divrem_s64(&mut s, a, b);
        LeftXgcdI64 { gcd, s }
    }
}

pub fn xgcd_brent_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_brent_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_brent_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_brent_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_lehmer_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_lehmer_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_lehmer_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_lehmer_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_flint_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_flint_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_flint_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_flint_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_shallit_i32(a: i32, b: i32) -> XgcdI32 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_shallit_s32(&mut s, &mut t, a, b);
        XgcdI32 { gcd, s, t }
    }
}

pub fn xgcd_shallit_i64(a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_shallit_s64(&mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_common_i64(core: XgcdCoreI64, a: i64, b: i64) -> XgcdI64 {
    unsafe {
        let core_fnc: sys::xgcd_s64_f = match core {
            XgcdCoreI64::BinaryL2r => Some(sys::xgcd_binary_l2r_s64 as _),
            XgcdCoreI64::BinaryL2rBranching => Some(sys::xgcd_binary_l2rbranching_s64 as _),
            XgcdCoreI64::Stein => Some(sys::xgcd_stein_s64 as _),
            XgcdCoreI64::Divrem => Some(sys::xgcd_divrem_s64 as _),
            XgcdCoreI64::Brent => Some(sys::xgcd_brent_s64 as _),
            XgcdCoreI64::Lehmer => Some(sys::xgcd_lehmer_s64 as _),
            XgcdCoreI64::Shallit => Some(sys::xgcd_shallit_s64 as _),
            XgcdCoreI64::Flint => Some(sys::xgcd_flint_s64 as _),
            XgcdCoreI64::BlockStein2 => Some(sys::xgcd_blockstein2_s64 as _),
            XgcdCoreI64::BlockStein3 => Some(sys::xgcd_blockstein3_s64 as _),
            XgcdCoreI64::BlockStein4 => Some(sys::xgcd_blockstein4_s64 as _),
            XgcdCoreI64::BlockStein5 => Some(sys::xgcd_blockstein5_s64 as _),
        };
        let mut s = 0;
        let mut t = 0;
        let gcd = sys::xgcd_common_s64(core_fnc, &mut s, &mut t, a, b);
        XgcdI64 { gcd, s, t }
    }
}

pub fn xgcd_smallq_case_i32(case: SmallQCase, u3: i32, v3: i32) -> XgcdI32 {
    unsafe {
        let mut u1 = 0;
        let mut u2 = 0;
        let gcd = match case {
            SmallQCase::Q0 => sys::xgcd_smallq0_case_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q1 => sys::xgcd_smallq1_case_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q2 => sys::xgcd_smallq2_case_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q3 => sys::xgcd_smallq3_case_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q4 => sys::xgcd_smallq4_case_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q5 => sys::xgcd_smallq5_case_s32(&mut u1, &mut u2, u3, v3),
        };
        XgcdI32 { gcd, s: u1, t: u2 }
    }
}

pub fn xgcd_smallq_case_i64(case: SmallQCase, u3: i64, v3: i64) -> XgcdI64 {
    unsafe {
        let mut u1 = 0;
        let mut u2 = 0;
        let gcd = match case {
            SmallQCase::Q0 => sys::xgcd_smallq0_case_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q1 => sys::xgcd_smallq1_case_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q2 => sys::xgcd_smallq2_case_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q3 => sys::xgcd_smallq3_case_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q4 => sys::xgcd_smallq4_case_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q5 => sys::xgcd_smallq5_case_s64(&mut u1, &mut u2, u3, v3),
        };
        XgcdI64 { gcd, s: u1, t: u2 }
    }
}

pub fn xgcd_smallq_loop_i32(case: SmallQCase, u3: i32, v3: i32) -> XgcdI32 {
    unsafe {
        let mut u1 = 0;
        let mut u2 = 0;
        let gcd = match case {
            SmallQCase::Q0 => sys::xgcd_smallq0_loop_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q1 => sys::xgcd_smallq1_loop_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q2 => sys::xgcd_smallq2_loop_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q3 => sys::xgcd_smallq3_loop_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q4 => sys::xgcd_smallq4_loop_s32(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q5 => sys::xgcd_smallq5_loop_s32(&mut u1, &mut u2, u3, v3),
        };
        XgcdI32 { gcd, s: u1, t: u2 }
    }
}

pub fn xgcd_smallq_loop_i64(case: SmallQCase, u3: i64, v3: i64) -> XgcdI64 {
    unsafe {
        let mut u1 = 0;
        let mut u2 = 0;
        let gcd = match case {
            SmallQCase::Q0 => sys::xgcd_smallq0_loop_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q1 => sys::xgcd_smallq1_loop_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q2 => sys::xgcd_smallq2_loop_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q3 => sys::xgcd_smallq3_loop_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q4 => sys::xgcd_smallq4_loop_s64(&mut u1, &mut u2, u3, v3),
            SmallQCase::Q5 => sys::xgcd_smallq5_loop_s64(&mut u1, &mut u2, u3, v3),
        };
        XgcdI64 { gcd, s: u1, t: u2 }
    }
}

pub fn xgcd_partial_binary_l2r_i32(mut state: PartialXgcdI32, bound: i32) -> (u32, PartialXgcdI32) {
    unsafe {
        let steps = sys::xgcd_partial_binary_l2r_s32(
            &mut state.r1,
            &mut state.r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        (steps, state)
    }
}

pub fn xgcd_partial_binary_l2r_i64(mut state: PartialXgcdI64, bound: i64) -> (u64, PartialXgcdI64) {
    unsafe {
        let steps = sys::xgcd_partial_binary_l2r_s64(
            &mut state.r1,
            &mut state.r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        (steps, state)
    }
}

pub fn xgcd_partial_divrem_i32(mut state: PartialXgcdI32, bound: i32) -> PartialXgcdI32 {
    unsafe {
        sys::xgcd_partial_divrem_s32(
            &mut state.r1,
            &mut state.r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state
    }
}

pub fn xgcd_partial_divrem_i64(mut state: PartialXgcdI64, bound: i64) -> PartialXgcdI64 {
    unsafe {
        sys::xgcd_partial_divrem_s64(
            &mut state.r1,
            &mut state.r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state
    }
}

pub fn xgcd_partial_brent_i32(mut state: PartialXgcdI32, bound: i32) -> PartialXgcdI32 {
    unsafe {
        sys::xgcd_partial_brent_s32(
            &mut state.r1,
            &mut state.r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state
    }
}

pub fn xgcd_partial_brent_i64(mut state: PartialXgcdI64, bound: i64) -> PartialXgcdI64 {
    unsafe {
        sys::xgcd_partial_brent_s64(
            &mut state.r1,
            &mut state.r0,
            &mut state.c1,
            &mut state.c0,
            bound,
        );
        state
    }
}

pub fn prime_index_at_least(n: u32) -> i32 {
    unsafe { sys::prime_index_ge(n) }
}

pub fn primes_up_to(n: i32) -> i32 {
    unsafe { sys::count_primes(n) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_i32(a: i32, b: i32, out: XgcdI32) {
        let lhs = out.s as i128 * a as i128 + out.t as i128 * b as i128;
        assert_eq!(lhs, out.gcd as i128);
    }

    fn check_i64(a: i64, b: i64, out: XgcdI64) {
        let lhs = out.s as i128 * a as i128 + out.t as i128 * b as i128;
        assert_eq!(lhs, out.gcd as i128);
    }

    #[test]
    fn binary_gcd_matches_known_values() {
        assert_eq!(gcd_binary_u32(48, 18), 6);
        assert_eq!(gcd_binary_u64(4096, 512), 512);
    }

    #[test]
    fn stein_gcd_matches_known_values() {
        assert_eq!(gcd_stein_i32(48, 18), 6);
        assert_eq!(gcd_stein_i64(4096, 512), 512);
    }

    #[test]
    fn xgcd_identities_hold() {
        let a32 = 252;
        let b32 = 198;
        for out in [
            xgcd_binary_i32(a32, b32),
            xgcd_stein_i32(a32, b32),
            xgcd_divrem_i32(a32, b32),
            xgcd_brent_i32(a32, b32),
            xgcd_lehmer_i32(a32, b32),
            xgcd_shallit_i32(a32, b32),
            xgcd_blockstein2_i32(a32, b32),
            xgcd_blockstein3_i32(a32, b32),
            xgcd_blockstein4_i32(a32, b32),
            xgcd_blockstein5_i32(a32, b32),
        ] {
            check_i32(a32, b32, out);
        }

        let a64 = 1_234_567_890_123_i64;
        let b64 = 9_876_543_210_i64;
        for out in [
            xgcd_binary_i64(a64, b64),
            xgcd_stein_i64(a64, b64),
            xgcd_divrem_i64(a64, b64),
            xgcd_brent_i64(a64, b64),
            xgcd_lehmer_i64(a64, b64),
            xgcd_shallit_i64(a64, b64),
            xgcd_blockstein2_i64(a64, b64),
            xgcd_blockstein3_i64(a64, b64),
            xgcd_blockstein4_i64(a64, b64),
            xgcd_blockstein5_i64(a64, b64),
        ] {
            check_i64(a64, b64, out);
        }
    }

    #[test]
    fn scalar_helpers_work() {
        assert_eq!(ceil_pow2_u32(17), 32);
        assert_eq!(sqrt_u32(10_000), 100);
        assert_eq!(sqrt_u64(10_000), 100);
        assert!(is_square_u64(10_000));
        assert!(is_square_i64(10_000));
        assert_eq!(divrem_u32(1000, 33), (30, 10));
        assert_eq!(divrem_i64(1000, 33), (30, 10));
    }

    #[test]
    fn prime_helpers_work() {
        assert_eq!(prime_index_at_least(97), 24);
        assert_eq!(primes_up_to(100), 25);
    }
}
