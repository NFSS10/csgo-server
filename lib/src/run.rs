use std::error::Error;
use std::process::Command;

use crate::utils::config::{load_commands, load_exec};
use crate::utils::paths::get_app_dir_path;

pub fn start_server(game_mode_args: &str) {
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
    args.push_str(" +exec __exec_custom.cfg /*");

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
