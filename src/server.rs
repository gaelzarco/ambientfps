use ambient_api::{
    core::{
        camera::concepts::{
            PerspectiveInfiniteReverseCamera, PerspectiveInfiniteReverseCameraOptional,
        },
        rendering::components::{ color, double_sided },
        primitives::{
            components::quad,
            concepts::Torus
        },
        transform::{
            components::lookat_target, 
            concepts::{Transformable, TransformableOptional}
        },
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
    .with_merge(Transformable {
        local_to_world: Mat4::IDENTITY,
        optional: TransformableOptional {
            scale: Some(Vec3::ONE * 10.),
            ..default()
        }
    })
    .with(quad(), ())
    .with(double_sided(), true)
    .with(color(), vec4(1., 1., 1., 1.))
    .spawn();

    Entity::new()
    .with_merge(Transformable {
        local_to_world: Mat4::IDENTITY,
        optional: TransformableOptional {
            translation: Some(vec3(0.0, -0.0, 0.5)),
            ..default()
        } 
    })
    .with_merge( Torus {
        torus: (),
        torus_inner_radius: 0.25,
        torus_outer_radius: 0.5,
        torus_slices: 32,
        torus_loops: 16
    })
    .with(color(), vec4(0.0, 0.0, 0.0, 0.0))
    .spawn();

    println!("Hello, Ambient!");
}