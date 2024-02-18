use crate::strategies::{Action, Strategy};

#[derive(Debug)]

/// When the opponent cooperates, cooperate. When the opponent defects, forever retaliate.
pub struct HoldsGrudge {
    grudge: bool,
}

impl Strategy for HoldsGrudge {
    fn put(&mut self, action: &Action) {
        match action {
            Action::Defect => self.grudge = true,
            _ => {}
        }
    }

    fn get(&self) -> Action {
        if self.grudge {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }
}

impl HoldsGrudge {
    pub fn new() -> Self {
        HoldsGrudge { grudge: false }
    }
}
