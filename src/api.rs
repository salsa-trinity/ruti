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
            ApiState::Sw => crate::sw::main(),
            ApiState::Cd => {}
        }
    }

    fn flag_h() {
        todo!("implement flag_h funcition");
    }
}
