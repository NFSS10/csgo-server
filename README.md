# CSGO Server

A simple application to setup and run a Counter-Strike: Global Offensive dedicated server.

### ℹ️: This project is no longer maintained. For new updates (support for CS2) follow this new project: https://github.com/NFSS10/steam-dedicated-server

## Setup

Make sure you have the correct PORTS open or you can't connect to the server. More information: https://developer.valvesoftware.com/wiki/Source_Dedicated_Server

#### OS Specific - Windows

Make sure you create a folder named "SteamCMD" and add the "steamcmd.exe" inside that folder. You can download the file here: https://developer.valvesoftware.com/wiki/SteamCMD#Downloading_SteamCMD

#### OS Specific - Linux

Follow the SteamCMD setup here: https://developer.valvesoftware.com/wiki/SteamCMD#Downloading_SteamCMD

## How to run

Just run the executable and it will show a menu with two options:
- "Run server" - Select this to run the server.
![run_example](https://user-images.githubusercontent.com/22588915/206875226-bd8878ec-363d-4ea6-b961-bde18a5e361b.gif)

- "Install server" -  Select this to install the server or to verify the server installation.
![install_example](https://user-images.githubusercontent.com/22588915/206875232-92384934-effb-41cd-aa93-86bff7d51bed.gif)

#### Server configuration

You can add custom commands and run custom `.cfg` files.
- To run custom commands, just add them to `config/commands.txt`.
- To run custom `.cfg`, add the files to `cfgs/` and then add their names to `config/exec.txt` (the order matters).

## Compatibility

Currently this was only tested in the following platforms:
- Windows 11
- Ubuntu-20.04 on WSL2
