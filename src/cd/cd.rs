use crate::{api::flags::ApiFlags, bgcd::bgcd_main};
use std::{path::Path, process};

pub fn cd_main(flags: &mut ApiFlags) {
    let data_path = Path::new("h");
    if flags.cd_command_defined > 0 {
        if flags.ls_defined == 1 {
            ls_command(&data_path);
        }
    } else {
        bgcd_main(flags);
    }
}

fn ls_command(data_path: &Path) {
    // TODO:
    println!("ls ");
    process::exit(1);
}
