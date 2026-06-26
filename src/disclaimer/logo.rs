// Copyright 2026 Athenas System
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


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
