use bochsym::*;
use std::path::PathBuf;

#[test]
fn print_help() {
    let app = create_cli();

    let parse = app.try_get_matches_from(vec!["bochsym", "--help"]);

    match parse {
        Err(err) => {
            assert!(err.kind() == clap::ErrorKind::DisplayHelp)
        }
        Ok(_) => panic!("Help does not print help"),
    };
}


