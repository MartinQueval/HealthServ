mod config;
mod clear_ram;

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

    // Clear the Ram
    clear_ram::clear_ram(&config);
}