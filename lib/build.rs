use std::process::Command;

use crate::Commands;

impl Commands {
    pub fn build() {
        Command::new("wasm-pack")
            .arg("build")
            .arg("-t")
            .arg("web")
            .spawn()
            .expect("failed to run wasm-pack")
            .wait()
            .expect("failed to wait for wasm-pack");
    }
}
