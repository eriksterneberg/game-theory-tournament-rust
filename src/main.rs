use crate::enums::{get_strategies, StrategyEnum};
use crate::scoreboard::Scoreboard;
use crate::strategies::always_cooperate::AlwaysCooperate;
use crate::strategies::always_defect::AlwaysDefect;
use crate::strategies::holds_grudge::HoldsGrudge;
use crate::strategies::tit_for_tat::TitForTat;
use crate::strategies::tit_for_two_tats::TitFor2Tats;
use crate::strategies::{Action, Strategy};
use clap::Parser;
use itertools::iproduct;
use crate::types::{Score, Parameters};

mod enums;
mod scoreboard;
mod strategies;
mod types;


fn main() {
    let parameters = Parameters::parse();

    // Keep a list of the scores
    let mut score_board = Scoreboard::default();

    if parameters.verbose {
        println!("Starting tournament");
    }

    // All strategies battle all strategies, including itself
    for (i, j) in iproduct!(get_strategies(), get_strategies()) {
        let (i_score, j_score) = battle(i, j, parameters);
        score_board.add_score(i, i_score);
        score_board.add_score(j, j_score);
    }

    score_board.print_scores();
}


/// Executes battle between two strategies
fn battle(i_enum: StrategyEnum, j_enum: StrategyEnum, parameters: Parameters) -> (Score, Score) {
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
        (i_score + i_, j_score + j_)
    })
}

/// Scores the actions of two players
///
/// Rules:
/// - If both sides cooperate, they each score 3 points.
/// - If one side defects while the other cooperates, they get 5 and 0 points respectively
/// - If both sides defect, they each score just 1 point.
fn score(action: Action, reaction: Action) -> (Score, Score) {
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
