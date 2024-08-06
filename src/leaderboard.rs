use std::collections::BTreeMap;

pub struct Leaderboard {
    scores: BTreeMap<String, u32>,
}

impl Leaderboard {
    pub fn new() -> Self {
        Leaderboard {
            scores: BTreeMap::new(),
        }
    }

    pub fn add_score(&mut self, name: String, score: u32) {
        self.scores.insert(name, score);
    }

    pub fn display(&self) {
        for (i, (name, score)) in self.scores.iter().enumerate() {
            println!("{}. {} - {}", i+1, name, score);
        }
    }
}