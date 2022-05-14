use crate::Cloth;
use bevy::ecs::prelude::*;
use bevy::log::error;
use bevy_rapier3d::prelude::*;

pub fn handle_collisions(
    mut cloth_query: Query<(Entity, &mut Cloth), (With<RigidBody>, With<Collider>)>,
    rapier_context: Res<RapierContext>,
) {
    for (entity, mut cloth) in cloth_query.iter_mut() {
        for (coll_a, coll_b, intersecting) in rapier_context.intersections_with(entity) {
            if !intersecting {
                continue;
            }
            let other_collider = if coll_a == entity { coll_b } else { coll_a };
        }
    }
}
