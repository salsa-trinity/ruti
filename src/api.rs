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
}

impl ApiFlags {
    fn new() -> ApiFlags {
        ApiFlags {
            pl: false,
            len: 0f64,
            len_defined: false,
        }
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
                "_cd" => {
                    self.state = ApiState::BgCd;
                    state_change_counter += 1;
                }
                "-h" | "--help" => Api::flag_h(),
                "-pl" | "--pause-lap" => self.flags.pl = true,
                _ => {
                    self.flags.len = match flag.trim().parse() {
                        Ok(num) => {
                            self.flags.len_defined = true;
                            num
                        }
                        Err(_) => {
                            println!("ERROR: Not a valid argument, use -h for list of arguments.");
                            std::process::exit(1);
                        }
                    };
                }
            }
        }

        // make sure only one mode is used at the time
        if state_change_counter >= 2 {
            println!("ERROR: only a single mode can be used at the time.");
            std::process::exit(1);
        }
        // ensure -pl flag is only used with sw
        else if self.flags.pl && self.state != ApiState::Sw {
            println!("ERROR: -pl flag must only be used with the sw command.");
            std::process::exit(1);
        }
        // make sure length is only specified with cd
        else if self.state != ApiState::BgCd
            && self.state != ApiState::Cd
            && self.flags.len_defined
        {
            println!("ERROR: length must not be specified with a command other than cd.");
            std::process::exit(1);
        }
        // ensure length is given to cd
        else if (self.state == ApiState::BgCd || self.state == ApiState::Cd)
            && !self.flags.len_defined
        {
            println!("ERROR: must specify a length for cd command.");
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
            ApiState::BgCd => crate::bgcd::main(self.flags.len, "TODO: test name".to_string()),
            ApiState::Cd => {}
        }
    }

    fn flag_h() {
        todo!("implement flag_h funcition");
    }
}
