mod cli;
use clap::Parser;
use cli::{Opts, SubCommand};
use full_stop::*;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Init => config::init(),
        SubCommand::Debug => {
            for module in config::read()?.module {
                dbg!(deploy::Deploy::from_config(module)?);
            }
            Ok(())
        }
        SubCommand::Deploy => {
            let modules = config::read()?.module;
            for module in modules {
                let deploy = deploy::Deploy::from_config(module)?;
                deploy.copy()?;
            }
            Ok(())
        }
    }
}
