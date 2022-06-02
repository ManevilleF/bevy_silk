use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
use bevy_rapier3d::prelude::*;
use bevy_silk::cloth::Cloth;
use bevy_silk::prelude::*;
use rand::{thread_rng, Rng};
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

struct ClothMovement {
    sign: f32,
    t: f32,
}

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
        .insert_resource(ClothMovement { sign: -1.0, t: 0.0 })
        .add_startup_system(spawn_cloth)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(6.0))
                .with_system(shoot_balls),
        )
        .add_system(move_cloth)
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
        Vec3::new(30.0, 40.0, -30.0),
        Vec3::Y * 10.0,
    ));
    let mesh_handle = meshes.add(shape::Cube::new(2.0).into());
    [
        (Color::BLUE, [-10.0, 0.0]),
        (Color::GREEN, [10.0, 0.0]),
        (Color::YELLOW, [0.0, -10.0]),
        (Color::RED, [0.0, 10.0]),
    ]
    .map(|(color, [x, z])| {
        commands
            .spawn_bundle(PbrBundle {
                mesh: mesh_handle.clone(),
                transform: Transform::from_xyz(x, 1.0, z),
                material: materials.add(StandardMaterial {
                    base_color: color,
                    double_sided: true,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .insert(Collider::cuboid(1.0, 1.0, 1.0));
    });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(shape::Cube { size: 24.0 }.into()),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_xyz(0.0, -12.0, 0.0),
            ..Default::default()
        })
        .insert(Collider::cuboid(12.0, 12.0, 12.0));
}

fn spawn_cloth(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    let flag_texture = asset_server.load("Bevy.png");
    let (size_x, size_y) = (60, 40);
    let mesh = rectangle_mesh((size_x, size_y), (-Vec3::X * 0.5, -Vec3::Y * 0.5), Vec3::Z);
    let cloth = ClothBuilder::new().with_pinned_vertex_ids(0..size_x);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(flag_texture),
                cull_mode: None,    // Option required to render back faces correctly
                double_sided: true, // Option required to render back faces correctly
                ..Default::default()
            }),
            transform: Transform::from_xyz(15.0, 15.0, 15.0),
            ..Default::default()
        })
        .insert(cloth)
        .insert(ClothCollider {
            dampen_others: Some(0.02),
            ..Default::default()
        })
        .insert(Name::new("Cloth"));
}

fn move_cloth(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Cloth>>,
    mut movement: ResMut<ClothMovement>,
) {
    let delta_time = time.delta_seconds();
    for mut transform in query.iter_mut() {
        movement.t += delta_time * 2.0;
        transform.translation.z += movement.sign * delta_time * 2.0;
        if movement.t > 30.0 {
            movement.t = 0.0;
            movement.sign = -movement.sign;
        }
    }
}

fn shoot_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = thread_rng();
    let radius = rng.gen_range(1.0..3.0);
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
            transform: Transform::from_xyz(0.0, 0.0, -20.0),
            ..Default::default()
        })
        .insert(Velocity::linear(Vec3::new(
            rng.gen_range(-5.0..5.0),
            rng.gen_range(10.0..15.0),
            rng.gen_range(20.0..30.0),
        )))
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(radius))
        .insert(Name::new("Ball"));
}
