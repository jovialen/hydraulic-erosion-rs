use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use log::info;
use noise::{NoiseFn, Perlin};

#[derive(Bundle, Default)]
pub struct TerrainBundle {
    pub terrain: Terrain,

    #[bundle]
    pub pbr: PbrBundle,
}

#[derive(Component)]
pub struct Terrain {
    pub seed: u32,
    pub size: f32,
    pub subdivisions: u32,
}

impl Default for Terrain {
    fn default() -> Self {
        Self {
            seed: rand::random(),
            size: 10.0,
            subdivisions: 100,
        }
    }
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_terrain_mesh);
    }
}

fn create_terrain_mesh(
    mut query: Query<(&Terrain, &mut Handle<Mesh>), Or<(Changed<Terrain>, Added<Handle<Mesh>>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (terrain, mut handle) in query.iter_mut() {
        info!("Updating terrain mesh");
        *handle = meshes.set(
            handle.id(),
            generate_initial_terrain(terrain.seed, terrain.size, terrain.subdivisions),
        );
    }
}

/// Generate a mesh for un-eroded terrain.
fn generate_initial_terrain(seed: u32, size: f32, subdivisions: u32) -> Mesh {
    let mut mesh = Mesh::from(shape::Plane { size, subdivisions });

    let noise = Perlin::new(seed);
    update_mesh_positions(&mut mesh, move |mut vertex| {
        let point: [f64; 2] = [vertex.x as f64 / 1.5, vertex.z as f64 / 1.5];
        vertex.y = noise.get(point) as f32;
        vertex
    });

    calculate_mesh_normals(&mut mesh);

    mesh
}

/// Modify all vertex positions in a mesh with the function `f`.
fn update_mesh_positions<F>(mesh: &mut Mesh, mut f: F)
where
    F: FnMut(Vec3) -> Vec3,
{
    // Update mesh positions
    let positions = match mesh
        .attribute_mut(Mesh::ATTRIBUTE_POSITION)
        .expect("mesh vertex positions")
    {
        VertexAttributeValues::Float32x3(positions, ..) => positions,
        _ => unreachable!("mesh vertex positions should be of type Vec3"),
    };

    for position in positions.iter_mut() {
        *position = f(Vec3::from_array(*position)).to_array();
    }
}

fn calculate_mesh_normals(mesh: &mut Mesh) {
    let indices = mesh
        .indices()
        .expect("mesh indices")
        .iter()
        .collect::<Vec<_>>();

    let positions = match mesh
        .attribute(Mesh::ATTRIBUTE_POSITION)
        .expect("mesh vertex positions")
    {
        VertexAttributeValues::Float32x3(positions, ..) => positions.clone(),
        _ => unreachable!("mesh vertex positions should be of type Vec3"),
    };

    let normals = match mesh
        .attribute_mut(Mesh::ATTRIBUTE_NORMAL)
        .expect("mesh vertex normals")
    {
        VertexAttributeValues::Float32x3(normals, ..) => normals,
        _ => unreachable!("mesh vertex normals should be of type Vec3"),
    };

    // Clear all normals
    normals
        .iter_mut()
        .for_each(|normal| *normal = [0.0, 0.0, 0.0]);

    // Calculate new normals
    for chunk in indices.chunks_exact(3) {
        if let [a, b, c] = chunk[..] {
            let va = Vec3::from_array(positions[a]);
            let vb = Vec3::from_array(positions[b]);
            let vc = Vec3::from_array(positions[c]);

            let vba = vb - va;
            let vca = vc - va;
            let nf = vba.cross(vca);
            let p = nf / nf.length();

            normals[a][0] += p.x;
            normals[b][0] += p.x;
            normals[c][0] += p.x;
            normals[a][1] += p.y;
            normals[b][1] += p.y;
            normals[c][1] += p.y;
            normals[a][2] += p.z;
            normals[b][2] += p.z;
            normals[c][2] += p.z;
        }
    }
}
