use disclaimer::{beta, logo};
use runtime::runtime::RuntimeState;
use utils::clean_terminal;

mod config;
mod disclaimer;
mod runtime;
mod utils;

fn main() {
    let mut runtime = RuntimeState::new();

    let state = RuntimeState::state(&runtime);

    startup();
    while state {}
}

fn startup() {
    clean_terminal::clean();

    logo::show_logo();
    beta::beta_warning();
}

fn verify_config() {}
