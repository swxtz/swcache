use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

/// Global storage for the configuration instance.
static CONFIG_MANAGER: OnceLock<ConfigManager> = OnceLock::new();

/// Manages the application configuration using an internal key-value map.
pub struct ConfigManager {
    settings: HashMap<String, String>,
}

impl ConfigManager {
    /// Checks if a file exists at the given path.
    pub fn verify_config_file_exists<P: AsRef<Path>>(path: P) -> bool {
        fs::metadata(path).is_ok()
    }

    /// Loads the TOML file into a dynamic Key-Value map.
    pub fn load_config(path: &str) -> Result<Self, String> {
        if !Self::verify_config_file_exists(path) {
            return Err(format!("Configuration file '{}' was not found.", path));
        }

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read the file: {}", e))?;

        // Parses the TOML as a generic flat HashMap of Strings
        let settings: HashMap<String, String> =
            toml::from_str(&content).map_err(|e| format!("Failed to parse TOML format: {}", e))?;

        Ok(Self { settings })
    }

    /// Initializes the global configuration instance.
    pub fn init_global(path: &str) -> Result<(), String> {
        let manager = Self::load_config(path)?;
        CONFIG_MANAGER
            .set(manager)
            .map_err(|_| "Global configuration has already been initialized".to_string())
    }

    /// Retrieves a value from the global configuration by its key.
    /// Returns `None` if the key does not exist.
    pub fn get(key: &str) -> Option<&str> {
        CONFIG_MANAGER
            .get()
            .expect(
                "Global configuration is not initialized. Call `ConfigManager::init_global` first.",
            )
            .settings
            .get(key)
            .map(|v| v.as_str())
    }

    /// Retrieves a value and attempts to parse it into a specific type (e.g., bool, i32).
    pub fn get_as<T: std::str::FromStr>(key: &str) -> Option<T> {
        Self::get(key)?.parse::<T>().ok()
    }
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
        assert!(ConfigManager::verify_config_file_exists(test_file_path));
        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_load_config_and_get_values() {
        let test_file_path = "swconfig_kv_test.toml";
        let mut file = File::create(test_file_path).expect("Failed to create test config file");

        file.write_all(b"single_thread_mode = \"true\"\ndebug = \"false\"\nport = \"8080\"\n")
            .expect("Failed to write to test file");

        let manager = ConfigManager::load_config(test_file_path).unwrap();

        assert_eq!(
            manager
                .settings
                .get("single_thread_mode")
                .map(|v| v.as_str()),
            Some("true")
        );
        assert_eq!(manager.settings.get("non_existent"), None);

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_global_kv_lifecycle() {
        let test_file_path = "swconfig_global_kv_test.toml";
        let mut file = File::create(test_file_path).expect("Failed to create test config file");
        file.write_all(b"single_thread_mode = \"false\"\ndebug = \"true\"\n")
            .expect("Failed to write to test file");

        assert!(ConfigManager::init_global(test_file_path).is_ok());

        assert_eq!(ConfigManager::get("debug"), Some("true"));

        assert_eq!(ConfigManager::get_as::<bool>("debug"), Some(true));
        assert_eq!(
            ConfigManager::get_as::<bool>("single_thread_mode"),
            Some(false)
        );
        assert_eq!(ConfigManager::get_as::<bool>("non_existent"), None);

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }
}
