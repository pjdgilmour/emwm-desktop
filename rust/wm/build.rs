fn main() {
    println!("cargo:rerun-if-env-changed=EMWM_RCDIR");
    let dir = std::env::var("EMWM_RCDIR").unwrap_or_else(|_| "/usr/share/emwm-desktop".into());
    println!("cargo:rustc-env=EMWM_RCDIR={dir}");
}
