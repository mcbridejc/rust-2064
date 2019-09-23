mod tests;
pub mod gameplay;

extern crate clap;
use clap::{App, Arg};

mod interactive;

fn main() {
    let matches = App::new("2064")
        .about("Implements 2064 game and tests play strategies")
        .arg(Arg::with_name("interactive")
            .short("i")
            .long("interactive")
            .help("Play the game interactively")
        ).get_matches();
    
    if matches.is_present("interactive") {
        println!("Running interactive");
        interactive::run();
    } else {
        println!("Unsupported options");
    }
}
