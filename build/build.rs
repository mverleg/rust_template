//use configure_me_codegen::build_script_with_man;
mod code_version;
use crate::code_version::extract_git_summary;

fn main() {
    // configure_me
//    build_script_with_man("build/config_spec.toml").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    let git_summary = extract_git_summary();
    println!("cargo:rerun-if-changed=.git/index");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rustc-env=VCS_SUMMARY={}", git_summary);
}
