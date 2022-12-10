use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn get_app_dir_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut executable_path = env::current_exe()?;
    executable_path.pop();

    // cleans Windows UNC path
    let app_dir_path_str = executable_path.to_str().unwrap();
    let app_dir_path = match app_dir_path_str.starts_with("\\\\?\\") {
        true => {
            let cropped_str = &app_dir_path_str[4..];
            let mut parsed_path = PathBuf::new();
            parsed_path.push(cropped_str);
            parsed_path
        }
        false => executable_path,
    };

    Ok(app_dir_path)
}

pub fn get_steamcmd_exe_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut steacmd_exe_path = get_app_dir_path()?;
    steacmd_exe_path.push("SteamCMD/steamcmd.exe");

    Ok(steacmd_exe_path)
}
