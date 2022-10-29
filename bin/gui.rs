use ebyte_e32_ui::{interface::App, process};
use klask::Settings;

fn main() {
    let settings = Settings {
        enable_stdin: Some("Text to be sent, line by line".to_string()),
        ..Default::default()
    };
    klask::run_derived::<App, _>(settings, process);
}
