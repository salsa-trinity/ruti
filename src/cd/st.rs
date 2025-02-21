use crate::{
    args::{Args, CdCmd, Cmd},
    cd::iface::CdIface,
    data_path,
};
use std::{fs, path, process};

pub fn cd_st_main(args: Args) {
    // init values
    let (p_name, pid, single, nameless) = match args.cmd {
        Cmd::Cd { cmd, .. } => match cmd {
            Some(CdCmd::St {
                p_name,
                pid,
                single,
                nameless,
            }) => (p_name, pid, single, nameless),
            _ => panic!(""),
        },
        _ => panic!(""),
    };

    // get cd uid
    let uid;
    let mut path = path::PathBuf::new();
    if p_name.is_some() {
        let p_name = p_name.unwrap();
        for file in fs::read_dir(&data_path()).unwrap() {
            let file = file.unwrap();
            if &file.file_name() != "dn" {
                if CdIface::from_path(&file.path()).unwrap().pn == p_name {
                    path = file.path();
                }
            }
        }
        uid = p_name;
    } else {
        let pid = pid.unwrap();
        path = data_path().join(pid.to_string());
        uid = pid.to_string();
    };

    // validate cd
    if !fs::exists(&path).unwrap() {
        println!("Please specify a valid cd.");
        process::exit(1);
    }

    // print data
    let iface = CdIface::from_path(&path).unwrap();
    if !single {
        if !nameless {
            println!("'{}' status:", uid);
        }
        println!(
            r"progress: {}s
target:   {}s
left:     {}s",
            iface.total.round(),
            iface.target.round(),
            (iface.target - iface.total).round()
        );
    } else {
        if !nameless {
            print!("{}: ", uid);
        }
        println!(
            "{}s/{}s  ({}s)",
            iface.total.round(),
            iface.target.round(),
            (iface.target - iface.total).round()
        );
    }
}
