#![allow(dead_code)]

use clap::ArgMatches;
use clap::{value_parser, Arg, SubCommand};
use log::*;

use std::process::ExitCode;
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process,
};
use std::{fs::OpenOptions, io::Write};


pub fn create_cli() -> clap::App<'static> {
    let app = clap::Command::new("bochsym")
        .author("Luis Hebendanz <luis.nixos@gmail.com")
        .about("Parses symbol files and makes them bochs compatible")
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .help("Enables verbose mode")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("vv")
                .long("vv")
                .help("Enables very verbose mode")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("symfile")
                .help("Path to symbol file")
                .short('s')
                .long("symfile")
                .takes_value(true)
                .value_parser(value_parser!(PathBuf)),
        );


    app
}

pub fn parse_matches(matches: &ArgMatches) -> Result<(), ExitCode> {
    let is_release = matches.is_present("release");
    let is_verbose = matches.is_present("verbose");
    let is_vv = matches.is_present("vv");
    if is_verbose || is_vv {
        log::set_max_level(LevelFilter::Debug);
    }
    debug!("Args: {:?}", std::env::args());

    let kernel_manifest_path: PathBuf = env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .or_else(|_| {
            info!("CARGO_MANIFEST_DIR not set. Using current directory");
            std::env::current_dir()
        })
        .expect("Failed to a cargo manifest path");

  

    Ok(())
}

