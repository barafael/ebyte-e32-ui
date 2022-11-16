use anyhow::Context;
use clap::StructOpt;
use ebyte_e32_ui::{arguments::Args, create, run};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let ebyte = create(&args).context("Failed to run app")?;
    run(&args, ebyte).context("Failed to run")
}
