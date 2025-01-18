use clap::Parser;
use ruti::{args::Args, args::CdCmd, args::Cmd};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args = Args::parse();
    match args.cmd {
        Cmd::BgCd { .. } => ruti::bgcd::bgcd_main(args),
        Cmd::Sw { .. } => ruti::sw::sw_main(args),
        Cmd::Cd { cmd, .. } => match cmd {
            Some(CdCmd::Ls {}) => println!("TODO: cd ls"),
            Some(CdCmd::Rm {}) => println!("TODO: cd rm"),
            _ => println!("TODO: cd"),
        },
    }
}
