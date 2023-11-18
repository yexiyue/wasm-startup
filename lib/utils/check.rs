use std::process::{Command, Stdio};
use tracing::{info, warn};
/// 检查是否安装wasm-pack 以及 target wasm是否存在
pub fn check() -> Result<bool, Box<dyn std::error::Error>> {
    // 查看是否安装wasm-pack
    if !Command::new("wasm-pack").output().is_ok() {
        warn!("未安装wasm-pack，将为您自动安装");
        // 安装wasm-pack (实时打印输出，记得wait)
        let child = Command::new("cargo")
            .arg("install")
            .arg("wasm-pack")
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .spawn()?
            .wait()?;
        println!("\n");
        if child.success() {
            info!("wasm-pack安装成功");
        } else {
            return Err("wasm-pack安装失败".into());
        }
    }
    // 查看是否安装wasm target
    let output = Command::new("rustup")
        .arg("target")
        .arg("list")
        .output()
        .expect("rustup failed");
    let out_str = String::from_utf8(output.stdout)?;
    if !out_str.contains("wasm32-unknown-unknown (installed)") {
        warn!("未安装wasm target，将为您自动安装");
        let child = Command::new("rustup")
            .arg("target")
            .arg("add")
            .arg("wasm32-unknown-unknown")
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .spawn()?
            .wait()?;
        println!("\n");
        if child.success() {
            info!("wasm target安装成功");
        } else {
            return Err("wasm target安装失败".into());
        }
    }

    Ok(true)
}
