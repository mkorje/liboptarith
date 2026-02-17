use optarith_sys as sys;
use std::ffi::CString;

const TEST_COUNT: usize = 20_000;

#[inline]
fn from_u128(x: u128) -> sys::u128_t {
    sys::u128_t {
        v0: x as u64,
        v1: (x >> 64) as u64,
    }
}

#[inline]
fn to_u128(x: sys::u128_t) -> u128 {
    ((x.v1 as u128) << 64) | (x.v0 as u128)
}

#[inline]
fn rand_u128() -> u128 {
    unsafe {
        let mut x = sys::u128_t { v0: 0, v1: 0 };
        sys::rand_u128(&mut x);
        to_u128(x)
    }
}

#[inline]
fn random_shift() -> i32 {
    (unsafe { sys::rand_u32() } & 127) as i32
}

#[test]
fn msb_sanity() {
    for _ in 0..TEST_COUNT {
        let x = rand_u128();
        if x == 0 {
            continue;
        }
        let xx = from_u128(x);
        let y = unsafe { sys::msb_u128(&xx) } as i32;
        let z = (127 - x.leading_zeros()) as i32;
        assert_eq!(z, y);
    }
}

#[test]
fn shl_shr_sanity() {
    for _ in 0..TEST_COUNT {
        let x = rand_u128();
        let c = random_shift();

        let mut l = from_u128(x);
        unsafe { sys::shl_u128_int(&mut l, c) };
        assert_eq!(to_u128(l), x.wrapping_shl(c as u32));

        let mut r = from_u128(x);
        unsafe { sys::shr_u128_int(&mut r, c) };
        assert_eq!(to_u128(r), x >> (c as u32));
    }
}

#[test]
fn add_sub_mul_sanity() {
    for _ in 0..TEST_COUNT {
        let x = rand_u128() >> ((unsafe { sys::rand_u32() } % 126) as u32);
        let y = rand_u128() >> ((unsafe { sys::rand_u32() } % 126) as u32);

        let mut a = from_u128(x);
        let by = from_u128(y);
        unsafe { sys::add_u128_u128(&mut a, &by) };
        assert_eq!(to_u128(a), x.wrapping_add(y));

        let mut s = from_u128(x);
        unsafe { sys::sub_u128_u128(&mut s, &by) };
        assert_eq!(to_u128(s), x.wrapping_sub(y));

        let mut m = sys::u128_t { v0: 0, v1: 0 };
        let bx = from_u128(x);
        unsafe { sys::mul_u128_u128_u128(&mut m, &bx, &by) };
        assert_eq!(to_u128(m), x.wrapping_mul(y));
    }
}

#[test]
fn divrem_sanity() {
    for _ in 0..TEST_COUNT {
        let mut x = rand_u128() >> ((unsafe { sys::rand_u32() } % 126) as u32);
        let mut y = rand_u128() >> ((unsafe { sys::rand_u32() } % 126) as u32);
        if x < y {
            std::mem::swap(&mut x, &mut y);
        }
        if y == 0 {
            y = 1;
        }

        let mut q = sys::u128_t { v0: 0, v1: 0 };
        let mut r = sys::u128_t { v0: 0, v1: 0 };
        let nx = from_u128(x);
        let dy = from_u128(y);
        unsafe { sys::divrem_u128_u128_u128_u128(&mut q, &mut r, &nx, &dy) };
        let qv = to_u128(q);
        let rv = to_u128(r);

        assert_eq!(qv.wrapping_mul(y).wrapping_add(rv), x);
        assert!(rv < y);
        assert_eq!(qv, x / y);
        assert_eq!(rv, x % y);
    }
}

#[test]
fn sqrt_sanity() {
    for _ in 0..TEST_COUNT {
        let x = rand_u128() >> ((unsafe { sys::rand_u32() } % 126) as u32);
        let mut root = sys::u128_t { v0: 0, v1: 0 };
        let xx = from_u128(x);
        unsafe { sys::sqrt_u128_u128(&mut root, &xx) };
        let s = to_u128(root);

        assert!(s.saturating_mul(s) <= x);
        let sp1 = s.saturating_add(1);
        assert!(sp1.saturating_mul(sp1) > x);
    }
}

#[test]
fn decimal_string_roundtrip() {
    for _ in 0..TEST_COUNT {
        let x = rand_u128();
        let mut buf = vec![0_i8; 64];
        let xx = from_u128(x);

        let written = unsafe { sys::to_decstr_u128(buf.as_mut_ptr(), buf.len() as i32, &xx) };
        assert!(written > 0);
        let cstr = unsafe { std::ffi::CStr::from_ptr(buf.as_ptr()) };
        let s = cstr.to_str().expect("utf8");

        let mut out = sys::u128_t { v0: 0, v1: 0 };
        let cs = CString::new(s).expect("cstring");
        unsafe { sys::from_decstr_u128(&mut out, cs.as_ptr(), s.len() as i32) };
        assert_eq!(to_u128(out), x);
    }
}
