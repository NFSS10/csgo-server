#![allow(clippy::needless_return)]

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    verify_os();

    // builds path to the CSGO server folder
    let mut server_dir_path = get_app_dir_path();
    server_dir_path.push("server/");

    // ensures csgo server is installed
    install_csgo_server(&mut server_dir_path);

    start_csgo_server(&mut server_dir_path);
}

fn get_app_dir_path() -> PathBuf {
    let mut executable_path = env::current_exe().expect("Failed to get the executable path");
    executable_path.pop();

    // check windows UNC path
    let app_dir_path_str = executable_path
        .to_str()
        .expect("Failed to transform into string");
    let app_dir_path = match app_dir_path_str.starts_with("\\\\?\\") {
        true => {
            let cropped_str = &app_dir_path_str[4..];
            let mut parsed_path = PathBuf::new();
            parsed_path.push(cropped_str);
            parsed_path
        }
        false => executable_path,
    };

    return app_dir_path;
}

fn get_windows_steamcmd_exe_path() -> PathBuf {
    let mut steacmd_exe_path = get_app_dir_path();
    steacmd_exe_path.push("SteamCMD/steamcmd.exe");

    return steacmd_exe_path;
}

fn verify_os() {
    if cfg!(windows) {
        let steacmd_exe_path = get_windows_steamcmd_exe_path();

        if !steacmd_exe_path.exists() {
            panic!("Setup needed, SteamCMD/steamcmd.exe not found")
        }
    } else if cfg!(unix) {
        Command::new("steamcmd")
            .arg("+quit")
            .output()
            .expect("Setup needed, can't run SteamCMD");
    }
}

fn install_csgo_server(server_dir_path: &mut PathBuf) {
    let server_dir_path_str = server_dir_path
        .to_str()
        .expect("Can't convert server folder path to string");
    let install_dir = format!("+force_install_dir {}", server_dir_path_str);
    let args = [
        install_dir.as_str(),
        "+login anonymous",
        "+app_update 740 validate",
        "+quit",
    ];

    if cfg!(windows) {
        let steacmd_exe_path = get_windows_steamcmd_exe_path();
        Command::new(steacmd_exe_path)    
            .args(args)
            .status()
            .expect("SteamCMD commands failed to run");
    } else if cfg!(unix) {
        Command::new("steamcmd")
            .args(args)
            .status()
            .expect("SteamCMD commands failed to run");
    }
}

fn start_csgo_server(server_dir_path: &mut PathBuf) {
    // casual args
    let args =
        "-game csgo -console -usercon +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2";
    if cfg!(windows) {
        Command::new("cmd")
            .current_dir(server_dir_path)
            .arg("/c srcds -game csgo -console -usercon +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2")
            .status()
            .expect("Server command failed to run");
    } else if cfg!(unix) {
        Command::new("./srcds_run")
            .current_dir(server_dir_path)
            .arg(args)
            .status()
            .expect("Server command failed to run");
    }
}
