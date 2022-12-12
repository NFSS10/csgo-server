#![allow(dead_code)]

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};

use crate::utils::paths::get_app_dir_path;

pub fn load_commands() -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_path = get_app_dir_path()?;
    file_path.push("config/commands.txt");

    let reader = match load_file(&file_path) {
        Ok(r) => r,
        Err(_) => {
            println!("Can't load \"config/commands.txt\"");
            Err("Error loading commands")?
        }
    };

    let mut entries = Vec::new();
    for line in reader.lines() {
        let mut line = line?;
        line = line.trim().to_owned();

        let ignore_line = line.starts_with('/') || line.is_empty();
        if ignore_line {
            continue;
        }

        // removes extra white spaces between words
        let words: Vec<_> = line.split_whitespace().collect();
        line = words.join(" ");

        // removes comments
        let words: Vec<_> = line.split(" //").collect();
        line = words[0].to_owned();

        // builds commands
        let command_str = format!("-{line}");

        entries.push(command_str);
    }

    Ok(entries)
}

pub fn load_exec() -> Result<PathBuf, Box<dyn Error>> {
    let mut server_cfgs_path = get_app_dir_path()?;
    server_cfgs_path.push("server/csgo/cfg/");
    if !server_cfgs_path.exists() {
        println!("Can't find server/csgo/cfg/ folder, make sure the server is installed");
        Err("Error loading exec")?
    }

    let mut file_path = get_app_dir_path()?;
    file_path.push("config/exec.txt");

    let exec_reader = match load_file(&file_path) {
        Ok(r) => r,
        Err(_) => {
            println!("Can't load \"config/exec.txt\"");
            Err("Error loading config/exec.txt")?
        }
    };

    // loads cfgs values
    let mut exec_values = Vec::new();
    for line in exec_reader.lines() {
        let line = line?;
        let mut cfg_values = load_cfg(&line)?;
        exec_values.append(&mut cfg_values);
    }

    let mut exec_cfg_path = PathBuf::from(&server_cfgs_path);
    exec_cfg_path.push("__exec_custom.cfg");

    // deletes custom cfg if already exists
    if exec_cfg_path.exists() {
        fs::remove_file(&exec_cfg_path)?;
    }

    // creates custom cfg file
    fs::write(&exec_cfg_path, exec_values.join("\n"))?;

    Ok(exec_cfg_path)
}

fn load_cfg(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_path = get_app_dir_path()?;
    file_path.push("cfgs/");
    file_path.push(filename);

    let reader = match load_file(&file_path) {
        Ok(r) => r,
        Err(_) => {
            println!("Can't load \"{}\"", filename);
            Err("Error loading .cfg")?
        }
    };

    let mut entries = Vec::new();
    for line in reader.lines() {
        let mut line = line?;
        line = line.trim().to_owned();

        let ignore_line = line.starts_with('/') || line.is_empty();
        if ignore_line {
            continue;
        }

        // removes extra white spaces between words
        let words: Vec<_> = line.split_whitespace().collect();
        line = words.join(" ");

        // removes comments
        let words: Vec<_> = line.split(" //").collect();
        line = words[0].to_owned();

        // builds commands
        let command_str = format!("+{line}");

        entries.push(command_str);
    }

    Ok(entries)
}

fn load_file(file_path: &Path) -> Result<BufReader<File>, Box<(dyn std::error::Error + 'static)>> {
    if !file_path.exists() {
        Err("Error loading file")?;
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    Ok(reader)
}
