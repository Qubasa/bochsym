
use clap::ArgMatches;
use clap::{value_parser, Arg};
use log::*;

use std::io::Write;
use std::path::PathBuf;
use std::process::ExitCode;
use symbolic::common::{ByteView, Name};
use symbolic::{debuginfo::{Object}, demangle::{Demangle, DemangleOptions}};

pub fn create_cli() -> clap::App<'static> {
    let app = clap::Command::new("bochsym")
        .author("Luis Hebendanz <luis.nixos@gmail.com")
        .about("Parses binary symbol files and makes them bochs compatible")
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
                .help("Path to binary file with symbols")
                .short('s')
                .long("symfile")
                .required(true)
                .multiple_occurrences(true)
                .takes_value(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::with_name("out")
                .help("Path to out file")
                .short('o')
                .long("out")
                .required(true)
                .takes_value(true)
                .value_parser(value_parser!(PathBuf)),
        );

    app
}

pub fn parse_matches(matches: &ArgMatches) -> Result<(), ExitCode> {
    let is_verbose = matches.is_present("verbose");
    let is_vv = matches.is_present("vv");
    if is_verbose {
        log::set_max_level(LevelFilter::Debug);
    }
    if is_vv {
        log::set_max_level(LevelFilter::Trace);
    }
    debug!("Args: {:?}", std::env::args());

    let symfiles: Vec<&PathBuf> = matches.get_many("symfile").unwrap().collect();

    let outfile: &PathBuf = matches.get_one("out").unwrap();

    let mut file = std::fs::File::create(outfile).expect("Couldn't create out file");
    let map = parse_symfiles(symfiles);

    for (address, name) in map {
        let data = format!("{:x} {}\n", address, name);
        file.write_all(data.as_bytes()).expect("Failed to write to file");
    }

    Ok(())
}
use std::collections::HashMap;



pub fn parse_symfiles(symfiles: Vec<&std::path::PathBuf>) -> HashMap<u64, String> {
    let mut map = HashMap::new();

    for symfile in symfiles {
        assert!(
            symfile.is_file(),
            "Symfile not found: {}",
            symfile.display()
        );
        let buffer = ByteView::open(symfile).unwrap();

        let object = Object::parse(&buffer).unwrap();
        debug!("File format: {}", object.file_format());

        for sym in object.symbols() {
            let symname = Name::from(sym.name().unwrap());
            let demangled_sym = symname.try_demangle(DemangleOptions::complete()).to_string();
            let data = format!("{:x} {}\n", sym.address, demangled_sym);
            trace!("{}", data);
            map.insert(sym.address + object.load_address(), demangled_sym);
        }
    }
    map
}

