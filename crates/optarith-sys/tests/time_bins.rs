use std::process::Command;

#[test]
fn timegcd_smoke() {
    let bin = env!("CARGO_BIN_EXE_timegcd");
    let out = Command::new(bin)
        .args(["123", "8"])
        .output()
        .expect("failed to run timegcd");
    assert!(
        out.status.success(),
        "timegcd failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn timepartial_smoke() {
    let bin = env!("CARGO_BIN_EXE_timepartial");
    let out = Command::new(bin)
        .args(["123", "8"])
        .output()
        .expect("failed to run timepartial");
    assert!(
        out.status.success(),
        "timepartial failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}
