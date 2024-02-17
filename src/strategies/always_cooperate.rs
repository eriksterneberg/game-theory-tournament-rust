use crate::strategies::{Action, Strategy};

#[derive(Debug)]

pub struct AlwaysCooperate {}

impl Strategy for AlwaysCooperate {
    fn name(&self) -> String {
        String::from("Always Cooperate")
    }
    fn put(&mut self, _: &Action) {}

    fn get(&self) -> Action {
        Action::Cooperate
    }

    fn reset(&mut self) {}
}

impl AlwaysCooperate {
    pub fn new() -> Self {
        Self {}
    }
}
