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

use bevy::app::{App, Plugin};
use bevy::prelude::{ParallelSystemDescriptorCoercion, SystemSet};

pub mod prelude {
    pub use crate::{cloth::Cloth, config::ClothConfig, stick::Stick, ClothPlugin};
}

#[derive(Copy, Clone, Default)]
pub struct ClothPlugin;

impl Plugin for ClothPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new().with_system(systems::update_cloth.label("CLOTH_UPDATE")),
        );
    }
}
