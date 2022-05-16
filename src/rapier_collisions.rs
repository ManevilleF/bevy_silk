#![allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::option_if_let_else,
    clippy::suboptimal_flops
)]
use crate::cloth_rendering::ClothRendering;
use crate::{Cloth, ClothCollider};
use bevy::log::error;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn handle_collisions(
    mut cloth_query: Query<(
        &GlobalTransform,
        &mut Cloth,
        &ClothRendering,
        &ClothCollider,
    )>,
    rapier_context: Res<RapierContext>,
    mut colliders_query: Query<(&Collider, &GlobalTransform, Option<&mut Velocity>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (transform, mut cloth, rendering, collider) in cloth_query.iter_mut() {
        let matrix: Mat4 = transform.compute_matrix();
        let (center, extents): (Vec3, Vec3) = rendering.compute_aabb(Some(collider.offset));
        rapier_context.intersections_with_shape(
            matrix.transform_point3(center),
            Quat::IDENTITY,
            &Collider::cuboid(extents.x, extents.y, extents.z),
            collider.interaction_groups,
            None,
            |entity| {
                let (other_collider, coll_transform, velocity) =
                    if let Ok(c) = colliders_query.get_mut(entity) {
                        c
                    } else {
                        error!("Couldn't find collider on entity {:?}", entity);
                        return false;
                    };
                let vel = velocity.as_ref().map_or(0.0, |v| {
                    v.linvel.length_squared()
                        * delta_time
                        * delta_time
                        * collider.velocity_coefficient
                });
                cloth.solve_collisions(|point| {
                    let projected_point = other_collider.project_point(
                        coll_transform.translation,
                        coll_transform.rotation,
                        *point,
                        false,
                    );
                    let normal: Vec3 = (projected_point.point - *point)
                        .try_normalize()
                        .unwrap_or(Vec3::Y);
                    if projected_point.is_inside {
                        Some(projected_point.point + (normal * collider.offset) + (normal * vel))
                    } else if point.distance_squared(projected_point.point)
                        < collider.offset * collider.offset
                    {
                        Some(projected_point.point - (normal * collider.offset))
                    } else {
                        None
                    }
                });
                if let Some((ref mut vel, dampen_coef)) = velocity.zip(collider.dampen_others) {
                    let damp = 1.0 - dampen_coef;
                    vel.linvel *= damp;
                    vel.angvel *= damp;
                }
                true
            },
        );
    }
}
