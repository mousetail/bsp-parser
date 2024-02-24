use std::collections::HashMap;

use crate::vector::{Vec2, Vec3};

use super::{
    brush_model::BrushModel,
    edge::Edge,
    face::Face,
    plane::Plane,
    surfedges::SurfEdge,
    texdata::TextureData,
    texinfo::{surface_flags, TextureInfo},
    texture_string_array::{TextureDataStringArray, TextureString},
    vertex::Vertex,
};

pub(super) struct MaterialGroup {
    pub verticies: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<usize>,
}

pub(super) fn to_primitives(
    bsp_faces: Vec<Face>,
    bsp_planes: Vec<Plane>,
    bsp_vertexes: Vec<Vertex>,
    bsp_edges: Vec<Edge>,
    bsp_surfedges: Vec<SurfEdge>,
    texture_infos: Vec<TextureInfo>,
    bsp_texture_data: Vec<TextureData>,
    bsp_texture_string_array: TextureDataStringArray,
    bsp_texture_string_table: Vec<TextureString>,
    bsp_models: Vec<BrushModel>,
) -> HashMap<String, MaterialGroup> {
    let mut groups: HashMap<String, MaterialGroup> = HashMap::new();

    let get_edge = |surface_edge: &SurfEdge| -> Edge {
        if surface_edge.0 > 0 {
            return bsp_edges[surface_edge.0 as usize];
        } else {
            return bsp_edges[-surface_edge.0 as usize].reverse();
        }
    };

    for (_index, face) in bsp_faces[bsp_models[0].first_face as usize
        ..bsp_models[0].first_face as usize + bsp_models[0].num_faces as usize]
        .iter()
        .enumerate()
    {
        let face_edges: Vec<_> = bsp_surfedges
            [face.first_edge as usize..(face.first_edge + face.num_edges as u32) as usize]
            .iter()
            .map(get_edge)
            .collect();
        let normal = bsp_planes[face.planenum as usize].normal;

        if face.displacement_info != -1 {
            continue;
        }

        let texture_info = texture_infos[face.tex_info as usize];

        if texture_info.flags & surface_flags::SURF_NODRAW > 0
            || texture_info.flags & surface_flags::SURF_SKIP > 0
            || texture_info.flags & surface_flags::SURF_SKY > 0
            || texture_info.flags & surface_flags::SURF_HINT > 0
        {
            continue;
        }

        let texture_data = bsp_texture_data[texture_info.texture_data_index as usize];
        let texture_name = bsp_texture_string_array
            .get_str(bsp_texture_string_table[texture_data.name_index as usize].0 as usize)
            .unwrap();
        if texture_name.starts_with("TOOL") {
            continue;
        }

        let category = match texture_name.split_once('/').unwrap_or(("", "")) {
            ("maps", k) => k.split('/').nth(1).unwrap().to_ascii_uppercase(),
            (a, _) => a.to_ascii_uppercase(),
        };

        let group = groups.entry(category).or_insert(MaterialGroup {
            verticies: vec![],
            normals: vec![],
            uvs: vec![],
            indices: vec![],
        });

        let initial_index = group.verticies.len();

        let mut push_vertex = |vertex: Vertex| {
            group.verticies.push(vertex.0.to_y_up() * 0.1);
            group.normals.push(normal.to_y_up());
            group.uvs.push(texture_info.get_uv(vertex.0, texture_data));
        };

        push_vertex(bsp_vertexes[face_edges[0].first as usize]);

        for (index, edge) in face_edges[1..face_edges.len() - 1].iter().enumerate() {
            group.indices.push(initial_index);
            group.indices.push(initial_index + 2 + 2 * index);
            group.indices.push(initial_index + 1 + 2 * index);

            push_vertex(bsp_vertexes[edge.first as usize]);
            push_vertex(bsp_vertexes[edge.second as usize]);
        }
    }

    return groups;
}
