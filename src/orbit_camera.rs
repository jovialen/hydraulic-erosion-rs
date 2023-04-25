use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct OrbitCameraBundle {
    pub orbit: OrbitComponent,

    #[bundle]
    pub camera: Camera3dBundle,
}

#[derive(Component)]
pub struct OrbitComponent {
    pub target: Vec3,
    pub rotation: Vec2,
    pub distance: f32,
}

impl Default for OrbitComponent {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            rotation: Vec2 { x: 0.0, y: 45.0 },
            distance: 10.0,
        }
    }
}

#[derive(Default)]
pub struct OrbitCameraPlugin;

impl Plugin for OrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_orbit).add_system(target_movement);
    }
}

fn update_orbit(mut orbits: Query<(&mut Transform, &OrbitComponent)>) {
    for (mut transform, orbit) in orbits.iter_mut() {
        let rot_x = orbit.rotation.x.to_radians();
        let rot_y = orbit.rotation.y.to_radians();

        let horizontal_distance = orbit.distance * rot_y.cos();
        let y = orbit.distance * rot_y.sin();
        let x = horizontal_distance * rot_x.cos();
        let z = horizontal_distance * rot_x.sin();
        let offset = Vec3 { x, y, z };

        transform.translation = orbit.target + offset;
        transform.look_at(orbit.target, Vec3::Y);
    }
}

fn target_movement(mut orbits: Query<&mut OrbitComponent>, time: Res<Time>) {
    for mut orbit in orbits.iter_mut() {
        orbit.rotation.x += 20.0 * time.delta_seconds();
    }
}
