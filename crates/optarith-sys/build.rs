use std::collections::{BTreeSet, hash_map::DefaultHasher};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_MAX_PRIME: u32 = 104_729;
const MAX_PRIME_ENV: &str = "OPTARITH_MAX_PRIME";
const NATIVE_CACHE_VERSION: &str = "v1";

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let source_root = manifest_dir
        .join("../..")
        .canonicalize()
        .expect("failed to locate workspace root");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let target = env::var("TARGET").unwrap_or_default();
    let include_root = out_dir.join("include");
    let include_liboptarith = include_root.join("liboptarith");
    let max_prime = parse_max_prime();
    let tracked = tracked_paths(&source_root);

    let mut includes = vec![source_root.clone(), include_root.clone(), out_dir.clone()];
    let mut link_dirs: BTreeSet<PathBuf> = BTreeSet::new();
    let mut link_libs: BTreeSet<String> = BTreeSet::new();

    link_libs.insert("gmp".to_owned());
    let gmp_include_dir = PathBuf::from(
        env::var("DEP_GMP_INCLUDE_DIR")
            .expect("DEP_GMP_INCLUDE_DIR not set; gmp-mpfr-sys metadata missing"),
    );
    let gmp_lib_dir = PathBuf::from(
        env::var("DEP_GMP_LIB_DIR")
            .expect("DEP_GMP_LIB_DIR not set; gmp-mpfr-sys metadata missing"),
    );
    includes.push(gmp_include_dir.clone());
    link_dirs.insert(gmp_lib_dir.clone());
    let cc = env::var("CC").unwrap_or_else(|_| "cc".to_owned());
    let ar = env::var("AR").unwrap_or_else(|_| "ar".to_owned());
    let cache_key = compute_native_cache_key(
        &manifest_dir,
        &tracked,
        max_prime,
        &target,
        &gmp_include_dir,
        &gmp_lib_dir,
    );
    let cache_dir = native_cache_root(&out_dir).join(cache_key);
    let cache_bindings = cache_dir.join("bindings.rs");
    let cache_archive = cache_dir.join("liboptarith_c.a");
    let bindings_rs = out_dir.join("bindings.rs");

    if cache_bindings.exists() && cache_archive.exists() {
        fs::copy(&cache_bindings, &bindings_rs).unwrap_or_else(|e| {
            panic!(
                "copy {} -> {} failed: {e}",
                cache_bindings.display(),
                bindings_rs.display()
            )
        });
        emit_link_instructions(&cache_dir, &link_dirs, &link_libs);
        emit_rerun_instructions(&target, &tracked);
        return;
    }

    let (generated_sqrtmodp_c, generated_sqrtmodp_h) = generate_sqrtmodp_files(
        &source_root,
        &out_dir,
        &gmp_include_dir,
        &gmp_lib_dir,
        max_prime,
    );

    copy_headers(
        &source_root,
        &include_liboptarith,
        Some(generated_sqrtmodp_h.as_path()),
    );

    let c_files = discover_c_files(&source_root, Some(generated_sqrtmodp_c.as_path()));
    let headers = discover_header_files(&source_root);
    let bindgen_wrapper = out_dir.join("liboptarith_bindgen_wrapper.h");
    let bindgen_static_c = out_dir.join("liboptarith_bindgen_static.c");
    let cc_system_includes = discover_cc_system_includes(&cc);

    write_bindgen_wrapper(&bindgen_wrapper, &headers);
    generate_bindings(
        &bindgen_wrapper,
        &bindings_rs,
        &bindgen_static_c,
        &includes,
        &cc_system_includes,
    );

    let mut object_files = Vec::with_capacity(c_files.len());
    for source in &c_files {
        let stem = source
            .strip_prefix(&source_root)
            .map(|rel| {
                rel.with_extension("")
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "_")
            })
            .unwrap_or_else(|_| {
                format!(
                    "generated_{}",
                    source.file_stem().unwrap_or_default().to_string_lossy()
                )
            });
        let obj = out_dir.join(format!("{stem}.o"));
        compile_object(&cc, source, &obj, &includes);
        object_files.push(obj);
    }
    if bindgen_static_c.exists() {
        let obj = out_dir.join("liboptarith_bindgen_static.o");
        compile_object(&cc, &bindgen_static_c, &obj, &includes);
        object_files.push(obj);
    }

    let archive_path = out_dir.join("liboptarith_c.a");
    archive_objects(&ar, &archive_path, &object_files);

    fs::create_dir_all(&cache_dir).expect("create native cache dir");
    fs::copy(&bindings_rs, &cache_bindings).unwrap_or_else(|e| {
        panic!(
            "copy {} -> {} failed: {e}",
            bindings_rs.display(),
            cache_bindings.display()
        )
    });
    fs::copy(&archive_path, &cache_archive).unwrap_or_else(|e| {
        panic!(
            "copy {} -> {} failed: {e}",
            archive_path.display(),
            cache_archive.display()
        )
    });

    emit_link_instructions(&cache_dir, &link_dirs, &link_libs);
    emit_rerun_instructions(&target, &tracked);
}

fn parse_max_prime() -> u32 {
    match env::var(MAX_PRIME_ENV) {
        Ok(raw) => raw.parse::<u32>().unwrap_or_else(|e| {
            panic!(
                "failed to parse {MAX_PRIME_ENV}='{raw}' as u32 (example: {DEFAULT_MAX_PRIME}): {e}"
            )
        }),
        Err(_) => DEFAULT_MAX_PRIME,
    }
}

fn generate_sqrtmodp_files(
    source_root: &Path,
    out_dir: &Path,
    gmp_include_dir: &Path,
    gmp_lib_dir: &Path,
    max_prime: u32,
) -> (PathBuf, PathBuf) {
    let cxx = env::var("CXX").unwrap_or_else(|_| "c++".to_owned());
    let generator_src = source_root.join("code_gen").join("gen_sqrtmodp.cc");
    let primes_src = source_root.join("primes.c");
    let generator_bin = out_dir.join("gen_sqrtmodp");

    let mut compile = Command::new(&cxx);
    compile
        .arg("-I")
        .arg(source_root)
        .arg("-I")
        .arg(gmp_include_dir)
        .arg(&generator_src)
        .arg(&primes_src)
        .arg("-o")
        .arg(&generator_bin)
        .arg("-L")
        .arg(gmp_lib_dir)
        .arg("-lgmp");
    if let Ok(cxxflags) = env::var("CXXFLAGS") {
        for flag in cxxflags.split_whitespace() {
            compile.arg(flag);
        }
    }
    let status = compile.status().unwrap_or_else(|e| {
        panic!("failed to spawn code generator compiler ({cxx}): {e}");
    });
    assert!(
        status.success(),
        "code generator compile failed with status {status}"
    );

    let status = Command::new(&generator_bin)
        .current_dir(out_dir)
        .arg(max_prime.to_string())
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "failed to run code generator {}: {e}",
                generator_bin.display()
            )
        });
    assert!(
        status.success(),
        "code generator execution failed with status {status}"
    );

    let generated_c = out_dir.join("sqrtmodp_list.c");
    let generated_h = out_dir.join("sqrtmodp_list.h");
    assert!(
        generated_c.exists() && generated_h.exists(),
        "code generator did not produce {} and {}",
        generated_c.display(),
        generated_h.display()
    );

    (generated_c, generated_h)
}

fn copy_headers(
    source_root: &Path,
    include_liboptarith: &Path,
    generated_sqrtmodp_h: Option<&Path>,
) {
    fs::create_dir_all(include_liboptarith.join("gcd")).expect("create include output dirs");

    for entry in fs::read_dir(source_root).expect("read source root") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() == Some(OsStr::new("h")) {
            let dst = include_liboptarith.join(entry.file_name());
            fs::copy(&path, &dst).unwrap_or_else(|e| {
                panic!("copy {} -> {} failed: {e}", path.display(), dst.display())
            });
        }
    }

    let gcd_dir = source_root.join("gcd");
    for entry in fs::read_dir(&gcd_dir).expect("read gcd dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() == Some(OsStr::new("h")) {
            let dst = include_liboptarith.join("gcd").join(entry.file_name());
            fs::copy(&path, &dst).unwrap_or_else(|e| {
                panic!("copy {} -> {} failed: {e}", path.display(), dst.display())
            });
        }
    }

    if let Some(generated_header) = generated_sqrtmodp_h {
        let dst = include_liboptarith.join("sqrtmodp_list.h");
        fs::copy(generated_header, &dst).unwrap_or_else(|e| {
            panic!(
                "copy {} -> {} failed: {e}",
                generated_header.display(),
                dst.display()
            )
        });
    }
}

fn discover_c_files(source_root: &Path, generated_sqrtmodp_c: Option<&Path>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(source_root).expect("read source root") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() == Some(OsStr::new("c")) {
            files.push(path);
        }
    }

    let gcd_dir = source_root.join("gcd");
    for entry in fs::read_dir(&gcd_dir).expect("read gcd dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() == Some(OsStr::new("c")) {
            if path.file_name() == Some(OsStr::new("gcd_pari.c")) {
                continue;
            }
            files.push(path);
        }
    }

    if let Some(generated_c) = generated_sqrtmodp_c {
        files.retain(|path| path.file_name() != Some(OsStr::new("sqrtmodp_list.c")));
        files.push(generated_c.into());
    }

    files.sort();
    files
}

fn discover_header_files(source_root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(source_root).expect("read source root") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() != Some(OsStr::new("h")) {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|x| x.to_str())
            .unwrap_or_default();
        if matches!(name, "u128_c.h" | "s128_c.h") {
            continue;
        }
        files.push(path);
    }

    let gcd_dir = source_root.join("gcd");
    for entry in fs::read_dir(&gcd_dir).expect("read gcd dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() != Some(OsStr::new("h")) {
            continue;
        }
        let name = path
            .file_name()
            .and_then(|x| x.to_str())
            .unwrap_or_default();
        if name == "gcd_stein_windowed.h" {
            continue;
        }
        if name == "gcd_pari.h" {
            continue;
        }
        files.push(path);
    }

    files.sort();
    files
}

fn write_bindgen_wrapper(wrapper_path: &Path, headers: &[PathBuf]) {
    let mut f = fs::File::create(wrapper_path).expect("create bindgen wrapper");
    writeln!(f, "#pragma once").expect("write wrapper");
    for header in headers {
        let include = if header
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|x| x.to_str())
            == Some("gcd")
        {
            format!(
                "liboptarith/gcd/{}",
                header.file_name().unwrap().to_string_lossy()
            )
        } else {
            format!(
                "liboptarith/{}",
                header.file_name().unwrap().to_string_lossy()
            )
        };
        writeln!(f, "#include \"{include}\"").expect("write include");
    }
}

fn generate_bindings(
    wrapper_path: &Path,
    bindings_path: &Path,
    static_wrappers_c: &Path,
    includes: &[PathBuf],
    cc_system_includes: &[PathBuf],
) {
    let include_root_s = includes
        .iter()
        .find(|p| p.file_name().and_then(|x| x.to_str()) == Some("include"))
        .map(|p| p.display().to_string())
        .unwrap_or_default();

    let mut builder = bindgen::Builder::default()
        .header(wrapper_path.display().to_string())
        .layout_tests(false)
        .generate_comments(false)
        .allowlist_file(format!("{include_root_s}/liboptarith/.*"))
        .clang_arg("-x")
        .clang_arg("c")
        .wrap_static_fns(true)
        .wrap_static_fns_path(static_wrappers_c);
    for include in includes {
        builder = builder.clang_arg(format!("-I{}", include.display()));
    }
    for include in cc_system_includes {
        builder = builder.clang_arg(format!("-isystem{}", include.display()));
    }
    builder = builder.clang_arg("-DNO_PARI");

    let bindings = builder.generate().expect("generate bindings");
    fs::write(bindings_path, bindings.to_string()).expect("write bindings");
}

fn compile_object(cc: &str, source: &Path, object: &Path, includes: &[PathBuf]) {
    let mut cmd = Command::new(cc);
    cmd.arg("-c")
        .arg(source)
        .arg("-o")
        .arg(object)
        .arg("-O3")
        .arg("-std=gnu99")
        .arg("-DNDEBUG")
        .arg("-DNO_PARI");

    for include in includes {
        cmd.arg("-I").arg(include);
    }

    if let Ok(cflags) = env::var("CFLAGS") {
        for flag in cflags.split_whitespace() {
            cmd.arg(flag);
        }
    }

    let status = cmd.status().unwrap_or_else(|e| {
        panic!("failed to spawn compiler for {}: {e}", source.display());
    });
    assert!(
        status.success(),
        "compile failed for {} with status {status}",
        source.display()
    );
}

fn archive_objects(ar: &str, archive: &Path, objects: &[PathBuf]) {
    let mut cmd = Command::new(ar);
    cmd.arg("rcs").arg(archive);
    for object in objects {
        cmd.arg(object);
    }
    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("failed to spawn archiver: {e}"));
    assert!(status.success(), "archiving failed with status {status}");
}

fn tracked_paths(source_root: &Path) -> Vec<PathBuf> {
    let mut tracked = Vec::new();
    for entry in fs::read_dir(source_root).expect("read source root") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() == Some(OsStr::new("c")) || path.extension() == Some(OsStr::new("h")) {
            tracked.push(path);
        }
    }
    let gcd_dir = source_root.join("gcd");
    for entry in fs::read_dir(gcd_dir).expect("read gcd dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.extension() == Some(OsStr::new("c")) || path.extension() == Some(OsStr::new("h")) {
            tracked.push(path);
        }
    }
    let codegen_dir = source_root.join("code_gen");
    for entry in fs::read_dir(codegen_dir).expect("read code_gen dir") {
        let entry = entry.expect("entry");
        let path = entry.path();
        if path.is_file() {
            tracked.push(path);
        }
    }
    tracked.sort();
    tracked
}

fn discover_cc_system_includes(cc: &str) -> Vec<PathBuf> {
    let output = Command::new(cc).args(["-E", "-x", "c", "-", "-v"]).output();
    let Ok(output) = output else {
        return Vec::new();
    };
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut in_block = false;
    let mut includes = Vec::new();
    for line in stderr.lines() {
        let t = line.trim();
        if t == "#include <...> search starts here:" {
            in_block = true;
            continue;
        }
        if t == "End of search list." {
            break;
        }
        if in_block && !t.is_empty() {
            includes.push(PathBuf::from(t));
        }
    }
    includes
}

fn native_cache_root(out_dir: &Path) -> PathBuf {
    out_dir
        .ancestors()
        .nth(4)
        .expect("OUT_DIR does not match Cargo target layout")
        .join("optarith-sys-native-cache")
}

fn compute_native_cache_key(
    manifest_dir: &Path,
    tracked: &[PathBuf],
    max_prime: u32,
    target: &str,
    gmp_include_dir: &Path,
    gmp_lib_dir: &Path,
) -> String {
    let mut hasher = DefaultHasher::new();
    NATIVE_CACHE_VERSION.hash(&mut hasher);
    max_prime.hash(&mut hasher);
    target.hash(&mut hasher);
    gmp_include_dir.display().to_string().hash(&mut hasher);
    gmp_lib_dir.display().to_string().hash(&mut hasher);

    for key in ["CC", "AR", "CFLAGS", "CXX", "CXXFLAGS", "HOST"] {
        key.hash(&mut hasher);
        env::var(key).unwrap_or_default().hash(&mut hasher);
    }
    for key in bindgen_env_keys(target) {
        key.hash(&mut hasher);
        env::var(&key).unwrap_or_default().hash(&mut hasher);
    }

    let build_rs = manifest_dir.join("build.rs");
    hash_file_contents(&mut hasher, &build_rs);
    for path in tracked {
        hash_file_contents(&mut hasher, path);
    }

    format!("{:016x}", hasher.finish())
}

fn hash_file_contents(hasher: &mut DefaultHasher, path: &Path) {
    path.display().to_string().hash(hasher);
    let bytes = fs::read(path)
        .unwrap_or_else(|e| panic!("failed to read {} for cache key: {e}", path.display()));
    bytes.hash(hasher);
}

fn bindgen_env_keys(target: &str) -> Vec<String> {
    vec![
        format!("BINDGEN_EXTRA_CLANG_ARGS_{target}"),
        format!("BINDGEN_EXTRA_CLANG_ARGS_{}", target.replace('-', "_")),
        "BINDGEN_EXTRA_CLANG_ARGS".to_owned(),
    ]
}

fn emit_link_instructions(
    native_lib_dir: &Path,
    link_dirs: &BTreeSet<PathBuf>,
    link_libs: &BTreeSet<String>,
) {
    println!(
        "cargo:rustc-link-search=native={}",
        native_lib_dir.display()
    );
    println!("cargo:rustc-link-lib=static=optarith_c");
    for dir in link_dirs {
        println!("cargo:rustc-link-search=native={}", dir.display());
    }
    for lib in link_libs {
        println!("cargo:rustc-link-lib={lib}");
    }
}

fn emit_rerun_instructions(target: &str, tracked: &[PathBuf]) {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={MAX_PRIME_ENV}");
    for key in ["CC", "AR", "CFLAGS", "CXX", "CXXFLAGS", "TARGET", "HOST"] {
        println!("cargo:rerun-if-env-changed={key}");
    }
    for key in bindgen_env_keys(target) {
        println!("cargo:rerun-if-env-changed={key}");
    }
    println!("cargo:rerun-if-env-changed=DEP_GMP_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=DEP_GMP_LIB_DIR");
    for path in tracked {
        println!("cargo:rerun-if-changed={}", path.display());
    }
}
