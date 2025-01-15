use crate::{
    api::{flags::ApiFlags, state::ApiState},
    bgcd::bgcd_main,
    cd::{cd::cd_main, flags::cd_flags},
    sw::{sw_flags, sw_main},
};
use std::process;

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
            process::exit(1);
        }

        // base commands
        let mut state_change_counter = 0;
        for arg in &args {
            match arg as &str {
                "sw" | "stopwatch" => {
                    self.state = ApiState::Sw;
                    state_change_counter += 1;
                }
                "_cd" => {
                    self.state = ApiState::BgCd;
                    state_change_counter += 1;
                }
                "cd" | "countdown" => {
                    self.state = ApiState::Cd;
                    state_change_counter += 1;
                }
                "-h" | "--help" | "help" => {
                    Api::flag_h();
                    process::exit(1);
                }
                _ => {}
            }
        }
        // TODO: -h (sw,cd)

        // only one base command
        if state_change_counter > 1 {
            println!("Please only use one mode at the time.");
            process::exit(1);
        }
        args.remove(0);

        match self.state {
            ApiState::Sw => sw_flags(&mut self.flags, args),
            ApiState::Cd | ApiState::BgCd => cd_flags(&mut self.flags, args),
            ApiState::Init => {
                println!("Failed to fail.");
                process::exit(1);
            }
        }
    }

    pub fn main(&mut self) {
        match self.state {
            ApiState::Init => {
                println!("Please give a valid argument, use -h for a list of arguments.");
                process::exit(1);
            }
            ApiState::Sw => sw_main(&mut self.flags),
            ApiState::Cd => cd_main(&mut self.flags),
            ApiState::BgCd => bgcd_main(&mut self.flags),
        }
    }

    fn flag_h() {
        todo!("TODO: implement flag_h funcition");
    }
}
