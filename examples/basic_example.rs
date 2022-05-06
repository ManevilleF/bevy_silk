use bevy::prelude::*;
use bevy_cloth::ClothPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(ClothPlugin)
        .add_startup_system(spawn_cloth)
        .add_startup_system(spawn_camera)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle::new_3d());
}

fn spawn_cloth() {}
