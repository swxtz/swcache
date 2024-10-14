use owo_colors::OwoColorize;

const LOGO: &str = r#"


███████╗██╗    ██╗       ██████╗ █████╗  ██████╗██╗  ██╗███████╗
██╔════╝██║    ██║      ██╔════╝██╔══██╗██╔════╝██║  ██║██╔════╝
███████╗██║ █╗ ██║      ██║     ███████║██║     ███████║█████╗
╚════██║██║███╗██║      ██║     ██╔══██║██║     ██╔══██║██╔══╝
███████║╚███╔███╔╝      ╚██████╗██║  ██║╚██████╗██║  ██║███████╗
╚══════╝ ╚══╝╚══╝        ╚═════╝╚═╝  ╚═╝ ╚═════╝╚═╝  ╚═╝╚══════╝


"#;

pub fn show_logo() {
    println!("{}", LOGO.fg_rgb::<0x2E, 0x31, 0x92>().bold())
}
