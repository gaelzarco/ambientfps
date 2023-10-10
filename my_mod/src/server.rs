use ambient_api::{prelude::*, core::transform::components::translation};
use packages::this::messages::Teleport;

#[main]
pub fn main() {
  Teleport::subscribe(|ctx, _| {
    let Some(player_id) = ctx.client_entity_id() else {
        return;
    };

    entity::mutate_component(player_id, translation(), |t| {
        *t += 10. * Vec3::Z;
    });
  });
}
