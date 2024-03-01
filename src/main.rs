use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
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
use crossbeam_channel::{bounded, Receiver, select};
use ctrlc;

mod scoreboard;
mod strategies;
mod types;
mod logging;
mod tests;

fn main() -> Result<()> {
    logging::init()?;
    let parameters = Parameters::parse();
    let running = get_cancel();
    let mut board = Scoreboard::default();

    info!("Starting tournament");

    // All strategies battle all strategies
    for (i, j) in iproduct!(get_strategies(), get_strategies()) {
        check_running(&running)?;

        if i == j {
            trace!("Skipping self battle");
            continue;
        }

        let (i_score, j_score) = battle(Some(running.clone()), i, j, parameters.iterations, parameters.verbose.is_present());
        board.add_score(i, i_score);
        board.add_score(j, j_score);
    }

    info!("Tournament finished! Total scores:");

    board.print_scores();

    Ok(())
}


/// Executes battle between two strategies
fn battle(running: Option<Arc<AtomicBool>>, i_enum: StrategyEnum, j_enum: StrategyEnum, iterations: i64, verbose: bool) -> (Score, Score) {

    // Create strategies
    let (mut i, mut j) = (get_strategy(i_enum), get_strategy(j_enum));

    trace!("Starting battle {:?} vs {:?}", i_enum, j_enum);

    let bar = ProgressBar::new(iterations as u64);
    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}] [{bar:40.red/pink}] {percent}% {msg}").unwrap());

    // Fold over the range of iterations to accumulate scores
    let (mut i_score, mut j_score) = (0, 0);

    for _ in 0..iterations {
        match running {
            Some(ref running) => {
                match check_running(&running) {
                    Ok(_) => {}
                    Err(_) => {
                        trace!("Cancelled in mid-battle");
                        return (i_score, j_score);
                    }
                }
            }
            None => {}
        }

        if verbose {
            bar.inc(1);
            bar.set_message(format!("{:?} vs {:?}", i_enum, j_enum));
            sleep(std::time::Duration::from_millis(1));
        }

        // Players make moves independently
        let (action, reaction) = (i.get(), j.get());

        // Players learn from the previous round
        j.put(&action);
        i.put(&reaction);

        // Score the round
        let (i_, j_) = score(action, reaction);

        i_score += i_;
        j_score += j_;
    }

    if verbose {
        let prefix = format!("{:?} vs {:?}:", i_enum, j_enum);
        if i_score > j_score {
            bar.finish_with_message(format!("{} {:?} won", prefix, i_enum));
        } else if i_score < j_score {
            bar.finish_with_message(format!("{} {:?} won", prefix, j_enum));
        } else {
            bar.finish_with_message(format!("{} Draw", prefix));
        }
    }

    (i_score, j_score)
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

/// Creates a cancellation flag that can be used to gracefully terminate the program in response to a SIGINT signal (Ctrl+C).
///
/// This function sets up a signal handler for SIGINT signals using the `ctrlc` crate. When a SIGINT signal is received,
/// the `running` flag is set to `false`, indicating that the program should terminate gracefully.
///
/// # Returns
///
/// * `Arc<AtomicBool>` - Returns an `Arc<AtomicBool>` representing the cancellation flag.
fn get_cancel() -> Arc<AtomicBool> {
    let (tx, _rx) = bounded(1);
    let running = Arc::new(AtomicBool::new(true));
    let running_ = running.clone();
    ctrlc::set_handler(move || {
        running_.store(false, Ordering::SeqCst);
        let _ = tx.send(());
    }).expect("Error setting Ctrl-C handler");
    running
}

/// Checks whether the program is still running based on the value of the `running` flag.
///
/// This function takes a reference to an `Arc<AtomicBool>` named `running` as its argument.
/// The `running` flag indicates whether the program should continue running.
///
/// # Arguments
///
/// * `running` - A reference to an `Arc<AtomicBool>` that represents the running flag.
///
/// # Returns
///
/// * `Result<()>` - Returns `Ok(())` if the program is still running (i.e., the `running` flag is `true`).
///                  Returns `Err` with a message "Cancelled" if the program has been cancelled (i.e., the `running` flag is `false`).
///
fn check_running(running: &Arc<AtomicBool>) -> Result<()> {
    if !running.load(Ordering::SeqCst) {
        Err(anyhow::anyhow!("Cancelled"))
    } else {
        Ok(())
    }
}