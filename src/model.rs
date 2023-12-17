use rand::prelude::*;
use std::{ops::Range, time};

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 420;
pub const ARC_WIDTH: usize = 20;
pub const BUFFER_WIDTH: usize = SCREEN_WIDTH + ARC_WIDTH * 2;
pub const ARC_COUNT: usize = BUFFER_WIDTH / ARC_WIDTH;
pub const SCROLL_SPEED: i32 = 3;

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
    pub scroll_since_last_arc_created: i32,
    pub player: Player,
    pub arcs: [Arc; BUFFER_WIDTH / ARC_WIDTH],
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
            scroll_since_last_arc_created: 0,
            player: Player::new(),
            arcs: [Arc::default(); ARC_COUNT],
        };

        let mut prev_arc = Arc {
            p0: 0.0,
            v0: 0.0,
            p1: 30.0,
            v1: 0.0,
            ys: [0.0; ARC_WIDTH],
        };
        for i in 0..game.arcs.len() {
            create_arc(&mut game.rng, -40..40, &mut game.arcs[i], &prev_arc);
            prev_arc = game.arcs[i];
        }

        game
    }

    pub fn get_ceiling(&self, x: i32) -> i32 {
        self.arcs[x as usize / ARC_WIDTH].ys[x as usize % ARC_WIDTH] as i32
    }

    pub fn get_floor(&self, x: i32) -> i32 {
        // self.arcs[x as usize / ARC_WIDTH].ys[x as usize % ARC_WIDTH] as i32
        440
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
        self.player.x = (self.player.x + SCROLL_SPEED) % BUFFER_WIDTH as i32;

        if self.player.y <= self.get_ceiling(self.player.x)
            || self.player.y >= self.get_floor(self.player.x)
        {
            self.is_over = true;
        }

        self.scroll = (self.scroll + SCROLL_SPEED) % BUFFER_WIDTH as i32;
        self.scroll_since_last_arc_created += SCROLL_SPEED;

        if self.scroll_since_last_arc_created >= ARC_WIDTH as i32 {
            let index =
                ((self.scroll / ARC_WIDTH as i32) - 1 + ARC_COUNT as i32) % ARC_COUNT as i32;
            let prev_index = (index - 1 + ARC_COUNT as i32) % ARC_COUNT as i32;
            let prev_arc = self.arcs[prev_index as usize];
            println!(
                "frame = {}, scroll = {}, index = {}, prev_index = {}",
                self.frame, self.scroll, index, prev_index
            );
            create_arc(
                &mut self.rng,
                -40..40,
                &mut self.arcs[index as usize],
                &prev_arc,
            );
            self.scroll_since_last_arc_created = 0;
        }

        self.frame += 1;
    }
}

#[derive(Clone, Copy, Default)]
pub struct Arc {
    p0: f32,
    v0: f32,
    p1: f32,
    v1: f32,
    ys: [f32; ARC_WIDTH],
}

fn create_arc(rng: &mut StdRng, range: Range<i32>, arc: &mut Arc, prev_arc: &Arc) {
    arc.p0 = prev_arc.p1;
    arc.v0 = prev_arc.v1;
    arc.p1 = arc.p0 + rng.gen_range(range.clone()) as f32;
    if arc.p1 < 0.0 {
        arc.p1 = 0.0;
    }
    if arc.p1 > 420.0 {
        arc.p1 = 420.0;
    }
    arc.v1 = rng.gen();
    for i in 0..arc.ys.len() {
        let t = (i as f32) / (ARC_WIDTH as f32);
        arc.ys[i] = hermite(arc.p0, arc.p1, arc.v0, arc.v1, t);
    }
}

// https://stacstar.jp/blog/?p=975
fn hermite(p0: f32, p1: f32, v0: f32, v1: f32, t: f32) -> f32 {
    (2.0 * p0 - 2.0 * p1 + v0 + v1) * t * t * t
        + (-3.0 * p0 + 3.0 * p1 - 2.0 * v0 - v1) * t * t
        + v0 * t
        + p0
}
