mod orbit_camera;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::pbr::prelude::*;
use bevy::pbr::wireframe::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use noise::{NoiseFn, Perlin};
use orbit_camera::*;

#[derive(Bundle, Default)]
struct TerrainBundle {
    #[bundle]
    pbr: PbrBundle,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WireframePlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_lights)
        .add_startup_system(setup_terrain)
        .add_system(config_system)
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
        let point: [f64; 2] = [vertex.x as f64 / 1.5, vertex.z as f64 / 1.5];
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

fn config_system(
    mut contexts: EguiContexts,
    mut wireframe_config: ResMut<WireframeConfig>,
    diagnostics: Res<Diagnostics>,
) {
    egui::Window::new("Renderer").show(contexts.ctx_mut(), |ui| {
        ui.set_min_width(200.0);

        ui.label("Settings");
        ui.checkbox(&mut wireframe_config.global, "Enable wireframe");

        ui.add_space(10.0);

        ui.label("Diagnostics");
        ui.group(|ui| {
            let fps = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .expect("fps diagnostic");
            let frame_count = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .expect("frame count diagnostic");
            let frame_time = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
                .expect("frame time diagnostic");

            ui.label(format!(
                "Frames per second: {} (avg {})",
                fps.smoothed().unwrap_or(0.0).round() as i32,
                fps.average().unwrap_or(0.0).round() as i32
            ));
            ui.label(format!(
                "Time to render frame: {:.3}ms (avg {:.3}ms)",
                frame_time.smoothed().unwrap_or(0.0),
                frame_time.average().unwrap_or(0.0)
            ));
            ui.label(format!(
                "Total frames: {}",
                frame_count.value().unwrap_or(0.0).floor() as i32
            ));
        })
    });
}
