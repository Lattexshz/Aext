use crate::lock::{CommandLock, ExtensionLock};
use clap::{ArgMatches, ColorChoice, Command};
use std::path::{Path, PathBuf};

mod aext;
mod command;
mod lock;
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RUST_VERSION: &str = env!("CARGO_PKG_RUST_VERSION");

static mut EXTENSIONS: Vec<ExtensionLock> = vec![];
static mut COMMANDS: Vec<CommandLock> = vec![];

fn main() {
    // Find files
    let ext: Vec<PathBuf> = match std::fs::read_dir(Path::new(&format!(
        "{}/extensions",
        std::env::current_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
    ))) {
        Ok(result) => {
            let mut vec: Vec<PathBuf> = vec![];
            for i in result {
                let entry = i.unwrap();
                if Path::new(&entry.file_name())
                    .extension()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    == "toml"
                {
                    vec.push(entry.path());
                }
            }
            vec
        }
        Err(_) => {
            vec![]
        }
    };
    // Do not assign anything to EXTENSION after this
    unsafe {
        let (e, c) = aext::parse_aext(ext);
        EXTENSIONS = e;
        COMMANDS = c;
    }
    // EXTENSIONS are guaranteed to have a value after assignment, so unwrapping is not a problem.

    let mut cmd = Command::new("aext")
        .about("Aext - Hackable build tool")
        .version(VERSION)
        .arg_required_else_help(true)
        .author("Overtime Coder")
        // Sync subcommand
        //
        // Only a few of its arguments are implemented below.
        // .subcommand(
        //     Command::new("build")
        //         .short_flag('B')
        //         .long_flag("build")
        //         .about("Build project."),
        // )
        .subcommand(Command::new("info").about("Show Aext's consider information"))
        .subcommand(Command::new("list").about("List plugins"));

    unsafe {
        for c in COMMANDS.clone() {
            cmd = cmd.subcommand(Command::new(c.name).about(c.description));
        }
    }

    match cmd.get_matches().subcommand() {
        Some(("build", _sync_matches)) => {}
        Some(("info",_sync_matches)) => {
            command::info();
        }
        Some(("list", _sync_matches)) => unsafe {
            command::list();
        },

        _ => {}
    }
    }

