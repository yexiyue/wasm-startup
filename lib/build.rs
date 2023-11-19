use std::process::Command;

use crate::Commands;
use tracing::trace;

impl Commands {
    pub fn build() {
        Command::new("wasm-pack")
            .arg("build")
            .arg("-t")
            .arg("web")
            .spawn()
            .expect("failed to run wasm-pack");
    }
}
