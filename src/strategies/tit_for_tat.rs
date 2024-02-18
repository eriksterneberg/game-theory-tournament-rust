use crate::strategies::{Action, Strategy};

#[derive(Debug)]

pub struct TitForTat {
    pub retaliate: bool,
}

impl Strategy for TitForTat {
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
}

impl TitForTat {
    pub fn new() -> Self {
        TitForTat { retaliate: false }
    }
}
