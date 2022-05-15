#![allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::option_if_let_else
)]
use crate::cloth_rendering::ClothRendering;
use crate::Cloth;
use bevy::ecs::prelude::*;
use bevy::log::error;
use bevy::math::{Quat, Vec3};
use bevy::prelude::GlobalTransform;
use bevy_rapier3d::prelude::*;

pub fn handle_collisions(
    mut cloth_query: Query<
        (
            Entity,
            &GlobalTransform,
            &mut Cloth,
            &mut ClothRendering,
            &mut Collider,
        ),
        With<Sensor>,
    >,
    rapier_context: Res<RapierContext>,
    colliders_query: Query<(&Collider, &GlobalTransform, Option<&Velocity>), Without<Cloth>>,
) {
    for (entity, transform, mut cloth, mut rendering, mut collider) in cloth_query.iter_mut() {
        let matrix = transform.compute_matrix();
        let mut collided = false;
        for (coll_a, coll_b, intersecting) in rapier_context.intersections_with(entity) {
            if !intersecting {
                continue;
            }
            collided = true;
            let other_entity = if coll_a == entity { coll_b } else { coll_a };
            let (other_collider, coll_transform, velocity) =
                if let Ok(c) = colliders_query.get(other_entity) {
                    c
                } else {
                    error!("Couldn't find collider on entity {:?}", other_entity);
                    continue;
                };
            cloth.solve_collisions(|point| {
                other_collider
                    .contains_point(coll_transform.translation, coll_transform.rotation, *point)
                    .then(|| {
                        let dir = *point - coll_transform.translation;
                        dir * velocity.map_or(1.0, |v| v.linvel.length())
                    })
            });
        }
        if collided {
            rendering.update_positions(cloth.compute_vertex_positions(&matrix));
        }
        let (center, half_extents): (Vec3, Vec3) = rendering.compute_aabb();
        *collider = Collider::compound(vec![(
            center,
            Quat::IDENTITY,
            Collider::cuboid(half_extents.x, half_extents.y, half_extents.z),
        )]);
    }
}
