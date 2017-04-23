#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate clap;
extern crate crypto;

use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path;
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    match matches.subcommand() {
        ("status", _) => {
            println!("Checking the status!");
            repository_directory();
        },
        _ => {
            println!("Sorry, that's not implemented yet.");
            process::exit(1);
        },
    }
}

fn repository_directory() -> io::Result<path::PathBuf> {
    let cwd = try!(env::current_dir());
    let mut cwd = try!(cwd.canonicalize());
    loop {
        cwd.push(".lucid");
        if cwd.exists() {
            return Ok(cwd);
        }
        cwd.pop();
        if !cwd.pop() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                                      "no repository"));
        }
    }
}

trait TreeEntry {}

struct Blob {
    contents: String,
}

struct Tree {
    entries: Vec<Box<TreeEntry>>,
}

impl TreeEntry for Blob {}
impl TreeEntry for Tree {}

struct Commit {
    parent: Option<Box<Commit>>,
    tree: Tree,
}

trait Object {}
