use clap::{App, Arg, SubCommand};
use git::objects::Blob;
use git::repository::Repository;

use std::fs::File;
use std::io::prelude::*;

mod git;

const DEFAULT_REPO: &str = ".";

fn main() {
    let matches = App::new("tig - Light Git")
        .version("0.1")
        .about("Lightweight Git for study")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a new git repository")
                .arg(Arg::with_name("directory").required(true).index(1)),
        )
        .subcommand(
            SubCommand::with_name("cat-file")
                .arg(Arg::with_name("TYPE").required(true).index(1))
                .arg(Arg::with_name("OBJECT").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("hash-object")
                .arg(Arg::with_name("w").short("w"))
                .arg(Arg::with_name("TYPE").required(true).index(1))
                .arg(Arg::with_name("FILE").required(true).index(2)),
        )
        .subcommand(
            SubCommand::with_name("update-ref")
                .arg(Arg::with_name("OBJECT").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => cmd_init(args.value_of("directory").unwrap()),
        ("cat-file", Some(args)) => cmd_cat_file(
            args.value_of("TYPE").unwrap(),
            args.value_of("OBJECT").unwrap(),
        ),
        ("hash-object", Some(args)) => cmd_hash_object(
            args.value_of("TYPE").unwrap(),
            args.value_of("FILE").unwrap(),
            args.is_present("w"),
        ),
        ("update-ref", Some(args)) => cmd_update_ref(
            args.value_of("OBJECT").unwrap()
        ),
        _ => println!("{}", matches.usage()),
    }
}

fn cmd_init(directory: &str) {
    let repo = Repository::create(directory.to_string()).unwrap();
    println!("new git repository created {}", repo.gitdir.display());
}

fn cmd_cat_file(_t: &str, p: &str) {
    let repo = Repository::new(DEFAULT_REPO.to_string());
    let blob = repo.object_read(p.to_string());
    print!("{}", blob.display());
}

fn cmd_hash_object(_t: &str, p: &str, w: bool) {
    let repo = Repository::new(DEFAULT_REPO.to_string());

    let mut file = File::open(p).unwrap();
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();

    let blob = Blob::new(buff);
    let hash = repo.object_write(blob, w).unwrap();
    println!("sha1: {}", hash);
}

fn cmd_update_ref(p: &str) {
    let repo = Repository::new(DEFAULT_REPO.to_string());
    repo.update_ref(p.to_string());
}
