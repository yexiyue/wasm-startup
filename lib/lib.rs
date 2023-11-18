use clap::{Parser, Subcommand};
use tracing::{error, trace};
mod build;
mod utils;
mod log;
mod new;

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
    New,
    /// 打包项目
    Build,
}

impl StartUp {
    fn start_up(&self) {
        trace!("{:?}", self);

        match &self.commands {
            Commands::New => Commands::new(),
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
