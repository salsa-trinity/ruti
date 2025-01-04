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
                            _ => {}
                        }
                    }
                });

                let sw_start = std::time::Instant::now();
                loop {
                    print!(
                        "\rTime: {}s",
                        sw_start.elapsed().as_millis() as f32 / 1000f32
                    );
                    std::io::stdout().flush().unwrap();
                    match rx.try_recv() {
                        Ok("ESC") => {
                            //println!("");
                            print!("\n\rExited successfully.\n\r");
                            std::io::stdout().flush().unwrap();
                            break;
                        }
                        Ok(_) => {
                            println!("help");
                            break;
                        }
                        _ => {}
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
