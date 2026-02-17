use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use gmp_mpfr_sys::gmp;
use optarith_sys as sys;
use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::path::Path;

unsafe extern "C" {
    fn free(ptr: *mut c_void);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseMpzError {
    InvalidInput,
    InvalidRadix,
    InteriorNul,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathError {
    InteriorNul,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GroupCost {
    pub compose: f64,
    pub square: f64,
    pub cube: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TwoThreeTerm {
    pub sign: i32,
    pub a: i32,
    pub b: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FactoredTwoThreeTerm16 {
    pub a: u16,
    pub b: u16,
}

impl From<GroupCost> for sys::group_cost_t {
    fn from(value: GroupCost) -> Self {
        Self {
            compose: value.compose,
            square: value.square,
            cube: value.cube,
        }
    }
}

impl From<sys::group_cost_t> for GroupCost {
    fn from(value: sys::group_cost_t) -> Self {
        Self {
            compose: value.compose,
            square: value.square,
            cube: value.cube,
        }
    }
}

impl From<TwoThreeTerm> for sys::two_three_term_t {
    fn from(value: TwoThreeTerm) -> Self {
        Self {
            sign: value.sign,
            a: value.a,
            b: value.b,
        }
    }
}

impl From<sys::two_three_term_t> for TwoThreeTerm {
    fn from(value: sys::two_three_term_t) -> Self {
        Self {
            sign: value.sign,
            a: value.a,
            b: value.b,
        }
    }
}

impl From<sys::factored_two_three_term16_t> for FactoredTwoThreeTerm16 {
    fn from(value: sys::factored_two_three_term16_t) -> Self {
        Self {
            a: value.a,
            b: value.b,
        }
    }
}

pub struct Mpz {
    raw: gmp::mpz_t,
}

impl Default for Mpz {
    fn default() -> Self {
        unsafe {
            let mut raw = MaybeUninit::<gmp::mpz_t>::uninit();
            gmp::mpz_init(raw.as_mut_ptr());
            Self {
                raw: raw.assume_init(),
            }
        }
    }
}

impl Clone for Mpz {
    fn clone(&self) -> Self {
        unsafe {
            let mut raw = MaybeUninit::<gmp::mpz_t>::uninit();
            gmp::mpz_init_set(raw.as_mut_ptr(), self.as_raw());
            Self {
                raw: raw.assume_init(),
            }
        }
    }
}

impl std::fmt::Debug for Mpz {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Mpz")
            .field(&self.to_string_radix(10))
            .finish()
    }
}

impl Drop for Mpz {
    fn drop(&mut self) {
        unsafe {
            gmp::mpz_clear(self.as_raw_mut());
        }
    }
}

impl Mpz {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_i64(value: i64) -> Self {
        let mut x = Self::new();
        x.set_i64(value);
        x
    }

    pub fn from_u64(value: u64) -> Self {
        let mut x = Self::new();
        x.set_u64(value);
        x
    }

    pub fn from_i128(value: i128) -> Self {
        let mut x = Self::new();
        unsafe {
            let y = crate::int128::to_s128_t_for_gmp(value);
            sys::mpz_set_s128(x.as_raw_mut().cast(), &y);
        }
        x
    }

    pub fn from_str_radix(value: &str, radix: u32) -> Result<Self, ParseMpzError> {
        let mut x = Self::new();
        x.set_str_radix(value, radix)?;
        Ok(x)
    }

    pub fn set_i64(&mut self, value: i64) {
        unsafe {
            gmp::mpz_set_si(self.as_raw_mut(), value as c_long);
        }
    }

    pub fn set_u64(&mut self, value: u64) {
        unsafe {
            gmp::mpz_set_ui(self.as_raw_mut(), value as c_ulong);
        }
    }

    pub fn set_str_radix(&mut self, value: &str, radix: u32) -> Result<(), ParseMpzError> {
        if !(2..=36).contains(&radix) {
            return Err(ParseMpzError::InvalidRadix);
        }
        let c_value = CString::new(value).map_err(|_| ParseMpzError::InteriorNul)?;
        let rc = unsafe { gmp::mpz_set_str(self.as_raw_mut(), c_value.as_ptr(), radix as c_int) };
        if rc == 0 {
            Ok(())
        } else {
            Err(ParseMpzError::InvalidInput)
        }
    }

    pub fn to_string_radix(&self, radix: u32) -> String {
        assert!((2..=36).contains(&radix), "radix must be in 2..=36");
        unsafe {
            let digits = gmp::mpz_sizeinbase(self.as_raw(), radix as c_int) as usize + 3;
            let mut buf = vec![0 as c_char; digits];
            let ptr = gmp::mpz_get_str(buf.as_mut_ptr(), radix as c_int, self.as_raw());
            CStr::from_ptr(ptr).to_string_lossy().into_owned()
        }
    }

    pub fn to_i64(&self) -> i64 {
        unsafe { sys::mpz_get_s64(self.as_raw().cast()) }
    }

    pub fn to_u64(&self) -> u64 {
        unsafe { sys::mpz_get_u64(self.as_raw().cast()) }
    }

    pub fn to_i128(&self) -> i128 {
        unsafe {
            let mut out = sys::s128_t { v0: 0, v1: 0 };
            sys::mpz_get_s128(&mut out, self.as_raw().cast());
            crate::int128::from_s128_t_for_gmp(out)
        }
    }

    pub fn cmp_i64(&self, value: i64) -> i32 {
        unsafe { sys::mpz_cmp_s64(self.as_raw().cast(), value) }
    }

    fn as_raw(&self) -> gmp::mpz_srcptr {
        &self.raw as *const gmp::mpz_t
    }

    fn as_raw_mut(&mut self) -> gmp::mpz_ptr {
        &mut self.raw as *mut gmp::mpz_t
    }
}

fn to_c_path(path: &Path) -> Result<CString, PathError> {
    let s = path.to_string_lossy();
    CString::new(s.as_bytes()).map_err(|_| PathError::InteriorNul)
}

unsafe fn copy_out_and_clear_mpz_array(ptr: *mut sys::mpz_t, n: usize) -> Vec<Mpz> {
    if ptr.is_null() || n == 0 {
        return Vec::new();
    }

    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let src = unsafe { ptr.add(i).cast::<gmp::mpz_t>() as *const gmp::mpz_t };
        let mut raw = MaybeUninit::<gmp::mpz_t>::uninit();
        unsafe { gmp::mpz_init_set(raw.as_mut_ptr(), src) };
        out.push(Mpz {
            raw: unsafe { raw.assume_init() },
        });
    }

    unsafe { sys::mpz_clear_array(ptr, n as c_int) };
    out
}

pub fn unit_costs() -> GroupCost {
    unsafe { GroupCost::from(sys::unit_costs) }
}

pub fn compose_only_costs() -> GroupCost {
    unsafe { GroupCost::from(sys::compose_only_costs) }
}

pub fn mod3(n: &Mpz) -> i32 {
    unsafe { sys::mpz_mod3(n.as_raw().cast()) }
}

pub fn mod9(n: &Mpz) -> i32 {
    unsafe { sys::mpz_mod9(n.as_raw().cast()) }
}

pub fn get_bit_window(n: &Mpz, i: i32, s: i32) -> u32 {
    unsafe { sys::mpz_get_bit_window(n.as_raw().cast(), i, s) }
}

pub fn product_list_u32(values: &[u32]) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_product_list_u32(
            out.as_raw_mut().cast(),
            values.as_ptr(),
            values.len() as c_int,
        );
    }
    out
}

pub fn mul_s64(a: &Mpz, b: i64) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_mul_s64(out.as_raw_mut().cast(), a.as_raw().cast(), b);
    }
    out
}

pub fn mulm(a: &Mpz, b: &Mpz, m: &Mpz) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_mulm(
            out.as_raw_mut().cast(),
            a.as_raw().cast(),
            b.as_raw().cast(),
            m.as_raw().cast(),
        );
    }
    out
}

pub fn addm(a: &Mpz, b: &Mpz, m: &Mpz) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_addm(
            out.as_raw_mut().cast(),
            a.as_raw().cast(),
            b.as_raw().cast(),
            m.as_raw().cast(),
        );
    }
    out
}

pub fn subm(a: &Mpz, b: &Mpz, m: &Mpz) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_subm(
            out.as_raw_mut().cast(),
            a.as_raw().cast(),
            b.as_raw().cast(),
            m.as_raw().cast(),
        );
    }
    out
}

pub fn to_string_via_c(n: &Mpz) -> String {
    unsafe {
        let p = sys::mpz_to_string(n.as_raw().cast());
        if p.is_null() {
            return String::new();
        }
        let s = CStr::from_ptr(p).to_string_lossy().into_owned();
        free(p.cast());
        s
    }
}

pub fn semiprime_list(count: i32, bits: i32, rand_seed: i32) -> Vec<Mpz> {
    if count <= 0 {
        return Vec::new();
    }
    unsafe {
        let ptr = sys::semiprime_list(count, bits, rand_seed);
        copy_out_and_clear_mpz_array(ptr, count as usize)
    }
}

pub fn save_array_or_die(values: &[Mpz], filename: &Path) -> Result<(), PathError> {
    if values.is_empty() {
        return Ok(());
    }

    let c_path = to_c_path(filename)?;
    unsafe {
        let raw = sys::mpz_init_array(values.len() as c_int);
        if raw.is_null() {
            return Ok(());
        }
        for (i, v) in values.iter().enumerate() {
            gmp::mpz_set(raw.add(i).cast::<gmp::mpz_t>(), v.as_raw());
        }
        sys::mpz_save_array_or_die(raw, values.len() as c_int, c_path.as_ptr());
        sys::mpz_clear_array(raw, values.len() as c_int);
    }
    Ok(())
}

pub fn load_array_or_die(filename: &Path) -> Result<Vec<Mpz>, PathError> {
    let c_path = to_c_path(filename)?;
    unsafe {
        let mut count = 0;
        let ptr = sys::mpz_load_array_or_die(&mut count, c_path.as_ptr());
        Ok(copy_out_and_clear_mpz_array(ptr, count.max(0) as usize))
    }
}

pub fn first_n_primes(n: i32) -> Vec<u32> {
    if n <= 0 {
        return Vec::new();
    }
    unsafe {
        let ptr = sys::first_n_primes(n);
        if ptr.is_null() {
            return Vec::new();
        }
        let out = std::slice::from_raw_parts(ptr, n as usize).to_vec();
        free(ptr.cast());
        out
    }
}

pub fn prime_powers(bound: u32) -> Vec<u32> {
    unsafe {
        let mut w = 0;
        let ptr = sys::mpz_prime_powers(&mut w, bound);
        if ptr.is_null() || w <= 0 {
            return Vec::new();
        }
        let out = std::slice::from_raw_parts(ptr, w as usize).to_vec();
        free(ptr.cast());
        out
    }
}

pub fn primorial(n: i32) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_primorial(out.as_raw_mut().cast(), n as c_int);
    }
    out
}

pub fn primorial_phi(n: i32) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_primorial_phi(out.as_raw_mut().cast(), n as c_int);
    }
    out
}

pub fn primorial_range(i: u32, j: u32) -> (Mpz, Mpz) {
    let mut prim = Mpz::new();
    let mut phi = Mpz::new();
    unsafe {
        sys::mpz_primorial_range(prim.as_raw_mut().cast(), phi.as_raw_mut().cast(), i, j);
    }
    (prim, phi)
}

pub fn bounded_power_primorial(limit: u32, bound: u32) -> (i32, Mpz) {
    let mut w = 0;
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_bounded_power_primorial(&mut w, out.as_raw_mut().cast(), limit, bound);
    }
    (w, out)
}

pub fn power_primorial(w: i32, bound: u32) -> Mpz {
    let mut out = Mpz::new();
    unsafe {
        sys::mpz_power_primorial(out.as_raw_mut().cast(), w, bound);
    }
    out
}

pub fn bounded_primorial(bound: &Mpz) -> (i32, Mpz, Mpz) {
    let mut w = 0;
    let mut primorial = Mpz::new();
    let mut phi = Mpz::new();
    unsafe {
        sys::mpz_bounded_primorial(
            &mut w as *mut c_int,
            primorial.as_raw_mut().cast(),
            phi.as_raw_mut().cast(),
            bound.as_raw().cast(),
        );
    }
    (w, primorial, phi)
}

pub fn primorials(n: i32, first_prime: i32) -> Vec<Mpz> {
    if n <= 0 {
        return Vec::new();
    }
    unsafe {
        let ptr = sys::mpz_primorials(n, first_prime);
        copy_out_and_clear_mpz_array(ptr, n as usize)
    }
}

pub fn rep_prune_closest(x: &Mpz, costs: GroupCost, keep_count: i32) -> Vec<TwoThreeTerm> {
    unsafe {
        let mut term_count = 0;
        let c_costs = sys::group_cost_t::from(costs);
        let ptr = sys::rep_prune_closest(&mut term_count, x.as_raw().cast(), &c_costs, keep_count);
        if ptr.is_null() || term_count <= 0 {
            return Vec::new();
        }
        let out = std::slice::from_raw_parts(ptr, term_count as usize)
            .iter()
            .copied()
            .map(TwoThreeTerm::from)
            .collect();
        free(ptr.cast());
        out
    }
}

pub fn factored_rep(rep: &mut [TwoThreeTerm]) -> Vec<FactoredTwoThreeTerm16> {
    if rep.is_empty() {
        return Vec::new();
    }

    unsafe {
        let mut term_count = 0;
        let mut c_rep: Vec<sys::two_three_term_t> = rep.iter().copied().map(Into::into).collect();
        let ptr = sys::factored_rep(&mut term_count, c_rep.as_mut_ptr(), c_rep.len() as c_int);
        for (dst, src) in rep.iter_mut().zip(c_rep.iter().copied()) {
            *dst = src.into();
        }
        if ptr.is_null() || term_count <= 0 {
            return Vec::new();
        }
        let out = std::slice::from_raw_parts(ptr, term_count as usize)
            .iter()
            .copied()
            .map(FactoredTwoThreeTerm16::from)
            .collect();
        free(ptr.cast());
        out
    }
}

pub fn factored_rep_prune_closest(
    x: &Mpz,
    costs: GroupCost,
    keep_count: i32,
) -> Vec<FactoredTwoThreeTerm16> {
    unsafe {
        let mut term_count = 0;
        let c_costs = sys::group_cost_t::from(costs);
        let ptr = sys::factored_rep_prune_closest(
            &mut term_count,
            x.as_raw().cast(),
            &c_costs,
            keep_count,
        );
        if ptr.is_null() || term_count <= 0 {
            return Vec::new();
        }
        let out = std::slice::from_raw_parts(ptr, term_count as usize)
            .iter()
            .copied()
            .map(FactoredTwoThreeTerm16::from)
            .collect();
        free(ptr.cast());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_formats() {
        let n = Mpz::from_str_radix("ff", 16).unwrap();
        assert_eq!(n.to_string_radix(10), "255");
    }

    #[test]
    fn computes_mods() {
        let n = Mpz::from_str_radix("123456789012345", 10).unwrap();
        assert_eq!(mod3(&n), 0);
        assert_eq!(mod9(&n), 6);
    }

    #[test]
    fn computes_products_and_primorials() {
        let p = product_list_u32(&[2, 3, 5, 7]);
        assert_eq!(p.to_string_radix(10), "210");

        let prim = primorial(5);
        assert_eq!(prim.to_string_radix(10), "2310");

        let phi = primorial_phi(5);
        assert_eq!(phi.to_string_radix(10), "480");

        let (range_prim, range_phi) = primorial_range(2, 5);
        assert_eq!(range_prim.to_string_radix(10), "30");
        assert_eq!(range_phi.to_string_radix(10), "8");
    }

    #[test]
    fn computes_bounded_primorial() {
        let bound = Mpz::from_u64(1000);
        let (w, prim, phi) = bounded_primorial(&bound);
        assert_eq!(w, 4);
        assert_eq!(prim.to_string_radix(10), "210");
        assert_eq!(phi.to_string_radix(10), "48");
    }

    #[test]
    fn list_helpers_work() {
        assert_eq!(first_n_primes(5), vec![2, 3, 5, 7, 11]);
        assert_eq!(prime_powers(10), vec![8, 9, 5, 7]);

        let ps = primorials(3, 2);
        assert_eq!(ps.len(), 3);
        assert_eq!(ps[0].to_string_radix(10), "2");
        assert_eq!(ps[1].to_string_radix(10), "6");
        assert_eq!(ps[2].to_string_radix(10), "30");
    }

    #[test]
    fn closest_rep_helpers_work() {
        let x = Mpz::from_u64(12345);
        let terms = rep_prune_closest(&x, unit_costs(), 8);
        assert!(!terms.is_empty());

        let mut terms2 = terms.clone();
        let factored = factored_rep(&mut terms2);
        assert!(!factored.is_empty());

        let factored2 = factored_rep_prune_closest(&x, unit_costs(), 8);
        assert!(!factored2.is_empty());
    }
}
