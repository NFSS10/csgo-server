use std::error::Error;
use std::path::Path;
use std::process::Command;

use crate::utils::paths::get_steamcmd_exe_path;

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

pub fn start_server(dir_path: &Path) -> Result<(), Box<dyn Error>> {
    let args =
        "-game csgo -console -usercon +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2";
    if cfg!(windows) {
        Command::new("cmd")
            .current_dir(dir_path)
            .arg("/c srcds -game csgo -console -usercon +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2")
            .status()?;
    } else if cfg!(unix) {
        Command::new("./srcds_run")
            .current_dir(dir_path)
            .arg(args)
            .status()?;
    } else {
        Err("OS not supported")?;
    }

    Ok(())
}
