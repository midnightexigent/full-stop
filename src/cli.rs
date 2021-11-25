use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    Init,
    Deploy,
    Debug,
}
