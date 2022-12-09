use std::env;
use std::process::Command;

fn main() {
    // gets application folder path
    let mut executable_path = env::current_exe().expect("Failed to get the executable path");
    executable_path.pop();
    let app_dir_path = executable_path
        .to_str()
        .expect("Failed to get the application folder path");

    // builds path to the cs go server
    let server_dir_path = format!("{}/{}", app_dir_path, "server/");

    // install csgo server
    Command::new("steamcmd")
        .arg(format!("+force_install_dir {}", server_dir_path))
        .arg("+login anonymous")
        .arg("+app_update 740 validate")
        .arg("+quit")
        .status()
        .expect("ls command failed to start");
}
