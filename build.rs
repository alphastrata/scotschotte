use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");

    match target_os.as_ref().map(|x| &**x) {
        Ok("windows") => {
            if let Some(gtk_lib_dir) = option_env!("GTK_LIB_DIR") {
                println!(r"cargo:rustc-link-search={}", gtk_lib_dir)
            }
        }
        _ => {}
    }
}
