use std::process::Command;
use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use predicates::prelude::predicate;
use rstest::rstest;
use crate::battle;
use crate::strategies::enums::StrategyEnum::{AlwaysDefect, TitFor2Tats, TitForTat};

/// Test the battle function with the AlwaysDefect strategy vs the TitFor2Tats strategy.
///
/// TitFor2Tats should learn to defect after the second iteration, and score one point in the third iteration
/// but not before then.
#[rstest(iterations, i_expected, j_expected, case(1, 0, 5), case(2, 0, 10), case(3, 1, 11))]
fn test_battle_tit_for_2_tats_learns_slowly(iterations: i64, i_expected: i32, j_expected: i32) {
    assert_eq!(battle(None, TitFor2Tats, AlwaysDefect, iterations, false), (i_expected, j_expected));
}

/// Test that the strategies TitForTat and TitFor2Tats always start to cooperate and continue
/// to do so until the opponent defects.
#[rstest(iterations, i, j, case(1, 3, 3), case(10, 30, 30))]
fn test_two_positive_strategies_always_cooperate(iterations: i64, i: i32, j: i32) {
    let expected = (i, j);
    let results = battle(None, TitForTat, TitFor2Tats, iterations, false);
    assert_eq!(results, expected);
}

/// Test the binary runs without error
///
/// Currently, the binary writes logs to STDERR. If that is changed, this test needs to be updated.
#[test]
fn run_binary() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("gamett")?;
    cmd.assert()
        .success()
        .stderr(predicate::str::contains("Tournament finished"));

    Ok(())
}