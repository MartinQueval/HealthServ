mod config;

/**
 * @brief Test the configuration management system, basically prints the config file.
 */

// Relative path to the config file
const CONFIG_PATH: &str = "config.toml";

//Main
fn main() {
    // Load the config file
    let config = config::Config::load(CONFIG_PATH)
        .expect("Impossible de charger la configuration");

    // Print the configuration details
    config.show_config();
}