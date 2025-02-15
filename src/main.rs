mod config;
mod clear_ram;
mod restart;
mod launch_services;

use std::thread;
use std::time::Duration;

/**
 * @brief Test to run the services
 */

// Relative path to the config file
const CONFIG_PATH: &str = "config.toml";

//Main
fn main() {
    // Load the config file
    let config = config::Config::load(CONFIG_PATH)
        .expect("Failed to load the configuration");

    //Launch services
    launch_services::launch_services(&config);

    // Launch the restart function
    //restart::restart(&config);

    // Get the clearing RAM frequency
    let interval_seconds = config.get_clearing_frequency();

    loop {
        // Wait for the defined clearing RAM frequency
        thread::sleep(Duration::from_secs(interval_seconds));

        // Clear RAM
        clear_ram::clear_ram(&config);
    }
}