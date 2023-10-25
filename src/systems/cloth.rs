#![allow(
    clippy::needless_pass_by_value,
    clippy::type_complexity,
    clippy::option_if_let_else
)]
use crate::{
    components::{cloth::Cloth, cloth_builder::ClothBuilder, cloth_rendering::ClothRendering},
    config::ClothConfig,
    wind::Winds,
};
use bevy::{log, math::Vec3, prelude::*, render::primitives::Aabb};

pub fn update(
    mut query: Query<(&mut Cloth, &GlobalTransform, Option<&ClothConfig>)>,
    anchor_query: Query<&GlobalTransform, Without<Cloth>>,
    config: Res<ClothConfig>,
    wind: Option<Res<Winds>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    let wind_force = wind.map_or(Vec3::ZERO, |w| w.current_velocity(time.elapsed_seconds()));
    for (mut cloth, transform, custom_config) in &mut query {
        let config: &ClothConfig = custom_config.unwrap_or(&config);
        cloth.update_points(
            config.friction_coefficient(),
            config.smoothed_acceleration(wind_force + config.gravity, delta_time),
        );
        cloth.update_anchored_points(transform, |entity| {
            if let Ok(t) = anchor_query.get(entity) {
                Some(t)
            } else {
                log::error!("Could not find cloth anchor target entity {:?}", entity);
                None
            }
        });
        cloth.update_sticks(config.sticks_computation_depth);
    }
}

pub fn render(
    mut cloth_query: Query<(
        &Cloth,
        &mut ClothRendering,
        &mut Aabb,
        &GlobalTransform,
        &Handle<Mesh>,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (cloth, mut rendering, mut aabb, transform, handle) in &mut cloth_query {
        if let Some(mesh) = meshes.get_mut(handle) {
            rendering.update_positions(cloth.compute_vertex_positions(transform));
            rendering.apply(mesh);
            // TODO set_if_neq
            *aabb = rendering.compute_aabb();
        } else {
            log::warn!("A Cloth has a `ClothRendering` component without a loaded mesh handle");
        }
    }
}

pub fn init(
    mut commands: Commands,
    mut query: Query<(Entity, &ClothBuilder, &GlobalTransform, &Handle<Mesh>), Added<ClothBuilder>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, builder, transform, handle) in &mut query {
        if let Some(mesh) = meshes.get(handle) {
            let matrix = transform.compute_matrix();
            log::debug!("Initializing Cloth entity {:?}", entity);
            let rendering = ClothRendering::init(mesh, builder.normals_computing).unwrap();
            let aabb = rendering.compute_aabb();
            let cloth = Cloth::new(
                &rendering.vertex_positions,
                &rendering.indices,
                builder.anchored_vertex_ids(mesh),
                builder.stick_generation,
                builder.stick_length,
                builder.default_stick_mode,
                &matrix,
            );
            commands.entity(entity).insert((rendering, cloth, aabb));
        }
    }
}
