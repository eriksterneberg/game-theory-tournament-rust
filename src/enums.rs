use enum_iterator::Sequence;

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

    loop {
        strategies.push(prev);

        prev = match prev.next() {
            Some(s) => s,
            None => break,
        };
    }

    strategies
}
