#![allow(clippy::needless_return)]

mod server;
mod utils;

use std::error::Error;
use std::process::Command;

use menu_rs::{Menu, MenuOption};
use server::ops::{install_server, start_server};
use utils::paths::{get_app_dir_path, get_steamcmd_exe_path};

fn main() {
    match verify_os() {
        Ok(_) => {}
        Err(_) => println!("Setup needed, make sure SteamCMD is correctly setup"),
    }

    match run() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }
}

fn verify_os() -> Result<(), Box<dyn Error>> {
    if cfg!(windows) {
        let steacmd_exe_path = get_steamcmd_exe_path()?;
        if !steacmd_exe_path.exists() {
            Err("steamcmd.exe not found")?;
        }
    } else if cfg!(unix) {
        Command::new("steamcmd").arg("+quit").output()?;
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let menu = Menu::new(vec![
        MenuOption::new("Run server", _run_server).hint("Runs the server"),
        MenuOption::new("Install server", _install_server).hint("Server installation/verification"),
    ]);
    menu.show();

    Ok(())
}

fn _run_server() {
    // builds path to the CSGO server folder
    let mut server_dir_path = get_app_dir_path().expect("Failed to get the application path");
    server_dir_path.push("server/");

    match start_server(&server_dir_path) {
        Ok(_) => {}
        Err(_) => println!("Failed to start the server. Make sure the server is installed before trying to run it."), 
    }
}

fn _install_server() {
    // builds path to the CSGO server folder
    let mut server_dir_path = get_app_dir_path().expect("Failed to get the application path");
    server_dir_path.push("server/");

    match install_server(&server_dir_path) {
        Ok(_) => {}
        Err(_) => println!("The server installation failed."),
    }
}
