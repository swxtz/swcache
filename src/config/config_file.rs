use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

/// Global storage for the configuration instance.
static CONFIG_MANAGER: OnceLock<ConfigManager> = OnceLock::new();

/// Manages the application configuration using an internal dynamic TOML map.
pub struct ConfigManager {
    settings: HashMap<String, toml::Value>,
}

impl ConfigManager {
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

    /// Loads the TOML file into a dynamic Key-Value map supporting various data types.
    ///
    /// # Arguments
    /// * `path` - A string slice representing the path to the configuration file.
    ///
    /// # Errors
    /// Returns an `Err` with a descriptive message if the file cannot be read or parsed.
    pub fn load_config(path: &str) -> Result<Self, String> {
        if !Self::verify_config_file_exists(path) {
            return Err(format!("Configuration file '{}' was not found.", path));
        }

        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read the file: {}", e))?;

        let settings: HashMap<String, toml::Value> =
            toml::from_str(&content).map_err(|e| format!("Failed to parse TOML format: {}", e))?;

        Ok(Self { settings })
    }

    /// Initializes the global configuration instance.
    ///
    /// # Errors
    /// Returns an error if the file cannot be loaded or if already initialized.
    pub fn init_global(path: &str) -> Result<(), String> {
        let manager = Self::load_config(path)?;
        CONFIG_MANAGER
            .set(manager)
            .map_err(|_| "Global configuration has already been initialized".to_string())
    }

    /// Retrieves a value and attempts to deserialize it into a specific type.
    /// Works out-of-the-box for common types like bool, String, i64, etc.
    ///
    /// # Arguments
    /// * `key` - The key string slice to look up in the settings.
    pub fn get_as<'a, T: serde::Deserialize<'a>>(key: &str) -> Option<T> {
        let manager = CONFIG_MANAGER.get().expect(
            "Global configuration is not initialized. Call `ConfigManager::init_global` first.",
        );

        let value = manager.settings.get(key)?;
        T::deserialize(value.clone()).ok()
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

        file.write_all(b"single_thread_mode = true\ndebug = false\nport = 8080\n")
            .expect("Failed to write to test file");

        let manager = ConfigManager::load_config(test_file_path).unwrap();

        assert_eq!(
            manager
                .settings
                .get("single_thread_mode")
                .and_then(|v| v.as_bool()),
            Some(true)
        );
        assert_eq!(
            manager.settings.get("port").and_then(|v| v.as_integer()),
            Some(8080)
        );
        assert_eq!(manager.settings.get("non_existent"), None);

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }

    #[test]
    fn test_global_kv_lifecycle() {
        let test_file_path = "swconfig_global_kv_test.toml";
        let mut file = File::create(test_file_path).expect("Failed to create test config file");
        file.write_all(b"single_thread_mode = false\ndebug = true\n")
            .expect("Failed to write to test file");

        assert!(ConfigManager::init_global(test_file_path).is_ok());

        assert_eq!(ConfigManager::get_as::<bool>("debug"), Some(true));
        assert_eq!(
            ConfigManager::get_as::<bool>("single_thread_mode"),
            Some(false)
        );
        assert_eq!(ConfigManager::get_as::<bool>("non_existent"), None);

        fs::remove_file(test_file_path).expect("Failed to remove test file");
    }
}
