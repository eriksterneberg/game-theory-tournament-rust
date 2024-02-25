use strategies::enums::{get_strategies, StrategyEnum};
use crate::scoreboard::Scoreboard;
use crate::strategies::{Action, get_strategy, Strategy};
use clap::Parser;
use itertools::iproduct;
use crate::types::{Parameters, Score};

mod scoreboard;
mod strategies;
mod types;


fn main() {
    let parameters = Parameters::parse();

    // Keep a list of the scores
    let mut board = Scoreboard::default();

    if parameters.verbose {
        println!("Starting tournament");
    }

    // All strategies battle all strategies, including itself
    for (i, j) in iproduct!(get_strategies(), get_strategies()) {
        if i == j {
            continue;
        }

        let (i_score, j_score) = battle(i, j, parameters);
        board.add_score(i, i_score);
        board.add_score(j, j_score);
    }

    if parameters.verbose {
        println!("Tournament finished");
    }

    board.print_scores();
}


/// Executes battle between two strategies
fn battle(i_enum: StrategyEnum, j_enum: StrategyEnum, parameters: Parameters) -> (Score, Score) {
    if parameters.verbose {
        println!("Executing battle: {:?} vs {:?}", i_enum, j_enum);
    }

    // Create strategies
    let (mut i, mut j) = (get_strategy(i_enum), get_strategy(j_enum));

    // Fold over the range of iterations to accumulate scores
    (0..parameters.iterations).fold((0, 0), |(i_score, j_score), _| {

        // Players make moves independently
        let (action_i, action_j) = (i.get(), j.get());

        // Players learn from the previous round
        j.put(&action_i);
        i.put(&action_j);

        // Score the round
        let (i_, j_) = score(action_i, action_j);

        // Return the updated scores
        (i_score + i_, j_score + j_)
    })
}

/// Scores the actions of two players
fn score(action: Action, reaction: Action) -> (Score, Score) {
    match (action, reaction) {
        (Action::Cooperate, Action::Cooperate) => (3, 3), // both players cooperate
        (Action::Defect, Action::Defect) => (1, 1), // both players defect
        (Action::Cooperate, Action::Defect) => (0, 5), // player 1 cooperates, player 2 defects
        (Action::Defect, Action::Cooperate) => (5, 0), // player 1 defects, player 2 cooperates
    }
}
