#[derive(PartialEq)]
enum ApiState {
    Init,
    Sw,
    Cd,
    BgCd,
}

#[derive(Clone)]
pub struct ApiFlags {
    pub pl: bool,
    pub len: f64,
    pub len_defined: bool,
    pub name: String,
    pub name_defined: i32,
}

impl ApiFlags {
    fn new() -> ApiFlags {
        ApiFlags {
            pl: false,
            len: 0f64,
            len_defined: false,
            name: String::new(),
            name_defined: 0,
        }
    }
}

pub struct Api {
    state: ApiState,
    pub flags: ApiFlags,
}

impl Api {
    pub fn new() -> Api {
        Api {
            state: ApiState::Init,
            flags: ApiFlags::new(),
        }
    }

    pub fn main_flags(&mut self, mut args: Vec<String>) {
        // empty
        if args.len() == 0 {
            println!("Please provide an argument. Use -h for available commands");
            std::process::exit(1);
        }

        // base commands
        let mut state_change_counter = 0;
        for arg in &args {
            match arg as &str {
                "sw" | "stopwatch" => {
                    self.state = ApiState::Sw;
                    state_change_counter += 1;
                }
                "cd" | "countdown" => {
                    self.state = ApiState::Cd;
                    state_change_counter += 1;
                }
                "_cd" => {
                    self.state = ApiState::BgCd;
                    state_change_counter += 1;
                }

                "-h" | "--help" | "help" => {
                    Api::flag_h();
                    std::process::exit(1);
                }
                _ => {}
            }
        }
        // TODO: -h (sw,cd,bgcd)

        // only one base command
        if state_change_counter > 1 {
            println!("Please only use one mode at the time.");
            std::process::exit(1);
        }
        args.remove(0);
        println!("FLAGS: {:?}", args);

        match self.state {
            ApiState::Sw => crate::sw::sw_flags(&mut self.flags, args),
            ApiState::Cd | ApiState::BgCd => crate::bgcd::bgcd_flags(&mut self.flags, args),
            _ => {
                println!("Failed to fail x2");
                std::process::exit(1);
            }
        }
    }

    pub fn main(&mut self) {
        match self.state {
            ApiState::Init => {
                println!("Please give a valid argument, use -h for a list of arguments.");
                std::process::exit(1);
            }
            ApiState::Sw => crate::sw::main(self.flags.clone()),
            ApiState::BgCd => crate::bgcd::main(self.flags.len, self.flags.name.clone()),
            ApiState::Cd => {}
        }
    }

    fn flag_h() {
        todo!("TODO: implement flag_h funcition");
    }
}
