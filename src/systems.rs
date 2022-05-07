#![allow(clippy::needless_pass_by_value)]
use crate::cloth::Cloth;
use crate::config::{ClothConfig, ClothTickUpdate};
use bevy::log;
use bevy::prelude::*;

#[allow(clippy::cast_possible_truncation)]
pub fn update_cloth(
    mut query: Query<(&mut Cloth, &GlobalTransform, &Handle<Mesh>)>,
    config: Option<Res<ClothConfig>>,
    time_step: Res<ClothTickUpdate>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let config = config.as_deref().cloned().unwrap_or_default();
    let delta_time = match &*time_step {
        ClothTickUpdate::DeltaTime => time.delta_seconds(),
        ClothTickUpdate::FixedDeltaTime(dt) => *dt as f32,
    };
    for (mut cloth, transform, handle) in query.iter_mut() {
        if let Some(mesh) = meshes.get_mut(handle) {
            if cloth.is_setup() {
                cloth.compute_mesh(mesh, transform, &config, delta_time);
            } else {
                log::debug!("Setting up sticks for uninitialized cloth");
                cloth.setup_sticks(mesh);
            }
        }
    }
}
