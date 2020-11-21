use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    if env::var("PROFILE").unwrap() == "release" {
        if !Path::new(&[&env::var("CARGO_MANIFEST_DIR").unwrap(), "/frontend/soundloop/node_modules"].concat()).exists() {
            Command::new("npm").arg("install")
                .current_dir(&[&env::var("CARGO_MANIFEST_DIR").unwrap(), "/frontend/soundloop"].concat())
                .status().unwrap();
        }
        Command::new("npm").arg("run").arg("build")
            .current_dir(&[&env::var("CARGO_MANIFEST_DIR").unwrap(), "/frontend/soundloop"].concat())
            .status().unwrap();

        if Path::new(".git/HEAD").exists() {
                println!("cargo:rerun-if-changed=.git/HEAD");
        }
    }
}
