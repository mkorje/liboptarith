use optarith_sys as sys;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

const MIN_BITS: i32 = 1;
const MAX_BITS: i32 = 64;

#[inline]
fn current_nanos(start: Instant) -> u128 {
    start.elapsed().as_nanos()
}

fn full_cpu_load(secs: u64) -> i64 {
    let start = Instant::now();
    let mut sum = 0_i64;
    while start.elapsed() < Duration::from_secs(secs) {
        let a = (unsafe { sys::rand_u64() } as i64) & ((1_i64 << 59) - 1);
        let b = (unsafe { sys::rand_u64() } as i64) & ((1_i64 << 59) - 1);
        let mut s = 0_i64;
        let mut t = 0_i64;
        sum = sum.wrapping_add(unsafe { sys::xgcd_divrem_s64(&mut s, &mut t, a, b) });
    }
    sum
}

fn rands_u64(n: usize, bits: i32) -> Vec<u64> {
    let mask = if bits == 64 {
        u64::MAX
    } else {
        (1_u64 << bits) - 1
    };
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let mut x = unsafe { sys::rand_u64() } & mask;
        x = unsafe { sys::setbit_u64(x, bits - 1) };
        out.push(x);
    }
    out
}

fn sane(g: i64, s: i64, t: i64, a: i64, b: i64) -> bool {
    (s as i128) * (a as i128) + (t as i128) * (b as i128) == g as i128
}

fn time_gcd_set(rands: &[u64], pairs: usize) -> Option<u128> {
    for i in 0..pairs {
        let a = rands[i * 2] as i64;
        let b = rands[i * 2 + 1] as i64;
        let mut s = 0_i64;
        let mut t = 0_i64;
        let g = unsafe { sys::xgcd_binary_l2r_s64(&mut s, &mut t, a, b) };
        if !sane(g, s, t, a, b) {
            return None;
        }
    }

    let start = Instant::now();
    for i in 0..pairs {
        let a = rands[i * 2] as i64;
        let b = rands[i * 2 + 1] as i64;
        let mut s = 0_i64;
        let mut t = 0_i64;
        unsafe { sys::xgcd_binary_l2r_s64(&mut s, &mut t, a, b) };
    }
    Some(current_nanos(start))
}

fn time_gcd_bits(bits: i32, pairs: usize) -> Option<u128> {
    let r = rands_u64(pairs * 2, bits);
    time_gcd_set(&r, pairs)
}

fn usage(prog: &str) {
    eprintln!("Usage: {prog} <rand_seed> <pairs> [-d dumpfile.txt]");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 && args.len() != 5 {
        usage(&args[0]);
        std::process::exit(1);
    }

    let _seed = args[1]
        .parse::<u32>()
        .expect("rand_seed should be an unsigned integer");
    let pairs = args[2]
        .parse::<usize>()
        .expect("pairs should be a positive integer");

    let mut dump_file = None;
    if args.len() == 5 {
        if args[3] != "-d" {
            usage(&args[0]);
            std::process::exit(1);
        }
        dump_file = Some(File::create(&args[4]).expect("failed to create dump file"));
    }

    eprintln!("Priming CPU for 1 second.");
    let _ = full_cpu_load(1);

    println!("# bits\t  ns/gcd");
    for bits in MIN_BITS..=MAX_BITS {
        let Some(total_nanos) = time_gcd_bits(bits, pairs) else {
            eprintln!("sanity check failed for {bits}-bit inputs");
            std::process::exit(2);
        };
        let ns_per = (total_nanos as f64) / (pairs as f64);
        println!("{bits:2}\t{ns_per:7.3}");
        if let Some(f) = dump_file.as_mut() {
            writeln!(f, "{bits}\t{ns_per}").expect("failed to write dump");
        }
    }
}
