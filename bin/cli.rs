use clap::StructOpt;
use ebyte_e32_ui::{interface::App, process};

fn main() {
    let args = App::parse();
    process(args);
}
