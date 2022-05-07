#![allow(clippy::needless_pass_by_value)]
use crate::cloth::Cloth;
use crate::config::ClothConfig;
use bevy::log;
use bevy::prelude::*;

#[allow(clippy::cast_possible_truncation)]
pub fn update_cloth(
    mut query: Query<(&mut Cloth, &GlobalTransform, &Handle<Mesh>)>,
    config: Option<Res<ClothConfig>>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let config = config.as_deref().cloned().unwrap_or_default();
    let delta_time = time.delta_seconds();
    for (mut cloth, transform, handle) in query.iter_mut() {
        if let Some(mesh) = meshes.get_mut(handle) {
            let matrix = transform.compute_matrix();
            if cloth.is_setup() {
                cloth.update(&config, delta_time, &matrix);
            } else {
                log::debug!("Setting up sticks for uninitialized cloth");
                cloth.init_from_mesh(mesh, &matrix);
            }
            cloth.apply_to_mesh(mesh, &matrix);
        }
    }
}
