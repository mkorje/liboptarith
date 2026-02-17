use optarith_sys::xgcd_binary_l2r_s32;

fn verify(a: i32, b: i32) {
    let mut s = 0_i32;
    let mut t = 0_i32;
    let g = unsafe { xgcd_binary_l2r_s32(&mut s, &mut t, a, b) };
    let lhs = (s as i64) * (a as i64) + (t as i64) * (b as i64);
    assert_eq!(
        lhs, g as i64,
        "verification failed for a={a}, b={b}, s={s}, t={t}, g={g}"
    );
}

#[test]
fn test_gcd_vectors_from_c_test() {
    verify(0, 0);

    verify(0, 10);
    verify(10, 0);
    verify(0, -10);
    verify(-10, 0);

    verify(10, 10);
    verify(-10, 10);
    verify(10, -10);
    verify(-10, -10);

    verify(5, 10);
    verify(10, 5);
    verify(-5, 10);
    verify(10, -5);
    verify(5, -10);
    verify(-10, 5);
    verify(-5, -10);
    verify(-10, -5);

    verify(2, 128);
    verify(128, 2);
    verify(-2, 128);
    verify(128, -2);
    verify(2, -128);
    verify(-128, 2);
    verify(-2, -128);
    verify(-128, -2);

    verify(21, 35);
    verify(21, -35);
    verify(-21, 35);
    verify(-21, -35);
    verify(35, 21);
    verify(35, -21);
    verify(-35, 21);
    verify(-35, -21);
}
