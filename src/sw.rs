use crate::api::ApiFlags;
use std::io::Write;
use termion::{input::TermRead, raw::IntoRawMode};

pub fn main(flags: ApiFlags) {
    let (tx, rx) = std::sync::mpsc::channel::<&str>();
    let tx2 = tx.clone();
    let _stdout = std::io::stdout().into_raw_mode().unwrap();
    // i don't know why, but changing the name to _ breaks it :(

    std::thread::spawn(move || {
        crate::sw::listener_thread(tx);
    });

    crate::sw::loop_thread(rx, tx2, &flags);
}

fn listener_thread(tx: std::sync::mpsc::Sender<&str>) {
    let stdin = std::io::stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            termion::event::Key::Esc => {
                tx.send("ESC").unwrap();
                std::process::exit(1);
            }
            termion::event::Key::Char(' ') => {
                tx.send("PAUSE").unwrap();
            }
            termion::event::Key::Char('\n') => {
                tx.send("LAP").unwrap();
            }
            _ => {}
        }
    }
}

fn loop_thread(
    rx: std::sync::mpsc::Receiver<&str>,
    tx2: std::sync::mpsc::Sender<&str>,
    flags: &ApiFlags,
) {
    let mut total_time = std::time::Duration::from_secs(0);
    let mut is_running = true;
    let mut lap_count = 0;

    loop {
        let loop_time = std::time::Instant::now();

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
                std::io::stdout().flush().unwrap();
            }
            _ => {}
        }

        // end of loop
        std::thread::sleep(std::time::Duration::from_micros(1000));
        if is_running {
            print!("\rTime: {}s", total_time.as_millis() as f32 / 1000f32);
            std::io::stdout().flush().unwrap();
            total_time += loop_time.elapsed();
        }
    }
}

pub fn sw_flags(flags: &mut crate::api::ApiFlags, args: Vec<String>) {
    for arg in &args {
        match arg as &str {
            "-pl" | "--pause-lap" => flags.pl = true,
            _ => {
                println!("Please give a valid command for sw. See available commands with -h sw");
                std::process::exit(1);
            }
        }
    }
}
