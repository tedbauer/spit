extern crate clap;
extern crate ini;

use clap::{App, Arg, SubCommand};
use ini::Ini;
use sha1::{Digest, Sha1};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

struct GitRepository {
    work_tree: PathBuf,
    git_dir: PathBuf,
    conf: PathBuf,
}

struct GitCommit {}
struct GitTree {}
struct GitTag {}
struct GitBlob {}

enum GitObject {
    GitCommit(GitCommit),
    GitTree(GitTree),
    GitTag(GitTag),
    GitBlob(GitBlob),
}

impl GitObject {
    fn serialize(data: &str) -> Self {
        todo!()
    }

    fn deserialize(&self) -> String {
        todo!()
    }
}

const DEFAULT_REPO_DESCRIPTION: &str =
    "Unnamed repository; edit this file 'description' to name the repository.\n";
const DEFAULT_HEAD: &str = "ref: refs/heads/master\n";

fn cmd_init(work_tree: &PathBuf) {
    if work_tree.exists() {
        if !work_tree.is_dir() {
            panic!("not a directory"); // TODO make this an error
        }
    } else {
        fs::create_dir(work_tree).unwrap();
    }

    fs::create_dir_all(work_tree.join(".git").join("branches")).unwrap();
    fs::create_dir_all(work_tree.join(".git").join("objects")).unwrap();
    fs::create_dir_all(work_tree.join(".git").join("refs").join("tags")).unwrap();
    fs::create_dir_all(work_tree.join(".git").join("refs").join("heads")).unwrap();

    fs::write(
        work_tree.join(".git").join("description"),
        DEFAULT_REPO_DESCRIPTION,
    )
    .unwrap();
    fs::write(work_tree.join(".git").join("HEAD"), DEFAULT_HEAD).unwrap();

    let mut conf = Ini::new();
    conf.with_section(Some("core"))
        .set("repositoryformatversion", "0")
        .set("filemode", "false")
        .set("bare", "false");
    conf.write_to_file(work_tree.join(".git").join("config"));
}

fn main() {
    let matches = App::new("Ted's git implementation")
        .version("0.1.0")
        .author("Ted Bauer <tjb272@cornell.edu>")
        .about("Ted's git implementation")
        .subcommand(
            SubCommand::with_name("init")
                .about("initializes new git repo")
                .arg(
                    Arg::with_name("dir")
                        .help("dir to put repo in")
                        .default_value(".")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();
    cmd_init(&PathBuf::from(
        matches
            .subcommand_matches("init")
            .unwrap()
            .value_of("dir")
            .unwrap(),
    ));
}
