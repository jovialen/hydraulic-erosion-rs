mod orbit_camera;
mod terrain;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::pbr::prelude::*;
use bevy::pbr::wireframe::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use orbit_camera::*;
use terrain::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(WireframePlugin)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_lights)
        .add_startup_system(setup_terrain)
        .add_system(config_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(OrbitUserCameraBundle::default());
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
    let empty_mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);

    commands.spawn(TerrainBundle {
        pbr: PbrBundle {
            mesh: meshes.add(empty_mesh.clone()),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });

    commands.spawn(TerrainBundle {
        pbr: PbrBundle {
            mesh: meshes.add(empty_mesh),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(15.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    });
}

fn config_system(
    mut contexts: EguiContexts,
    mut terrains: Query<&mut Terrain>,
    mut wireframe_config: ResMut<WireframeConfig>,
    diagnostics: Res<Diagnostics>,
) {
    egui::Window::new("Terrain").show(contexts.ctx_mut(), |ui| {
        ui.set_min_width(200.0);

        for (i, mut terrain) in terrains.iter_mut().enumerate() {
            egui::CollapsingHeader::new(format!("Terrain {i}")).show(ui, |ui| {
                egui::Grid::new(format!("terrain {i}"))
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        let mut seed = terrain.seed;
                        let mut subd = terrain.subdivisions;
                        let mut size = terrain.size;

                        let mut changed = false;

                        ui.horizontal(|ui| {
                            changed |= ui.add(egui::DragValue::new(&mut seed)).changed();
                            if ui.button("Random").clicked() {
                                seed = rand::random();
                                changed = true;
                            }
                        });
                        ui.label("Seed");
                        ui.end_row();

                        changed |= ui.add(egui::Slider::new(&mut subd, 1..=300)).changed();
                        ui.label("Subdivisions");
                        ui.end_row();

                        changed |= ui.add(egui::Slider::new(&mut size, 1.0..=100.0)).changed();
                        ui.label("Size");
                        ui.end_row();

                        // Must be done to avoid triggering the mesh regeneration every frame
                        if changed {
                            terrain.seed = seed;
                            terrain.subdivisions = subd;
                            terrain.size = size;
                        }
                    });
            });
        }
    });

    egui::Window::new("Renderer")
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::Vec2::new(-10.0, -10.0))
        .default_open(false)
        .movable(false)
        .resizable(false)
        .show(contexts.ctx_mut(), |ui| {
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
