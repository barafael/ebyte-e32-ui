use clap::StructOpt;
use ebyte_e32_ui::{cli::App, config::Config, load_config, process};

fn main() -> anyhow::Result<()> {
    let config = load_config().map_err(|e| {
        eprintln!(
            "Here's an example:\n{}",
            toml::to_string(&Config::example()).unwrap()
        );
        e.context("Failed to load configuration")
    })?;
    let args = App::parse();
    process(config, args).expect("Failed to run app");
    Ok(())
}
