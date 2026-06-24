use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Represents the application configuration structure mapped from a TOML file.
#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub single_thread_mode: Option<bool>,
    pub debug: Option<bool>,
}

/// Checks if a file exists at the given path.
///
/// # Arguments
/// * `path` - A reference to a path that can be converted into a `Path`.
///
/// # Returns
/// `true` if the file exists, `false` otherwise.
pub fn verify_config_file_exists<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).is_ok()
}

/// Loads and parses the configuration file into an `AppConfig` instance.
///
/// # Arguments
/// * `path` - A string slice representing the path to the configuration file.
///
/// # Errors
/// Returns an `Err` with a descriptive message if:
/// * The configuration file is not found.
/// * The file cannot be read from disk.
/// * The file content fails to parse into the `AppConfig` structure.
pub fn load_config(path: &str) -> Result<AppConfig, String> {
    if !verify_config_file_exists(path) {
        return Err(format!("Configuration file '{}' was not found.", path));
    }

    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read the file: {}", e))?;

    let config: AppConfig = toml::from_str(&content)
        .map_err(|e| format!("Failed to parse TOML format: {}", e))?;

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_verify_config_file_exists_success() {
        let test_file_path = "test_config_existing.toml";

        File::create(test_file_path).expect("Failed to create test file");

        let result = verify_config_file_exists(test_file_path);
        assert!(result);

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_verify_config_file_exists_fail() {
        let non_existent_path = "non_existent_file_12345.toml";

        let result = verify_config_file_exists(non_existent_path);
        assert!(!result);
    }

    #[test]
    fn test_load_config_success() {
        let test_file_path = "swconfig_test_success.toml";

        let mut file = File::create(test_file_path).expect("Failed to create test config file");
        file.write_all(b"single_thread_mode = true\ndebug = false\n").expect("Failed to write to test file");

        let result = load_config(test_file_path);

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.single_thread_mode, Some(true));
        assert_eq!(config.debug, Some(false));

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_load_config_optional_fields_missing() {
        let test_file_path = "swconfig_test_missing_fields.toml";

        let mut file = File::create(test_file_path).expect("Failed to create test config file");
        file.write_all(b"").expect("Failed to write to test file");

        let result = load_config(test_file_path);

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.single_thread_mode, None);
        assert_eq!(config.debug, None);

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_load_config_file_not_found() {
        let non_existent_path = "swconfig_missing.toml";

        let result = load_config(non_existent_path);

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("was not found"));
    }

    #[test]
    fn test_load_config_invalid_toml() {
        let test_file_path = "swconfig_test_invalid.toml";

        let mut file = File::create(test_file_path).expect("Failed to create test config file");
        file.write_all(b"single_thread_mode = \"not_a_bool\"\n").expect("Failed to write to test file");

        let result = load_config(test_file_path);

        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert!(err_msg.contains("Failed to parse"));

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }
}