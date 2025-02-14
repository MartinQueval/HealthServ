use serde::Deserialize;
use std::fs;
use std::path::Path;

/**
 * @brief Define a struct to represent the configuration file.
 * It contains the operating system, the reboot time, the RAM clearing frequency,
 * and the list of services  that should be restarted after a reboot.
 */

//Constructor
#[derive(Debug, Deserialize)]
pub struct Config {
    pub os: String,
    pub restart_hour: String,
    pub cache_clear_interval: u32,
    pub services: Vec<String>,
}

// Implement Config
impl Config {
    /**
    * @brief Load the configuration from the configuration file
    * @param String Path to file
    * @returns config the struct of the config file
    */
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        //Load the configuration from the configuration file
        let content = fs::read_to_string(Path::new(path))?;

        // Parse the contents of the file into a Config object
        let config: Config = toml::from_str(&content)?;

        // Return the configuration
        Ok(config)
    }

    /**
    * @brief Convert the clearing duration frequency in seconds
    * @returns u64 the clearing duration frequency
    */
    pub fn get_clearing_frequency(&self) -> u64 {
        self.cache_clear_interval as u64 * 3600
    }

    /**
    * @brief Print the information from the configuration file
    */
    pub fn show_config(&self) {
        println!("OS: {}", self.os);
        println!("Reboot time: {}", self.restart_hour);
        println!("Cache clear interval: {}", self.cache_clear_interval);
        println!("Services: {:?}", self.services);
    }
}