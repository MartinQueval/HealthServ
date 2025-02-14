use std::process::Command;
use crate::config::Config;
use sysinfo::{System};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ, PROCESS_SET_QUOTA};
use windows::Win32::System::ProcessStatus::EmptyWorkingSet;
use windows::Win32::Foundation::CloseHandle;

/**
* @brief Clear the RAM based on the OS
* @param config The configuration struct
*/
pub fn clear_ram(config: &Config) {

    // Get the OS and call the corresponding RAM clearing function
    match config.os.to_lowercase().as_str() {
        "windows" => clear_ram_windows(),
        "linux" => clear_ram_linux(),
        _ => println!("Unknown OS in config file: {}", config.os),
    }
}

/**
* @brief Function to clear RAM on windows
*/
fn clear_ram_windows() {
    // Initialize a new System instance to get all processes
    let system = System::new_all();

    // Iterate over all running processes
    for (pid, _) in system.processes() {
        unsafe {
            // Try to open the process with required permissions
            if let Ok(handle) = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_SET_QUOTA, false, pid.as_u32()) {
                // Attempt to clear the working set of the process (free unused memory)
                let _ = EmptyWorkingSet(handle);
                // Close the process handle after usage
                let _ = CloseHandle(handle);
            }
        }
    }
}

/**
* @brief Function to clear RAM on Linux
*/
fn clear_ram_linux() {
    // Run the 'sync' command to flush the file system buffers
    let _ = Command::new("sync").status();

    // Run the command to drop the caches (echo 3 to /proc/sys/vm/drop_caches)
    let _ = Command::new("sh").arg("-c").arg("echo 3 > /proc/sys/vm/drop_caches").status();
}
