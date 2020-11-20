use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    if env::var("PROFILE").unwrap() == "release" {
        Command::new("npm").args(&["run", "build"])
            .current_dir(&[&env::var("CARGO_MANIFEST_DIR").unwrap(), "/frontend/soundloop"].concat())
            .status().unwrap();

        if Path::new(".git/HEAD").exists() {
                println!("cargo:rerun-if-changed=.git/HEAD");
        }
    }
}
