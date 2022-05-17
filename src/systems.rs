#![allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::option_if_let_else
)]

use crate::cloth::Cloth;
use crate::cloth_rendering::ClothRendering;
use crate::config::ClothConfig;
use crate::wind::Winds;
use crate::ClothBuilder;
use bevy::asset::{Assets, Handle};
use bevy::core::Time;
use bevy::ecs::prelude::*;
use bevy::log::{debug, error, warn};
use bevy::math::Vec3;
use bevy::render::prelude::Mesh;
use bevy::transform::prelude::GlobalTransform;

pub fn update_cloth(
    mut query: Query<(&mut Cloth, &GlobalTransform, Option<&ClothConfig>)>,
    config: Res<ClothConfig>,
    wind: Option<Res<Winds>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    let wind_force = wind.map_or(Vec3::ZERO, |w| {
        w.current_velocity(time.time_since_startup().as_secs_f32())
    });
    for (mut cloth, transform, custom_config) in query.iter_mut() {
        let matrix = transform.compute_matrix();
        let config: &ClothConfig = custom_config.unwrap_or(&config);
        cloth.update_points(
            config.friction_coefficient(),
            config.smoothed_acceleration(wind_force + config.gravity, delta_time),
        );
        cloth.update_sticks(&matrix, config.sticks_computation_depth);
    }
}

pub fn render_cloth(
    mut cloth_query: Query<(&Cloth, &mut ClothRendering, &GlobalTransform, &Handle<Mesh>)>,
    anchor_query: Query<&GlobalTransform, Without<Cloth>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (cloth, mut rendering, transform, handle) in cloth_query.iter_mut() {
        if let Some(mesh) = meshes.get_mut(handle) {
            rendering.update_positions(cloth.compute_vertex_positions(transform, |entity| {
                if let Ok(t) = anchor_query.get(entity) {
                    Some(*t)
                } else {
                    error!("Could not find cloth anchor target entity {:?}", entity);
                    None
                }
            }));
            rendering.apply(mesh);
        } else {
            warn!("A Cloth has a `ClothRendering` component without a loaded mesh");
        }
    }
}

pub fn init_cloth(
    mut commands: Commands,
    query: Query<(Entity, &ClothBuilder, &GlobalTransform, &Handle<Mesh>), Without<Cloth>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, builder, transform, handle) in query.iter() {
        if let Some(mesh) = meshes.get(handle) {
            let matrix = transform.compute_matrix();
            debug!("Initializing Cloth");
            let rendering = ClothRendering::init(mesh, builder.normals_computing).unwrap();
            let cloth = Cloth::new(
                &rendering.vertex_positions,
                &rendering.indices,
                builder.anchored_vertex_ids(mesh),
                builder.stick_generation,
                builder.stick_length,
                &matrix,
            );
            // TODO: should the cloth builder be removed ?
            commands.entity(entity).insert(rendering).insert(cloth);
        }
    }
}
