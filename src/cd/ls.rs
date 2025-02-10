use std::{fs, process};

use super::iface::CdIface;

pub fn cd_ls_main() {
    let data_path = CdIface::get_data_path();
    let data_path = data_path.data_local_dir();
    let mut no_file = true;
    for file in fs::read_dir(&data_path).unwrap() {
        let file = file.unwrap();
        if file.file_name() != "dn" {
            print!("{}: ", CdIface::from_path(&file.path()).unwrap().pn);
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
