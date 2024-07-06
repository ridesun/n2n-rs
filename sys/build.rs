use std::env;
use std::path::{Path, PathBuf};

fn main() {
    if !Path::new("./n2n/libn2n.a").exists() {
        let cur_path = PathBuf::from("./n2n/")
            .canonicalize()
            .expect("cannot canonicalize path");
        if cfg!(target_os = "linux") {
            if !Path::new("./n2n").exists() && !std::process::Command::new("git")
                .arg("clone")
                .arg("https://github.com/ntop/n2n.git")
                .output()
                .expect("")
                .status
                .success() {
                panic!("git error")
            }
            if !std::process::Command::new("bash")
                .current_dir(cur_path.clone())
                .arg("./autogen.sh")
                .output()
                .expect("")
                .status
                .success()
            {
                panic!("autogen error")
            }
            if !std::process::Command::new("bash")
                .current_dir(cur_path.clone())
                .arg("./configure")
                .output()
                .expect("")
                .status
                .success()
            {
                panic!("configure error")
            }
        } else if cfg!(target_os = "windows") {
            if !Path::new("./n2n").exists() && !std::process::Command::new("git")
                .arg("clone")
                .arg("https://github.com/ntop/n2n.git")
                .output()
                .expect("")
                .status
                .success() {
                panic!("git error")
            }
            let cur_path = PathBuf::from("./n2n/")
                .canonicalize()
                .expect("cannot canonicalize path");
            if !std::process::Command::new("git-bash")
                .current_dir(cur_path.clone())
                .arg("./scripts/hack_fakeautoconf.sh")
                .output()
                .expect("")
                .status
                .success()
            {
                panic!("autogen error")
            }

            panic!("see in https://github.com/ntop/n2n/blob/dev/doc/Building.md")
        }
        if !std::process::Command::new("make")
            .current_dir(cur_path.clone())
            .output()
            .expect("")
            .status
            .success()
        {
            panic!("make error")
        }
    }

        let libdir_path = PathBuf::from("./n2n/")
            .canonicalize()
            .expect("cannot canonicalize path");
        println!(
            "cargo:rustc-link-search={}",
            libdir_path.clone().to_str().unwrap()
        );

        println!("cargo:rustc-link-lib=n2n");

        let bindings = bindgen::Builder::default()
            .clang_args(["-I", libdir_path.clone().join("include").to_str().unwrap()])
            .ctypes_prefix("libc")
            .header(
                libdir_path
                    .clone()
                    .join("include")
                    .join("n2n.h")
                    .to_str()
                    .unwrap(),
            )
            .header(
                libdir_path
                    .clone()
                    .join("include")
                    .join("random_numbers.h")
                    .to_str()
                    .unwrap(),
            )
            .layout_tests(false)
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .generate()
            .expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
        bindings
            .write_to_file(out_path)
            .expect("Couldn't write bindings!");
}
