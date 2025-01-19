use clap::Parser;
use ruti::{args, args::Args, args::CdCmd, args::Cmd};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args = Args::parse();
    args::tests(&args);
    match args.cmd {
        Cmd::BgCd { .. } => ruti::bgcd::bgcd_main(args),
        Cmd::Sw { .. } => ruti::sw::sw_main(args),
        Cmd::Cd { ref cmd, .. } => match cmd {
            Some(CdCmd::Ls) => println!("TODO: cd ls"),
            Some(CdCmd::Rm) => println!("TODO: cd rm"),
            Some(CdCmd::Clean) => println!("TODO: cd clean"),
            Some(CdCmd::St) => println!("TODO: cd st"),
            _ => ruti::cd::cd::cd_main(args),
        },
    }
}
