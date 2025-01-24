use crate::args::Args;
use directories::ProjectDirs;
use std::fs;

pub fn cd_clean_main(args: Args) {
    let data_dir = ProjectDirs::from("com", "github", "ruti").unwrap();
    let data_dir = data_dir.data_local_dir();
    let mut count = 0;
    for file in data_dir.read_dir().unwrap() {
        fs::remove_file(file.unwrap().path()).unwrap();
        count += 1;
    }
    // TODO: kill running countdowns
    println!("Successfully deleted {} files.", count);
}
