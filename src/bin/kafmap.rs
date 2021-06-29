use clap::Clap;
use kafmap::cmd::graph;
use kafmap::config::*;
use kafmap::result::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Graph(opts) => {
            graph::graph(&opts).await?;
        }
    }

    Ok(())
}
