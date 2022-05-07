#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style,
    rustdoc::broken_intra_doc_links
)]
#![allow(
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::redundant_pub_crate
)]

pub mod cloth;
pub mod config;
mod mesh;
pub mod stick;
mod systems;

use crate::config::ClothTickUpdate;
use bevy::app::{App, Plugin};
use bevy::core::FixedTimestep;
use bevy::prelude::{ParallelSystemDescriptorCoercion, SystemSet};

pub mod prelude {
    pub use crate::{
        cloth::Cloth,
        config::{ClothConfig, ClothTickUpdate},
        stick::Stick,
        ClothPlugin,
    };
}

#[derive(Copy, Clone, Default)]
pub struct ClothPlugin {
    /// Defines a custom time step for cloth update.
    /// If not set, cloths will be updated every frame
    pub custom_tick: Option<f64>,
}

impl Plugin for ClothPlugin {
    fn build(&self, app: &mut App) {
        let system_set = SystemSet::new().with_system(systems::update_cloth.label("CLOTH_UPDATE"));
        let system_set = if let Some(tick) = self.custom_tick {
            app.insert_resource(ClothTickUpdate::FixedDeltaTime(tick));
            system_set.with_run_criteria(FixedTimestep::step(tick))
        } else {
            app.insert_resource(ClothTickUpdate::DeltaTime);
            system_set
        };
        app.add_system_set(system_set);
    }
}
