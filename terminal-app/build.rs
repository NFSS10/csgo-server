use std::env;
use std::path::Path;
use std::{fs, io};

fn main() {
    // gets build profile name
    let build_profile = env::var_os("PROFILE").unwrap();
    let build_profile_str = build_profile.to_str().unwrap();

    // builds build target path
    let target_dir = format!("./target/{}/", build_profile_str);

    // moves everything inside "resources" folder to the target folder
    copy_all("../resources/", target_dir).unwrap();
}

// copies everything inside "src" folder to the "dest" folder
fn copy_all(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dest)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_all(entry.path(), dest.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dest.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}
