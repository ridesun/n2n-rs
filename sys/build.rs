use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("./n2n/")
        .canonicalize()
        .expect("cannot canonicalize path");
    println!(
        "cargo:rustc-link-search={}",
        libdir_path.clone().to_str().unwrap()
    );

    println!("cargo:rustc-link-lib=n2n");

    // if !std::process::Command::new("git")
    //     .arg("clone")
    //     .arg("https://github.com/ntop/n2n.git")
    //     .output()
    //     .expect("")
    //     .status
    //     .success()
    // {
    //     panic!("autogen error")
    // }
    // if !std::process::Command::new("bash")
    //     .current_dir(libdir_path.clone())
    //     .arg("./autogen.sh")
    //     .output()
    //     .expect("")
    //     .status
    //     .success()
    // {
    //     panic!("autogen error")
    // }
    // if !std::process::Command::new("bash")
    //     .current_dir(libdir_path.clone())
    //     .arg("./configure")
    //     .output()
    //     .expect("")
    //     .status
    //     .success()
    // {
    //     panic!("configure error")
    // }
    // if !std::process::Command::new("make")
    //     .current_dir(libdir_path.clone())
    //     .status()
    //     .expect("")
    //     .success()
    // {
    //     panic!("make error")
    // }
    let bindings = bindgen::Builder::default()
        .clang_args(["-I", libdir_path.clone().join("include").to_str().unwrap()])
        .ctypes_prefix("libc")
        .header(libdir_path.join("include").join("n2n.h").to_str().unwrap())
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
