mod http;
mod socket;

use std::fs::File;
use simplelog::*;
use log::info;
use crate::http::start_http;

const HTTP_PORT:u32 = 80;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("logs/logs.log").unwrap()),
        ]
    ).unwrap();
    info!("Hello, world!");
    start_http(HTTP_PORT)
}
