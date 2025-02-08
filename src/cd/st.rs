use crate::{
    args::{Args, CdCmd, Cmd},
    cd::iface::CdIface,
};
use std::{fs, path, process};

pub fn cd_st_main(args: Args) {
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
    let data_path = CdIface::get_data_path();
    let data_path = data_path.data_local_dir();

    let uid;

    let mut path = path::PathBuf::new();
    if p_name.is_some() {
        let p_name = p_name.unwrap();
        for file in fs::read_dir(&data_path).unwrap() {
            let file = file.unwrap();
            if &file.file_name() != "dn" {
                for (i, line) in fs::read_to_string(file.path().clone())
                    .unwrap()
                    .lines()
                    .enumerate()
                {
                    match (i, line) {
                        (2, l) => {
                            if *l == p_name {
                                path = file.path();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        uid = p_name;
    } else {
        let pid = pid.unwrap();
        path = data_path.join(pid.to_string());
        uid = pid.to_string();
    };

    if !fs::exists(&path).unwrap() {
        println!("Please specify a valid cd.");
        process::exit(1);
    }
    let buff = fs::read_to_string(&path).unwrap();
    let (mut progress, mut target, mut left) = (0f64, 0f64, 0f64);
    for (i, line) in buff.lines().enumerate() {
        match (i, line) {
            (0, l) => progress = l.parse().unwrap(),
            (1, l) => {
                target = l.parse().unwrap();
                left = target - progress;
            }
            _ => {}
        }
    }

    if !single {
        if !nameless {
            println!("'{}' status:", uid);
        }
        println!(
            r"progress: {}s
target:   {}s
left:     {}s",
            progress.round(),
            target.round(),
            left.round()
        );
    } else {
        if !nameless {
            print!("{}: ", uid);
        }
        println!(
            "{}s/{}s  ({}s)",
            progress.round(),
            target.round(),
            left.round()
        );
    }
}
