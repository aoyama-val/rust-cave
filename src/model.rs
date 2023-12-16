use rand::prelude::*;
use std::time;

const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 420;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub velocity: f32,
    pub old_ys: [i32; 4],
}

impl Player {
    pub fn new() -> Self {
        let player = Player {
            x: 20,
            y: 390,
            velocity: 0.0,
            old_ys: [20; 4],
        };
        player
    }

    pub fn up(&mut self) {
        self.velocity -= 3.0;
    }

    pub fn apply_gravity(&mut self) {
        self.velocity += 0.2;
    }

    pub fn do_move(&mut self) {
        for i in 0..(self.old_ys.len() - 1) {
            self.old_ys[i + 1] = self.old_ys[i];
        }
        self.old_ys[0] = self.y;
        self.y = (self.y as f32 + self.velocity) as i32;
    }
}

pub struct Game {
    pub rng: StdRng,
    pub is_over: bool,
    pub frame: i32,
    pub scroll: i32,
    pub ys: [i32; SCREEN_WIDTH],
    pub player: Player,
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
            scroll: 0,
            ys: [0; SCREEN_WIDTH],
            player: Player::new(),
        };
        game.create_curve();
        game
    }

    fn create_curve(&mut self) {
        let mut begin = 0;
        let step = 20;
        let mut prev_p: f32 = 0.0;
        let mut prev_v: f32 = 0.0;
        while begin < SCREEN_WIDTH {
            let p0 = prev_p as f32;
            let mut p1 = p0 + self.rng.gen_range(-40..40) as f32;
            if p1 < 0.0 {
                p1 = 0.0;
            }
            let v0 = prev_v;
            let v1 = self.rng.gen();
            for x in begin..(begin + step) {
                let t = ((x - begin) as f32) / (step as f32);
                // println!("{}", t);
                self.ys[x] = hermite(p0, p1, v0, v1, t) as i32
            }
            prev_p = p1;
            prev_v = v1;
            begin += step;
        }
    }

    pub fn update(&mut self, command: &str) {
        if self.is_over {
            return;
        }

        match command {
            "up" => self.player.up(),
            _ => {}
        }

        self.player.apply_gravity();
        self.player.do_move();

        if self.frame % 1 == 0 {
            self.scroll += 1;
        }
        self.frame += 1;
    }
}

fn hermite(p0: f32, p1: f32, v0: f32, v1: f32, t: f32) -> f32 {
    (2.0 * p0 - 2.0 * p1 + v0 + v1) * t * t * t
        + (-3.0 * p0 + 3.0 * p1 - 2.0 * v0 - v1) * t * t
        + v0 * t
        + p0
}
