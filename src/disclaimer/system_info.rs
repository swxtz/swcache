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


use bytesize::ByteSize;
use sysinfo::System;

pub struct SystemInfo {
    system: System,
}

impl SystemInfo {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        SystemInfo { system }
    }

    // Method to get CPU information
    pub fn get_cpu_info(&self) -> String {
        if let Some(cpu) = self.system.cpus().first() {
            let rounded_number = (cpu.frequency() as f64 / 1000.0).round();
            format!("Processor name: {} @{:.2} MHz", cpu.brand(), rounded_number)
        } else {
            "Processor information not found.".to_string()
        }
    }

    // Method to get memory information
    pub fn get_memory_info(&self) -> String {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();

        let formatted_total_memory = ByteSize::b(total_memory);
        let formatted_used_memory = ByteSize::b(used_memory);

        format!(
            "Total RAM: {} \nUsed RAM: {} ",
            formatted_total_memory, formatted_used_memory,
        )
    }
}
