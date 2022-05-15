#![allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::option_if_let_else
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
        &mut ClothRendering,
        &ClothCollider,
    )>,
    rapier_context: Res<RapierContext>,
    colliders_query: Query<(&Collider, &GlobalTransform, Option<&Velocity>)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (transform, mut cloth, mut rendering, collider) in cloth_query.iter_mut() {
        let matrix: Mat4 = transform.compute_matrix();
        let mut collided = false;
        let (center, extents): (Vec3, Vec3) = rendering.compute_aabb();
        rapier_context.intersections_with_shape(
            matrix.transform_point3(center),
            Quat::IDENTITY,
            &Collider::cuboid(extents.x, extents.y, extents.z),
            collider.interaction_groups,
            None,
            |entity| {
                collided = true;
                let (other_collider, coll_transform, velocity) =
                    if let Ok(c) = colliders_query.get(entity) {
                        c
                    } else {
                        error!("Couldn't find collider on entity {:?}", entity);
                        return false;
                    };
                cloth.solve_collisions(|point| {
                    other_collider
                        .contains_point(coll_transform.translation, coll_transform.rotation, *point)
                        .then(|| {
                            let dir = *point - coll_transform.translation;
                            dir * (1.0
                                + velocity.map_or(0.0, |v| v.linvel.length_squared() * delta_time))
                        })
                });
                true
            },
        );
        if collided {
            rendering.update_positions(cloth.compute_vertex_positions(&matrix));
        }
    }
}
