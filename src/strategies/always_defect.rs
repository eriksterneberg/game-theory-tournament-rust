use crate::strategies::{Action, Strategy};

#[derive(Debug)]
/// No matter what the other player does, this strategy will always defect
pub struct AlwaysDefect {}

impl Strategy for AlwaysDefect {
    fn put(&mut self, _: &Action) {}

    fn get(&self) -> Action {
        Action::Defect
    }
}

impl AlwaysDefect {
    pub fn new() -> Self {
        Self {}
    }
}
