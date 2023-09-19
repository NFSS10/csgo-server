use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::utils::paths::{get_app_dir_path, get_steamcmd_exe_path};

// verifies if steamcmd is correctly installed
pub fn verify_os() -> Result<(), Box<dyn Error>> {
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

// builds path to the CSGO server folder
pub fn setup_server_structure() -> Result<PathBuf, Box<dyn Error>> {
    let mut server_dir_path: PathBuf =
        get_app_dir_path().expect("Failed to get the application path");
    server_dir_path.push("server/");

    Ok(server_dir_path)
}

// installs the CSGO server
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
