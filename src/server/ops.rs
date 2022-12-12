use std::error::Error;
use std::path::Path;
use std::process::Command;

use menu_rs::{Menu, MenuOption};

use crate::utils::config::{load_commands, load_exec};
use crate::utils::paths::{get_app_dir_path, get_steamcmd_exe_path};

pub fn install_server(dir_path: &Path) -> Result<(), Box<dyn Error>> {
    let dir_path_str = dir_path.to_str().unwrap();
    let install_dir = format!("+force_install_dir {}", dir_path_str);
    let args = [
        install_dir.as_str(),
        "+login anonymous",
        "+app_update 740 validate",
        "+quit",
    ];

    if cfg!(windows) {
        let steacmd_exe_path = get_steamcmd_exe_path()?;
        Command::new(steacmd_exe_path).args(args).status()?;
    } else if cfg!(unix) {
        Command::new("steamcmd").args(args).status()?;
    }

    Ok(())
}

pub fn run_server_menu() {
    let menu = Menu::new(vec![
        MenuOption::new("Classic Competitive", start_classic_competitive_server),
        MenuOption::new("Classic Casual", start_classic_casual_server),
        MenuOption::new("Arms Race", start_arms_race_server),
        MenuOption::new("Demolition", start_demolition_server),
        MenuOption::new("Deathmatch", start_deathmatch_server),
    ]);
    menu.show();
}

fn start_classic_competitive_server() {
    let args = "-game csgo +game_type 0 +game_mode 1 +mapgroup mg_active +map de_dust2";
    start_server(args);
}

fn start_classic_casual_server() {
    let args = "-game csgo +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2";
    start_server(args);
}

fn start_arms_race_server() {
    let args = "-game csgo +game_type 1 +game_mode 0 +mapgroup mg_armsrace +map ar_shoots";
    start_server(args);
}

fn start_demolition_server() {
    let args = "-game csgo +game_type 1 +game_mode 1 +mapgroup mg_demolition +map de_lake";
    start_server(args);
}

fn start_deathmatch_server() {
    let args = "-game csgo +game_type 1 +game_mode 2 +mapgroup mg_allclassic +map de_dust";
    start_server(args);
}

fn start_server(game_mode_args: &str) {
    match _start_server(game_mode_args) {
        Ok(_) => {}
        Err(_) => println!("Failed to start the server. Make sure the server is installed before trying to run it."), 
    }
}

fn _start_server(game_mode_args: &str) -> Result<(), Box<dyn Error>> {
    // builds path to the CSGO server folder
    let mut server_dir_path = get_app_dir_path().expect("Failed to get the application path");
    server_dir_path.push("server/");

    let commands = load_commands()?;
    load_exec()?;

    let mut args = game_mode_args.to_owned();
    let commands_str = commands.join(" ");
    args.push(' ');
    args.push_str(&commands_str);
    args.push_str(" +exec __exec_custom.cfg");

    println!("Starting server...");
    println!("Args: {}", args);

    if cfg!(windows) {
        let windows_args = format!("/c srcds {}", args);
        Command::new("cmd")
            .current_dir(server_dir_path)
            .arg(windows_args)
            .status()?;
    } else if cfg!(unix) {
        Command::new("./srcds_run")
            .current_dir(server_dir_path)
            .arg(args)
            .status()?;
    } else {
        Err("OS not supported")?;
    }

    Ok(())
}
