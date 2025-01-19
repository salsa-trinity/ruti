use clap::{Parser, Subcommand};
use std::process;

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
        // TODO: fix ugly print bug. opions:
        // - device-query
        // - crossterm
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
    Ls,
    /// Stop and remove a running countdown.
    Rm,
    /// Clean cache of countdown files.
    Clean,
    /// Get the status for a specific countdown
    St,
}

pub fn tests(args: &Args) {
    match &args.cmd {
        Cmd::Cd {
            len,
            cmd,
            update_time,
            ..
        } => match cmd {
            // when no subcommand is used
            None => {
                // length given
                if len.is_none() {
                    println!("Please give a length.");
                    process::exit(1);
                }
                // vaild length
                else if len.is_some() && len.unwrap() <= 0f64 {
                    println!("Please give a valid length.");
                    process::exit(1);
                }
                // valid update time
                else if update_time.is_some() && update_time.unwrap() <= 0f64 {
                    println!("Please give a valid update time.");
                    process::exit(1);
                }
            }
            _ => {}
        },
        _ => {}
    }
}
