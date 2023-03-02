use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};
use bevy_silk::prelude::*;

mod camera_plugin;

#[derive(Debug, Clone, Reflect, Resource)]
struct MovingAnimation {
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
        .register_type::<MovingAnimation>()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(ResourceInspectorPlugin::<ClothConfig>::new())
        .add_plugin(ResourceInspectorPlugin::<MovingAnimation>::new())
        .add_plugin(camera_plugin::CameraPlugin)
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
    let mesh_handle = meshes.add(shape::Cube::new(1.0).into());
    [
        (Color::BLUE, [-10.0, 0.0]),
        (Color::GREEN, [10.0, 0.0]),
        (Color::YELLOW, [0.0, -10.0]),
        (Color::RED, [0.0, 10.0]),
    ]
    .map(|(color, [x, z])| {
        commands.spawn(PbrBundle {
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
    let cloth = ClothBuilder::new().with_pinned_vertex_ids(0..size_x);
    let base_entity = Some(
        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(0.0, 3.0, 0.0),
                    ..Default::default()
                },
                Name::new("Cloth Controller"),
            ))
            .with_children(|b| {
                b.spawn((
                    PbrBundle {
                        mesh: meshes.add(shape::Cube::new(2.0).into()),
                        material: materials.add(Color::WHITE.into()),
                        transform: Transform::from_xyz(10.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    Name::new("Cube"),
                ))
                .with_children(|b2| {
                    b2.spawn((
                        PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(StandardMaterial {
                                base_color_texture: Some(flag_texture),
                                cull_mode: None, // Option required to render back faces correctly
                                double_sided: true, // Option required to render back faces correctly
                                ..Default::default()
                            }),
                            transform: Transform::from_xyz(-1.0, 1.0, 1.01),
                            ..Default::default()
                        },
                        cloth,
                        Name::new("Cloth"),
                    ));
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
    base_transform.translation.y = 3.0 + (time.elapsed_seconds() * 3.0).sin() * 2.0;
}
