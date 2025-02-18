use crate::{
    args::{Args, CdCmd, Cmd},
    cd::iface::CdIface,
    data_path,
};
use std::{fs, os::unix::process::CommandExt, path, process};

pub fn cd_rm_main(args: Args) {
    let (p_name, mut pid) = match args.cmd {
        Cmd::Cd { cmd, .. } => match cmd {
            Some(CdCmd::Rm { p_name, pid }) => (p_name, pid),
            _ => panic!(""),
        },
        _ => panic!(""),
    };
    let data_path = &data_path();
    // TODO: when rm a cd with a default name, remove it from the dn file

    let mut path = path::PathBuf::new();
    if p_name.is_some() {
        let p_name = p_name.clone().unwrap();
        for file in fs::read_dir(&data_path).unwrap() {
            let file = file.unwrap();
            if file.file_name() != "dn" {
                if CdIface::from_path(&file.path()).unwrap().pn == p_name {
                    path = file.path();
                    pid = Some(
                        file.file_name()
                            .into_string()
                            .unwrap()
                            .parse::<u32>()
                            .unwrap(),
                    );
                }
            }
        }
    } else {
        path = data_path.join(pid.unwrap().to_string());
    };
    let pid = match pid {
        Some(p) => p,
        None => {
            println!("Please specify a valid countdown.");
            process::exit(1);
        }
    };
    if !fs::exists(&path).unwrap() {
        println!("Please specify a valid cd.");
        process::exit(1);
    }
    fs::remove_file(pid.to_string()).unwrap();

    process::Command::new("kill")
        .arg("-9")
        .arg(&pid.to_string())
        .exec();
}
