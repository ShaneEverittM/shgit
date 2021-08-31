use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use std::collections::HashSet;
use std::io::{Read, Result as IOResult, Write};
use std::path::PathBuf;

mod author;
mod blob;
mod blob_type;
mod commit;
mod database;
mod entry;
mod tree;
mod workspace;

use author::Author;
use blob::Blob;
use blob_type::BlobLike;
use commit::Commit;
use database::Database;
use entry::Entry;
use std::fs::OpenOptions;
use std::time::SystemTime;
use tree::Tree;
use workspace::Workspace;

fn parse_args() -> ArgMatches<'static> {
    App::new("shgit")
        .version("0.1")
        .author("Shane Murphy <shane.everitt.murphy@gmail.com")
        .subcommand(
            App::new("init").about("Initialize a new repository").arg(
                Arg::with_name("PATH")
                    .help(
                        "Sets path at which to initialize, defaults to current working directory ",
                    )
                    .required(false)
                    .index(1),
            ),
        )
        .subcommand(App::new("commit").about("Commit current changes"))
        .get_matches()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let matches = parse_args();

    maybe_init(&matches)?;
    maybe_commit(&matches)?;

    Ok(())
}

fn maybe_commit(matches: &ArgMatches) -> Result<(), std::io::Error> {
    if matches.subcommand_matches("commit").is_some() {
        let ws = Workspace::in_dir(std::env::current_dir()?);
        let files = ws.list_files()?;
        println!("{:#?}", &files);
        commit(files)?;
    }
    Ok(())
}

fn commit(files: HashSet<PathBuf>) -> IOResult<()> {
    let root_path = std::env::current_dir()?;
    let git_path = root_path.join(".shgit");
    let db_path = git_path.join("objects");

    let workspace = Workspace::in_dir(root_path);
    let database = Database::in_dir(db_path);

    let mut entries = Vec::new();
    for file in files {
        let data = workspace.read_file(&file)?;
        let mut blob = Blob::from(data);

        database.store(&mut blob)?;

        let file_name = file
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        entries.push(Entry::from(file_name, blob.get_oid().to_string()));
    }

    let mut tree = Tree::from(entries);
    database.store(&mut tree)?;

    let name = std::env::var("GIT_AUTHOR_NAME").unwrap();
    let email = std::env::var("GIT_AUTHOR_EMAIL").unwrap();

    let local: DateTime<Local> = Local::now();

    let author = Author::new(
        name,
        email,
        format!("{} {}", local.timestamp(), local.offset()),
    );
    let mut message = String::new();
    std::io::stdin().read_to_string(&mut message)?;

    let mut commit = Commit::new(tree.get_oid().into(), author, message.clone());
    database.store(&mut commit)?;

    let head_path = git_path.join("HEAD");
    let mut file = if head_path.exists() {
        OpenOptions::new().read(true).write(true).open(head_path)?
    } else {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(head_path)?
    };
    file.write_all(commit.get_oid().as_bytes())?;

    println!(
        "[(root-commit) {}] {}",
        commit.get_oid(),
        message.lines().next().unwrap()
    );

    Ok(())
}

fn maybe_init(matches: &ArgMatches) -> Result<(), std::io::Error> {
    if let Some(matches) = matches.subcommand_matches("init") {
        let root_path = get_path(matches)?;
        init(root_path)
    } else {
        Ok(())
    }
}

fn init(root_path: PathBuf) -> Result<(), std::io::Error> {
    let git_path = root_path.join(".shgit");
    ["objects", "refs"]
        .iter()
        .map(|dir| git_path.join(dir))
        .try_for_each(std::fs::create_dir_all)
}

fn get_path(matches: &ArgMatches) -> Result<PathBuf, std::io::Error> {
    let root_path = if let Some(path) = matches.value_of("PATH") {
        // Use user specified value if it exists
        PathBuf::from(path)
    } else {
        // Otherwise use cwd
        std::env::current_dir()?
    };

    if !root_path.exists() {
        std::fs::create_dir_all(&root_path)?;
    }

    root_path.canonicalize()
}
