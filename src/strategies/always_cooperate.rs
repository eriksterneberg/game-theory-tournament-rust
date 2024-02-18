use crate::strategies::{Action, Strategy};

#[derive(Debug)]

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
