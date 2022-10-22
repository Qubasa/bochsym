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

    let symfile:PathBuf = PathBuf::from(res.join("bootloader.sym").to_str().unwrap());
    let map = parse_symfiles(vec![&symfile]);

    let bootloader_main = map.into_iter().find(|x| x.1 == "bootloader_main").unwrap();
    assert!(bootloader_main.0 == 0x1076a0, "bootloader_main should be 0x1076a0 but is {:#x}", bootloader_main.0);
}

