extern crate bindgen;

use std::env;
use std::path::PathBuf;
use cmake::Config;

fn main() {
    let dst = Config::new("espeak-ng")
      .define("BUILD_SHARED_LIBS", "OFF")
      .define("COMPILE_INTONATIONS", "ON")
      .define("ENABLE_TESTS", "ON")
      .define("ESPEAK_BUILD_MANPAGES", "OFF")
      .define("EXTRA_cmn", "ON")
      .define("EXTRA_ru", "ON")
      .define("EXTRA_yue", "ON")
      .define("USE_ASYNC", "ON")
      .define("USE_MBROLA", "OFF")
      .define("USE_LIBSONIC", "ON")
      .define("USE_LIBPCAUDIO", "OFF")
      .define("USE_KLATT", "ON")
      .define("USE_SPEECHPLAYER", "ON")
      .build();
    println!("cargo:rustc-link-search=native={}", dst.join("lib").display());
    println!("cargo:rustc-link-lib=static=espeak-ng");
    println!("cargo:rustc-link-search=native={}", dst.join("build").join("src").join("speechPlayer").display());
    println!("cargo:rustc-link-lib=static=speechPlayer");
    println!("cargo:rustc-link-search=native={}", dst.join("build").join("src").join("ucd-tools").display());
    println!("cargo:rustc-link-lib=static=ucd");
    println!("cargo:rerun-if-changed=headers/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("headers/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}