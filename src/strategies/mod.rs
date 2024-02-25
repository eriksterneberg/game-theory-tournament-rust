use crate::strategies::always_cooperate::AlwaysCooperate;
use crate::strategies::always_defect::AlwaysDefect;
use crate::strategies::enums::StrategyEnum;
use crate::strategies::holds_grudge::HoldsGrudge;
use crate::strategies::tit_for_tat::TitForTat;
use crate::strategies::tit_for_two_tats::TitFor2Tats;

pub mod always_cooperate;
pub mod always_defect;
pub mod holds_grudge;
pub mod tit_for_tat;
pub mod tit_for_two_tats;
pub mod enums;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Cooperate,
    Defect,
}

pub trait Strategy {
    // Here we will tell the strategy what the previous action taken against was
    fn put(&mut self, action: &Action);

    // Based on the past actions taken against the strategy, return a new action
    fn get(&self) -> Action;
}


/// Returns a new strategy based on the enum
///
/// # Note
/// This function must return a Box<dyn Strategy> because the strategies are of different types
/// and we want to return a trait object.
pub fn get_strategy(strategy_enum: StrategyEnum) -> Box<dyn Strategy> {
    match strategy_enum {
        StrategyEnum::AlwaysCooperate => Box::new(AlwaysCooperate::new()),
        StrategyEnum::HoldsGrudge => Box::new(HoldsGrudge::new()),
        StrategyEnum::TitForTat => Box::new(TitForTat::new()),
        StrategyEnum::TitFor2Tats => Box::new(TitFor2Tats::new()),
        StrategyEnum::AlwaysDefect => Box::new(AlwaysDefect::new()),
    }
}
