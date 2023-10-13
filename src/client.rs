use ambient_api::{
  element::use_entity_component,
  core::{
    transform::components::translation,
    rect::components::{
      line_from, line_to,
      line_width,
      background_color
    }
  },
  prelude::*, ui::use_window_logical_resolution
};
use packages::this::messages::Paint;

#[element_component]
fn PlayerPosition(hooks: &mut Hooks) -> Element {
  let pos = use_entity_component(hooks, player::get_local(), translation());
  Text::el(format!("Player Position: {:?}", pos.unwrap_or_default()))
}

// Crosshair from Ambient FPS repo:
// https://github.com/AmbientRun/afps/blob/main/core/fpsui/src/client.rs
#[element_component]
fn Crosshair(hooks: &mut Hooks) -> Element {
    let size = use_window_logical_resolution(hooks);
    let center_x = size.x as f32 / 2.;
    let center_y = size.y as f32 / 2.;

    Group::el([
        Line.el()
            .with(line_from(), vec3(center_x - 10., center_y, 0.))
            .with(line_to(), vec3(center_x + 10., center_y, 0.))
            .with(line_width(), 2.)
            .with(background_color(), vec4(1., 1., 1., 1.)),
        Line.el()
            .with(line_from(), vec3(center_x, center_y - 10., 0.))
            .with(line_to(), vec3(center_x, center_y + 10., 0.))
            .with(line_width(), 2.)
            .with(background_color(), vec4(1., 1., 1., 1.)),
    ])
}

#[main]
pub fn main() {
  fixed_rate_tick(Duration::from_millis(20), move |_| {
    let Some(camera_id) = camera::get_active() else {
      return;
    };

    let input = input::get();
    if input.mouse_buttons.contains(&MouseButton::Left) {
      let ray = camera::clip_position_to_world_ray(camera_id, Vec2::ZERO);

      Paint {
        ray_origin: ray.origin,
        ray_dir: ray.dir
      }
      .send_server_unreliable();
    }
  });

  PlayerPosition.el().spawn_interactive();
  Crosshair.el().spawn_interactive();
}
