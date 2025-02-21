use clap::Parser;
use ruti::{
    args::{self, Args, CdCmd, Cmd},
    bgcd::bgcd_main,
    cd::{cd::cd_main, clean::cd_clean_main, ls::cd_ls_main, rm::cd_rm_main, st::cd_st_main},
    sw::sw_main,
};
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args = Args::parse();
    args::tests(&args);
    match args.cmd {
        Cmd::BgCd { .. } => bgcd_main(args),
        Cmd::Sw { .. } => sw_main(args),
        Cmd::Cd { ref cmd, .. } => match cmd {
            Some(CdCmd::Ls) => cd_ls_main(),
            Some(CdCmd::Rm { .. }) => cd_rm_main(args),
            Some(CdCmd::Clean) => cd_clean_main(),
            Some(CdCmd::St { .. }) => cd_st_main(args),
            _ => cd_main(args),
        },
    }
}

// TODO: clean files
// - [x] main
// - [ ] bgcd
// - [x] sw
// - [x] args
// - [x] iface
// - [x] st
// - [x] clean
// - [ ] ls
// - [ ] rm
// - [ ] cd
