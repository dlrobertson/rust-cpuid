use std::env;
use std::process::Command;

#[cfg(all(target_arch="x86_64"))]
fn create_cpuid_lib(path: &str) {
    Command::new("as").arg("--64").arg("-g")
                      .arg("-o").arg(&format!("{}/{}.o", path, "cpuid"))
                      .arg("./src/cpuid.s").status().unwrap();
    Command::new("ar").arg("crus")
                      .arg("-o").arg(&format!("{}/lib{}.a", path, "cpuid"))
                      .arg(&format!("{}/{}.o", path, "cpuid"))
                      .status()
                      .unwrap();
}

#[cfg(not(target_arch="x86_64"))]
fn create_cpuid_lib(_: &str) {
    panic!("The target archetecture of this machine is not supported for this crate");
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    create_cpuid_lib(&*out_dir);
    println!("cargo:rustc-link-search=native={}", out_dir);
}
