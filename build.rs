extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    run_cmd(Command::new("make")
        .current_dir("libopencm3"));

    println!("cargo:rustc-link-search=libopencm3/lib");
    // println!("cargo:rustc-link-lib=static=libopencm3_stm32f1");
    // You must remove the `lib` prefix; lib is part of the naming convention.
    println!("cargo:rustc-link-lib=static=opencm3_stm32f1");

    let bindings = bindgen::Builder::default()
        .use_core() // nostd
        .generate_inline_functions(true)
        .clang_arg("-Ilibopencm3/include/") // add libopencm3 to the search path
        .clang_arg("-DSTM32F1")
        .ctypes_prefix("raw_c_types")
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings.");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // copied verbatim from https://rust-lang-nursery.github.io/rust-bindgen/tutorial-3.html
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn run_cmd(command: &mut Command) {
    match command.status() {
        Ok(ref exitcode) if exitcode.success() => {}
        Ok(ref exitcode) => {
            panic!("Command failed: `{:?}` — Exit Code {}.", command, exitcode)
        }
        Err(ref err) => {
            panic!("Unable to run command: `{:?}` — {}.", command, err)
        }
    }
}
