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
#[serde(untagged)]
enum PackageDefinition {
    Raw(PackageRaw),
    WithManager(PackageWithManager),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct PackagesDefinitions(Vec<PackageDefinition>);

impl PackagesDefinitions {
    pub fn get_from_stdio() -> Self {
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

        dbg!(&stdin_data);

        match serde_json::from_str::<Self>(&stdin_data) {
            Ok(packages_defs) => packages_defs,
            Err(e) => {
                dbg!(e);
                PackagesDefinitions::default()
            }
        }
    }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(_) => String::from("[]"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PackageRaw {
    name: String,
    cmd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PackageWithManager {
    manager: PackageManager,
    #[serde(skip_serializing_if = "Option::is_none")]
    os: Option<String>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    let packages = PackagesDefinitions::get_from_stdio();
    println!("{}", packages.to_json());

    Ok(())
}
