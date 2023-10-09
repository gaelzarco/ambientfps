use ambient_api::{
    core::{
        rendering::components::color,
        primitives::components::quad,
        transform::components::scale,
        physics::components::plane_collider, model::components::model_from_url, player::components::is_player,
    },
    prelude::*,
};
use packages::{character_controller::components::use_character_controller, character_animation::components::basic_character_animations};

#[main]
pub fn main() {
    Entity::new()
        .with(quad(), ())
        .with(scale(), Vec3::ONE * 10.0)
        .with(color(), vec4(1.0, 0.0, 0.0, 1.0))
        .with(plane_collider(), ())
        .spawn();

    spawn_query(is_player()).bind(move |players| {
        for (id, _) in players {
            entity::add_components(
                id,
                    Entity::new()
                        .with(use_character_controller(), ())
                        .with(model_from_url(), packages::base_assets::assets::url("Y Bot.fbx"))
                        .with(basic_character_animations(), id)
            );
        }
    });
}