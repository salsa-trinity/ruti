use std::io::Write;

pub fn main(len: f64, p_name: String) {
    let life = std::time::Instant::now();
    let target = std::time::Duration::from_secs_f64(len);

    // time_update makes it so that every n seconds,
    // you update the file containing the cd data
    let time_update = std::time::Duration::from_secs(2);
    let mut loop_time;

    println!("PID: {}, PN: {}, LEN: {}", std::process::id(), p_name, len);

    // sleep for x - n
    let mut total = std::time::Duration::from_secs(0);
    // TODO: fix let total behaviour
    if target > time_update {
        loop {
            loop_time = std::time::Instant::now();
            if total >= target - time_update {
                break;
            }

            std::thread::sleep(time_update);
            total += loop_time.elapsed();
            println!("PROGRESS: {}", total.as_secs_f64());
            std::io::stdout().flush().unwrap();
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
    println!(
        "total: {}, lifespan: {}",
        total.as_secs_f64(),
        life.elapsed().as_secs_f64()
    );
}
