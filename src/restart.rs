use crate::config::Config;
use chrono::{Local, NaiveTime};
use std::process::Command;

/**
* @brief Restart the computer compared to the restart hour in the config file
* @param config The configuration struct
*/
pub fn restart(config: &Config) {

    // Get the restart hour from the config
    let restart_time = NaiveTime::parse_from_str(&config.restart_hour, "%H:%M")
        .expect("Invalid restart time format in config");

    // Calculate the number of seconds until restart
    let restart_duration = (restart_time - Local::now().time()).num_seconds().unsigned_abs();

    // Get the OS and call the corresponding restart function
    match config.os.to_lowercase().as_str() {
        "windows" => restart_windows(restart_duration),
        "linux" => restart_linux(restart_duration),
        "mac" => restart_mac(restart_duration),
        _ => println!("Unknown OS in config file: {}", config.os),
    }
}

/**
* @brief Function to restart the computer on Windows os
* @param u64 time to restart
*/
fn restart_windows(restart_duration: u64){
    //Windows restart command
    let _ = Command::new("shutdown").args(&["/r", "/t", &restart_duration.to_string()]).spawn();
}

/**
* @brief Function to restart the computer on Linux os
* @param u64 time to restart
*/
fn restart_linux(restart_duration: u64){
    //Linux restart Command
    let _ = Command::new("shutdown").args(&["-r", format!("+{}", restart_duration / 60).as_str()]).spawn();
}

/**
* @brief Function to restart the computer on macOS
* @param u64 time to restart
*/
fn restart_mac(restart_duration: u64){
    //Mac restart Command
    let _ = Command::new("sudo").args(&["shutdown","-r", format!("+{}", restart_duration / 60).as_str()]).spawn();
}