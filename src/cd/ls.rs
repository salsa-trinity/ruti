use crate::{cd::iface::CdIface, data_path};
use std::fs;

// struct for it to work with tabled
#[derive(tabled::Tabled)]
struct LsOut {
    name: String,
    total: f64,
    target: f64,
    left: f64,
}

impl LsOut {
    fn from_cdiface(iface: CdIface) -> LsOut {
        LsOut {
            name: iface.pn.clone(),
            total: iface.total.round(),
            target: iface.target.round(),
            left: iface.target.round() - iface.total.round(),
        }
    }
}

pub fn cd_ls_main() {
    // push statuses
    let data_path = &data_path();
    let mut no_file = true;
    let mut statuses: Vec<LsOut> = Vec::new();
    for file in fs::read_dir(&data_path).unwrap() {
        let file = file.unwrap();
        if file.file_name() != "dn" {
            statuses.push(LsOut::from_cdiface(
                CdIface::from_path(&file.path()).unwrap(),
            ));
            no_file = false;
        }
    }

    // print ls
    if no_file {
        println!("No cd running.");
    } else {
        let style = tabled::settings::Style::sharp();
        println!("{}", tabled::Table::new(statuses).with(style).to_string());
    }
}
