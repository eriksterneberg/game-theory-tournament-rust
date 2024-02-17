use crate::strategies::always_cooperate::AlwaysCooperate;

pub mod always_cooperate;
pub mod always_defect;
pub mod tit_for_tat;
pub mod tit_for_two_tats;

#[derive(Debug)]

pub enum Action {
    Cooperate,
    Defect,
}

pub trait Strategy {
    fn name(&self) -> String;
    fn put(&mut self, action: &Action);
    fn get(&self) -> Action;
}
