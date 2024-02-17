use crate::strategies::always_cooperate::AlwaysCooperate;
use crate::strategies::always_defect::AlwaysDefect;
use crate::strategies::tit_for_tat::TitForTat;
use crate::strategies::tit_for_two_tats::TitFor2Tats;
use crate::strategies::{Action, Strategy};
use std::collections::HashMap;

mod strategies;

fn main() {
    let mut strategies: Vec<Box<dyn Strategy>> = vec![
        // Box::new(AlwaysCooperate::new()),
        Box::new(TitForTat::new()),
        Box::new(TitFor2Tats::new()),
        Box::new(AlwaysDefect::new()),
    ];

    let mut strategies2: Vec<Box<dyn Strategy>> = vec![
        // Box::new(AlwaysCooperate::new()),
        Box::new(TitForTat::new()),
        Box::new(TitFor2Tats::new()),
        Box::new(AlwaysDefect::new()),
    ];

    // for mut strategy in &strategies {
    //     println!(
    //         "Initial action for strategy {} is {:?}",
    //         strategy.name(),
    //         strategy.get()
    //     );
    //
    //     strategy.put(&Action::Defect);
    //
    //     println!("After defecting, next move is {:?}", strategy.get());
    // }

    let mut scores: HashMap<String, i32> = HashMap::new();

    println!("Starting tournament");

    for mut i in &mut strategies {
        for mut j in &mut strategies2 {
            // if i.name() == j.name() {
            //     continue;
            // }

            println!("Executing battle: {} vs {}", i.name(), j.name());

            for _ in 0..200 {
                let action = i.get();
                let reaction = j.get();
                i.put(&reaction);
                j.put(&action);

                match (action, reaction) {
                    (Action::Cooperate, Action::Cooperate) => {
                        println!("Both players cooperated! Will assign both 3 points each.");
                        add_score(&mut scores, i.name(), 3);
                        add_score(&mut scores, j.name(), 3);
                    }
                    (Action::Defect, Action::Defect) => {
                        println!("Both defected, so both get a single point.");
                        add_score(&mut scores, i.name(), 1);
                        add_score(&mut scores, j.name(), 1);
                    }
                    (Action::Cooperate, Action::Defect) => {
                        println!("One defected and gets 5, one cooperated and gets zero");
                        add_score(&mut scores, j.name(), 5);
                    }
                    (Action::Defect, Action::Cooperate) => {
                        println!("One defected and gets 5, one cooperated and gets zero");
                        add_score(&mut scores, i.name(), 5);
                    }
                }
            }
        }
    }

    println!("Tournament finished");

    // I have a hashmap with values and scores. I want to sort by scores and print the results
    let mut scores: Vec<_> = scores.into_iter().collect();
    scores.sort_by(|a, b| b.1.cmp(&a.1));

    for (player, score) in &scores {
        println!("{}\t{}", score, player);
    }
}

fn add_score(scores: &mut HashMap<String, i32>, player: String, score: i32) {
    let entry = scores.entry(String::from(player)).or_insert(0);
    *entry += score;
}
