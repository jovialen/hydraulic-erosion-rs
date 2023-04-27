use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

#[derive(Bundle, Default)]
pub struct OrbitUserCameraBundle {
    pub user_input: UserInput,

    #[bundle]
    pub orbit: OrbitCameraBundle,
}

#[derive(Bundle, Default)]
pub struct OrbitCameraBundle {
    pub orbit: OrbitComponent,

    #[bundle]
    pub camera: Camera3dBundle,
}

#[derive(Component)]
pub struct UserInput {
    pub move_speed: f32,
    pub rotation_speed: f32,
    pub zoom_speed: f32,
}

impl Default for UserInput {
    fn default() -> Self {
        Self {
            move_speed: 3.0,
            rotation_speed: 40.0,
            zoom_speed: 50.0,
        }
    }
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

fn target_movement(
    mut orbits: Query<(&mut OrbitComponent, &UserInput)>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut scroll: EventReader<MouseWheel>,
    mut motion: EventReader<MouseMotion>,
) {
    let dt = time.delta_seconds();

    for (mut orbit, user_input) in orbits.iter_mut() {
        let mut velocity = Vec3::ZERO;

        // Position
        if keys.any_pressed([KeyCode::W, KeyCode::Up]) {
            velocity.x -= user_input.move_speed * dt;
        }

        if keys.any_pressed([KeyCode::S, KeyCode::Down]) {
            velocity.x += user_input.move_speed * dt;
        }

        if keys.any_pressed([KeyCode::A, KeyCode::Left]) {
            velocity.z += user_input.move_speed * dt;
        }

        if keys.any_pressed([KeyCode::D, KeyCode::Right]) {
            velocity.z -= user_input.move_speed * dt;
        }

        if keys.pressed(KeyCode::LShift) {
            velocity *= 2.0;
        }

        velocity = Quat::from_rotation_y(-orbit.rotation.x.to_radians()) * velocity;
        orbit.target += velocity;

        // Rotation
        if mouse.pressed(MouseButton::Middle) {
            for ev in motion.iter() {
                orbit.rotation.x += user_input.rotation_speed * ev.delta.x * dt;
                orbit.rotation.y += user_input.rotation_speed * ev.delta.y * dt;
            }
        }

        // Distance from target
        for ev in scroll.iter() {
            orbit.distance -= user_input.zoom_speed * ev.y * dt;
        }
    }
}
