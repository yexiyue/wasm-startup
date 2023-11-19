use clap::{Args, Parser, Subcommand};
use tracing::{error, trace};
mod build;
mod log;
mod new;
mod utils;

/// 快速启动wasm
#[derive(Debug, Parser)]
#[command(author,version,long_about = None)]
struct StartUp {
    /// Debug mode
    #[arg(short, long, action=clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Commands {
    /// 新建项目
    New(NewArgs),
    /// 打包项目
    Build,
}
#[derive(Debug, Args)]
struct NewArgs {
    #[arg(short, long,action=clap::ArgAction::Count)]
    /// 使用vite typescript 搭建实时测试环境
    vite: u8,
    /// vite项目名称
    #[arg(short, long)]
    name: Option<String>,
}

impl StartUp {
    fn start_up(&self) {
        trace!("{:?}", self);

        match &self.commands {
            Commands::New(NewArgs { vite, name }) => Commands::new(*vite, name),
            Commands::Build => Commands::build(),
        }
    }
}

pub fn init() {
    let cli = StartUp::parse();
    log::log(cli.debug == 1);
    match utils::check() {
        Ok(_) => cli.start_up(),
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
