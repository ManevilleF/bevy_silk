use bevy::pbr::wireframe::{Wireframe, WireframePlugin};
use bevy::prelude::*;
use bevy_cloth::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor::default())
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(InspectorPlugin::<ClothConfig>::new())
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(WireframePlugin)
        .insert_resource(ClothConfig {
            gravity: -Vec3::Y,
            sticks_computation_depth: 10,
            ..Default::default()
        })
        .add_plugin(ClothPlugin)
        .add_startup_system(spawn_cloth)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(20.0, 20.0, 20.0),
        Vec3::ZERO,
    ));
    let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
    [
        (Color::BLUE, [-10.0, 0.0]),
        (Color::GREEN, [10.0, 0.0]),
        (Color::YELLOW, [0.0, -10.0]),
        (Color::RED, [0.0, 10.0]),
    ]
    .map(|(color, [x, z])| {
        commands.spawn_bundle(PbrBundle {
            mesh: mesh_handle.clone(),
            transform: Transform::from_xyz(x, 0.0, z),
            material: materials.add(StandardMaterial {
                base_color: color,
                double_sided: true,
                ..Default::default()
            }),
            ..Default::default()
        });
    });
}

fn spawn_cloth(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius: 5.0,
                    subdivisions: 10,
                }
                .into(),
            ),
            material: materials.add(Color::YELLOW.into()),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..Default::default()
        })
        .insert(ClothBuilder::new().with_fixed_points(0..=0).build())
        .insert(Wireframe)
        .insert(Name::new("Balloon"));
}
