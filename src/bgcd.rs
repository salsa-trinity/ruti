use std::io::Write;

pub fn main(len: f64, p_name: String) {
    let life = std::time::Instant::now();
    let mut target = std::time::Duration::from_secs_f64(len);

    // time_update makes it so that every n seconds,
    // you update the file containing the cd data
    // TODO: config for time_update
    let time_update = std::time::Duration::from_secs(2);
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
