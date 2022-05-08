//! # Bevy Cloth
//!
//! [![workflow](https://github.com/ManevilleF/bevy_cloth/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_cloth/actions/workflows/rust.yml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//!
//! CPU driven Cloth engine for Bevy using Verlet integration.
//!
//! by [Félix Lescaudey de Maneville](https://linktree.com/ManevilleF)
//!
//! ## Get started
//!
//! ### Dependency
//!
//! Add `bevy_cloth` as a dependency in the `Cargo.toml`
//!
//! `bevy_cloth = { git = "https://github.com/ManevilleF/bevy_cloth" }`
//!
//! ### Plugin
//!
//! Add the `ClothPlugin` to your bevy app
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_cloth::prelude::*;
//!
//! fn main() {
//!   App::new()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(ClothPlugin)
//!     // ... Add your resources and systems
//!     .run();
//! }
//! ```
//!
//! ### Add cloth to a mesh
//!
//! For a mesh to be used as cloth, add the `Cloth` component to any entity with a `Handle<Mesh>` component.
//!
//! > Note: `Transform` and `GlobalTransform` are also required
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_cloth::prelude::*;
//!
//! fn spawn(mut commands: Commands) {
//!     commands.spawn_bundle(PbrBundle {
//!         // Add your mesh, material and your custom PBR dat    
//!         ..Default::default()
//!     }).insert(Cloth::new());
//! }
//! ```
//!
//! The entity's mesh will now behave as cloth and will fall downwards.
//! To avoid this, you need to specify **fixed points** which will keep the cloth attached to the entity.
//! To do this you need to specify the vertex indexes to keep fixed by:
//! - Using `Cloth::with_fixed_points` instead of `Cloth::new`
//! - Editing the `Cloth::fixed_points` field
//!
//! ### Configuration
//!
//! You can customize the cloth physics by inserting the `ClothConfig` resource to your app:
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_cloth::prelude::*;
//!
//! fn main() {
//!   App::new()
//!     .add_plugins(DefaultPlugins)
//!     .insert_resource(ClothConfig {
//!         gravity: Vec3::new(0.0, -9.81, 0.0),
//!         friction: 0.02,
//!         sticks_computation_depth: 5
//!     })
//!     .add_plugin(ClothPlugin)
//!     // ... Add your resources and systems
//!     .run();
//! }
//! ```
//!
//! ## Mesh utils
//!
//! `bevy_cloth` provides a plane mesh generation function `rectangle_mesh` useful for classic cloth uses like flags or capes
//!
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
/// cloth module
pub mod cloth;
/// config module
pub mod config;
/// mesh module
pub mod mesh;
/// systems module
mod systems;
/// wind module
pub mod wind;

use crate::cloth::Cloth;
use crate::config::ClothConfig;
use crate::wind::Wind;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};

/// Prelude module, providing every public type of the lib
pub mod prelude {
    pub use crate::{
        cloth::{Cloth, StickGeneration},
        config::ClothConfig,
        mesh::rectangle_mesh,
        wind::Wind,
        ClothPlugin,
    };
}

/// Plugin for cloth physics
#[derive(Copy, Clone, Default)]
pub struct ClothPlugin;

impl Plugin for ClothPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClothConfig>();
        app.register_type::<ClothConfig>();
        app.register_type::<Wind>();
        app.register_type::<Cloth>();
        app.add_system_set(
            SystemSet::new().with_system(systems::update_cloth.label("CLOTH_UPDATE")),
        );
        #[cfg(feature = "debug")]
        {
            use bevy_inspector_egui::RegisterInspectable;
            app.register_inspectable::<ClothConfig>();
            app.register_inspectable::<Wind>();
        }
    }
}
