use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Sequence)]
pub enum StrategyEnum {
    AlwaysCooperate,
    HoldsGrudge,
    TitForTat,
    TitFor2Tats,
    AlwaysDefect,
}

pub fn get_strategies() -> Vec<StrategyEnum> {
    let mut strategies = Vec::new();

    let mut prev = StrategyEnum::first().unwrap();

    println!("First: {:?}", prev);
    println!("Last: {:?}", StrategyEnum::last().unwrap());

    // while let Some(next) = prev.next() {
    while true {
        strategies.push(prev);
        println!("Pushed {:?}", prev);

        if prev == StrategyEnum::last().unwrap() {
            break;
        }

        prev = prev.next().unwrap();
    }

    strategies
}
