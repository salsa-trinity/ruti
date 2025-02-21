use crate::args::{Args, Cmd};
use std::{
    io::{self, Stdout, Write},
    process,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub fn sw_main(args: Args) {
    let (tx, rx) = mpsc::channel::<&str>();
    let tx2 = tx.clone();
    let raw_handle = io::stdout().into_raw_mode().unwrap();

    thread::spawn(move || listener_thread(tx, raw_handle));

    loop_thread(rx, tx2, args);
}

fn listener_thread(tx: mpsc::Sender<&str>, raw_handle: RawTerminal<Stdout>) {
    // keypresses
    let stdin = io::stdin();
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Esc => {
                tx.send("ESC").unwrap();
                // fix ugly print bug
                raw_handle.suspend_raw_mode().unwrap();
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

fn loop_thread(rx: mpsc::Receiver<&str>, tx: mpsc::Sender<&str>, args: Args) {
    let mut total_time = Duration::from_secs(0);
    let mut is_running = true;
    let mut lap_count = 0;

    // parse -pl
    let pl = match args.cmd {
        Cmd::Sw { pause_lap } => pause_lap,
        _ => false,
    };

    loop {
        let loop_time = Instant::now();

        match rx.try_recv() {
            Ok("ESC") => {
                break;
            }
            Ok("PAUSE") => {
                is_running = !is_running;
                // pause lap
                if !is_running && pl {
                    tx.send("LAP").unwrap();
                }
            }
            Ok("LAP") => {
                // print
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

        // end
        if is_running {
            print!("\rTime: {}s", total_time.as_millis() as f32 / 1000f32);
            io::stdout().flush().unwrap();
            total_time += loop_time.elapsed();
        }
    }
}
