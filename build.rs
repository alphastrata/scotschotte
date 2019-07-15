fn main() {
    println!(r"cargo:rustc-link-search={}", env!("GTK_LIB_DIR"));
}