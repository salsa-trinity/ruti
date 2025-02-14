use crate::{
    args::{Args, Cmd},
    cd::iface::CdIface,
};
use std::{
    fs,
    io::Write,
    path::Path,
    process, thread,
    time::{Duration, Instant},
};

pub fn bgcd_main(args: Args) {
    let data_path = CdIface::get_data_path();
    let data_path = data_path.data_local_dir();
    if !fs::exists(data_path).unwrap() {
        println!("LOG: no data directory found, creating a new one.");
        fs::create_dir_all(data_path).unwrap();
    }
    bgcd(&data_path, args);
}

fn bgcd(data_path: &Path, args: Args) {
    let life = Instant::now();
    let (len, u_time, mut name) = match args.cmd {
        Cmd::BgCd {
            len,
            name,
            update_time,
        } => (
            len,
            update_time.unwrap_or(60f64),
            name.clone().unwrap_or("".to_string()),
        ),
        _ => (0f64, 60f64, "".to_string()),
    };
    let mut target = Duration::from_secs_f64(len);

    // time_update makes it so that every n seconds,
    // you update the file containing the cd data
    let time_update = Duration::from_secs_f64(u_time);
    let mut loop_time;

    // verify pn isnt already used
    if !name.is_empty() {
        for file in fs::read_dir(&data_path).unwrap() {
            if name == CdIface::from_path(&file.unwrap().path()).unwrap().pn {
                println!("Name already in use. Try a different one");
                process::exit(1);
            }
        }
    }

    // file handling
    let mut name_num = -1;
    let dn_path = &data_path.join("dn");
    let cd_path = &data_path.join(process::id().to_string());
    if name.is_empty() {
        name_num = default_name(data_path);
        name = String::from("cd-").to_owned() + &name_num.to_string();
    }
    // TODO: make the create_bgcd_file nicer with the iface
    let mut lines = create_bgcd_file(&data_path, len, &name);
    let mut iface = CdIface::from_pid(process::id()).unwrap();
    println!("PID: {}, PN: {}, LEN: {}", process::id(), name, len);

    // sleep for x - n
    let mut total = Duration::from_secs(0);
    target -= life.elapsed();
    if target > time_update {
        loop {
            loop_time = Instant::now();
            if total >= target - time_update {
                break;
            }

            thread::sleep(time_update);

            iface.total = (total + time_update).as_secs_f64();
            iface.save();

            println!("LOG: updated");
            total += loop_time.elapsed();
        }
    }
    // sleep for sync
    loop {
        loop_time = Instant::now();
        if total >= target {
            break;
        }
        thread::sleep(Duration::from_millis(1));
        total += loop_time.elapsed();
    }
    // TODO: notification code
    println!(
        "total: {}, lifespan: {}",
        total.as_secs_f64(),
        life.elapsed().as_secs_f64()
    );

    // remove files
    if name_num > -1 {
        delete_dn(&dn_path, name_num);
    }
    fs::remove_file(cd_path).unwrap();
}

fn create_bgcd_file(data_path: &Path, len: f64, p_name: &str) -> Vec<String> {
    let cd_path = &data_path.join(process::id().to_string());
    fs::File::create(&cd_path).unwrap();
    fs::OpenOptions::new()
        .append(true)
        .open(&cd_path)
        .expect("ERROR: failed to create cd file.")
        .write_all(("0\n".to_owned() + &len.to_string() + "\n" + &p_name).as_bytes())
        .unwrap();
    let lines: Vec<String> = fs::read_to_string(&cd_path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    lines
}

fn default_name(data_path: &Path) -> i32 {
    let name_num;
    let dn_path = data_path.join("dn");
    if !fs::exists(&dn_path).unwrap() {
        println!("LOG: creating a new dn file.");
        fs::File::create(&dn_path).unwrap();
    }
    let lines = fs::read_to_string(&dn_path).unwrap();
    let mut lines: Vec<&str> = lines.lines().collect();
    let mut file = fs::File::options().append(true).open(&dn_path).unwrap();
    if lines.is_empty() {
        println!("INITING");
        file.write_all(b"0").unwrap();
        name_num = 0;
    } else {
        println!("ADDING");
        let nums: Vec<i32> = lines.iter().map(|l| l.parse::<i32>().unwrap()).collect();
        if nums.len() > 0 {
            let mut i: i32 = 0;
            loop {
                if !nums.contains(&i) {
                    name_num = i;
                    break;
                }
                i += 1;
            }
        } else {
            name_num = lines.last().unwrap().parse::<i32>().unwrap() + 1;
        }
        let name_num_str = "\n".to_owned() + &name_num.to_string();
        lines.push(&name_num_str);
        file.write_all(lines.last().unwrap().as_bytes()).unwrap();
    }
    name_num
}

fn delete_dn(dn_path: &Path, name_num: i32) {
    let lines: &str = &fs::read_to_string(&dn_path).unwrap();
    let lines: Vec<&str> = lines.lines().collect();
    println!("lines: {:?}", lines);
    let mut new_lines: Vec<&str> = Vec::new();
    for line in &lines {
        if line.parse::<i32>().unwrap() != name_num {
            new_lines.push(line);
        }
    }
    println!("new_lines: {:?}", new_lines);
    let lines = new_lines.join("\n");
    println!("new_new_lines: {}", lines);
    fs::write(&dn_path, lines).unwrap();
}
