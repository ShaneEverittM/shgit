use std::path::PathBuf;

use clap::{App, Arg, ArgMatches};

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
        .get_matches()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let matches = parse_args();

    // Only argument is subcommand init, parse its arguments
    maybe_init(&matches)?;

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
    let git_path = root_path.join(".git");
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
