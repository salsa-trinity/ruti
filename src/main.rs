fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0);

    let mut api = ruti::api::Api::new();
    api.main_flags(args);
    api.main();
}

// ruti cd
// ruti countdown
// # starts a countdown, which one started, initializes input handling:
//      - s: start/pause the countdown
//      - esc: exit and cancel the countdown
//      - d: detach from the current countdown (it keeps running in the background)
// ruti cds
// ruti status
// # shows the status for all running countdowns
//
