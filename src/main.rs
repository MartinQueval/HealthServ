mod config;
mod clear_ram;
use std::thread;
use std::time::Duration;

/**
 * @brief Test the clearing of the ram
 */

// Relative path to the config file
const CONFIG_PATH: &str = "config.toml";

//Main
fn main() {
    // Load the config file
    let config = config::Config::load(CONFIG_PATH)
        .expect("Impossible de charger la configuration");

    // Get the clearing RAM frequency
    let interval_seconds = config.get_clearing_frequency();

    loop {
        // Wait for the defined clearing RAM frequency
        thread::sleep(Duration::from_secs(interval_seconds));

        // Clear RAM
        clear_ram::clear_ram(&config);
    }
}