use rand::prelude::*;
use std::{ops::Range, time};

pub const SCREEN_WIDTH: usize = 640;
pub const SCREEN_HEIGHT: usize = 420;
pub const ARC_WIDTH: usize = 20; // 1個の3次曲線の幅
pub const WORLD_WIDTH: usize = SCREEN_WIDTH + ARC_WIDTH * 2; // 世界の広さ。画面幅＋曲線2個分の空間を用意しておき、スクロールに応じて循環して利用する（リングバッファ）
pub const ARC_COUNT: usize = WORLD_WIDTH / ARC_WIDTH;
pub const SCROLL_PER_FRAME: i32 = 3;
pub const SPACE_HEIGHT: i32 = 200; // 天井と床の間の高さ

pub enum Command {
    None,
    Up,
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub vy: f32,
    pub old_ys: [i32; 4], // 軌跡
}

impl Player {
    pub fn new() -> Self {
        let player = Player {
            x: 20,
            y: 200,
            vy: 0.0,
            old_ys: [20; 4],
        };
        player
    }

    pub fn up(&mut self) {
        self.vy -= 1.8;
    }

    pub fn apply_gravity(&mut self) {
        self.vy += 0.3;
    }

    pub fn do_move(&mut self) {
        // x座標更新
        self.x = (self.x + SCROLL_PER_FRAME) % WORLD_WIDTH as i32;

        // 軌跡を保存
        for i in 0..(self.old_ys.len() - 1) {
            self.old_ys[i + 1] = self.old_ys[i];
        }
        self.old_ys[0] = self.y;

        // y座標更新
        self.y = (self.y as f32 + self.vy) as i32;
    }
}

#[derive(Clone, Copy, Default)]
pub struct Arc {
    pub p0: f32,
    pub v0: f32,
    pub p1: f32,
    pub v1: f32,
    pub ys: [f32; ARC_WIDTH],
}

pub struct Game {
    pub rng: StdRng,
    pub is_over: bool,
    pub frame: i32,
    pub scroll: i32, // スクロール位置（画面左端に表示する座標）
    pub player: Player,
    pub arcs: [Arc; ARC_COUNT], // 天井の曲線
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
            player: Player::new(),
            arcs: [Arc::default(); ARC_COUNT],
        };

        game.arcs[0] = Arc {
            p0: 0.0,
            v0: 0.0,
            p1: 30.0,
            v1: 0.0,
            ys: [0.0; ARC_WIDTH],
        };
        for i in 1..ARC_COUNT {
            game.create_arc(i);
        }

        game.player.y = (game.get_ceiling(game.player.x) + game.get_floor(game.player.x)) / 2 - 50;

        game
    }

    pub fn get_ceiling(&self, x: i32) -> i32 {
        self.arcs[x as usize / ARC_WIDTH].ys[x as usize % ARC_WIDTH] as i32
    }

    pub fn get_floor(&self, x: i32) -> i32 {
        self.get_ceiling(x) + SPACE_HEIGHT
    }

    pub fn create_arc(&mut self, index: usize) {
        let prev_index = if index == 0 { ARC_COUNT - 1 } else { index - 1 };
        let prev_arc = self.arcs[prev_index];
        let arc = &mut self.arcs[index];
        arc.p0 = prev_arc.p1;
        arc.v0 = prev_arc.v1;
        arc.p1 = arc.p0 + self.rng.gen_range(-40..40) as f32;
        if arc.p1 < 0.0 {
            arc.p1 = 0.0;
        }
        if arc.p1 > 300.0 {
            arc.p1 = 300.0;
        }
        arc.v1 = self.rng.gen();
        for i in 0..arc.ys.len() {
            let t = (i as f32) / (ARC_WIDTH as f32);
            arc.ys[i] = hermite(arc.p0, arc.p1, arc.v0, arc.v1, t);
        }
        println!(
            "Created: index = {}, p0 = {}, prev.p1 = {}, p1 = {}",
            index, self.arcs[index].p0, prev_arc.p1, self.arcs[index].p1
        );
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

        if self.player.y <= self.get_ceiling(self.player.x)
            || self.player.y >= self.get_floor(self.player.x)
        {
            self.is_over = true;
        }

        // スクロール量更新
        self.scroll = (self.scroll + SCROLL_PER_FRAME) % WORLD_WIDTH as i32;

        if self.scroll % (ARC_WIDTH as i32) < SCROLL_PER_FRAME {
            let index = ((self.scroll as usize / ARC_WIDTH) + ARC_COUNT - 1) % ARC_COUNT;
            self.create_arc(index);
        }

        self.frame += 1;
    }
}

// エルミート補間（p0, p1をそれぞれ傾きv0, v1で通る3次曲線で補間）
// https://stacstar.jp/blog/?p=975
fn hermite(p0: f32, p1: f32, v0: f32, v1: f32, t: f32) -> f32 {
    (2.0 * p0 - 2.0 * p1 + v0 + v1) * t * t * t
        + (-3.0 * p0 + 3.0 * p1 - 2.0 * v0 - v1) * t * t
        + v0 * t
        + p0
}
