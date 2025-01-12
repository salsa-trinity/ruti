use std::io::Write;

pub fn main(len: f64, mut p_name: String, u_time: f64) {
    let life = std::time::Instant::now();
    let mut target = std::time::Duration::from_secs_f64(len);

    // time_update makes it so that every n seconds,
    // you update the file containing the cd data
    let time_update = std::time::Duration::from_secs_f64(u_time);
    let mut loop_time;

    // file handling
    let path = match directories::ProjectDirs::from("com", "github", "ruti") {
        Some(p) => p.to_owned(),
        None => {
            println!("ERROR: failed to get project directory.");
            std::process::exit(1);
        }
    };
    let data_path = path.data_local_dir();
    if !std::fs::exists(data_path).unwrap() {
        println!("LOG: no directory found, creating a new one.");
        std::fs::create_dir_all(data_path).unwrap();
    }

    let path = data_path.join(std::process::id().to_string());
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

    // default name
    let mut name_num: i32 = -1;
    let pn_path = data_path.join("pn");
    if p_name.is_empty() {
        if !std::fs::exists(&pn_path).unwrap() {
            println!("LOG: creating a new pn file.");
            std::fs::File::create(&pn_path).unwrap();
        }
        let mut lines: Vec<String> = std::fs::read_to_string(&pn_path)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();
        let mut file = std::fs::File::options()
            .append(true)
            .open(&pn_path)
            .unwrap();
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
            lines.push("\n".to_string() + &name_num.to_string());
            file.write_all(lines.last().unwrap().as_bytes()).unwrap();
        }
        p_name = "cd-".to_string() + &name_num.to_string();
        println!("{:?}", lines);
    }
    println!("PID: {}, PN: {}, LEN: {}", std::process::id(), p_name, len);

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

    // remove cd_name
    if name_num > -1 {
        let lines: Vec<String> = std::fs::read_to_string(&pn_path)
            .unwrap()
            .lines()
            .map(|s| s.to_string())
            .collect();
        let mut new_lines = Vec::new();
        for line in &lines {
            if line.replace("cd-", "").parse::<i32>().unwrap() != name_num {
                new_lines.push(line);
            }
        }
        println!("new line: {:?}", new_lines);
        let mut lines = String::new();
        if new_lines.len() > 0 {
            for i in 0..new_lines.len() - 1 {
                lines += &(new_lines[i].to_owned() + "\n");
            }
            lines += new_lines[new_lines.len() - 1];
        }
        std::fs::write(pn_path, lines).unwrap();
    }
}

pub fn bgcd_flags(flags: &mut crate::api::ApiFlags, args: Vec<String>) {
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
