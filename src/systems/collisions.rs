#![allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::option_if_let_else,
    clippy::suboptimal_flops
)]
use crate::components::{cloth::Cloth, collider::ClothCollider};
use bevy::log;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_rapier3d::prelude::*;

fn get_collider(aabb: &Aabb, collider: &ClothCollider) -> Collider {
    let extents = aabb.half_extents + collider.offset;
    Collider::compound(vec![(
        aabb.center.into(),
        Quat::IDENTITY,
        Collider::cuboid(extents.x, extents.y, extents.z),
    )])
}

pub fn handle_collisions(
    mut cloth_query: Query<(Entity, &mut Cloth, &Aabb, &ClothCollider, &mut Collider)>,
    rapier_context: Res<RapierContext>,
    mut colliders_query: Query<
        (&Collider, &GlobalTransform, Option<&mut Velocity>),
        Without<Cloth>,
    >,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (entity, mut cloth, aabb, collider, mut rapier_collider) in cloth_query.iter_mut() {
        for contact_pair in rapier_context.contacts_with(entity) {
            let other_entity = if contact_pair.collider1() == entity {
                contact_pair.collider2()
            } else {
                contact_pair.collider1()
            };
            let Ok((other_collider, other_transform, other_velocity)) = colliders_query.get_mut(other_entity) else {
                log::error!("Couldn't find collider on entity {:?}", entity);
                continue;
            };
            let vel = other_velocity.as_ref().map_or(0.0, |v| {
                v.linvel.length_squared() * delta_time * delta_time * collider.velocity_coefficient
            });
            cloth.solve_collisions(|point| {
                let other_transform = other_transform.compute_transform();
                let projected_point = other_collider.project_point(
                    other_transform.translation,
                    other_transform.rotation,
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
            if let Some((ref mut vel, dampen_coef)) = other_velocity.zip(collider.dampen_others) {
                let damp = 1.0 - dampen_coef;
                vel.linvel *= damp;
                vel.angvel *= damp;
            }
        }
        *rapier_collider = get_collider(aabb, collider);
    }
}

pub fn init_cloth_collider(
    mut commands: Commands,
    cloth_query: Query<(Entity, &Aabb, &ClothCollider), (With<Cloth>, Without<Collider>)>,
) {
    for (entity, aabb, collider) in cloth_query.iter() {
        log::debug!("Initializing Cloth collisions for {:?}", entity);
        commands.entity(entity).insert((
            RigidBody::KinematicPositionBased,
            get_collider(aabb, collider),
            SolverGroups::new(Group::NONE, Group::NONE),
        ));
    }
}

// Old collision detection code through shape casting every frame
//
// pub fn handle_collisions(
//     mut cloth_query: Query<(
//         &GlobalTransform,
//         &mut Cloth,
//         &ClothRendering,
//         &ClothCollider,
//     )>,
//     rapier_context: Res<RapierContext>,
//     mut colliders_query: Query<(&Collider, &GlobalTransform, Option<&mut Velocity>)>,
//     time: Res<Time>,
// ) {
//     let delta_time = time.delta_seconds();
//     for (transform, mut cloth, rendering, collider) in cloth_query.iter_mut() {
//         let matrix: Mat4 = transform.compute_matrix();
//         let (center, extents): (Vec3, Vec3) = rendering.compute_aabb(Some(collider.offset));
//         rapier_context.intersections_with_shape(
//             matrix.transform_point3(center),
//             Quat::IDENTITY,
//             &Collider::cuboid(extents.x, extents.y, extents.z),
//             collider.interaction_groups,
//             None,
//             |entity| {
//                 let (other_collider, coll_transform, velocity) =
//                     if let Ok(c) = colliders_query.get_mut(entity) {
//                         c
//                     } else {
//                         error!("Couldn't find collider on entity {:?}", entity);
//                         return false;
//                     };
//                 let vel = velocity.as_ref().map_or(0.0, |v| {
//                     v.linvel.length_squared()
//                         * delta_time
//                         * delta_time
//                         * collider.velocity_coefficient
//                 });
//                 cloth.solve_collisions(|point| {
//                     let projected_point = other_collider.project_point(
//                         coll_transform.translation,
//                         coll_transform.rotation,
//                         *point,
//                         false,
//                     );
//                     let normal: Vec3 = (projected_point.point - *point)
//                         .try_normalize()
//                         .unwrap_or(Vec3::Y);
//                     if projected_point.is_inside {
//                         Some(projected_point.point + (normal * collider.offset) + (normal * vel))
//                     } else if point.distance_squared(projected_point.point)
//                         < collider.offset * collider.offset
//                     {
//                         Some(projected_point.point - (normal * collider.offset))
//                     } else {
//                         None
//                     }
//                 });
//                 if let Some((ref mut vel, dampen_coef)) = velocity.zip(collider.dampen_others) {
//                     let damp = 1.0 - dampen_coef;
//                     vel.linvel *= damp;
//                     vel.angvel *= damp;
//                 }
//                 true
//             },
//         );
//     }
// }
//
