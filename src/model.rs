use rand::prelude::*;
use std::time;

pub struct Game {
    pub rng: StdRng,
    pub is_over: bool,
    pub frame: i32,
}

impl Game {
    pub fn new() -> Self {
        let now = time::SystemTime::now();
        let timestamp = now
            .duration_since(time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();
        let rng = StdRng::seed_from_u64(timestamp);

        let mut game = Game {
            rng: rng,
            is_over: false,
            frame: 0,
        };
        game
    }

    pub fn update(&mut self, command: &str) {
        if self.is_over {
            return;
        }

        match command {
            _ => {}
        }

        self.frame += 1;
    }
}
