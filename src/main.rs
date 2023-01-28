use clap::Command;
use std::path::{Path, PathBuf};
mod aext;
const VERSION: &str = env!("CARGO_PKG_VERSION");

static mut EXTENSIONS: Vec<aext::Aext> = vec![];

fn main() {
    // Find files
    let ext:Vec<PathBuf> = match std::fs::read_dir(Path::new(&format!("{}/extensions",std::env::current_dir().unwrap().as_os_str().to_str().unwrap()))) {
        Ok(result) => {
            println!("OK");
            let mut vec:Vec<PathBuf> = vec!();
            for i in result {
                let entry = i.unwrap();
                println!("ext:{}",Path::new(&entry.file_name()).extension().unwrap().to_str().unwrap());
                if Path::new(&entry.file_name()).extension().unwrap().to_str().unwrap() == "toml" {
                    println!("{}",entry.path().as_os_str().to_str().unwrap());
                    vec.push(entry.path());
                }
            }
            vec
        }
        Err(_) => {
            vec!()
        }
    };
    // Do not assign anything to EXTENSION after this
    unsafe { EXTENSIONS = find_aexts(ext); }
    // EXTENSIONS are guaranteed to have a value after assignment, so unwrapping is not a problem.

    let matches = Command::new("aext")
        .about("Aext - Hackable build tool")
        .version(VERSION)
        .subcommand_required(true)
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
        .subcommand(Command::new("list")
            .about("List plugins"))
        .get_matches();

    match matches.subcommand() {
        Some(("build", _sync_matches)) => {}
        Some(("list",_sync_matches)) => unsafe {
            println!("{} Aext scripts loaded.\n",EXTENSIONS.len());
            for e in EXTENSIONS.clone() {
                let plugin = e.plugin.unwrap();
                println!("Name:{} Version:{}",plugin.name.unwrap(),plugin.version.unwrap());
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}

fn find_aexts(path: Vec<PathBuf>) -> Vec<aext::Aext> {
    let aexts: Vec<aext::Aext> = aext::parse_aext(path);
    aexts
}
