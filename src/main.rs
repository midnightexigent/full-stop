mod cli;
use clap::Parser;
use cli::{Opts, SubCommand};
use full_stop::*;

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Init => config::init(),
        SubCommand::Debug => {
            for (_, include) in config::read()?.includes {
                dbg!(deploy::Deploy::from_config(include)?);
            }
            Ok(())
        }
        SubCommand::Deploy => {
            let modules = config::read()?.includes;
            for (_, include) in modules {
                let deploy = deploy::Deploy::from_config(include)?;
                deploy.copy()?;
            }
            Ok(())
        }
    }
}
