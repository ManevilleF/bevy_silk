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
pub mod point;
pub mod stick;
mod systems;

use bevy::app::{App, Plugin};

pub mod prelude {
    pub use crate::{cloth::Cloth, config::ClothConfig, point::Point, ClothPlugin};
}

pub struct ClothPlugin;

impl Plugin for ClothPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<cloth::Cloth>();
    }
}
