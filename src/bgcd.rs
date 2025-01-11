use std::io::Write;

pub fn main(len: f64, p_name: String, u_time: f64) {
    let life = std::time::Instant::now();
    let mut target = std::time::Duration::from_secs_f64(len);

    // time_update makes it so that every n seconds,
    // you update the file containing the cd data
    // TODO: config for time_update
    let time_update = std::time::Duration::from_secs_f64(u_time);
    let mut loop_time;

    println!("PID: {}, PN: {}, LEN: {}", std::process::id(), p_name, len);

    // file handling
    let path = match directories::ProjectDirs::from("com", "github", "ruti") {
        Some(p) => p.to_owned(),
        None => {
            println!("ERROR: failed to get project directory.");
            std::process::exit(1);
        }
    };
    let path = path.data_local_dir();
    if !std::fs::exists(path).unwrap() {
        println!("LOG: no directory found, creating a new one.");
        std::fs::create_dir_all(path).unwrap();
    }

    let path = path.join(std::process::id().to_string());
    std::fs::File::create(&path).unwrap();
    std::fs::OpenOptions::new()
        .append(true)
        .open(&path)
        .expect("ERROR: failed to create cd file.")
        .write_all(("0\n".to_owned() + &len.to_string() + "\n" + &p_name).as_bytes())
        .unwrap();
    // NOTE: file format. the file name is the pid
    // - total: (or progress), the amout of time that has currently passed
    // - target: the target time for the cd, cd ends when total reaches target
    // - pn: the human readable name, for not having to type the pid
    let file: String = std::fs::read_to_string(&path).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut lines: Vec<String> = lines.iter().map(|s| s.to_string()).collect();

    // sleep for x - n
    target -= life.elapsed();
    let mut total = std::time::Duration::from_secs(0);
    if target > time_update {
        loop {
            loop_time = std::time::Instant::now();
            if total >= target - time_update {
                break;
            }

            std::thread::sleep(time_update);

            lines[0] = (total + time_update).as_secs_f64().to_string();
            std::fs::write(&path, lines.join("\n")).unwrap();
            println!("LOG: updated");
            total += loop_time.elapsed();
        }
    }
    // sleep for sync
    loop {
        loop_time = std::time::Instant::now();
        if total >= target {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
        total += loop_time.elapsed();
    }
    // TODO: notification code
    println!(
        "total: {}, lifespan: {}",
        total.as_secs_f64(),
        life.elapsed().as_secs_f64()
    );

    std::fs::remove_file(path).unwrap();
}

pub fn bgcd_flags(flags: &mut crate::api::ApiFlags, args: Vec<String>) {
    // TODO:
    // - file for ensuring different cd have different names by default.
    for arg in &args {
        match arg as &str {
            "-n" | "--name" => {
                flags.name_defined += 1;
                if flags.name_defined > 1 {
                    println!("Please only use the -n flag once.");
                    std::process::exit(1);
                }
            }
            "-ut" | "--update-time" => {
                flags.u_time_defined += 1;
                if flags.u_time_defined > 1 {
                    println!("Please only use the -ut flag once.");
                    std::process::exit(1);
                }
            }
            _ => {
                match arg.parse() {
                    Ok(a) => {
                        // len
                        if !flags.len_defined {
                            flags.len = a;
                            flags.len_defined = true;
                        }
                        // utime
                        if flags.u_time_defined == 1 {
                            flags.u_time = a as f64;
                            flags.u_time_defined += 1;
                        }
                    }
                    Err(_) => {
                        // name
                        if flags.name_defined == 1 {
                            flags.name = arg.to_string();
                            flags.name_defined += 1;
                        }
                    }
                }
            }
        }
    }
    // ensure that a lenght is given
    if !flags.len_defined {
        println!("Please give a length.");
        std::process::exit(1);
    }
    // ensure that the length is valid
    if flags.len_defined && flags.len <= 0f64 {
        println!("Please provide a valid length.");
        std::process::exit(1);
    }
    // ensure that if -n is used, an value is passed
    if flags.name_defined != 0 && flags.name.is_empty() {
        println!("Flag -n used, but no name was given.");
        std::process::exit(1);
    }
    // ensure that utime is valid
    if flags.u_time <= 0f64 {
        println!("Please give a valid update time.");
        std::process::exit(1);
    }
    // ensure that if -ut is used, an value is passed
    if flags.u_time_defined % 2 != 0 {
        println!("Flag -ut used, but no duration was given.");
        std::process::exit(1);
    }
}
