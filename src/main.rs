use error::CommonError;
use structopt::StructOpt;

mod agents;
mod commands;
mod error;
mod opt;
mod parser;
mod runner;
mod utils;

fn run() -> Result<(), CommonError> {
    let opt = opt::Opt::from_args();

    let mut parser = parser::Parser::parser_opt(&opt)?;

    let cmd = parser.gene_command()?;

    println!("Execute: {}", &cmd);

    if !opt.debug {
        runner::Runner::run(&cmd)?;
    }

    Ok(())
}

fn main() -> () {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}
