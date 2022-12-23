extern crate clap;
extern crate ini;

use clap::{App, Arg, SubCommand};
use ini::Ini;
use sha1::{Digest, Sha1};
use std::ffi::OsStr;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

struct GitRepository {
    work_tree: PathBuf,
    git_dir: PathBuf,
    conf: PathBuf,
}

struct GitCommit {}
struct GitTree {}
struct GitTag {}
struct GitBlob {
    content: String,
}

enum GitObject {
    GitCommit(GitCommit),
    GitTree(GitTree),
    GitTag(GitTag),
    GitBlob(GitBlob),
}

impl GitObject {
    fn serialize(&self) -> String {
        match self {
            Self::GitBlob(b) => b.content.clone(),
            _ => panic!("unimplemented"),
        }
    }

    fn deserialize(data: String) -> Self {
        todo!()
    }

    fn type_header(&self) -> String {
        match self {
            Self::GitCommit(_) => "commit".to_string(),
            Self::GitTree(_) => "tree".to_string(),
            Self::GitTag(_) => "tag".to_string(),
            Self::GitBlob(_) => "blob".to_string(),
        }
    }
}

const DEFAULT_REPO_DESCRIPTION: &str =
    "Unnamed repository; edit this file 'description' to name the repository.\n";
const DEFAULT_HEAD: &str = "ref: refs/heads/master\n";

fn cmd_cat_file(type_: &str, object_sha: &str) {
    let object_content = fs::read_to_string(
        PathBuf::from(".git")
            .join("objects")
            .join(object_sha[..2].to_string())
            .join(object_sha[2..].to_string()),
    )
    .unwrap();

    println!("{}", object_content);
}

fn object_write(object: &GitObject) {
    let object_data = object.serialize();
    let object_data_with_header = format!(
        "{} {}.{}",
        object.type_header(),
        object_data.len(),
        object_data
    );

    let mut hasher = Sha1::new();
    hasher.update(&object_data_with_header);
    let sha = format!("{:x}", hasher.finalize());

    let path = PathBuf::from(".")
        .join(".git")
        .join("objects")
        .join(&sha[..2].to_string());

    fs::create_dir_all(&path);
    // TODO: content should be compressed
    fs::write(path.join(&sha[2..]), object_data_with_header);
}

fn cmd_hash_object(path: &PathBuf, type_: &str) {
    let file_data = fs::read_to_string(path).expect("couldn't open file");
    let object = match type_ {
        "commit" => GitObject::GitCommit(GitCommit {}),
        "tree" => GitObject::GitTree(GitTree {}),
        "tag" => GitObject::GitTag(GitTag {}),
        "blob" => GitObject::GitBlob(GitBlob { content: file_data }),
        _ => panic!("ugh handle this later"),
    };
    object_write(&object);
}

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
        .subcommand(SubCommand::with_name("hash-object").args(&vec![
            Arg::with_name("type"),
            Arg::with_name("write"),
            Arg::with_name("path"),
        ]))
        .subcommand(
            SubCommand::with_name("cat-file")
                .args(&vec![Arg::with_name("type"), Arg::with_name("object")]),
        )
        .get_matches();

    match matches.subcommand_name().as_deref() {
        Some("init") => cmd_init(&PathBuf::from(
            matches
                .subcommand_matches("init")
                .unwrap()
                .value_of("dir")
                .unwrap(),
        )),
        Some("hash-object") => cmd_hash_object(
            &PathBuf::from(
                matches
                    .subcommand_matches("hash-object")
                    .unwrap()
                    .value_of("path")
                    .unwrap(),
            ),
            matches
                .subcommand_matches("hash-object")
                .unwrap()
                .value_of("type")
                .unwrap(),
        ),
        Some("cat-file") => cmd_cat_file(
            matches
                .subcommand_matches("cat-file")
                .unwrap()
                .value_of("type")
                .unwrap(),
            matches
                .subcommand_matches("cat-file")
                .unwrap()
                .value_of("object")
                .unwrap(),
        ),
        Some(_) => println!("Command not recognized"),
        None => println!("No command supplied."),
    }
}
