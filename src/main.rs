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
    let mut scores: HashMap<StrategyEnum, i32> = HashMap::new();

    println!("Starting tournament");

    // Get two lists of strategies to iterate over. All strategies battle all strategies, including self.
    // Use iterator instead of function that returns vector for memory efficiency
    for i in StrategyEnum::iter() {
        for j in StrategyEnum::iter() {
            let (i_score, j_score) = battle(i, j, parameters);

            *(scores.entry(i).or_insert(0)) += i_score;
            *(scores.entry(j).or_insert(0)) += j_score;
        }
    }

    println!("Tournament finished");

    print_scores(&scores);
}
#[derive(Clone, Copy)]
struct Parameters {
    iterations: i32,
    verbose: bool,
}

/// Parse command line parameter --iterations and --verbose
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
fn battle(i_enum: StrategyEnum, j_enum: StrategyEnum, parameters: Parameters) -> (i32, i32) {
    if parameters.verbose {
        println!("Executing battle: {:?} vs {:?}", i_enum, j_enum);
    }

    // Create strategies
    let mut i = get_strategy(i_enum);
    let mut j = get_strategy(j_enum);

    // Keep scor
    // Fold over the range of iterations to accumulate scores
    (0..parameters.iterations).fold((0, 0), |(i_score, j_score), _| {
        // First player makes a move
        let action_i = i.get();
        j.put(&action_i);

        // Second player makes a move
        let action_j = j.get();
        i.put(&action_j);

        // Score the round
        let (i_, j_) = score(action_i, action_j);

        // Return the updated scores
        (i_score + i_, j_score + j_)
    })
}

/// Scores the actions of two players
///
/// Rules:
/// - If both sides cooperate, they each score 3 points.
/// - If one side defects while the other cooperates, they get 5 and 0 points respectively
/// - If both sides defect, they each score just 1 point.
fn score(action: Action, reaction: Action) -> (i32, i32) {
    match (action, reaction) {
        (Action::Cooperate, Action::Cooperate) | (Action::Defect, Action::Defect) => {
            let points = if action == Action::Cooperate { 3 } else { 1 };
            (points, points)
        }
        (_, _) => {
            let (defector, cooperator) = if action == Action::Defect {
                (5, 0)
            } else {
                (0, 5)
            };
            (defector, cooperator)
        }
    }
}

/// 顾名思义 -- As the name suggests
fn print_scores(scores: &HashMap<StrategyEnum, i32>) {
    let mut scores: Vec<_> = scores.into_iter().collect();
    scores.sort_by(|a, b| b.1.cmp(&a.1));

    for (player, score) in &scores {
        println!("{}\t{:?}", score, player);
    }
}

/// An iterator over the strategies
///
/// Unfortunately, we can't use the derive macro to implement the iterator trait for the enum,
/// so we have to implement it manually.
impl StrategyEnum {
    pub fn iter() -> impl Iterator<Item = StrategyEnum> {
        let variants = [
            StrategyEnum::AlwaysCooperate,
            StrategyEnum::HoldsGrudge,
            StrategyEnum::TitForTat,
            StrategyEnum::TitFor2Tats,
            StrategyEnum::AlwaysDefect,
        ];
        variants.into_iter()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum StrategyEnum {
    AlwaysCooperate,
    HoldsGrudge,
    TitForTat,
    TitFor2Tats,
    AlwaysDefect,
}

/// Returns a new strategy based on the enum
///
/// # Arguments
///     * `strategy_enum` - The enum of the strategy to be returned
///
/// # Returns
///     * A new strategy based on the enum
///
/// # Example
/// ```
/// let strategy = get_strategy(StrategyEnum::AlwaysCooperate);
/// ```
///
/// # Note
/// This function must return a Box<dyn Strategy> because the strategies are of different types
/// and we want to return a trait object.
fn get_strategy(strategy_enum: StrategyEnum) -> Box<dyn Strategy> {
    match strategy_enum {
        StrategyEnum::AlwaysCooperate => Box::new(AlwaysCooperate::new()),
        StrategyEnum::HoldsGrudge => Box::new(HoldsGrudge::new()),
        StrategyEnum::TitForTat => Box::new(TitForTat::new()),
        StrategyEnum::TitFor2Tats => Box::new(TitFor2Tats::new()),
        StrategyEnum::AlwaysDefect => Box::new(AlwaysDefect::new()),
    }
}
