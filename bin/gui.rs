use ebyte_e32_ui::interface::App;
use ebyte_e32_ui::process;
use klask::Settings;

fn main() {
    let mut settings = Settings::default();
    settings.enable_stdin = Some("Description".to_string());
    klask::run_derived::<App, _>(settings, process);
}
