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
            Some(CdCmd::Ls) => ruti::cd::ls::cd_ls_main(args),
            Some(CdCmd::Rm { .. }) => ruti::cd::rm::cd_rm_main(args),
            Some(CdCmd::Clean) => ruti::cd::clean::cd_clean_main(args),
            Some(CdCmd::St { .. }) => ruti::cd::st::cd_st_main(args),
            _ => ruti::cd::cd::cd_main(args),
        },
    }
}
