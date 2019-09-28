

mod interactive;

extern crate twentysixtyfour;
use twentysixtyfour::{simulate, gameplay, algorithm};

extern crate clap;
use clap::{App, Arg};

extern crate flame;

use serde_yaml;


use std::fs::{write};
use std::collections::BTreeMap;



struct AlgoEntry {
    name: String,
    func: fn(&mut gameplay::GamePlayer, &gameplay::Board) -> gameplay::MoveDir,
}

fn main() {
    let matches = App::new("2064")
        .about("Implements 2064 game and tests play strategies")
        .arg(Arg::with_name("interactive")
            .short("i")
            .long("interactive")
            .help("Play the game interactively")
        )
        .arg(Arg::with_name("flame")
            .short("f")
            .long("flame")
            .help("Run flame profile")
        )
        .get_matches();
    
    if matches.is_present("interactive") {
        println!("Running interactive");
        let algo: fn(&mut gameplay::GamePlayer, &gameplay::Board) -> gameplay::MoveDir = |mut player, board| {algorithm::naive_lookahead(&mut player, board, 5, algorithm::ScoreFunction::FreeSpaceWithSortedness)};
        interactive::run(algo);
    } else if matches.is_present("flame") {
        let board = gameplay::Board{
            values:  [128, 2,  2,  8,
                      256, 8,  16, 8,
                      256, 8,  0,  0,
                      64,  32, 0,  0],
            score: 0
        };
        //flame::start("a");
        let mut player = gameplay::GamePlayer::default();
        loop
        {
            //let _guard = flame::start_guard("naive_lookahead3");
            let _dir = algorithm::naive_lookahead(&mut player, &board, 5, algorithm::ScoreFunction::FreeSpaceWithSortedness);
        }
        // flame::end("a");
        // flame::dump_html(&mut File::create("flame-graph.html").unwrap()).unwrap();
    } else {
        
        const NRUNS: i32 = 200;
        let mut report = BTreeMap::new();

        let tests = vec![
            AlgoEntry{name: "random".to_string(), func: algorithm::random}, 
            AlgoEntry{name: "random_3dir".to_string(), func: algorithm::random_3dir},
            AlgoEntry{name: "max_free_space".to_string(), func: algorithm::max_free_space},
            AlgoEntry{name: "max_free_space_3dir".to_string(), func: algorithm::max_free_space_3dir},
            AlgoEntry{name: "lookahead1".to_string(), func: |mut player, board| algorithm::naive_lookahead(&mut player, board, 1, algorithm::ScoreFunction::FreeSpace)},
            AlgoEntry{name: "lookaheadsorted1".to_string(), func: |mut player, board| algorithm::naive_lookahead(&mut player, board, 1, algorithm::ScoreFunction::FreeSpaceWithSortedness)},
            AlgoEntry{name: "lookahead3".to_string(), func: |mut player, board| algorithm::naive_lookahead(&mut player, board, 3, algorithm::ScoreFunction::FreeSpace)},
            AlgoEntry{name: "lookaheadsorted3".to_string(), func: |mut player, board| algorithm::naive_lookahead(&mut player, board, 3, algorithm::ScoreFunction::FreeSpaceWithSortedness)},
            AlgoEntry{name: "lookahead5".to_string(), func: |mut player, board| algorithm::naive_lookahead(&mut player, board, 5, algorithm::ScoreFunction::FreeSpace)},
            AlgoEntry{name: "lookaheadsorted5".to_string(), func: |mut player, board| algorithm::naive_lookahead(&mut player, board, 5, algorithm::ScoreFunction::FreeSpaceWithSortedness)},
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
