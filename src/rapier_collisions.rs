use crate::cloth_rendering::ClothRendering;
use crate::Cloth;
use bevy::ecs::prelude::*;
use bevy::ecs::query::QueryEntityError;
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
    colliders_query: Query<&Collider, Without<Cloth>>,
) {
    for (entity, transform, mut cloth, mut rendering, mut collider) in cloth_query.iter_mut() {
        let matrix = transform.compute_matrix();
        for (coll_a, coll_b, intersecting) in rapier_context.intersections_with(entity) {
            if !intersecting {
                continue;
            }
            let other_entity = if coll_a == entity { coll_b } else { coll_a };
            let collider = match colliders_query.get(other_entity) {
                Ok(c) => c,
                Err(_) => {
                    error!("Couldn't find collider on entity {:?}", other_entity);
                    continue;
                }
            };
            // println!("COLLISION");
        }
        rendering.update_positions(cloth.compute_vertex_positions(&matrix));
        let (center, half_extents): (Vec3, Vec3) = rendering.compute_aabb();
        *collider = Collider::compound(vec![(
            center,
            Quat::IDENTITY,
            Collider::cuboid(half_extents.x, half_extents.y, half_extents.z),
        )]);
    }
}
