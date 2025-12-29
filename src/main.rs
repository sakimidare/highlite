use clap::Parser;
use hilite::arg_parser::CliArgs;
use hilite::run;

fn main() -> anyhow::Result<()>{
    let cli = CliArgs::parse();
    run(cli)
}
