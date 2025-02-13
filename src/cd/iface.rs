use directories::ProjectDirs;
use std::{fs, path::Path, process};

// file format: filename is the pid
// 0 - total: (or progress), the amout of time that has currently passed
// 1 - target: the target time for the cd, cd ends when total reaches target
// 2 - pn: the human readable name, for not having to type the pid
// 3 - dn: true/false of wether its name has been assigned by the dn file
pub struct CdIface {
    // NOTE: update here 0
    pub total: f64,
    pub target: f64,
    pub pn: String,
    pub dn: bool,

    pub pid: u32,
}

impl CdIface {
    // TODO: add a get_path function
    pub fn save(&self) {
        // NOTE: update here 1
        let out = self.total.to_string()
            + "\n"
            + &self.target.to_string()
            + "\n"
            + &self.pn.to_string()
            + "\n"
            + &self.dn.to_string();

        let path = ProjectDirs::from("com", "github", "ruti").unwrap();
        let path = path.data_local_dir().join(self.pid.to_string());
        fs::write(path, out).unwrap();
    }
    pub fn from_path(path: &Path) -> Option<CdIface> {
        let (mut total, mut target, mut pn, mut dn) = (0f64, 0f64, "", false);

        if fs::read_to_string(&path).is_err() {
            return None;
        }

        let lines = fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = lines.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            match (i, line) {
                // NOTE: update here 2
                (0, l) => total = l.parse().unwrap(),
                (1, l) => target = l.parse().unwrap(),
                (2, l) => pn = l,
                (3, l) => dn = matches!(l, &"true"),

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
        match ProjectDirs::from("com", "github", "ruti") {
            Some(p) => return p,
            None => {
                println!("Failed to get project direcory.");
                process::exit(1);
            }
        };
    }
}
