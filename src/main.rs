use clap::Command;
mod aext;
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let aexts = find_aexts();
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
        .get_matches();

    match matches.subcommand() {
        Some(("build", sync_matches)) => {

        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}

fn find_aexts() -> Vec<aext::Aext> {
    let mut aexts:Vec<aext::Aext> = vec!();
    println!("{} Aext scripts found.",aexts.len());
    aexts
}