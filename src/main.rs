fn main() {
    let mut flags: Vec<String> = std::env::args().collect();
    flags.remove(0);

    let mut api = ruti::api::Api::new();
    api.process_flags(flags);
    api.main();
}

// ruti sw
// ruti stopwatch
// # starts a stopwatch, which once started, begins to do input handling:
//      - s: start stop the time (pause, not reset) (stopping also adds a new lap)
//      - r: reset stopwatch and exit ruti
//      - l: add a new lap and print it after the previous lap. (note. the time is below the laps)
//
// ruti cd
// ruti countdown
// # starts a countdown, which one started, initializes input handling:
//      - s: start/pause the countdown
//      - c: exit and cancel the countdown
//      - d: detach from the current countdown (it keeps running in the background)
// ruti cds
// ruti status
// # shows the status for all running countdowns
//
