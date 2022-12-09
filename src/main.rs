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
        .expect("SteamCMD commands failed to run");

    // casual args
    let args =
        "-game csgo -console -usercon +game_type 0 +game_mode 0 +mapgroup mg_active +map de_dust2";
    Command::new("./srcds_run")
        .current_dir(server_dir_path)
        .arg(args)
        .status()
        .expect("Server command failed to run");
}
