mod http;
mod socket;

use std::fs::File;
use simplelog::*;
use log::info;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("logs/logs.log").unwrap()),
        ]
    ).unwrap();
    info!("Hello, world!");

}
