mod tests;
pub mod gameplay;
pub mod simulate;


extern crate clap;
use clap::{App, Arg};

mod interactive;
mod algorithm;

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
        
        
        //let results = simulate::bulk(algorithm::random_3dir, 1000);
        let results = simulate::bulk(algorithm::random, 1000);
        
        println!("Results: ");
        println!("cdf x: {:?}", results.score_cdf_x);
        println!("cdf y: {:?}", results.score_cdf_y);
    }
}
