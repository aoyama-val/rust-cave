use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;
use std::path::Path;
use std::time::{Duration, SystemTime};
mod model;
use crate::model::*;

const FPS: u32 = 30;
const PLAYER_SIZE: u32 = 4;

struct Image<'a> {
    texture: Texture<'a>,
    w: u32,
    h: u32,
}

struct Resources<'a> {
    images: HashMap<String, Image<'a>>,
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-cave", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    sdl_context.mouse().show_cursor(false);

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);

    let texture_creator = canvas.texture_creator();
    let resources = load_resources(&texture_creator);

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::new();

    println!("Keys:");
    println!("    Up : Move player up");

    'running: loop {
        let started = SystemTime::now();

        let mut command = Command::None;
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::Up)
        {
            command = Command::Up;
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        game.update(command);
        render(&mut canvas, &game, &resources)?;

        let finished = SystemTime::now();
        let elapsed = finished.duration_since(started).unwrap();
        let frame_duration = Duration::new(0, 1_000_000_000u32 / FPS);
        if elapsed < frame_duration {
            ::std::thread::sleep(frame_duration - elapsed)
        }
    }

    Ok(())
}

fn load_resources<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Resources {
    let mut resources = Resources {
        images: HashMap::new(),
    };

    let image_paths = ["numbers.bmp"];
    for path in image_paths {
        let full_path = "resources/image/".to_string() + path;
        let temp_surface = sdl2::surface::Surface::load_bmp(Path::new(&full_path)).unwrap();
        let texture = texture_creator
            .create_texture_from_surface(&temp_surface)
            .expect(&format!("cannot load image: {}", path));

        let q = texture.query();
        let image: Image = Image {
            texture: texture,
            w: q.width,
            h: q.height,
        };
        resources.images.insert(path.to_string(), image);
    }
    resources
}

fn render(canvas: &mut Canvas<Window>, game: &Game, resources: &Resources) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(238, 130, 238));
    for i in 0..SCREEN_WIDTH {
        let x = (game.scroll + i as i32) % WORLD_WIDTH as i32;

        // render ceiling
        canvas.draw_line(
            Point::new(i as i32, 0),
            Point::new(i as i32, game.get_ceiling(x)),
        )?;

        // render floor
        canvas.draw_line(
            Point::new(i as i32, game.get_floor(x)),
            Point::new(i as i32, SCREEN_HEIGHT as i32 - 1),
        )?;
    }

    // render player
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.fill_rect(Rect::new(
        (game.player.x - game.scroll + WORLD_WIDTH as i32) % WORLD_WIDTH as i32,
        game.player.y,
        PLAYER_SIZE,
        PLAYER_SIZE,
    ))?;
    for i in 0..(game.player.old_ys.len()) {
        canvas.set_draw_color(Color::RGBA(255, 255, 0, (255 - 40 * (i + 1)) as u8));
        canvas.fill_rect(Rect::new(
            (game.player.x - game.scroll + WORLD_WIDTH as i32
                - PLAYER_SIZE as i32 * (i + 1) as i32)
                % WORLD_WIDTH as i32,
            game.player.old_ys[i],
            PLAYER_SIZE,
            PLAYER_SIZE,
        ))?;
    }

    render_number(
        canvas,
        resources,
        SCREEN_WIDTH as i32 - 8 * 8,
        0,
        format!("{0: >8}", game.frame),
    );

    if game.is_over {
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));
        canvas.fill_rect(Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))?;
    }

    canvas.present();

    Ok(())
}

fn render_number(
    canvas: &mut Canvas<Window>,
    resources: &Resources,
    x: i32,
    y: i32,
    numstr: String,
) {
    let mut x = x;
    let image = resources.images.get("numbers.bmp").unwrap();
    let digit_width_in_px = 8;
    for c in numstr.chars() {
        if 0x30 <= c as i32 && c as i32 <= 0x39 {
            canvas
                .copy(
                    &image.texture,
                    Rect::new(
                        digit_width_in_px * (c as i32 - 0x30),
                        0,
                        digit_width_in_px as u32,
                        image.h,
                    ),
                    Rect::new(x, y, digit_width_in_px as u32, image.h),
                )
                .unwrap();
        }
        x += digit_width_in_px;
    }
}
