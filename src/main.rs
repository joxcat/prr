use std::io;

use clap::StructOpt;
use cli::{Commands, DefinePackageArgs};
use serde::{Deserialize, Serialize};

mod cli;

pub type EyreResult<Output> = color_eyre::eyre::Result<Output>;

fn setup_logging(verbosity: usize) -> EyreResult<()> {
    if verbosity > 0 {
        color_eyre::install().unwrap();
    }

    Ok(())
}

fn main() -> EyreResult<()> {
    match Commands::parse() {
        Commands::DefinePackage(args) => handle_define_package(args)?,
        _ => unimplemented!(),
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum PackageDefinition {
    Raw(PackageRaw),
    WithManager(PackageWithManager),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PackageRaw {
    name: String,
    cmd: String,
    os: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PackageWithManager {
    manager: PackageManager,
    os: String,
    name: String,
    version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum PackageManager {
    OsxBrew,
    LinuxPacman,
    LinuxYay,
    LinuxApt,
}

fn handle_define_package(args: DefinePackageArgs) -> EyreResult<()> {
    setup_logging(args.verbose)?;

    let previous_packages_definitions = if args.stdio {
        let stdin = io::stdin();
        let mut stdin_data = String::new();

        let mut line = String::new();
        while let Ok(nb_bytes) = stdin.read_line(&mut line) {
            if nb_bytes == 0 {
                break;
            };
            stdin_data.push_str(&line);
            line.clear();
        }

        stdin_data
    } else {
        String::new()
    };

    dbg!(previous_packages_definitions);

    Ok(())
}
