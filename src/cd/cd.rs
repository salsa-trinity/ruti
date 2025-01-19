use crate::{args::Args, args::Cmd};
use std::{os::unix::process::CommandExt, process::Command};

pub fn cd_main(args: Args) {
    let mut bgcd_args = Vec::new();
    let mut _len = 0f64;
    match args.cmd {
        Cmd::Cd {
            len,
            ref name,
            update_time,
            ..
        } => {
            _len = len.unwrap();
            if name.is_some() {
                bgcd_args.push("-n".to_string());
                bgcd_args.push(name.clone().unwrap().to_string());
            }
            if update_time.is_some() {
                bgcd_args.push("-u".to_string());
                bgcd_args.push(update_time.unwrap().to_string());
            }
        }
        _ => {}
    }
    let len = _len;
    let bgcd_pid = Command::new("ruti")
        .arg("bg-cd")
        .arg(len.to_string())
        .args(bgcd_args)
        .exec();
    //.spawn()
    //.unwrap()
    //.id();
    //println!("pid: {}", bgcd_pid);

    // TODO: make this nice, missing:
    // - status command
}
