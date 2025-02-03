use directories::ProjectDirs;
use std::{fs, process};

pub fn cd_ls_main() {
    let data_path = ProjectDirs::from("com", "github", "ruti").unwrap();
    let data_path = data_path.data_local_dir();
    let mut no_file = true;
    for file in fs::read_dir(&data_path).unwrap() {
        let file = file.unwrap();
        if file.file_name() != "dn" {
            for (i, line) in fs::read_to_string(file.path()).unwrap().lines().enumerate() {
                match (i, line) {
                    (2, l) => print!("{}: ", l),
                    _ => {}
                }
            }
            no_file = false;
            // TODO: make it print in a grid-like format
            print!(
                "{}",
                String::from_utf8_lossy(
                    &process::Command::new("ruti")
                        .arg("cd")
                        .arg("st")
                        .arg("-s")
                        .arg("-n")
                        .arg("-p")
                        .arg(file.file_name())
                        .output()
                        .unwrap()
                        .stdout
                )
            );
        }
    }
    if no_file {
        println!("No cd running.");
    }
}
