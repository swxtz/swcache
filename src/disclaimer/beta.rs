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


use crate::disclaimer::system_info::SystemInfo;

pub fn beta_warning() {
    println!(
        "
        ======================================================
        |                                                    |
        |   Tool in beta process, do not use in production!  |
        |                                                    |
        ======================================================
        "
    );

    let system_info = SystemInfo::new();
    let memory_use = system_info.get_memory_info();

    // Print CPU information
    println!("{}", system_info.get_cpu_info());

    // Print memory information
    println!("{}", memory_use);
}
