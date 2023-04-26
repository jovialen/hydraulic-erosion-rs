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

fn setup_terrain(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(TerrainBundle {
        pbr: PbrBundle {
            mesh: Handle::default(),
            material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
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

        for (i, mut t) in terrains.iter_mut().enumerate() {
            if i != 0 {
                ui.separator();
            }

            ui.label(format!("Seed: {}", t.seed));

            let mut subd = t.subdivisions;
            let mut size = t.size;
            let r1 = ui.add(egui::Slider::new(&mut subd, 1..=300).text("Subdivisions"));
            let r2 = ui.add(egui::Slider::new(&mut size, 1.0..=100.0).text("Size"));

            if r1.changed() || r2.changed() {
                t.subdivisions = subd;
                t.size = size;
            }
        }
    });

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
