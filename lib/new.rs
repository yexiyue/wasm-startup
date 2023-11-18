use crate::utils::{self, read_json_config};
use crate::Commands;
use console::{style, Emoji};
use std::{fs::OpenOptions, io::Write, path::PathBuf, process::Command};
use tracing::{error, info, trace};
impl Commands {
    pub fn new() {
        // 读取配置文件
        let config = read_json_config();
        // 获取用户输入的项目名
        let project_name = utils::dialogue::input("请输入项目名称");
        trace!("{:?}", project_name);
        // 获取用户想要的安装依赖
        let mut selected_dependencies = vec![];
        for item in &config.dependencies {
            let res = item.multi_select();
            selected_dependencies.extend_from_slice(res.as_slice());
        }
        trace!("{:?}", selected_dependencies);
        let spinner = utils::Spinner::new(format!("创建项目中{}...", Emoji("🚚 ", "")));
        spinner.start();
        let project_dir = create_project(project_name.as_str());
        append_create_type(&project_dir);
        for item in selected_dependencies {
            let mut build = Command::new("cargo");
            build
                .current_dir(&project_dir)
                .arg("add")
                .arg(item.name.as_str());
            match &item.features {
                Some(features) => {
                    build.arg("--features").arg(features.join(","));
                }
                None => {}
            }
            build.output().expect("failed to execute process");
        }
        spinner.stop(None);

        println!("{} {}", Emoji("✨", ""), style("创建成功").green());
        println!("cd {}", project_name);
    }
}

fn create_project(project_name: &str) -> PathBuf {
    // 获取当前目录
    let mut dir = std::env::current_dir().unwrap();
    dir.push(project_name);
    // 创建项目目录
    std::process::Command::new("cargo")
        .arg("new")
        .arg("--lib")
        .arg(&dir)
        .output()
        .expect("failed to execute process");
    dir.clone()
}

fn append_create_type(path: &PathBuf) {
    let path = path.join("Cargo.toml");
    let append = r#"

[lib]
crate-type = ["cdylib", "rlib"]
"#;

    let mut file = OpenOptions::new().append(true).open(&path).unwrap();
    file.write(append.to_string().as_bytes()).unwrap();
}
