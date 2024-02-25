use rstest::rstest;
use crate::battle;
use crate::strategies::enums::StrategyEnum;

// Define a parameterized test using the `rstest` attribute
#[rstest(iterations, i_expected, j_expected,
case(1, 0, 5),
case(2, 0, 10),
case(3, 1, 11)
)]
fn test_battle_1_iteration(iterations: i64, i_expected: i32, j_expected: i32) {
    let (i_score, j_score) = battle(
        StrategyEnum::TitFor2Tats,
        StrategyEnum::AlwaysDefect,
        iterations,
        false,
    );
    assert_eq!(i_score, i_expected);
    assert_eq!(j_score, j_expected);
}