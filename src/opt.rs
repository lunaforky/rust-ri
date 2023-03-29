use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Ri", about = "A rust version ni.", rename_all = "kebab-case")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,

    #[structopt(short, long)]
    pub frozen: bool,

    /// Debug mode will not run the command
    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(short, long)]
    pub global: bool,
}

#[derive(StructOpt, Debug, Clone)]
pub enum SubCommand {
    /// Uninstall package
    Un { package_name: Vec<String> },

    /// Run script
    R { run_name: Option<String> },

    /// Special for `run dev`
    Rd,

    /// Remove node_modules
    Rm,

    /// Remove lockfile
    Rl,

    /// Git clone
    Cl { src: Option<String> },

    /// Git pull
    Pl,

    /// Git push
    Ps,

    /// Git log
    Log,

    /// Get package info
    Info,

    #[structopt(external_subcommand)]
    Other(Vec<String>),
}
