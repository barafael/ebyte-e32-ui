use clap::StructOpt;
use ebyte_e32_ui::{interface::App, load_default_config, process};

fn main() {
    let config = load_default_config();
    let args = App::parse();
    process(config, args);
}
