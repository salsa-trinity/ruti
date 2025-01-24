use crate::args::Args;
use directories::ProjectDirs;
use std::{fs, process};

pub fn cd_clean_main(args: Args) {
    let data_dir = ProjectDirs::from("com", "github", "ruti").unwrap();
    let data_dir = data_dir.data_local_dir();
    let mut count = 0;
    for file in data_dir.read_dir().unwrap() {
        let file = file.unwrap();
        count += 1;
        if file.file_name() != "pn" {
            let _ = process::Command::new("kill")
                .arg("-9")
                .arg(&file.file_name())
                .stdout(process::Stdio::null())
                .stderr(process::Stdio::null())
                .spawn();
        }
        fs::remove_file(file.path()).unwrap();
    }
    match count {
        0 => println!("Countdowns already cleaned."),
        1 => println!("Successfully cleaned 1 countdown."),
        _ => println!("Successfully cleaned {} countdowns.", count),
    }
}
