use disclaimer::{beta, logo};
use runtime::runtime::RuntimeState;
use utils::clean_terminal;
use crate::config::config_file::ConfigManager;

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

    ConfigManager::init_global("swconfig.toml").expect("Error initializing config manager");

    let is_single_threaded: bool = ConfigManager::get_as("single_thread_mode").unwrap_or(false);

   println!("Is single thread mode enabled? {}", is_single_threaded);

}

/// Verify config file with the default path our custom path, if config file not exists, the program
/// will running in default configuration
fn verify_config(path: String) -> bool {
    let config_file_exists = true;

    return if config_file_exists {
        println!("Verify config file exists");
        true
    } else {
        println!("Verify config file not exists");
        false
    };
}
