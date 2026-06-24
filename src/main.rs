use config::config_file::verify_config_file_exists;
use disclaimer::{beta, logo};
use runtime::runtime::RuntimeState;
use utils::clean_terminal;
mod config;
mod disclaimer;
mod runtime;
mod utils;

fn main() {
    let runtime = RuntimeState::new();

    let state = RuntimeState::state(&runtime);

    startup();
    while state {}
}

fn startup() {
    clean_terminal::clean();

    logo::show_logo();
    beta::beta_warning();
}

/// Verify config file with the default path our custom path, if config file not exists, the program
/// will running in default configuration
fn verify_config(path: String) -> bool {
    let config_file_exists = verify_config_file_exists(path);

    return if config_file_exists {
        println!("Verify config file exists");
        true
    } else {
        println!("Verify config file not exists");
        false
    };
}
