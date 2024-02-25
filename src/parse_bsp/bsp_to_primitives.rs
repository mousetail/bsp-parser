use std::collections::HashMap;

use ordered_float::OrderedFloat;

use crate::vector::{Vec2, Vec3};

use super::{
    displacement::DisplacementInfo,
    edge::Edge,
    face::Face,
    surfedges::SurfEdge,
    texdata::TextureData,
    texinfo::{surface_flags, TextureInfo},
    vertex::Vertex,
    ParsedBspFile,
};

pub(super) struct MaterialGroup {
    pub verticies: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<usize>,
}

pub(super) fn to_primitives(bsp: ParsedBspFile) -> HashMap<String, MaterialGroup> {
    let mut groups: HashMap<String, MaterialGroup> = HashMap::new();

    let get_edge = |surface_edge: &SurfEdge| -> Edge {
        if surface_edge.0 > 0 {
            return bsp.edges[surface_edge.0 as usize];
        } else {
            return bsp.edges[-surface_edge.0 as usize].reverse();
        }
    };

    for (_index, face) in bsp.faces[bsp.brush_models[0].first_face as usize
        ..bsp.brush_models[0].first_face as usize + bsp.brush_models[0].num_faces as usize]
        .iter()
        .enumerate()
    {
        let face_edges: Vec<_> = bsp.surface_edges
            [face.first_edge as usize..(face.first_edge + face.num_edges as u32) as usize]
            .iter()
            .map(get_edge)
            .collect();
        let normal = bsp.planes[face.planenum as usize].normal;

        let texture_info = bsp.texture_infos[face.tex_info as usize];

        if texture_info.flags & surface_flags::SURF_NODRAW > 0
            || texture_info.flags & surface_flags::SURF_SKIP > 0
            || texture_info.flags & surface_flags::SURF_SKY > 0
            || texture_info.flags & surface_flags::SURF_HINT > 0
        {
            continue;
        }

        let texture_data = bsp.texture_data[texture_info.texture_data_index as usize];
        let texture_name = bsp
            .texture_string_array
            .get_str(bsp.texture_string_table[texture_data.name_index as usize].0 as usize)
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

        if face.displacement_info != -1 {
            handle_displacement_face(
                group,
                &bsp,
                *face,
                normal,
                texture_info,
                texture_data,
                face_edges,
                bsp.displacement_info[face.displacement_info as usize],
            )
        } else {
            handle_normal_face(group, &bsp, normal, texture_info, texture_data, face_edges)
        }
    }

    return groups;
}

fn handle_normal_face(
    group: &mut MaterialGroup,
    bsp: &ParsedBspFile,
    normal: Vec3,
    texture_info: TextureInfo,
    texture_data: TextureData,
    face_edges: Vec<Edge>,
) {
    let initial_index = group.verticies.len();

    let mut push_vertex = |vertex: Vertex| {
        group.verticies.push(vertex.0.to_y_up() * 0.1);
        group.normals.push(normal.to_y_up());
        group.uvs.push(texture_info.get_uv(vertex.0, texture_data));
    };

    push_vertex(bsp.vertexes[face_edges[0].first as usize]);

    for (index, edge) in face_edges[1..face_edges.len() - 1].iter().enumerate() {
        group.indices.push(initial_index);
        group.indices.push(initial_index + 2 + 2 * index);
        group.indices.push(initial_index + 1 + 2 * index);

        push_vertex(bsp.vertexes[edge.first as usize]);
        push_vertex(bsp.vertexes[edge.second as usize]);
    }
}

fn handle_displacement_face(
    group: &mut MaterialGroup,
    bsp: &ParsedBspFile,
    face: Face,
    normal: Vec3,
    texture_info: TextureInfo,
    texture_data: TextureData,
    face_edges: Vec<Edge>,
    displacement_info: DisplacementInfo,
) {
    let power = displacement_info.power;

    let initial_index = group.verticies.len();
    assert_eq!(face_edges.len(), 4);
    assert!(1 <= power && power <= 4);

    let edges: [Edge; 4] = face_edges.try_into().unwrap();
    let mut corners = edges.map(|i| bsp.vertexes[i.first as usize].0);

    let mut starting_corner = (find_lowest_index(corners, displacement_info)) % 4;
    starting_corner = (2 + starting_corner) % 4;
    corners.rotate_left(starting_corner);

    let faces_per_side = (2 << (power - 1));

    let lerp = |first, second, value| (first * value) + (second * (1.0 - value));
    let interpolate = |x, y| {
        lerp(
            lerp(corners[0], corners[1], (x as f32 / faces_per_side as f32)),
            lerp(corners[3], corners[2], (x as f32 / faces_per_side as f32)),
            (y as f32 / faces_per_side as f32),
        )
    };

    // let mut max_index = (0, 0);
    // let mut max_magnitude = 0.0;
    for x in 0..=faces_per_side {
        for y in 0..=faces_per_side {
            let raw_position = interpolate(x, y);
            let vertex = bsp.displacement_vertexes
                [displacement_info.vertex_start as usize + x * (faces_per_side + 1) + y];

            // if vertex.length > max_magnitude {
            //     max_magnitude = vertex.length;
            //     max_index = (x, y);
            // }

            group
                .verticies
                .push((raw_position + vertex.direction * vertex.length).to_y_up() * 0.1);
            group.normals.push(normal.to_y_up());
            group
                .uvs
                .push(texture_info.get_uv(raw_position, texture_data));
        }
    }

    for x in 0..faces_per_side {
        for y in 0..faces_per_side {
            let mut indicies = [
                initial_index + x + y * (faces_per_side + 1),
                initial_index + x + 1 + y * (faces_per_side + 1),
                initial_index + x + (y + 1) * (faces_per_side + 1),
                initial_index + x + (y + 1) * (faces_per_side + 1),
                initial_index + x + y * (faces_per_side + 1) + 1,
                initial_index + x + (y + 1) * (faces_per_side + 1) + 1,
            ];

            // if face.side {
            //     indicies.reverse();
            // }

            group.indices.extend_from_slice(&indicies);
        }
    }
}

fn find_lowest_index(vertexes: [Vec3; 4], disp: DisplacementInfo) -> usize {
    let min = disp.start_position;

    [0, 1, 2, 3]
        .into_iter()
        .min_by_key(|k| OrderedFloat(vertexes[*k].distance_squared(&min)))
        .unwrap()
}
