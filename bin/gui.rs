use ebyte_e32_ui::{interface::App, load_default_config, process};
use klask::Settings;

fn main() {
    let config = load_default_config();
    let settings = Settings {
        enable_stdin: Some("Text to be sent, line by line".to_string()),
        ..Default::default()
    };
    klask::run_derived::<App, _>(settings, |args| process(config, args));
}
