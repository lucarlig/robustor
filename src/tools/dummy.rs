pub fn diagnose(main_rs: &str, cargo_toml: &str) -> String {
    format!("wrapped: ######## main.rs: {main_rs} cargo.toml: {cargo_toml} ########")
}
