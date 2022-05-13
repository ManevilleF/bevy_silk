//! # Bevy Silk
//!
//! [![workflow](https://github.com/ManevilleF/bevy_silk/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/bevy_silk/actions/workflows/rust.yml)
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//! [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
//! [![Crates.io](https://img.shields.io/crates/v/bevy_silk.svg)](https://crates.io/crates/bevy_silk)
//! [![Docs.rs](https://docs.rs/bevy_silk/badge.svg)](https://docs.rs/bevy_silk)
//! [![dependency status](https://deps.rs/crate/bevy_silk/0.1.0/status.svg)](https://deps.rs/crate/bevy_silk)
//!
//! CPU driven Cloth engine for Bevy using Verlet integration.
//!
//! by [Félix Lescaudey de Maneville](https://linktree.com/ManevilleF)
//!
//! ## Get started
//!
//! ### Dependency
//!
//! Add `bevy_silk` as a dependency in the `Cargo.toml`
//!
//! `bevy_silk = "0.1"`
//!
//! Or follow the main git branch
//!
//! `bevy_silk = { git = "https://github.com/ManevilleF/bevy_silk" }`
//!
//! ### Plugin
//!
//! Add the `ClothPlugin` to your bevy app
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_silk::prelude::*;
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
//! For a mesh to be used as cloth, add the `ClothBuilder` component to any entity with a `Handle<Mesh>` component.
//!
//! > Note: `Transform` and `GlobalTransform` are also required
//!
//! cloth data which will be populated automatically from the associated `Handle<Mesh>`.
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_silk::prelude::*;
//!
//! fn spawn(mut commands: Commands) {
//!     commands.spawn_bundle(PbrBundle {
//!         // Add your mesh, material and your custom PBR data   
//!         ..Default::default()
//!     }).insert(ClothBuilder::new()
//!         // Define fixed vertices using an Iterator
//!         .with_fixed_points(0..9)
//!         // Define the stick generation mode
//!         .with_stick_generation(StickGeneration::Quads)
//!         // Defines the sticks target length option
//!         .with_stick_length(StickLen::Auto)
//!         // The cloth will compute flat mesh normals
//!         .with_flat_normal_computation()
//!         // ...
//!     );
//! }
//! ```
//!
//! ### Configuration
//!
//! You can customize the global cloth physics by inserting the `ClothConfig` resource to your app:
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_silk::prelude::*;
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
//! `ClothConfig` can also be used as a *component* to override the global configuration.
//!
//! ## Wind
//!
//! You may add wind forces to the simulation for a more dynamic clothing effect, for each force you may choose from:
//! - `Wind::Constant` for constant wind force
//! - `Wind::SinWave` for a sinwave following wind intensity with custom force and frequency.
//!
//! `Wind` forces can be added as a resource to your app through the `Winds` container:
//!
//! ```rust no_run
//! use bevy::prelude::*;
//! use bevy_silk::prelude::*;
//!
//! fn main() {
//!   App::new()
//!     .add_plugins(DefaultPlugins)
//!     .insert_resource(Winds {
//!         wind_forces: vec![Wind::SinWave {
//!             max_velocity: Vec3::new(10.0, 15.0, -5.0),
//!             frequency: 3.0,
//!             normalize: false,
//!             abs: false
//!         }]
//!     })
//!     .add_plugin(ClothPlugin)
//!     // ... Add your resources and systems
//!     .run();
//! }
//! ```
//!
//! > Check the flag example for simple wind effect.
//!
//! ## Mesh utils
//!
//! `bevy_silk` provides a plane mesh generation function `rectangle_mesh` useful for classic cloth uses like flags or capes
//!
//! ## Q&A
//!
//! - `My mesh falls immediately and infinitely when I add a Cloth component, how to fix it?`
//!
//! You probably didn't specify any *fixed points*, meaning there are no vertices anchored to your entity's `GlobalTransform`.
//!
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
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
/// cloth builder module
pub mod cloth_builder;
/// cloth rendering module
pub mod cloth_rendering;
/// config module
pub mod config;
/// error module
pub mod error;
/// mesh module
pub mod mesh;
/// stick module
pub mod stick;
/// systems module
mod systems;
/// wind module
pub mod wind;

use crate::cloth::Cloth;
use crate::prelude::*;
use bevy_app::{App, Plugin};
use bevy_ecs::schedule::ParallelSystemDescriptorCoercion;

/// Prelude module, providing every public type of the lib
pub mod prelude {
    pub use crate::{
        cloth_builder::ClothBuilder,
        cloth_rendering::NormalComputing,
        config::ClothConfig,
        error::Error,
        mesh::rectangle_mesh,
        stick::{StickGeneration, StickLen},
        wind::{Wind, Winds},
        ClothPlugin,
    };
}

/// Plugin for cloth physics
#[derive(Copy, Clone, Default)]
pub struct ClothPlugin;

impl Plugin for ClothPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClothConfig>();
        app.register_type::<ClothConfig>()
            .register_type::<Wind>()
            .register_type::<Winds>()
            .register_type::<ClothBuilder>()
            .register_type::<Cloth>();
        app.add_system(systems::init_cloth.label("CLOTH_INIT"));
        app.add_system(systems::update_cloth.label("CLOTH_UPDATE"));
    }
}
