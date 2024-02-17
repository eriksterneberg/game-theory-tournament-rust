use crate::strategies::{Action, Strategy};

#[derive(Debug)]

pub struct AlwaysDefect {}

impl Strategy for AlwaysDefect {
    fn name(&self) -> String {
        String::from("Always Defect")
    }
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
