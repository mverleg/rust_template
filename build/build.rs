use configure_me_codegen::build_script_with_man;

fn main() {
    // configure_me
    build_script_with_man("build/config_spec.toml").unwrap();

}
