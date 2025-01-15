use crate::api::flags::ApiFlags;
use std::process;

pub fn cd_flags(flags: &mut ApiFlags, args: Vec<String>) {
    let mut cd_command_change_counter = 0;
    for arg in &args {
        match arg as &str {
            "-n" | "--name" => {
                flags.name_defined += 1;
                if flags.name_defined > 1 {
                    println!("Please only use the -n flag once.");
                    process::exit(1);
                }
            }
            "-ut" | "--update-time" => {
                flags.u_time_defined += 1;
                if flags.u_time_defined > 1 {
                    println!("Please only use the -ut flag once.");
                    process::exit(1);
                }
            }
            "ls" | "list" => {
                flags.ls_defined += 1;
                cd_command_change_counter += 1;
                if flags.ls_defined > 1 {
                    println!("Please only use the ls command once.");
                    process::exit(1);
                }
            }
            _ => {
                match arg.parse() {
                    Ok(a) => {
                        // len
                        if !flags.len_defined {
                            flags.len = a;
                            flags.len_defined = true;
                        }
                        // utime
                        if flags.u_time_defined == 1 {
                            flags.u_time = a as f64;
                            flags.u_time_defined += 1;
                        }
                    }
                    Err(_) => {
                        // name
                        if flags.name_defined == 1 {
                            flags.name = arg.to_string();
                            flags.name_defined += 1;
                        } else {
                            println!("Please give a valid argument. Use -h cd for a list of available commands.");
                            process::exit(1);
                        }
                    }
                }
            }
        }
    }
    // ensure that only one cd command is used at the time
    // TODO: proposal: make sure that when now using a command, the next argument is the lenght
    flags.cd_command_defined = cd_command_change_counter;
    if cd_command_change_counter > 1 {
        println!("Please only use one cd command at the time.");
        process::exit(1);
    }
    // ensure that cd commands dont interact with flags
    if cd_command_change_counter > 0 {
        if flags.u_time_defined > 0 || flags.name_defined > 0 {
            println!("Please only use flags when not using a command.");
            process::exit(1);
        }
    } else {
        // ensure that a lenght is given
        if !flags.len_defined {
            println!("Please give a length.");
            process::exit(1);
        }
        // ensure that the length is valid
        if flags.len_defined && flags.len <= 0f64 {
            println!("Please provide a valid length.");
            process::exit(1);
        }
        // ensure that if -n is used, an value is passed
        if flags.name_defined != 0 && flags.name.is_empty() {
            println!("Flag -n used, but no name was given.");
            process::exit(1);
        }
        // ensure that utime is valid
        if flags.u_time <= 0f64 {
            println!("Please give a valid update time.");
            process::exit(1);
        }
        // ensure that if -ut is used, an value is passed
        if flags.u_time_defined % 2 != 0 {
            println!("Flag -ut used, but no duration was given.");
            process::exit(1);
        }
    }
}
