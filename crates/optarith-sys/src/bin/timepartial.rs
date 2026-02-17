use optarith_sys as sys;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

const MIN_BITS: i32 = 1;
const MAX_BITS: i32 = 32;

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

fn rands_u32(n: usize, bits: i32) -> Vec<u32> {
    let mask = if bits == 32 {
        u32::MAX
    } else {
        (1_u32 << bits) - 1
    };
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        let mut x = unsafe { sys::rand_u32() } & mask;
        if x == 0 {
            x = 1;
        }
        out.push(x);
    }
    out
}

fn time_partial_bits(bits: i32, triples: usize) -> u128 {
    let bs = rands_u32(triples, bits / 2);
    let mut r1s = rands_u32(triples, bits);
    let r0s = rands_u32(triples, bits);
    for i in 0..triples {
        r1s[i] %= r0s[i];
    }

    let start = Instant::now();
    for i in 0..triples {
        let mut r1 = r1s[i] as i32;
        let mut r0 = r0s[i] as i32;
        let mut c1 = 0_i32;
        let mut c0 = 0_i32;
        let bound = bs[i] as i32;
        unsafe { sys::xgcd_partial_divrem_s32(&mut r1, &mut r0, &mut c1, &mut c0, bound) };
    }
    start.elapsed().as_nanos()
}

fn usage(prog: &str) {
    eprintln!("Usage: {prog} <rand_seed> <triples> [-d dumpfile.txt]");
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
    let triples = args[2]
        .parse::<usize>()
        .expect("triples should be a positive integer");

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

    println!("# bits\t  ns/partial");
    for bits in MIN_BITS..=MAX_BITS {
        let total_nanos = time_partial_bits(bits, triples);
        let ns_per = (total_nanos as f64) / (triples as f64);
        println!("{bits:2}\t{ns_per:7.3}");
        if let Some(f) = dump_file.as_mut() {
            writeln!(f, "{bits}\t{ns_per}").expect("failed to write dump");
        }
    }
}
