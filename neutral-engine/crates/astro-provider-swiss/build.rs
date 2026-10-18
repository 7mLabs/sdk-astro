fn main() {
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let data = std::fs::read("data/seas_18.se1").unwrap();
    let bytes = data
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(",");
    std::fs::write(
        out.join("astro_ephemeris.h"),
        format!("static const unsigned char astro_ephemeris[] = {{{bytes}}};"),
    )
    .unwrap();
    let mut build = cc::Build::new();
    // Swiss rates are used to refine roots close to zero. Keep their arithmetic
    // identical when Cargo builds the wrapper in debug or release mode.
    build.opt_level(3);
    build.define("ASTRO_MOSHIER_ONLY", None);
    // Strict C99 hides Linux libc's POSIX file-offset and GNU dl declarations.
    // Swiss uses off_t/fseeko/ftello and recommends this feature-test macro.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        build.define("_GNU_SOURCE", None);
    }
    build
        .include("vendor")
        .include(out)
        .file("embedded_ephemeris.c")
        .warnings(false)
        .flag_if_supported("-std=c99")
        .flag_if_supported("-fvisibility=hidden");
    for name in [
        "swedate", "swehouse", "swejpl", "swemmoon", "swemplan", "sweph", "swephlib", "swecl",
        "swehel",
    ] {
        build.file(format!("vendor/{name}.c"));
    }
    build.compile("astrology_swiss");
    println!("cargo:rerun-if-changed=vendor");
    println!("cargo:rerun-if-changed=data");
    println!("cargo:rerun-if-changed=embedded_ephemeris.c");
    if cfg!(target_family = "unix") {
        println!("cargo:rustc-link-lib=m");
    }
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dl");
    }
}
