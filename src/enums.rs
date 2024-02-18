use enum_iterator::Sequence;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Sequence)]
pub enum StrategyEnum {
    AlwaysCooperate,
    HoldsGrudge,
    TitForTat,
    TitFor2Tats,
    AlwaysDefect,
}

/// We implement a function to get all the strategies because Rust doesn't actually allow you
/// to iterate over an enum. This is a workaround.
///
/// Why does Rust not allow you to iterate over Enums? Because enums are algebraic data types.
/// For instance, you can't iterate over this Enum in any meaningful way:
///
/// enum Option<T> {
//     None,
//     Some(T)
// }
pub fn get_strategies() -> Vec<StrategyEnum> {
    let mut strategies = Vec::new();

    let mut prev = StrategyEnum::first();

    while let Some(prev_) = prev {
        strategies.push(prev_);
        prev = prev_.next();
    }

    strategies
}
