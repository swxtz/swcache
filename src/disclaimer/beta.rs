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
