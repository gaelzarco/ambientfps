use ambient_api::{
  core::rect::components::{
    line_from, line_to,
    line_width,
    background_color
  },
  prelude::*, ui::use_window_logical_resolution,
  element::use_entity_component
};
use packages::this::messages::Paint;
use packages::this::components::player_health;

// Crosshair from Ambient FPS repo:
// https://github.com/AmbientRun/afps/blob/main/core/fpsui/src/client.rs
#[element_component]
fn Crosshair(hooks: &mut Hooks) -> Element {
    let size = use_window_logical_resolution(hooks);
    let center_x = size.x as f32 / 2.;
    let center_y = size.y as f32 / 2.;

    Group::el([
        Line.el()
            .with(line_from(), vec3(center_x - 5., center_y - 5., 0.))
            .with(line_to(), vec3(center_x + 5., center_y + 5., 0.))
            .with(line_width(), 2.)
            .with(background_color(), vec4(1., 1., 1., 1.)),
        Line.el()
            .with(line_from(), vec3(center_x + 5., center_y - 5., 0.))
            .with(line_to(), vec3(center_x - 5., center_y + 5., 0.))
            .with(line_width(), 2.)
            .with(background_color(), vec4(1., 1., 1., 1.)),
    ])
}

#[element_component]
fn Hud(hooks: &mut Hooks) -> Element {
  let local_health = use_entity_component(hooks, player::get_local(), player_health());

  WindowSized::el([Dock::el([Text::el(format!(
      "health: {:?}",
      local_health.unwrap_or(100)
  ))
  // .header_style()
  .with(docking(), Docking::Bottom)
  .with_margin_even(10.)])])
  .with_padding_even(20.)
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

  Crosshair.el().spawn_interactive();
  Hud.el().spawn_interactive();
}
