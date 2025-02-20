# HealthServ

You need to keep your computer on 24/7 for any reason, use `HealthServ`.
`HealthServ` is a Rust-based system health management service designed to
automate essential maintenance tasks on computer.
It provides functionalities such as scheduled system restarts, RAM cache clearing,
and automatic service launching after reboot.

## Features

- **Automatic Service Launching**: Starts predefined services after a system reboot.
- **Scheduled Restart**: Reboots the system daily at a specified time.
- **RAM Cache Clearing**: Clears RAM cache at configurable intervals to optimize performance.
- **Cross-Platform Support**: Works on Windows, Linux, and macOS.

## Installation

Download the relase file for your OS.

- **Windows**: [HealthServ-1.0-windows](https://)
- **Linux**: [HealthServ-1.0-linux](https://)
- **MacOS**: [HealthServ-1.0-mac](https://)

Then extract the folder and place it wherever you like.

# #Configuration according to your OS
[Windows configuration](#windows)
[Linux configuration](#linux)
[Mac configuration](#macos)

## Windows
### Automatic program start
For HealthServ to launch automatically, you need to add it to the Windows startup list.
First press **Win + R**
Then copy and paste this : ```sh shell:startup```.
Then right-click and select "new" and "shortcut".
Finally, click on browse, go to the `HealthServ` folder, then click on "HealthServ" and "ok", "next", "finish".
Now you can close the file explorer.

### Config file edit
Open the folder `HealthServ`, then you should find a config.toml file.
Open it with a text editor.

#### 1. Change OS
The OS line should look like this :
```sh
os = "windows"
```
If that is not it, copy this line and replace your line with the one you copied.

To switch to Linux or MacOS, download the good release and repeat the steps for your OS.

#### 2. Change restart time
Under the OS section, there is the reboot time.
It should look something like this :
```sh
restart_hour = "12:00"
```
You can change the restart time by modifying this line, but you must keep the format "HH:MM".
Please note that the format is based on a 24h00 format.
Example for a reboot at 4pm :
- restart_hour = 4pm ❌
- restart_hour = "4pm" ❌
- restart_hour = 16:00 ❌
- restart_hour = "16:00" ✅

#### 3. Change Memory clear frequence
Under the reboot section, there is the memory clear rate.
It should look something like this :
```sh
cache_clear_interval = 6
```
By default, the memory is cleared every 6 hours,
but you can change this value by replacing the 6 with the interval you want.
Be careful to use an integer as shown above.

Example for a clear every 4 hours :
- cache_clear_interval = "4" ❌
- cache_clear_interval = four ❌
- cache_clear_interval = "four" ❌
- cache_clear_interval = 4 ✅

#### 4. Change services run
The last section of the config file is the service startup after reboot.
It should look something like this :
```sh
services = [
	"C:\\Users\\marti\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
	"C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
]
```
By default, HealthServ launches vs code and chrome, but you may not have these two programs,
or you may not want to launch them at startup. If so, delete the 2 link lines.

Now, if you want to launch a program at startup, go to its executable, right-click and click on "copy as path".
Then go to the config.toml file in the HealthServ folder and paste it into the sevices section.
Finally, add a second "\" to each slash in the path and add a "," to the end of the line.

Example for discord :
- "C:\Users\marti\AppData\Local\Discord\app-1.0.9182\Discord.exe", ❌
- "C:\\Users\\marti\\AppData\\Local\\Discord\\app-1.0.9182\\Discord.exe" ❌
- C:\\Users\\marti\\AppData\\Local\\Discord\\app-1.0.9182\\Discord.exe, ❌
- "C:\\Users\\marti\\AppData\\Local\\Discord\\app-1.0.9182\\Discord.exe", ✅

## Linux

## MacOS
## Author

Developed by Martin QUEVAL.

