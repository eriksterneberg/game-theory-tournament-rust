use crate::strategies::{Action, Strategy};

#[derive(Debug)]

pub struct TitFor2Tats {
    patience: i32,
}

impl Strategy for TitFor2Tats {
    fn name(&self) -> String {
        String::from("Tit for 2 Tats")
    }

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

    fn reset(&mut self) {
        self.patience = 2;
    }
}

impl TitFor2Tats {
    pub fn new() -> Self {
        TitFor2Tats { patience: 2 }
    }
}
