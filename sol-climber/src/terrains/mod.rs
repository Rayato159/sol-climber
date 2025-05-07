use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

pub mod death_zone;
pub mod map;
pub mod skybox;
pub mod sun;

fn extract_mesh_data(mesh: &Mesh) -> Option<(Vec<Vec3>, Vec<[u32; 3]>)> {
    if mesh.primitive_topology() != PrimitiveTopology::TriangleList {
        return None;
    }

    let vertex_positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?.as_float3()?;
    let vertices = vertex_positions.iter().map(|v| Vec3::from(*v)).collect();

    let indices = match mesh.indices()? {
        Indices::U32(ind) => ind.chunks(3).map(|i| [i[0], i[1], i[2]]).collect(),
        Indices::U16(ind) => ind
            .chunks(3)
            .map(|i| [i[0] as u32, i[1] as u32, i[2] as u32])
            .collect(),
    };

    Some((vertices, indices))
}
