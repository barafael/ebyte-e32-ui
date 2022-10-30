use ebyte_e32_ui::{cli::App, config::Config, load_config, process};
use klask::Settings;

fn main() -> anyhow::Result<()> {
    let config = load_config().map_err(|e| {
        eprintln!(
            "Here's an example:\n{}",
            toml::to_string(&Config::example()).unwrap()
        );
        e.context("Failed to load configuration")
    })?;
    let settings = Settings {
        enable_stdin: Some("Text to be sent, line by line".to_string()),
        ..Default::default()
    };
    klask::run_derived::<App, _>(settings, |args| {
        process(config, args).expect("Failed to run app");
    });
    Ok(())
}
