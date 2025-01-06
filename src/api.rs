#[derive(PartialEq)]
enum ApiState {
    Init,
    Sw,
    Cd,
}

#[derive(Clone)]
pub struct ApiFlags {
    pub pl: bool,
}

impl ApiFlags {
    fn new() -> ApiFlags {
        ApiFlags { pl: false }
    }
}

pub struct Api {
    state: ApiState,
    flags: ApiFlags,
}

impl Api {
    pub fn new() -> Api {
        Api {
            state: ApiState::Init,
            flags: ApiFlags::new(),
        }
    }

    pub fn process_flags(&mut self, flags: Vec<String>) {
        let mut state_change_counter = 0;
        for flag in flags {
            match &flag as &str {
                "sw" | "stopwatch" => {
                    self.state = ApiState::Sw;
                    state_change_counter += 1;
                }
                "cd" | "countdown" => {
                    self.state = ApiState::Cd;
                    state_change_counter += 1;
                }
                "-h" | "--help" => Api::flag_h(),
                "-pl" | "--pause-lap" => self.flags.pl = true,
                _ => {
                    println!("ERROR: Not a valid argument, use -h for list of arguments.");
                    std::process::exit(1);
                }
            }
        }
        if state_change_counter >= 2 {
            println!("ERROR: only a single mode can be used at the time.");
            std::process::exit(1);
        }
        if self.flags.pl && self.state != ApiState::Sw {
            println!("ERROR: -pl flag must only be used with the sw command.");
            std::process::exit(1);
        }
    }

    pub fn main(&mut self) {
        match self.state {
            ApiState::Init => {
                println!("Please give a valid argument, use -h for list of arguments.");
                std::process::exit(1);
            }
            ApiState::Sw => crate::sw::main(self.flags.clone()),
            ApiState::Cd => {}
        }
    }

    fn flag_h() {
        todo!("implement flag_h funcition");
    }
}
