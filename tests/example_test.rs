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

#[test]
fn parse_sym() {
    let res = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources");

    let app = create_cli();
    let symfile = res.join("bootloader.sym");
    let cmd = vec!["bochsym", "--symfile", symfile.to_str().unwrap(), "-o", "resources/bootloader.sym.bochs"];

    let matches = app.try_get_matches_from(cmd).unwrap();

    bochsym::parse_matches(&matches).expect("Failed to execute test");
}
