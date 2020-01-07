use crate::components::*;
use specs::prelude::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
  type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

  fn run(&mut self, mut data: Self::SystemData) {
    use self::Direction::*;
    for (pos, vel) in (&mut data.0, &data.1).join() {
      let (x, y) = match vel.direction {
        Left => (-vel.speed, 0),
        Right => (vel.speed, 0),
        Up => (0, -vel.speed),
        Down => (0, vel.speed),
      };
      pos.0 = pos.0.offset(x, y)
    }
  }
}
