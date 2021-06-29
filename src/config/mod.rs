use clap::{Clap, ValueHint};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Clap)]
#[clap(version = "1.0", author = "Chris Dawes <chris.dawes@elsevier.com>")]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(version = "1.0", author = "Chris Dawes <chris.dawes@elsevier.com>")]
    Graph(GraphOpts),
}

#[derive(Clap)]
pub struct GraphOpts {
    #[clap(short='o', long="out", parse(from_os_str), value_hint = ValueHint::FilePath)]
    pub out: Option<PathBuf>,
    #[clap(long = "format", default_value = "dot")]
    pub output_format: OutputFormat,
}

#[derive(Clap)]
pub enum OutputFormat {
    Dot,
    None,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Dot
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dot" => Ok(OutputFormat::Dot),
            "none" => Ok(OutputFormat::None),
            other => Err(format!("unsupported output format {}", other)),
        }
    }
}
