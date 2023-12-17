use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{BlendMode, Canvas};
use sdl2::video::Window;
use std::time::Duration;

mod model;
use crate::model::*;

const FPS: u32 = 30;
const PLAYER_SIZE: u32 = 4;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    sdl_context.mouse().show_cursor(false);

    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-cave", SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::new();

    'running: loop {
        let mut command = Command::None;
        if event_pump
            .keyboard_state()
            .is_scancode_pressed(sdl2::keyboard::Scancode::Up)
        {
            command = Command::Up;
            println!("up");
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
        render(&mut canvas, &game)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(())
}

fn render(canvas: &mut Canvas<Window>, game: &Game) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // render ceiling
    canvas.set_draw_color(Color::RGB(238, 130, 238));
    for i in 0..SCREEN_WIDTH {
        let x = (game.scroll + i as i32) % SCREEN_WIDTH as i32;
        canvas.draw_line(
            Point::new(i as i32, 0),
            Point::new(i as i32, game.ceiling[x as usize] as i32),
        )?;
    }

    // render floor
    for i in 0..SCREEN_WIDTH {
        let x = (game.scroll + i as i32) % SCREEN_WIDTH as i32;
        canvas.draw_line(
            Point::new(i as i32, game.floor[x as usize] as i32),
            Point::new(i as i32, SCREEN_HEIGHT as i32 - 1),
        )?;
    }

    // render player
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.fill_rect(Rect::new(
        game.player.x,
        game.player.y,
        PLAYER_SIZE,
        PLAYER_SIZE,
    ))?;
    for i in 0..(game.player.old_ys.len()) {
        canvas.set_draw_color(Color::RGBA(255, 255, 0, (255 - 40 * (i + 1)) as u8));
        canvas.fill_rect(Rect::new(
            game.player.x - PLAYER_SIZE as i32 * (i + 1) as i32,
            game.player.old_ys[i],
            PLAYER_SIZE,
            PLAYER_SIZE,
        ))?;
    }

    if game.is_over {
        canvas.set_draw_color(Color::RGBA(255, 0, 0, 128));
        canvas.fill_rect(Rect::new(0, 0, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32))?;
    }

    canvas.present();

    Ok(())
}
