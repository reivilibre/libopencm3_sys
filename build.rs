extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let opencm3_copy = out_path.join("libopencm3");

    // copy libopencm3 to OUT_DIR because Cargo says we're not allowed to modify anything except
    // OUT_DIR. A bit of a waste of space but ah well.
    run_cmd(Command::new("cp")
        .arg("-u").arg("--reflink=auto").arg("-r").arg("-T")
        .arg("libopencm3").arg(&opencm3_copy));

    run_cmd(Command::new("make")
        .current_dir(&opencm3_copy));

    // println!("cargo:rustc-link-search=libopencm3/lib");
    // Use the copied version of course:
    println!("cargo:rustc-link-search={}", opencm3_copy.join("lib").to_str().expect("Erroneous string ($OUT_DIR/lib)."));

    // println!("cargo:rustc-link-lib=static=libopencm3_stm32f1");
    // must remove the `lib` prefix; lib is part of the naming convention.
    println!("cargo:rustc-link-lib=static=opencm3_stm32f1");

    let bindings = bindgen::Builder::default()
        .use_core() // nostd
        .generate_inline_functions(true)
        //.clang_arg("-Ilibopencm3/include/") // add libopencm3 to the search path
        // libopencm3 generates headers; must use copy
        .clang_arg("-I".to_owned() + opencm3_copy.join("include").to_str().expect("Erroneous string ($OUT_DIR/include)."))
        .clang_arg("-DSTM32F1")
        .ctypes_prefix("raw_c_types")
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings.");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // copied verbatim from https://rust-lang-nursery.github.io/rust-bindgen/tutorial-3.html
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
