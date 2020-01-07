mod animator;
mod components;
mod keyboard;
mod physics;
mod renderer;

use crate::components::*;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use std::time::Duration;

pub enum MovementCommand {
  Stop,
  Move(Direction),
}

fn direction_spreedsheet_row(direction: Direction) -> i32 {
  use self::Direction::*;
  match direction {
    Up => 3,
    Down => 0,
    Left => 1,
    Right => 2,
  }
}

fn character_animation_frames(
  sprite_sheet: usize,
  top_left_frame: Rect,
  direction: Direction,
) -> Vec<Sprite> {
  let (frame_w, frame_h) = top_left_frame.size();
  let y_offset = top_left_frame.y() + frame_h as i32 * direction_spreedsheet_row(direction);

  let mut frames = Vec::new();
  for i in 0..3 {
    frames.push(Sprite {
      sprite_sheet,
      region: Rect::new(
        top_left_frame.x() + frame_w as i32 * i,
        y_offset,
        frame_w,
        frame_h,
      ),
    })
  }
  frames
}

fn main() -> Result<(), String> {
  let sdl_ctx = sdl2::init()?;
  let video = sdl_ctx.video()?;
  let window = video
    .window("bla", 800, 600)
    .position_centered()
    .build()
    .expect("could not create a window!");
  let mut canvas = window
    .into_canvas()
    .build()
    .expect("could not create a canvas!");

  let mut dispatcher = DispatcherBuilder::new()
    .with(keyboard::Keyboard, "Keyboard", &[])
    .with(physics::Physics, "Physics", &["Keyboard"])
    .with(animator::Animator, "Animator", &["Keyboard"])
    .build();
  let mut world = World::new();
  dispatcher.setup(&mut world);
  renderer::SystemData::setup(&mut world);

  let movement_command: Option<MovementCommand> = None;
  world.insert(movement_command);

  let texture_creator = canvas.texture_creator();
  let texture = [texture_creator.load_texture("assets/bardo.png")?];

  let player_sprite_sheet = 0;
  let player_top_left_frame = Rect::new(0, 0, 26, 36);
  let player_animation = MovementAnimation {
    current_frame: 0,
    up_frames: character_animation_frames(
      player_sprite_sheet,
      player_top_left_frame,
      Direction::Up,
    ),
    down_frames: character_animation_frames(
      player_sprite_sheet,
      player_top_left_frame,
      Direction::Down,
    ),
    left_frames: character_animation_frames(
      player_sprite_sheet,
      player_top_left_frame,
      Direction::Left,
    ),
    right_frames: character_animation_frames(
      player_sprite_sheet,
      player_top_left_frame,
      Direction::Right,
    ),
  };

  world
    .create_entity()
    .with(KeyboardControlled)
    .with(Position(Point::new(0, 0)))
    .with(Velocity {
      speed: 0,
      direction: Direction::Right,
    })
    .with(player_animation.right_frames[0].clone())
    .with(player_animation)
    .build();

  let mut event_pump = sdl_ctx.event_pump()?;
  'running: loop {
    let mut movement_command = None;
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => {
          break 'running;
        }
        Event::KeyDown {
          keycode: Some(Keycode::Left),
          ..
        } => {
          movement_command = Some(MovementCommand::Move(Direction::Left));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Right),
          ..
        } => {
          movement_command = Some(MovementCommand::Move(Direction::Right));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Up),
          ..
        } => {
          movement_command = Some(MovementCommand::Move(Direction::Up));
        }
        Event::KeyDown {
          keycode: Some(Keycode::Down),
          ..
        } => {
          movement_command = Some(MovementCommand::Move(Direction::Down));
        }
        Event::KeyUp {
          keycode: Some(Keycode::Left),
          repeat: false,
          ..
        }
        | Event::KeyUp {
          keycode: Some(Keycode::Right),
          repeat: false,
          ..
        }
        | Event::KeyUp {
          keycode: Some(Keycode::Up),
          repeat: false,
          ..
        }
        | Event::KeyUp {
          keycode: Some(Keycode::Down),
          repeat: false,
          ..
        } => {
          movement_command = Some(MovementCommand::Stop);
        }
        _ => {}
      }
    }

    *world.write_resource() = movement_command;
    dispatcher.dispatch(&mut world);
    world.maintain();
    renderer::render(
      &mut canvas,
      Color::RGB(1, 64, 255),
      &texture,
      world.system_data(),
    )?;
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
  }
  Ok(())
}
