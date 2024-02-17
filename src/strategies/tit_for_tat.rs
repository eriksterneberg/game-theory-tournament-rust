use crate::strategies::{Action, Strategy};

#[derive(Debug)]

pub struct TitForTat {
    pub retaliate: bool,
}

impl Strategy for TitForTat {
    fn name(&self) -> String {
        String::from("Tit for Tat")
    }

    fn put(&mut self, action: &Action) {
        match action {
            Action::Defect => self.retaliate = true,
            Action::Cooperate => self.retaliate = false,
        }
    }

    fn get(&self) -> Action {
        if self.retaliate {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }

    fn reset(&mut self) {
        self.retaliate = false;
    }
}

impl TitForTat {
    pub fn new() -> Self {
        TitForTat { retaliate: false }
    }
}
