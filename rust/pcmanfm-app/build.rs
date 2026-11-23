fn main() {
    for package in ["libfm-gtk3", "x11", "gtk+-3.0"] {
        pkg_config::Config::new()
            .probe(package)
            .unwrap_or_else(|err| panic!("{package}: {err}"));
    }
    println!("cargo:rustc-link-lib=m");
}
