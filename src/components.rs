use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Position(pub Point);

#[derive(Component)]
#[storage(VecStorage)]
pub struct Velocity {
  pub speed: i32,
  pub direction: Direction,
}

#[derive(Component, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
  pub sprite_sheet: usize,
  pub region: Rect,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct MovementAnimation {
  pub current_frame: usize,
  pub up_frames: Vec<Sprite>,
  pub down_frames: Vec<Sprite>,
  pub left_frames: Vec<Sprite>,
  pub right_frames: Vec<Sprite>,
}
