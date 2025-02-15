use crate::config::Config;
use std::process::Command;

/**
* @brief Run the list of services that should be restarted after a restart based on the os
* @param config The configuration struct
*/
pub fn launch_services(config: &Config) {
    match config.os.to_lowercase().as_str() {
        "windows" => launch_windows_services(&config.services),
        "linux" => launch_linux_services(&config.services),
        _ => println!("Unknown OS in config file: {}", config.os),
    }
}

/**
* @brief Function to run a program on windows
* @param Vec<String> the list of services that should be restarted
*/
fn launch_windows_services(services : &Vec<String>){
    for service in services {
        //command cmd to run a program
        let _ = Command::new("cmd").args(&["/C", service]).spawn()
            .expect(&format!("Failed to start service: {}", service));
    }
}

/**
* @brief Function to run a program on Linux
* @param Vec<String> the list of services that should be restarted
*/
fn launch_linux_services(services : &Vec<String>){
    for service in services {
        //command system to run a program
        let _ = Command::new("systemctl").args(&["start", service]).spawn();
    }
}