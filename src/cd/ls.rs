use crate::args::Args;
use directories::ProjectDirs;
use std::{fs, os::unix::process::CommandExt, process};

pub fn cd_ls_main(args: Args) {
    // TODO: show the identifier for that cd, either pn or pid
    let data_path = ProjectDirs::from("com", "github", "ruti").unwrap();
    let data_path = data_path.data_local_dir();
    let mut no_file = true;
    for file in fs::read_dir(&data_path).unwrap() {
        let file = file.unwrap();
        if file.file_name() != "pn" {
            no_file = false;
            process::Command::new("ruti")
                .arg("cd")
                .arg("st")
                .arg("-s")
                .arg("-p")
                .arg(file.file_name())
                .spawn()
                .unwrap();
        }
    }
    if no_file {
        println!("No cd running.");
    }
}
