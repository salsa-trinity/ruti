use std::{fs, path::Path};

pub struct CdIface {
    pub total: f64,
    pub target: f64,
    pub pn: String,
    pub dn: bool,
}

impl CdIface {
    pub fn new(path: &Path) -> CdIface {
        let (mut total, mut target, mut pn, mut dn) = (0f64, 0f64, "", false);

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
        CdIface {
            total,
            target,
            pn: pn.to_string(),
            dn,
        }
    }
}
