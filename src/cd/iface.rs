use directories::ProjectDirs;
use std::{fs, path::Path};

pub struct CdIface {
    pub total: f64,
    pub target: f64,
    pub pn: String,
    pub dn: bool,
    pub pid: u32,
}

impl CdIface {
    pub fn from_path(path: &Path) -> Option<CdIface> {
        let (mut total, mut target, mut pn, mut dn) = (0f64, 0f64, "", false);

        if fs::read_to_string(&path).is_err() {
            return None;
        }

        let lines = fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = lines.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            match (i, line) {
                (0, l) => total = l.parse().unwrap(),
                (1, l) => target = l.parse().unwrap(),
                (2, l) => pn = l,
                (3, l) => {
                    dn = match l {
                        &"true" => true,
                        _ => false,
                    }
                }
                _ => {}
            }
        }

        Some(CdIface {
            total,
            target,
            pn: pn.to_string(),
            dn,
            pid: path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        })
    }

    pub fn from_pid(pid: u32) -> Option<CdIface> {
        let data_path = CdIface::get_data_path();
        let data_path = data_path.data_local_dir();

        CdIface::from_path(&data_path.join(pid.to_string()))
    }

    pub fn from_pn(pn: &str) -> Option<CdIface> {
        let data_path = CdIface::get_data_path();
        let data_path = data_path.data_local_dir();

        for file in fs::read_dir(&data_path).unwrap() {
            let file = file.unwrap();

            if CdIface::from_path(&file.path()).is_some() {
                if CdIface::from_path(&file.path()).unwrap().pn == pn {
                    return CdIface::from_path(&file.path());
                }
            }
        }

        None
    }

    pub fn get_data_path() -> ProjectDirs {
        ProjectDirs::from("com", "github", "ruti").unwrap()
    }
}
