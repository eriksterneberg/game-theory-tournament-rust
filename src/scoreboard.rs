use crate::strategies::enums::StrategyEnum;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Scoreboard(HashMap<StrategyEnum, i32>);

impl Default for Scoreboard {
    fn default() -> Self {
        Scoreboard(HashMap::new())
    }
}

impl Scoreboard {
    pub fn add_score(&mut self, player: StrategyEnum, score: i32) {
        self.0
            .entry(player)
            .and_modify(|s| *s += score)
            .or_insert(score);
    }

    fn sorted_scores(&self) -> Vec<(i32, StrategyEnum)> {
        let mut sorted_scores: Vec<_> = self
            .0
            .iter()
            .map(|(player, &score)| (score, *player))
            .collect();
        sorted_scores.sort_by_key(|&(score, _)| std::cmp::Reverse(score));
        sorted_scores
    }

    pub fn print_scores(&self) {
        for (score, player) in self.sorted_scores() {
            println!("{}\t{:?}", score, player);
        }
    }
}
