use crate::api::flags::ApiFlags;
use std::{io, io::Write, process, sync::mpsc, thread, time::Duration, time::Instant};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};

pub fn sw_main(flags: ApiFlags) {
    let (tx, rx) = mpsc::channel::<&str>();
    let tx2 = tx.clone();
    let _stdout = std::io::stdout().into_raw_mode().unwrap();
    // i don't know why, but changing the name to _ breaks it :(

    thread::spawn(move || listener_thread(tx));

    loop_thread(rx, tx2, &flags);
}

fn listener_thread(tx: mpsc::Sender<&str>) {
    let stdin = io::stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => {
                tx.send("ESC").unwrap();
                process::exit(1);
            }
            Key::Char(' ') => {
                tx.send("PAUSE").unwrap();
            }
            Key::Char('\n') => {
                tx.send("LAP").unwrap();
            }
            _ => {}
        }
    }
}

fn loop_thread(rx: mpsc::Receiver<&str>, tx2: mpsc::Sender<&str>, flags: &ApiFlags) {
    let mut total_time = Duration::from_secs(0);
    let mut is_running = true;
    let mut lap_count = 0;

    loop {
        let loop_time = Instant::now();

        match rx.try_recv() {
            Ok("ESC") => {
                break;
            }
            Ok("PAUSE") => {
                is_running = !is_running;
                if !is_running && flags.pl {
                    tx2.send("LAP").unwrap();
                }
            }
            Ok("LAP") => {
                println!(
                    "\rLap {}: {}s",
                    lap_count,
                    total_time.as_millis() as f32 / 1000f32,
                );
                lap_count += 1;
                if !is_running {
                    print!("\rTime: {}s", total_time.as_millis() as f32 / 1000f32,)
                }
                io::stdout().flush().unwrap();
            }
            _ => {}
        }

        // end of loop
        thread::sleep(Duration::from_millis(1));
        if is_running {
            print!("\rTime: {}s", total_time.as_millis() as f32 / 1000f32);
            io::stdout().flush().unwrap();
            total_time += loop_time.elapsed();
        }
    }
}

pub fn sw_flags(flags: &mut ApiFlags, args: Vec<String>) {
    for arg in &args {
        match arg as &str {
            "-pl" | "--pause-lap" => flags.pl = true,
            _ => {
                println!("Please give a valid command for sw. See available commands with -h sw");
                process::exit(1);
            }
        }
    }
}
