use error::CommonError;
use structopt::StructOpt;

mod agents;
mod commands;
mod error;
mod opt;
mod parser;
mod runner;
mod utils;

fn main() -> Result<(), CommonError> {
    let opt = opt::Opt::from_args();

    let mut parser = parser::Parser::parser_opt(&opt)?;

    let cmd = parser.gene_command(&opt)?;

    if cmd.len() > 0 {
        println!("Execute: {}", &cmd);
    }

    if opt.debug {
        println!("Debug mode, not execute command");
    } else {
        runner::Runner::run(&cmd)?;
    }

    Ok(())
}
