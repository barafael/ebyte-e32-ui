use ebyte_e32_ui::{interface::App, process};
use klask::Settings;

fn main() {
    let mut settings = Settings::default();
    settings.enable_stdin = Some("Text to be sent, line by line".to_string());
    klask::run_derived::<App, _>(settings, process);
}
