use crate::enums::{get_strategies, StrategyEnum};
use crate::scoreboard::Scoreboard;
use crate::strategies::always_cooperate::AlwaysCooperate;
use crate::strategies::always_defect::AlwaysDefect;
use crate::strategies::holds_grudge::HoldsGrudge;
use crate::strategies::tit_for_tat::TitForTat;
use crate::strategies::tit_for_two_tats::TitFor2Tats;
use crate::strategies::{Action, Strategy};
use itertools::iproduct;
use std::env;

mod enums;
mod scoreboard;
mod strategies;

fn main() {
    let parameters = parse_args();

    println!("Strategies: {:?}", get_strategies());

    let strategies = get_strategies();
    let strategies2 = get_strategies();

    // Keep a list of the scores
    let mut score_board = Scoreboard::default();

    println!("Starting tournament");

    // All strategies battle all strategies, including itself
    for (i, j) in iproduct!(strategies, strategies2) {
        let (i_score, j_score) = battle(i, j, parameters);

        score_board.add_score(i, i_score);
        score_board.add_score(j, j_score);
    }

    score_board.print_scores();
}
#[derive(Clone, Copy)]
struct Parameters {
    iterations: i32,
    verbose: bool,
}

/// Executes battle between two strategies
fn battle(i_enum: StrategyEnum, j_enum: StrategyEnum, parameters: Parameters) -> (i32, i32) {
    if parameters.verbose {
        println!("Executing battle: {:?} vs {:?}", i_enum, j_enum);
    }

    // Create strategies
    let (mut i, mut j) = (get_strategy(i_enum), get_strategy(j_enum));

    // Keep score
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
        ((i_score + i_), (j_score + j_))
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
        (Action::Cooperate, Action::Cooperate) => (3, 3),
        (Action::Defect, Action::Defect) => (1, 1),
        (Action::Cooperate, Action::Defect) => (0, 5),
        (Action::Defect, Action::Cooperate) => (5, 0),
    }
}

/// Returns a new strategy based on the enum
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

/// Parse command line parameter --iterations and --verbose
fn parse_args() -> Parameters {
    let mut args = env::args().peekable();
    let mut parameters = Parameters {
        iterations: 20,
        verbose: false,
    };

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--iterations" => {
                parameters.iterations =
                    args.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
                        eprintln!("Error: Invalid value for iterations");
                        std::process::exit(1);
                    });
            }
            "--verbose" => parameters.verbose = true,
            _ => (),
        }
    }
    parameters
}
