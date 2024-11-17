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
