use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use crate::utils::paths::get_app_dir_path;

pub fn load_cfg(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_path = get_app_dir_path()?;
    file_path.push("cfgs/");
    file_path.push(filename);

    if !file_path.exists() {
        println!("Can't load \"{}\"", filename);
        Err("Error loading .cfg")?;
    }

    let mut entries = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
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
        println!("aaaaa {}", command_str);

        entries.push(command_str);
    }

    Ok(entries)
}
