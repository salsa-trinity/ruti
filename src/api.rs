use std::io::Write;
use termion::{input::TermRead, raw::IntoRawMode};

enum ApiState {
    Init,
    Sw,
    Cd,
}

pub struct Api {
    state: ApiState,
}

impl Api {
    pub fn new() -> Api {
        Api {
            state: ApiState::Init,
        }
    }

    pub fn process_flags(&mut self, flags: Vec<String>) {
        for flag in flags {
            match &flag as &str {
                "sw" | "stopwatch" => {
                    self.state = ApiState::Sw;
                }
                "cd" | "countdown" => {
                    self.state = ApiState::Cd;
                }
                "-h" | "--help" => {
                    Api::flag_h();
                }
                _ => {
                    println!("ERROR: Not a valid argument, use -h for list of arguments.");
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn main(&mut self) {
        match self.state {
            ApiState::Init => {
                println!("Please give a valid argument, use -h for list of arguments.");
                std::process::exit(1);
            }
            ApiState::Sw => {
                let (tx, rx) = std::sync::mpsc::channel::<&str>();
                let _stdout = std::io::stdout().into_raw_mode().unwrap();

                std::thread::spawn(move || {
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
                });

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
                    std::thread::sleep(std::time::Duration::from_micros(100));
                    if is_running {
                        print!("\rTime: {}s", total_time.as_millis() as f32 / 1000f32);
                        std::io::stdout().flush().unwrap();
                        total_time += loop_time.elapsed();
                    }
                }
            }
            ApiState::Cd => {}
        }
    }

    fn flag_h() {
        todo!("implement flag_h funcition");
    }
}
