#![allow(clippy::needless_return)]

mod server;

use std::error::Error;
use std::io::stdin;

use csgo_server_lib::setup::{install_server, setup_server_structure, verify_os};
use menu_rs::{Menu, MenuOption};
use server::ops::run_server_menu;

fn main() {
    match verify_os() {
        Ok(_) => {}
        Err(_) => {
            println!("Setup needed, make sure SteamCMD is correctly setup");
            return;
        }
    }

    match run() {
        Ok(_) => {}
        Err(e) => println!("Error: {}", e),
    }

    // avoids immediately closing the console in Windows
    if cfg!(windows) {
        println!("\nPress any key to close");
        stdin().read_line(&mut String::new()).unwrap();
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let menu = Menu::new(vec![
        MenuOption::new("Run server", run_server_menu).hint("Runs the server"),
        MenuOption::new("Install server", _install_server).hint("Server installation/verification"),
    ]);
    menu.show();

    Ok(())
}

fn _install_server() {
    let server_dir_path = match setup_server_structure() {
        Ok(path) => path,
        Err(_) => {
            println!("Failed to setup the server structure");
            return;
        }
    };

    match install_server(&server_dir_path) {
        Ok(_) => {}
        Err(_) => println!("The server installation failed."),
    }
}
