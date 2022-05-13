use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin, WorldInspectorPlugin};
use bevy_silk::prelude::*;
use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

#[derive(Debug, Clone, Inspectable)]
struct MovingAnimation {
    #[inspectable(collapse)]
    pub base_entity: Option<Entity>,
    pub rotation_speed: f32,
}

impl Default for MovingAnimation {
    fn default() -> Self {
        Self {
            base_entity: None,
            rotation_speed: 1.0,
        }
    }
}

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
        .add_plugin(InspectorPlugin::<MovingAnimation>::new())
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(ClothPlugin)
        .add_startup_system(spawn_cloth)
        .add_startup_system(setup)
        .add_system(animate_cube)
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
    asset_server: Res<AssetServer>,
) {
    let flag_texture = asset_server.load("France.png");
    let (size_x, size_y) = (20, 40);
    let mesh = rectangle_mesh((size_x, size_y), (Vec3::X * 0.1, -Vec3::Y * 0.1), Vec3::Z);
    let cloth = ClothBuilder::new().with_fixed_points(0..size_x);
    let base_entity = Some(
        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(0.0, 3.0, 0.0),
                ..Default::default()
            })
            .insert(Name::new("Cloth Controller"))
            .with_children(|b| {
                b.spawn_bundle(PbrBundle {
                    mesh: meshes.add(shape::Cube::new(2.0).into()),
                    material: materials.add(Color::WHITE.into()),
                    transform: Transform::from_xyz(10.0, 0.0, 0.0),
                    ..Default::default()
                })
                .insert(Name::new("Cube"))
                .with_children(|b2| {
                    b2.spawn_bundle(PbrBundle {
                        mesh: meshes.add(mesh),
                        material: materials.add(StandardMaterial {
                            base_color_texture: Some(flag_texture),
                            cull_mode: None, // Option required to render back faces correctly
                            double_sided: true, // Option required to render back faces correctly
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(-1.0, 1.0, 1.01),
                        ..Default::default()
                    })
                    .insert(cloth)
                    .insert(Name::new("Cloth"));
                });
            })
            .id(),
    );
    commands.insert_resource(MovingAnimation {
        base_entity,
        ..Default::default()
    });
}

fn animate_cube(
    animation: Res<MovingAnimation>,
    mut query: Query<&mut Transform>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    let mut base_transform = query.get_mut(animation.base_entity.unwrap()).unwrap();
    base_transform.rotate(Quat::from_rotation_y(delta_time * animation.rotation_speed));
    base_transform.translation.y =
        3.0 + (time.time_since_startup().as_secs_f32() * 3.0).sin() * 2.0;
}
