use menu_rs::{Menu, MenuOption};

use csgo_server_lib::run::start_server;

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
