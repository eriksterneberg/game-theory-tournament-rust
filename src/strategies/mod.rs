pub mod always_cooperate;
pub mod always_defect;
pub mod holds_grudge;
pub mod tit_for_tat;
pub mod tit_for_two_tats;

#[derive(Debug)]

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
