mod tests;
pub mod gameplay;
pub mod simulate;
mod interactive;
mod algorithm;


extern crate clap;
use clap::{App, Arg};

use serde_yaml;


use std::fs::write;
use std::collections::BTreeMap;


struct AlgoEntry {
    name: String,
    func: fn(&gameplay::Board) -> gameplay::MoveDir,
}

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
        
        const NRUNS: i32 = 1000;
        let mut report = BTreeMap::new();

        let tests = vec![
            AlgoEntry{name: "random".to_string(), func: algorithm::random}, 
            AlgoEntry{name: "random_3dir".to_string(), func: algorithm::random_3dir},
            AlgoEntry{name: "max_free_space".to_string(), func: algorithm::max_free_space},
            AlgoEntry{name: "max_free_space_3dir".to_string(), func: algorithm::max_free_space_3dir},
            AlgoEntry{name: "naive_lookahead3".to_string(), func: |board| algorithm::naive_lookahead(board, 3, algorithm::ScoreFunction::FreeSpace)},
            AlgoEntry{name: "naive_lookaheadsorted3".to_string(), func: |board| algorithm::naive_lookahead(board, 3, algorithm::ScoreFunction::FreeSpaceWithSortedness)},
            AlgoEntry{name: "naive_lookahead6".to_string(), func: |board| algorithm::naive_lookahead(board, 6, algorithm::ScoreFunction::FreeSpace)},
            AlgoEntry{name: "naive_lookaheadsorted6".to_string(), func: |board| algorithm::naive_lookahead(board, 6, algorithm::ScoreFunction::FreeSpaceWithSortedness)},
        ];

        for t in tests {
            println!("Running {}...", t.name);
            let results = simulate::bulk(t.func, NRUNS);
            report.insert(t.name, results);
        }

        println!("Done. Writing report");

        let s = serde_yaml::to_string(&report).unwrap();
        write("report.yml", s).unwrap();
    }
}
