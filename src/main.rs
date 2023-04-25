mod orbit_camera;

use bevy::pbr::prelude::*;
use bevy::pbr::wireframe::*;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use orbit_camera::*;

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
        .add_plugin(OrbitCameraPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_lights)
        .add_startup_system(setup_terrain)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(OrbitCameraBundle {
        camera: Camera3dBundle {
            transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
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
    let mut terrain_mesh = Mesh::from(shape::Plane {
        size: 10.0,
        subdivisions: 100,
    });

    let noise = Perlin::new(rand::random());
    update_mesh_positions(&mut terrain_mesh, move |mut vertex| {
        let point: [f64; 2] = [vertex.x as f64, vertex.z as f64];
        vertex.y = noise.get(point) as f32;
        vertex
    });

    commands.spawn(TerrainBundle {
        pbr: PbrBundle {
            mesh: meshes.add(terrain_mesh),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
}

fn update_mesh_positions<F>(mesh: &mut Mesh, mut f: F)
where
    F: FnMut(Vec3) -> Vec3,
{
    use bevy::render::mesh::VertexAttributeValues;

    let positions = mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION).unwrap();

    if let VertexAttributeValues::Float32x3(vertices, ..) = positions {
        for vertex in vertices {
            *vertex = f(Vec3::from_array(*vertex)).to_array();
        }
    }
}
