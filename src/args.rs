use clap::{Parser, Subcommand};

/// A CLI for time utilities.
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Start a countdown timer.
    Cd {
        /// Specify a length in seconds.
        #[clap()]
        len: Option<f64>, // option to dont break with subcommands

        /// Optionally specify a custom name.
        #[clap(short, long)]
        name: Option<String>,

        /// Optionally specify the interval in seconds to update the countdown's file.
        #[clap(short, long)]
        update_time: Option<f64>,

        /// Optionally use the countdown's subcommands.
        #[command(subcommand)]
        cmd: Option<CdCmd>,
    },
    /// Start a stopwatch.
    Sw {
        // TODO: fix ugly print bug
        //
        /// Lap when pausing
        #[clap(short, long)]
        pause_lap: bool,
    },
    /// Start a background countdown timer.
    BgCd {
        /// Specify a length in seconds.
        #[clap()]
        len: f64,

        /// Optionally specify a custom name.
        #[clap(short, long)]
        name: Option<String>,

        /// Optionally specify the interval in seconds to update the countdown's file.
        #[clap(short, long)]
        update_time: Option<f64>,
    },
}

#[derive(Subcommand, Debug)]
pub enum CdCmd {
    /// List currently running countdowns.
    Ls {},
    /// Stop and remove a running countdown.
    Rm {},
    /// Clean cache of countdown files.
    Clean {},
}

pub fn tests(args: &Args) {
    // TODO: ALL
}
