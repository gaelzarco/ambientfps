use ambient_api::{
    core::{
        rendering::components::color,
        primitives::components::{
            quad, cube
        },
        transform::components::{
            scale, translation
        },
        physics::components::{
            plane_collider,
            cube_collider,
            visualize_collider
        },
        model::components::model_from_url,
        player::components::is_player
    },
    prelude::*
};

use packages::{
    character_controller::components::{
        use_character_controller,
        camera_distance
    },
    character_animation::components::basic_character_animations,
    this::messages::Paint
};

#[main]
pub fn main() {
    // Plane
    Entity::new()
        .with(quad(), ())
        .with(scale(), Vec3::ONE * 100.0)
        .with(color(), vec4(0.0, 0.0, 0.0, 0.5))
        .with(plane_collider(), ())
        .spawn();

    // Player
    spawn_query(is_player()).bind(move |players| {
        for (id, _) in players {
            entity::add_components(
                id,
                    Entity::new()
                        .with(use_character_controller(), ())
                        .with(model_from_url(), packages::base_assets::assets::url("Y Bot.fbx"))
                        .with(basic_character_animations(), id)
                        .with(camera_distance(), -0.4)
            );
        }
    });

    // Obstacles
    for _ in 0..100 {
        let rand_size = random::<Vec3>()*thread_rng().gen_range(6.5..10.0);

        Entity::new()
            .with(cube(), ())
            .with(scale(), rand_size)
            .with(cube_collider(), Vec3::ONE)
            .with(visualize_collider(), ())
            .with(translation(), (random::<Vec2>()*thread_rng().gen_range(-100.0..100.0)).extend(0.5))
            .spawn();
    }

    // Hitscan
    Paint::subscribe(|ctx, msg| {
        if ctx.client_user_id().is_none() {
            return;
        }

        let Some(hit) = physics::raycast_first(msg.ray_origin, msg.ray_dir) else {
            return;
        };

        Entity::new()
            .with(cube(), ())
            .with(translation(), hit.position)
            .with(scale(), Vec3::ONE * 0.1)
            .with(color(), vec4(0., 1., 0., 1.))
            .spawn();
    });
}