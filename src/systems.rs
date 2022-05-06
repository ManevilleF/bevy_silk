#![allow(clippy::needless_pass_by_value)]
use crate::cloth::Cloth;
use crate::config::{ClothConfig, ClothTickUpdate};
use bevy::prelude::*;

#[allow(clippy::cast_possible_truncation)]
pub fn update_cloth(
    mut query: Query<(&mut Cloth, &GlobalTransform)>,
    config: Option<Res<ClothConfig>>,
    time_step: Res<ClothTickUpdate>,
    time: Res<Time>,
) {
    let config = config.as_deref().cloned().unwrap_or_default();
    let delta_time = match &*time_step {
        ClothTickUpdate::DeltaTime => time.delta_seconds(),
        ClothTickUpdate::FixedDeltaTime(dt) => *dt as f32,
    };
    for (mut cloth, transform) in query.iter_mut() {
        cloth.update_points(delta_time, &config);
        cloth.update_sticks(&config, transform);
    }
}
