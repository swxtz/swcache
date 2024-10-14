use disclaimer::{beta, logo};
use utils::clean_terminal;

mod disclaimer;
mod utils;

fn main() {
    clean_terminal::clean();

    logo::show_logo();
    beta::beta_warning();
}
