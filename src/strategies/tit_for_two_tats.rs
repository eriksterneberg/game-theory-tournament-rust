use crate::strategies::{Action, Strategy};

#[derive(Debug)]
/// When the opponent defects twice in a row, defect until the opponent cooperates.
pub struct TitFor2Tats {
    patience: i32,
}

impl Strategy for TitFor2Tats {
    fn put(&mut self, action: &Action) {
        match action {
            Action::Defect => self.patience -= 1,
            Action::Cooperate => self.patience = 2,
        }
    }

    fn get(&self) -> Action {
        if self.patience <= 0 {
            Action::Defect
        } else {
            Action::Cooperate
        }
    }
}

impl TitFor2Tats {
    pub fn new() -> Self {
        TitFor2Tats { patience: 2 }
    }
}
