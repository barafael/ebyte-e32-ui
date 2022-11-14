use ebyte_e32_ui::{cli::Args, process};
use klask::Settings;

fn main() {
    let settings = Settings {
        enable_stdin: Some("Text to be sent, line by line".to_string()),
        ..Default::default()
    };
    klask::run_derived::<Args, _>(settings, |args| {
        process(args).expect("Failed to run app");
    });
}
