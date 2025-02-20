# HealthServ

You need to keep your computer on 24/7 for any reason, use `HealthServ`.<br>
`HealthServ` is a Rust-based system health management service designed to automate essential maintenance tasks on computer.
It provides functionalities such as scheduled system restarts, RAM cache clearing,and automatic service launching after reboot.

## Features

- **Automatic Service Launching** : Starts predefined services after a system reboot.
- **Scheduled Restart** : Reboots the system daily at a specified time.
- **RAM Cache Clearing** : Clears RAM cache at configurable intervals to optimize performance.
- **Cross-Platform Support** : Works on Windows, Linux, and macOS.

## Installation

Download the relase file for your OS.

- **Windows** : [HealthServ-1.0-windows](https://)
- **Linux** : [HealthServ-1.0-linux](https://)
- **MacOS** : [HealthServ-1.0-mac](https://)

Then extract the folder and place it wherever you like.

## Configuration according to your OS
- [Windows configuration](#windows)
- [Linux configuration](#linux)
- [MacOS configuration](#macos)

## Windows
### Automatic program start
For ̀ HealthServ` to launch automatically, you need to add it to the `Windows startup list`.
- First press **Win + R**
- Then copy and paste this :
```sh
shell:startup
```
- Then right-click and select "new" and "shortcut".
- Finally, click on browse, go to the `HealthServ` folder, then click on "HealthServ" and "ok", "next", "finish".<br>
Now you can close the file explorer.

### Config file edit
- Open the folder `HealthServ`, then you should find a `config.toml` file.
- Open it with a text editor.

#### 1. Change OS
The OS line should look like this :
```sh
os = "windows"
```
If that is not it, copy this line and replace your line with the one you copied.

To switch to Linux or MacOS, download the good [release](#installation) and repeat the steps for your OS.

#### 2. Change restart time
Under the OS section, there is the reboot time.
It should look something like this :
```sh
restart_hour = "12:00"
```
You can change the restart time by modifying this line, but you must keep the format "HH:MM".<br>
⚠️ **Be careful** : the format is based on a `24h00` format.

Example for a reboot at 4pm :
- restart_hour = 4pm ❌
- restart_hour = "4pm" ❌
- restart_hour = 16:00 ❌
- restart_hour = "16:00" ✅

`⚠️ Please note that this change will take effect on the next automatic restart.`

#### 3. Change Memory clear frequence
Under the reboot section, there is the memory clear rate.
It should look something like this :
```sh
cache_clear_interval = 6
```
By default, the memory is cleared every 6 hours,
but you can change this value by replacing the 6 with the interval you want.<br>
⚠️ **Be careful** : use an `integer` as shown above.

Example for a clear every 4 hours :
- cache_clear_interval = "4" ❌
- cache_clear_interval = four ❌
- cache_clear_interval = "four" ❌
- cache_clear_interval = 4 ✅

`⚠️ Please note that this change will take effect on the next automatic restart.`

#### 4. Change services run
The last section of the config file is the service startup after reboot.
It should look something like this :
```sh
services = [
	"C:\\Users\\marti\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
	"C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
]
```
By default, HealthServ launches vs code and chrome.<br>
But you may not have these two programs, or you may not want to launch them at startup.<br>
If so, delete the 2 link lines and the services section should look like this :
```sh
services = [
]
```

Now, maybe you want to launch a program at startup.
- Go to its executable, right-click and click on "copy as path".
- Then go to the `config.toml` file in the HealthServ folder and paste it into the sevices section.
- Finally, add a second "\\" to each slash in the path and add a "," to the end of the line.

Example for discord :
- "C:\Users\marti\AppData\Local\Discord\app-1.0.9182\Discord.exe", ❌
- "C:\\\Users\\\marti\\\AppData\\\Local\\\Discord\\\app-1.0.9182\\\Discord.exe" ❌
- C:\\\Users\\\marti\\\AppData\\\Local\\\Discord\\\app-1.0.9182\\\Discord.exe, ❌
- "C:\\\Users\\\marti\\\AppData\\\Local\\\Discord\\\app-1.0.9182\\\Discord.exe", ✅

Now the services section should look like this :
```sh
services = [
	"C:\\Users\\marti\\AppData\\Local\\Discord\\app-1.0.9182\\Discord.exe",
]
```

`⚠️ Please note that this change will take effect on the next automatic restart.`

#### End of configuration
Great, you've finished configuring `HealthServ` for `Windows`.<br>
Now you can leave your computer 24/7 without worrying about it.<br>
You can change reboot, memory cleanup and service launch parameters at any time.<br>
Don't hesitate to come back and consult the documentation if you've forgotten something.<br>
Thank you for using `HealthServ`, you're contributing to the project in your own way.

## Linux
### Automatic program start
For `HealthServ` to launch automatically, you need to add it to the `Linux Crontab`.
- press **Ctrl + Alt + T** to open a terminal
- Then copy and paste this :
```sh
crontab -e
```
- Press **Enter**, and **1** to open with Nano
- Use your keyboard's **Down Arrow** to scroll down at the end
- Copy the following line :
```sh
@reboot /path/to/your/programme
```
- Then right-click and select paste
- Go to the `HealthServ` folder, copy the path in the top search bar
- Replace `/path/to/your/programme` by paste and add `/HealthServ`
- Finally press **Ctrl + X**, press **Enter**, type on **Y**and press **Enter**.
Now you can close the terminal, HealthServ should be starting automatically on the next reboot.

### Activate memory clearing command
To clear the ram on Linux, the command requires sudo rights. So we will allow the command to be executed without a password.
- First open a terminal with **Ctrl + Alt + T**
- Then type sudo visudo and press **Enter**
- Now enter your password and press **Enter**
It will open the sudo rights file in an editor
- Use your keyboard's **Down Arrow** to scroll down at the section: "# Allow members of group sudo to execute any command"
- Copy the following line :
```sh
your_user_name ALL=(ALL) NOPASSWD: /bin/sh
```
- Then right-click and select paste
- Replace `your_username`by your own username
- Finally press **Ctrl + X**, press **Enter**, type on **Y**and press **Enter**.
Now the command is configured, so you can close the terminal

### Config file edit
- Open the folder `HealthServ`, then you should find a `config.toml` file.
- Open it with a text editor.

#### 1. Change OS
The OS line should look like this :
```sh
os = "linux"
```
If that is not it, copy this line and replace your line with the one you copied.

To switch to Linux or MacOS, download the good [release](#installation) and repeat the steps for your OS.

#### 2. Change restart time
Under the OS section, there is the reboot time.
It should look something like this :
```sh
restart_hour = "12:00"
```
You can change the restart time by modifying this line, but you must keep the format "HH:MM".<br>
⚠️ **Be careful** : the format is based on a `24h00` format.

Example for a reboot at 4pm :
- restart_hour = 4pm ❌
- restart_hour = "4pm" ❌
- restart_hour = 16:00 ❌
- restart_hour = "16:00" ✅

`⚠️ Please note that this change will take effect on the next automatic restart.`

#### 3. Change Memory clear frequence
Under the reboot section, there is the memory clear rate.
It should look something like this :
```sh
cache_clear_interval = 6
```
By default, the memory is cleared every 6 hours,
but you can change this value by replacing the 6 with the interval you want.<br>
⚠️ **Be careful** : use an `integer` as shown above.

Example for a clear every 4 hours :
- cache_clear_interval = "4" ❌
- cache_clear_interval = four ❌
- cache_clear_interval = "four" ❌
- cache_clear_interval = 4 ✅

`⚠️ Please note that this change will take effect on the next automatic restart.`

#### 4. Change services run
The last section of the config file is the service startup after reboot.
It should look something like this :
```sh
services = [
    "/snap/bin/code",
    "/usr/bin/firefox",
]
```
By default, HealthServ launches vs code and firefox.<br>
But you may not have these two programs, or you may not want to launch them at startup.<br>
If so, delete the 2 link lines and the services section should look like this :
```sh
services = [
]
```

Now, maybe you want to launch a program at startup.
- Go to its executable, if its download with snap, it should be in `snap/bin`.
- Then copy the path in the top search bar.
- Then, go to the `config.toml` file in the HealthServ folder, paste it into the sevices section.
- Finally, add the name of your program, add ajoute des adds quotation marks at the beginning and end and a comma.

Example for discord :
- "/snap/bin/", ❌
- "/snap/bin/discord" ❌
- /snap/bin/discord, ❌
- "/snap/bin/discord", ✅

Now the services section should look like this :
```sh
services = [
	"/snap/bin/discord",
]
```

`⚠️ Please note that this change will take effect on the next automatic restart.`

#### End of configuration
Great, you've finished configuring `HealthServ` for `Linux`.<br>
Now you can leave your computer 24/7 without worrying about it.<br>
You can change reboot, memory cleanup and service launch parameters at any time.<br>
Don't hesitate to come back and consult the documentation if you've forgotten something.<br>
Thank you for using `HealthServ`, you're contributing to the project in your own way.

## MacOS
# Author
Developed by Martin QUEVAL

