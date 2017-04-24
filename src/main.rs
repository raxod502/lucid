#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate clap;
extern crate crypto;

use crypto::digest::Digest;
use crypto::sha1::Sha1;
use std::env;
use std::fmt;
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
            let blob = Blob { contents: "Hello, world!\n" };
            let tree = Tree { entries: vec![
                TreeEntry { name: "hello.txt",
                            data:
                            TreeEntryData::Blob
                            { blob: &blob,
                              filetype: FileType::NormalFile }},
            ]};
            let commit = Commit { tree: &tree, parent: None };
            let new_blob = Blob { contents: "Says hello.\n" };
            let new_tree = Tree { entries: vec![
                TreeEntry { name: "README.md",
                            data:
                            TreeEntryData::Blob
                            { blob: &blob,
                              filetype: FileType::NormalFile }},
                TreeEntry { name: "data",
                            data:
                            TreeEntryData::Tree
                            { tree: &tree }},
            ]};
            let new_commit = Commit { tree: &new_tree,
                                      parent: Some(&commit) };
            println!("Here is a blob ({}):\n---\n{}---",
                     blob.to_hash(), blob);
            println!("Here is a tree ({}):\n---\n{}---",
                     tree.to_hash(), tree);
            println!("Here is a commit ({}):\n---\n{}---",
                     commit.to_hash(), commit);
            println!("Here is a new blob ({}):\n---\n{}---",
                     new_blob.to_hash(), new_blob);
            println!("Here is a new tree ({}):\n---\n{}---",
                     new_tree.to_hash(), new_tree);
            println!("Here is a new commit ({}):\n---\n{}---",
                     new_commit.to_hash(), new_commit);
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
            if cwd.is_dir() {
                return Ok(cwd);
            }
            else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("not a directory: {}", cwd.to_string_lossy())));
            }
        }
        cwd.pop();
        if !cwd.pop() {
            return Err(io::Error::new(io::ErrorKind::NotFound,
                                      "no repository"));
        }
    }
}

fn worktree_root() -> io::Result<path::PathBuf> {
    let mut path = try!(repository_directory());
    path.pop();
    Ok(path)
}

struct Blob<'a> {
    contents: &'a str,
}

struct Tree<'a> {
    entries: Vec<TreeEntry<'a>>,
}

struct TreeEntry<'a> {
    name: &'a str,
    data: TreeEntryData<'a>,
}

enum FileType {
    NormalFile,
    Executable,
    SymbolicLink,
}

enum TreeEntryData<'a> {
    Blob {
        blob: &'a Blob<'a>,
        filetype: FileType,
    },
    Tree {
        tree: &'a Tree<'a>,
    },
}

struct Commit<'a> {
    tree: &'a Tree<'a>,
    parent: Option<&'a Commit<'a>>,
}

trait Object : fmt::Display {
    fn to_hash(&self) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(self.to_string().as_str());
        hasher.result_str()
    }
}

impl<'a> fmt::Display for Blob<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "blob\n{}", self.contents)
    }
}

impl<'a> fmt::Display for Tree<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "tree\n"));
        for entry in &self.entries {
            try!(write!(f, "{} {} {}\n", match &entry.data {
                &TreeEntryData::Blob { ref filetype, .. } => match filetype {
                    &FileType::NormalFile => "file",
                    &FileType::Executable => "exec",
                    &FileType::SymbolicLink => "link",
                },
                &TreeEntryData::Tree { .. } => "tree",
            }, match &entry.data {
                &TreeEntryData::Blob { ref blob, .. } => blob.to_hash(),
                &TreeEntryData::Tree { ref tree } => tree.to_hash(),
            }, entry.name));
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Commit<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "commit\ntree {}\n", self.tree.to_hash()));
        if let Some(ref parent) = self.parent {
            try!(write!(f, "parent {}\n", parent.to_hash()));
        }
        Ok(())
    }
}

impl<'a> Object for Blob<'a> {}
impl<'a> Object for Tree<'a> {}
impl<'a> Object for Commit<'a> {}
