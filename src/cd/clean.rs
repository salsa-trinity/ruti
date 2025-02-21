use crate::data_path;
use std::{fs, process};

pub fn cd_clean_main() {
    // kill & rm files
    let mut count = 0;
    for file in data_path().read_dir().unwrap() {
        count += 1;
        let file = file.unwrap();
        if file.file_name() != "dn" {
            process::Command::new("kill")
                .arg("-9")
                .arg(&file.file_name())
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .spawn()
                .unwrap();
        }
        fs::remove_file(file.path()).unwrap();
    }

    // print end status
    match count {
        0 => println!("Files already cleaned."),
        1 => println!("Successfully cleaned 1 file."),
        _ => println!("Successfully cleaned {} files.", count),
    }
}
