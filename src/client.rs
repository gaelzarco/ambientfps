use ambient_api::{
  element::use_entity_component,
  core::transform::components::translation,
  prelude::*
};
use packages::this::messages::Paint;

#[element_component]
fn PlayerPosition(hooks: &mut Hooks) -> Element {
  let pos = use_entity_component(hooks, player::get_local(), translation());
  Text::el(format!("Player Position: {:?}", pos.unwrap_or_default()))
}

#[main]
pub fn main() {
  fixed_rate_tick(Duration::from_millis(20), move |_| {
    let Some(camera_id) = camera::get_active() else {
      return;
    };

    let input = input::get();
    if input.keys.contains(&KeyCode::Q) {
      let ray = camera::clip_position_to_world_ray(camera_id, Vec2::ZERO);

      Paint {
        ray_origin: ray.origin,
        ray_dir: ray.dir
      }
      .send_server_unreliable();
    }
  });

  PlayerPosition.el().spawn_interactive();
}
