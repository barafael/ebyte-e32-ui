use ebyte_e32_ui::{arguments::Args, create, run};
use klask::Settings;

fn main() {
    let settings = Settings {
        enable_stdin: Some("Text to be sent, line by line".to_string()),
        ..Default::default()
    };
    klask::run_derived::<Args, _>(settings, |args| {
        let ebyte = create(&args).expect("Failed to run app");
        run(&args, ebyte).expect("Failed to run");
    });
}
