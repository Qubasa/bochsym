#![allow(dead_code)]

use log::*;

fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .with_timestamps(false)
        .init()
        .unwrap();
    log::set_max_level(LevelFilter::Info);

    let app = bochsym::create_cli();
    let matches = app.get_matches();
    bochsym::parse_matches(&matches).unwrap();
}
