use std::thread::sleep;
use strategies::enums::{get_strategies, StrategyEnum};
use crate::scoreboard::Scoreboard;
use crate::strategies::{Action, get_strategy};
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::iproduct;
use crate::types::{Parameters, Score};
use log::{info, trace};
use anyhow::{Result};
use crossbeam_channel::{bounded, tick, Receiver, select};
use crossbeam_channel::internal::select;
use ctrlc;

mod scoreboard;
mod strategies;
mod types;
mod logging;
mod tests;

fn main() -> Result<()> {
    logging::init()?;
    let parameters = Parameters::parse();

    // Keep a list of the scores
    let mut board = Scoreboard::default();

    info!("Starting tournament");

    let ctrl = ctrl_channel();

    println!("Starting tournament, sleeping. Please wait...");
    // sleep(std::time::Duration::from_millis(10000));

    // All strategies battle all strategies
    for (i, j) in iproduct!(get_strategies(), get_strategies()) {
        select! {
            recv(ctrl) -> _ => {
                println!("Battle interrupted");
                std::process::exit(0);
            },
            default => {}
        }

        if i == j {
            trace!("Skipping self battle");
            continue;
        }

        let (i_score, j_score) = battle(i, j, parameters.iterations, parameters.verbose.is_present());
        board.add_score(i, i_score);
        board.add_score(j, j_score);
    }

    info!("Tournament finished! Total scores:");

    board.print_scores();

    Ok(())
}


/// Executes battle between two strategies
fn battle(i_enum: StrategyEnum, j_enum: StrategyEnum, iterations: i64, verbose: bool) -> (Score, Score) {

    // Create strategies
    let (mut i, mut j) = (get_strategy(i_enum), get_strategy(j_enum));

    trace!("Starting battle {:?} vs {:?}", i_enum, j_enum);


    let bar = ProgressBar::new(iterations as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}] [{bar:40.red/pink}] {percent}% {msg}").unwrap());

    // Fold over the range of iterations to accumulate scores
    let scores = (0..iterations).fold((0, 0), |(i_score, j_score), _| {
        if verbose {
            bar.inc(1);
            bar.set_message(format!("{:?} vs {:?}", i_enum, j_enum));
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

    if verbose {
        let prefix = format!("{:?} vs {:?}:", i_enum, j_enum);
        if scores.0 > scores.1 {
            bar.finish_with_message(format!("{} {:?} won", prefix, i_enum));
        } else if scores.0 < scores.1 {
            bar.finish_with_message(format!("{} {:?} won", prefix, j_enum));
        } else {
            bar.finish_with_message(format!("{} Draw", prefix));
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

fn ctrl_channel() -> Receiver<()> {
    let (tx, rx) = bounded(1);
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    }).expect("Error setting Ctrl-C handler");
    rx
}