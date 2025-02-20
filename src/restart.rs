use crate::config::Config;
use chrono::{Local, NaiveTime,Duration};
use std::process::Command;

/**
* @brief Restart the computer compared to the restart hour in the config file
* @param config The configuration struct
*/
pub fn restart(config: &Config) {

    // Get the restart hour from the config
    let restart_time = NaiveTime::parse_from_str(&config.restart_hour, "%H:%M")
        .expect("Invalid restart time format in config");

    // Get the current time
    let now = Local::now().time();

    // Calculate the number of seconds until restart and check if the restart is tomorrow
    let diff = if restart_time < now {
        Duration::seconds(86400) - (now.signed_duration_since(restart_time))
    } else {
        restart_time.signed_duration_since(now)
    };

    //Parse the duration in u64
    let restart_duration: u64 = diff.num_seconds() as u64;

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