use std::fs::File;

use argparse::ArgumentParser;

#[macro_use]
extern crate log;
extern crate simplelog;
use simplelog::*;

fn set_logger(log_path: String) {
    if cfg!(debug_assertions) {
        CombinedLogger::init(vec![
            TermLogger::new(
                LevelFilter::Trace,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                LevelFilter::Info,
                Config::default(),
                File::create(format!("{}.iota", log_path)).unwrap(),
            ),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create(format!("{}.iota", log_path)).unwrap(),
        )])
        .unwrap();
    }

    let mut ap = ArgumentParser::new();
    ap.set_description(
        "Don't use this for anything remotely production worthy. I am learning how databases work by creating one so this will be slower, less-stable, and generally worse than any database out there ;).  Learn more at: https://github.com/temannin/iotadb.",
    );
    ap.parse_args_or_exit();
}

mod io;
fn main() {
    set_logger("log".to_string());
    io::network::start_network();
}
