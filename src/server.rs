use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        primitives::components::{cube, quad, torus},
        transform::components::{lookat_target, translation, euler_rotation},
    },
    prelude::*,
};

#[main]
pub fn main() {
    PerspectiveInfiniteReverseCamera {
        optional: PerspectiveInfiniteReverseCameraOptional {
            aspect_ratio_from_window: Some(entity::resources()),
            main_scene: Some(()),
            translation: Some(Vec3::ONE * 10.),
            ..default()
        },
        ..PerspectiveInfiniteReverseCamera::suggested()
    }
    .make()
    .with(lookat_target(), vec3(2., 2., 2.))
    .spawn();

    Entity::new()
    .with(torus(), ())
    .with(euler_rotation(), vec3(2., 2., 2.))
    .spawn();

    println!("Hello, Ambient!");
}
