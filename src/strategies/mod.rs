use crate::strategies::always_cooperate::AlwaysCooperate;

pub mod always_cooperate;
pub mod tit_for_tat;

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
