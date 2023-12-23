use crate::utils::{
    self,
    dependencies::{Dependence, DEFAULT_DEPENDENCE, DEPENDENCE},
};
use crate::Commands;
use console::{style, Emoji};
use dialogue_macro::dialoguer;
use dialogue_macro::{dialogue_define, Dialogue};
use handlebars::Handlebars;
use serde_json::json;
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    process::Command,
};
use tracing::{debug, trace};

dialogue_define!({
    project_name=>{
        prompt:"请输入项目名称",
        default:"wasm-project"
    },
    dependencies<Dependence>=>{
        ty:"multiselect",
        options:DEPENDENCE,
        prompt:"请选择需要安装的依赖",
        default:DEFAULT_DEPENDENCE
    },
});

impl Commands {
    pub fn new(vite: u8, name: &Option<String>) {
        // 获取用户输入的项目名
        let mut d = Dialogue::new();
        d.project_name();
        d.dependencies();
        let project_name = d.project_name.expect("project_name is None");
        trace!("{:?}", project_name);

        // 获取用户想要的安装依赖
        let selected_dependencies = d.dependencies.expect("dependencies is None");
        trace!("{:?}", selected_dependencies);

        let spinner = utils::Spinner::new(format!("创建项目中{}...", Emoji("🚚 ", "")));
        spinner.start();
        let project_dir = create_project(project_name.as_str());
        append_create_type(&project_dir);
        let mut dependencies_args = vec![];
        let mut features_args = vec![];
        for item in selected_dependencies.iter() {
            dependencies_args.push(item.name);
            features_args.push(item.features);
        }
        debug!("{:?}", dependencies_args);
        debug!("{:?}", features_args);
        Command::new("cargo")
            .arg("add")
            .args(dependencies_args)
            .arg("--features")
            .arg(format!(" {}", features_args.join(","),))
            .current_dir(&project_dir)
            .output()
            .expect("failed to execute process");

        let main_rs = render_template(&selected_dependencies);
        // 写入文件
        fs::write(project_dir.join("src/lib.rs"), main_rs).unwrap();
        spinner.stop(None);

        // 识别参数创建vite环境
        if vite == 1 {
            create_vite(&project_dir, name, &project_name);
        }

        println!("{} {}", Emoji("✨", ""), style("创建成功").green());
        println!("cd {}", project_name);
    }
}

// 创建项目
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

// 添加toml配置
fn append_create_type(path: &PathBuf) {
    let path = path.join("Cargo.toml");
    let append = r#"

[lib]
crate-type = ["cdylib", "rlib"]
"#;

    let mut file = OpenOptions::new().append(true).open(&path).unwrap();
    file.write(append.to_string().as_bytes()).unwrap();
}

// 根据依赖渲染模版
fn render_template(dev_list: &Vec<Dependence>) -> String {
    let handlebars = Handlebars::new();

    let mut dev_json = json!({});
    for item in dev_list {
        dev_json[item.name] = serde_json::Value::Bool(true);
    }
    handlebars
        .render_template(include_str!("../templates/basic.hbs"), &dev_json)
        .unwrap()
}

//  创建vite环境
fn create_vite(dir: &PathBuf, name: &Option<String>, project_name: &str) {
    let vite_name = name.as_deref().unwrap_or("vite-test");
    let spinner = utils::Spinner::new(format!("创建vite环境中{}...", Emoji("🚚 ", "")));
    spinner.start();
    Command::new("npm")
        .args([
            "create",
            "vite",
            vite_name,
            "--",
            "--template",
            "vanilla-ts",
        ])
        .current_dir(dir)
        .output()
        .expect("failed to execute process");
    let pkg_json = include_str!("../templates/vite.hbs");
    let value = json!({
        "name":vite_name,
        "project_name":project_name
    });
    let handlebars = Handlebars::new();
    let vite_pkg_json = handlebars.render_template(pkg_json, &value).unwrap();

    fs::write(dir.join(vite_name).join("package.json"), vite_pkg_json).unwrap();
    spinner.stop(None);

    let spinner = utils::Spinner::new(format!("安装npm依赖中{}...", Emoji("🚚 ", "")));
    spinner.start();
    Command::new("wasm-pack")
        .arg("build")
        .arg("-t")
        .arg("web")
        .current_dir(dir)
        .output()
        .expect("failed to run wasm-pack");
    Command::new("npm")
        .args(["install"])
        .current_dir(dir.join(vite_name))
        .output()
        .expect("failed to execute process");
    spinner.stop(None);
}
