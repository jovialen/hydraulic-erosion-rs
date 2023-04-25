use bevy::pbr::prelude::*;
use bevy::pbr::wireframe::*;
use bevy::prelude::*;

#[derive(Bundle, Default)]
struct TerrainBundle {
    #[cfg(feature = "wireframe")]
    wireframe: Wireframe,

    #[bundle]
    pbr: PbrBundle,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_lights)
        .add_startup_system(setup_terrain)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn setup_lights(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 10.0, 0.0),
        ..default()
    });
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(TerrainBundle {
        pbr: PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(5.0).into()),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
}
