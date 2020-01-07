use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Texture, WindowCanvas};
use std::time::Duration;

struct Player {
  position: Point,
  sprite: Rect,
}

fn render(
  canvas: &mut WindowCanvas,
  color: Color,
  texture: &Texture,
  player: &Player,
) -> Result<(), String> {
  canvas.set_draw_color(color);
  canvas.clear();
  let (w, h) = canvas.output_size()?;
  let screen_posititon = player.position + Point::new(w as i32 / 2, h as i32 / 2);
  let screen_rect = Rect::from_center(
    screen_posititon,
    player.sprite.width(),
    player.sprite.height(),
  );
  canvas.copy(texture, player.sprite, screen_rect)?;
  canvas.present();
  Ok(())
}

fn main() -> Result<(), String> {
  let sdl_ctx = sdl2::init()?;
  let video = sdl_ctx.video()?;
  let window = video
    .window("bla", 800, 600)
    .position_centered()
    .build()
    .expect("could not create window!");
  let mut canvas = window
    .into_canvas()
    .build()
    .expect("could not create a canvas!");

  let texture_creator = canvas.texture_creator();
  let texture = texture_creator.load_texture("assets/bardo.png")?;
  let player = Player {
    position: Point::new(0, 0),
    sprite: Rect::new(0, 0, 26, 36),
  };

  let mut event_pump = sdl_ctx.event_pump()?;
  let mut i = 0;
  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => {
          break 'running;
        }
        _ => {}
      }
    }

    i = (i + 1) % 255;
    render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture, &player)?;
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
  }
  Ok(())
}