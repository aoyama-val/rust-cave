use rand::prelude::*;
use std::{ops::Range, time};

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 420;
pub const ARC_WIDTH: usize = 20;
pub const BUFFER_WIDTH: usize = SCREEN_WIDTH + ARC_WIDTH;

pub enum Command {
    None,
    Up,
}

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
            y: 200,
            velocity: 0.0,
            old_ys: [20; 4],
        };
        player
    }

    pub fn up(&mut self) {
        self.velocity -= 1.8;
    }

    pub fn apply_gravity(&mut self) {
        self.velocity += 0.3;
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
    pub ceiling: [i32; BUFFER_WIDTH],
    pub floor: [i32; BUFFER_WIDTH],
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
            ceiling: [0; BUFFER_WIDTH],
            floor: [0; BUFFER_WIDTH],
            player: Player::new(),
        };
        create_curve(&mut game.rng, &mut game.ceiling, -40..40, 0.0, 0.0);
        create_curve(&mut game.rng, &mut game.floor, -40..40, 350.0, 0.0);
        game
    }

    pub fn update(&mut self, command: Command) {
        if self.is_over {
            return;
        }

        match command {
            Command::Up => self.player.up(),
            Command::None => {}
        }

        self.player.apply_gravity();
        self.player.do_move();

        let x = (self.scroll + self.player.x as i32) % SCREEN_WIDTH as i32;
        if self.player.y <= self.ceiling[x as usize]
            || self.player.y >= self.floor[x as usize] as i32
        {
            self.is_over = true;
        }

        self.scroll += 3;
        self.frame += 1;
    }
}

fn create_curve(
    rng: &mut StdRng,
    dst: &mut [i32; BUFFER_WIDTH],
    range: Range<i32>,
    prev_p: f32,
    prev_v: f32,
) {
    let mut begin = 0;
    let step = ARC_WIDTH;
    let mut prev_p: f32 = prev_p;
    let mut prev_v: f32 = prev_v;
    while begin < SCREEN_WIDTH {
        let p0 = prev_p as f32;
        let mut p1 = p0 + rng.gen_range(range.clone()) as f32;
        if p1 < 0.0 {
            p1 = 0.0;
        }
        if p1 > 420.0 {
            p1 = 420.0;
        }
        let v0 = prev_v;
        let v1 = rng.gen();
        for x in begin..(begin + step) {
            let t = ((x - begin) as f32) / (step as f32);
            dst[x] = hermite(p0, p1, v0, v1, t) as i32
        }
        prev_p = p1;
        prev_v = v1;
        begin += step;
    }
}

// https://stacstar.jp/blog/?p=975
fn hermite(p0: f32, p1: f32, v0: f32, v1: f32, t: f32) -> f32 {
    (2.0 * p0 - 2.0 * p1 + v0 + v1) * t * t * t
        + (-3.0 * p0 + 3.0 * p1 - 2.0 * v0 - v1) * t * t
        + v0 * t
        + p0
}
