use anyhow::Context;
use clap::StructOpt;
use ebyte_e32_ui::{cli::Args, process};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    process(args).context("Failed to run app")?;
    Ok(())
}
