//! # Bevy Cloth
//!
//! Cloth engine for Bevy
//!
//! by [Félix Lescaudey de Maneville](https://linktree.com/ManevilleF)
#![forbid(unsafe_code, missing_docs)]
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
#[doc(hidden)]
pub mod cloth;
#[doc(hidden)]
pub mod config;
mod mesh;
mod systems;

use bevy::app::{App, Plugin};
use bevy::prelude::{ParallelSystemDescriptorCoercion, SystemSet};

#[doc(hidden)]
pub mod prelude {
    pub use crate::{cloth::Cloth, config::ClothConfig, mesh::rectangle_mesh, ClothPlugin};
}

/// Plugin for cloth physics
#[derive(Copy, Clone, Default)]
pub struct ClothPlugin;

impl Plugin for ClothPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new().with_system(systems::update_cloth.label("CLOTH_UPDATE")),
        );
    }
}
