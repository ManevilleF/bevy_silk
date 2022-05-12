#![allow(clippy::needless_pass_by_value)]

use crate::cloth::Cloth;
use crate::config::ClothConfig;
use crate::wind::*;
use bevy_asset::{Assets, Handle};
use bevy_core::Time;
use bevy_ecs::prelude::*;
use bevy_log::debug;
use bevy_math::Vec3;
use bevy_render::prelude::Mesh;
use bevy_transform::prelude::GlobalTransform;

#[allow(clippy::cast_possible_truncation)]
pub fn update_cloth(
    mut query: Query<(
        &mut Cloth,
        &GlobalTransform,
        &Handle<Mesh>,
        Option<&ClothConfig>,
    )>,
    config: Res<ClothConfig>,
    wind: Option<Res<Winds>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let delta_time = time.delta_seconds();
    let wind_force = wind.map_or(Vec3::ZERO, |w| {
        w.current_velocity(time.time_since_startup().as_secs_f32())
    });
    for (mut cloth, transform, handle, custom_config) in query.iter_mut() {
        if let Some(mesh) = meshes.get_mut(handle) {
            let matrix = transform.compute_matrix();
            if !(cloth.is_setup()) {
                debug!("Setting up sticks for uninitialized cloth");
                cloth.init_from_mesh(mesh, &matrix);
            }
            cloth.update(
                custom_config.unwrap_or(&config),
                delta_time,
                &matrix,
                wind_force,
            );
            cloth.apply_to_mesh(mesh, &matrix);
        }
    }
}
