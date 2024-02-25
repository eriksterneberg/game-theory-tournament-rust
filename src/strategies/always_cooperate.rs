use crate::strategies::{Action, Strategy};

#[derive(Debug)]
/// No matter what the other player does, this strategy will always cooperate
pub struct AlwaysCooperate {}

impl Strategy for AlwaysCooperate {
    fn put(&mut self, _: &Action) {}

    fn get(&self) -> Action {
        Action::Cooperate
    }
}

impl AlwaysCooperate {
    pub fn new() -> Self {
        Self {}
    }
}
