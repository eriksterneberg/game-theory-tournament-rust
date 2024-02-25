use std::thread::sleep;
use strategies::enums::{get_strategies, StrategyEnum};
use crate::scoreboard::Scoreboard;
use crate::strategies::{Action, get_strategy};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::iproduct;
use crate::types::{Parameters, Score};
use log::{info, debug, warn, trace};

mod scoreboard;
mod strategies;
mod types;


fn main() {
    let parameters = Parameters::parse();

    if parameters.verbose.is_present() {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Trace)
            .init();
    } else {
        pretty_env_logger::formatted_builder()
            .filter_level(log::LevelFilter::Info)
            .init();
    }

    // Keep a list of the scores
    let mut board = Scoreboard::default();

    info!("Starting tournament");

    // All strategies battle all strategies, including itself
    for (i, j) in iproduct!(get_strategies(), get_strategies()) {
        if i == j {
            continue;
        }

        let (i_score, j_score) = battle(i, j, &parameters);
        board.add_score(i, i_score);
        board.add_score(j, j_score);
    }

    info!("Tournament finished! Total scores:");

    board.print_scores();
}


/// Executes battle between two strategies
fn battle(i_enum: StrategyEnum, j_enum: StrategyEnum, parameters: &Parameters) -> (Score, Score) {
    trace!("Executing battle: {:?} vs {:?}", i_enum, j_enum);

    // Create strategies
    let (mut i, mut j) = (get_strategy(i_enum), get_strategy(j_enum));

    let bar = ProgressBar::new(parameters.iterations as u64);
    bar.set_message(format!("{:?} vs {:?}", i_enum, j_enum));
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}] [{bar:40.red/pink}] {percent}% {msg}").unwrap());

    // Fold over the range of iterations to accumulate scores
    let scores = (0..parameters.iterations).fold((0, 0), |(i_score, j_score), _| {
        if parameters.verbose.is_present() {
            bar.inc(1);
            sleep(std::time::Duration::from_millis(1));
        }

        // Players make moves independently
        let (action_i, action_j) = (i.get(), j.get());

        // Players learn from the previous round
        j.put(&action_i);
        i.put(&action_j);

        // Score the round
        let (i_, j_) = score(action_i, action_j);

        // Return the updated scores
        (i_score + i_, j_score + j_)
    });

    if parameters.verbose.is_present() {
        if scores.0 > scores.1 {
            bar.finish_with_message(format!("{:?} won", i_enum));
        } else if scores.0 < scores.1 {
            bar.finish_with_message(format!("{:?} won", j_enum));
        } else {
            bar.finish_with_message("Draw".to_string());
        }
    }

    scores
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
