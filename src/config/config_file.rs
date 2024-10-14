use std::fs;

pub fn verify_config_file_exists(path: &str) -> bool {
    let file = fs::metadata(path).is_ok();
    return file;
}
