use crate::strategies::always_cooperate::AlwaysCooperate;
use crate::strategies::always_defect::AlwaysDefect;
use crate::strategies::holds_grudge::HoldsGrudge;
use crate::strategies::tit_for_tat::TitForTat;
use crate::strategies::tit_for_two_tats::TitFor2Tats;
use crate::strategies::{Action, Strategy};
use std::collections::HashMap;
use std::env;

mod strategies;

fn main() {
    let parameters = parse_args();

    // Keep a list of the scores
    let mut scores: HashMap<String, i32> = HashMap::new();

    println!("Starting tournament");

    // Get two lists of strategies to iterate over. All strategies battle all strategies, including self.
    for i in &mut get_strategies() {
        for j in &mut get_strategies() {
            let (i_score, j_score) = battle(i, j, &parameters);
            add_score(&mut scores, i.name(), i_score);
            add_score(&mut scores, j.name(), j_score);
        }
    }

    println!("Tournament finished");

    print_scores(&scores);
}

struct Parameters {
    iterations: i32,
    verbose: bool,
}

// Parse command line parameter --iterations
fn parse_args() -> Parameters {
    let args: Vec<String> = env::args().collect();
    let mut iterations = 20; // Default value
    let mut verbose = false;

    for (index, arg) in args.iter().enumerate() {
        if arg == "--iterations" {
            if let Some(value) = args.get(index + 1) {
                iterations = value.parse().unwrap_or_else(|_| {
                    eprintln!("Error: Invalid value for iterations");
                    std::process::exit(1);
                });
            } else {
                eprintln!("Error: Missing value for --iterations");
                std::process::exit(1);
            }
        }

        if arg == "--verbose" {
            verbose = true;
        }
    }
    Parameters {
        iterations,
        verbose,
    }
}

/// Executes battle between two strategies
///
/// If both sides cooperate, they each score 3 points.
/// If one side defects while the other cooperates, they get 5 and 0 points respectively
/// If both sides defect, they each score just 1 point.
fn battle(
    i: &mut Box<dyn Strategy>,
    j: &mut Box<dyn Strategy>,
    parameters: &Parameters,
) -> (i32, i32) {
    // Reset the strategies for the next round
    i.reset();
    j.reset();

    if parameters.verbose {
        println!("Executing battle: {} vs {}", i.name(), j.name());
    }

    let mut i_score = 0;
    let mut j_score = 0;

    for _ in 0..parameters.iterations {
        let action = i.get();
        let reaction = j.get();
        i.put(&reaction);
        j.put(&action);

        match (action, reaction) {
            (Action::Cooperate, Action::Cooperate) => {
                if parameters.verbose {
                    println!("Both players cooperated! Will assign both 3 points each.");
                }
                i_score += 3;
                j_score += 3;
            }
            (Action::Defect, Action::Defect) => {
                if parameters.verbose {
                    println!("Both players defected! Will assign both 1 point each.");
                }
                i_score += 1;
                j_score += 1;
            }
            (Action::Cooperate, Action::Defect) => {
                if parameters.verbose {
                    println!("One defected and gets 5, one cooperated and gets zero");
                }
                j_score += 5;
            }
            (Action::Defect, Action::Cooperate) => {
                if parameters.verbose {
                    println!("One defected and gets 5, one cooperated and gets zero");
                }
                i_score += 5;
            }
        }
    }

    return (i_score, j_score);
}

/// Get all strategies
///
/// It is not possible to copy an array, so instead we return a vector of boxes.
/// Why boxes? Because we want to return a vector of different types,
/// and the only way to do that is to use a trait object.
///
/// Copilot says:
/// A trait object is a pointer to a trait, which can be used to call the methods of the trait.
/// In this case, we are returning a vector of boxes that contain a trait object.
/// This means that we can call the methods of the trait on the objects in the vector.
/// The trait object is a pointer to the trait, and the box is a pointer to the object.
/// This is the only way to return a vector of different types.
fn get_strategies() -> Vec<Box<dyn Strategy>> {
    vec![
        Box::new(AlwaysCooperate::new()),
        Box::new(HoldsGrudge::new()),
        Box::new(TitForTat::new()),
        Box::new(TitFor2Tats::new()),
        Box::new(AlwaysDefect::new()),
    ]
}

/// 顾名思义 -- As the name suggests
fn add_score(scores: &mut HashMap<String, i32>, player: String, score: i32) {
    let entry = scores.entry(String::from(player)).or_insert(0);
    *entry += score;
}

/// 顾名思义 -- As the name suggests
fn print_scores(scores: &HashMap<String, i32>) {
    let mut scores: Vec<_> = scores.into_iter().collect();
    scores.sort_by(|a, b| b.1.cmp(&a.1));

    for (player, score) in &scores {
        println!("{}\t{}", score, player);
    }
}
