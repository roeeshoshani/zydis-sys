use std::env;

fn build_zydis() {
    let mut config = cmake::Config::new("zydis");

    config
        .define("ZYDIS_BUILD_EXAMPLES", "OFF")
        .define("ZYDIS_BUILD_TOOLS", "OFF")
        .define("ZYDIS_FEATURE_DECODER", "ON");

    config.define("ZYDIS_MINIMAL_MODE", "OFF");
    config.define("ZYDIS_FEATURE_FORMATTER", "ON");
    config.define("ZYDIS_FEATURE_ENCODER", "ON");
    config.define("ZYAN_NO_LIBC", "OFF");

    let dst = config.build();

    let target = env::var("TARGET").unwrap_or("(unknown)".to_string());
    let is_msvc = target.ends_with("windows-msvc");

    let relative_build_dir = if is_msvc { config.get_profile() } else { "" };

    println!(
        "cargo:rustc-link-search=native={}/build/{}",
        dst.display(),
        relative_build_dir
    );
    println!(
        "cargo:rustc-link-search=native={}/build/zycore/{}",
        dst.display(),
        relative_build_dir
    );

    println!("cargo:rustc-link-lib=static=Zydis");
    println!("cargo:rustc-link-lib=static=Zycore");
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .clang_arg("-Izydis/include/")
        .clang_arg("-Izydis/dependencies/zycore/include/")
        .header("zydis.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    bindings
        .write_to_file(format!("{}/zydis.rs", out_dir))
        .unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=zydis");
    build_zydis();
    generate_bindings();
}
