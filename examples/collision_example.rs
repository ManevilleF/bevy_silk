use bevy::prelude::*;
use bevy_core::FixedTimestep;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use bevy_silk::prelude::*;
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
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(InspectorPlugin::<ClothConfig>::new())
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(ClothPlugin)
        .insert_resource(ClothConfig {
            sticks_computation_depth: 4,
            acceleration_smoothing: AccelerationSmoothing::FixedCoefficient(0.005),
            ..Default::default()
        })
        .add_startup_system(spawn_cloth)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(2.0))
                .with_system(shoot_balls),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_rotation(Quat::from_rotation_y(5.0)),
        ..Default::default()
    });
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(20.0, 20.0, -20.0),
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
    asset_server: Res<AssetServer>,
) {
    let flag_texture = asset_server.load("Bevy.png");
    let (size_x, size_y) = (80, 40);
    let mesh = rectangle_mesh((size_x, size_y), (-Vec3::X * 0.5, -Vec3::Y * 0.5), Vec3::Z);
    let cloth = ClothBuilder::new().with_fixed_points(0..size_x);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(flag_texture),
                cull_mode: None,    // Option required to render back faces correctly
                double_sided: true, // Option required to render back faces correctly
                ..Default::default()
            }),
            transform: Transform::from_xyz(20.0, 20.0, 10.0),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Sensor(true))
        .insert(Collider::cuboid(0.0, 0.0, 0.0))
        .insert(cloth)
        .insert(Name::new("Cloth"));
}

fn shoot_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 1.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(
                shape::Icosphere {
                    radius,
                    subdivisions: 5,
                }
                .into(),
            ),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),
            ..Default::default()
        })
        .insert(Velocity::linear(Vec3::new(0.0, 10.0, 10.0)))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(Name::new("Ball"));
}
