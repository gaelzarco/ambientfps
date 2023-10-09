use ambient_api::{
    core::{
        rendering::components::color,
        primitives::{
            components::{quad, cube},
            concepts::Sphere
        },
        transform::{
            components::{scale, translation},
            concepts::Transformable
        },
        physics::components::{
            plane_collider,
            cube_collider,
            sphere_collider,
            dynamic
        },
        model::components::model_from_url,
        player::components::is_player, ecs::components::remove_at_game_time,
    },
    prelude::*,
};

use packages::{
    character_controller::components::use_character_controller,
    character_animation::components::basic_character_animations, this::components::bouncy_created
};

#[main]
pub fn main() {
    // Plane
    Entity::new()
        .with(quad(), ())
        .with(scale(), Vec3::ONE * 10.0)
        .with(color(), vec4(1.0, 0.0, 0.0, 1.0))
        .with(plane_collider(), ())
        .spawn();

    // Spawn Query
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

    for _ in 0..30 {
        Entity::new()
            .with(cube(), ())
            .with(cube_collider(), Vec3::ONE)
            .with(translation(), (random::<Vec2>()* 10.0 - 10.0).extend(1.))
            .spawn();
    }

    fixed_rate_tick(Duration::from_secs_f32(0.5), |_| {
        Entity::new()
            .with_merge(Sphere::suggested())
            .with_merge(Transformable::suggested())
            .with(scale(), Vec3::ONE * 0.2)
            .with(translation(), Vec3::X * 10. + (random::<Vec2>() * 2.0 - 1.0).extend(10.))
            .with(sphere_collider(), 0.5)
            .with(dynamic(), true)
            .with(bouncy_created(), game_time())
            .with(remove_at_game_time(), game_time() + Duration::from_secs_f32(5.0))
            .spawn();
    });

    // THIS IS HOW remove_at_game_time() works under the hood
    // query(bouncy_created()).each_frame(|entities| {
    //     for (id, created) in entities {
    //         if (game_time() - created).as_secs_f32() > 5.0 {
    //             entity::despawn(id);
    //         }
    //     }
    // });
}