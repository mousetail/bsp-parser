mod edge;
mod face;
mod plane;
mod texdata;
mod texinfo;
mod texture_string_array;
mod vertex;

use crate::{
    comma_format::CommaFormat,
    gltf_export,
    vector::{Vec2, Vec3},
};
use std::{
    fs::{File, OpenOptions},
    io::*,
};

use self::{
    edge::Edge,
    face::Face,
    plane::Plane,
    surfedges::SurfEdge,
    texdata::TextureData,
    texinfo::{
        surface_flags::{self},
        TextureInfo,
    },
    texture_string_array::{TextureDataStringArray, TextureString},
    vertex::Vertex,
};
mod surfedges;

#[allow(unused)]
mod lump_names {
    pub(crate) const LUMP_ENTITIES: usize = 0;
    pub(crate) const LUMP_PLANES: usize = 1;
    pub(crate) const LUMP_TEXDATA: usize = 2;
    pub(crate) const LUMP_VERTEXES: usize = 3;
    pub(crate) const LUMP_VISIBILITY: usize = 4;
    pub(crate) const LUMP_NODES: usize = 5;
    pub(crate) const LUMP_TEXINFO: usize = 6;
    pub(crate) const LUMP_FACES: usize = 7;
    pub(crate) const LUMP_LIGHTING: usize = 8;
    pub(crate) const LUMP_OCCLUSION: usize = 9;
    pub(crate) const LUMP_LEAFS: usize = 10;
    pub(crate) const LUMP_FACEIDS: usize = 11;
    pub(crate) const LUMP_EDGES: usize = 12;
    pub(crate) const LUMP_SURFEDGES: usize = 13;
    pub(crate) const LUMP_MODELS: usize = 14;
    pub(crate) const LUMP_WORLDLIGHTS: usize = 15;
    pub(crate) const LUMP_LEAFFACES: usize = 16;
    pub(crate) const LUMP_LEAFBRUSHES: usize = 17;
    pub(crate) const LUMP_BRUSHES: usize = 18;
    pub(crate) const LUMP_BRUSHSIDES: usize = 19;
    pub(crate) const LUMP_AREAS: usize = 20;
    pub(crate) const LUMP_AREAPORTALS: usize = 21;
    pub(crate) const LUMP_PORTALS: usize = 22;
    pub(crate) const LUMP_CLUSTERS: usize = 23;
    pub(crate) const LUMP_PORTALVERTS: usize = 24;
    pub(crate) const LUMP_CLUSTERPORTALS: usize = 25;
    pub(crate) const LUMP_DISPINFO: usize = 26;
    pub(crate) const LUMP_ORIGINALFACES: usize = 27;
    pub(crate) const LUMP_PHYSDISP: usize = 28;
    pub(crate) const LUMP_PHYSCOLLIDE: usize = 29;
    pub(crate) const LUMP_VERTNORMALS: usize = 30;
    pub(crate) const LUMP_VERTNORMALINDICES: usize = 31;
    pub(crate) const LUMP_DISP_LIGHTMAP_ALPHAS: usize = 32;
    pub(crate) const LUMP_DISP_VERTS: usize = 33;
    pub(crate) const LUMP_DISP_LIGHTMAP_SAMPLE_POSITIONS: usize = 34;
    pub(crate) const LUMP_GAME_LUMP: usize = 35;
    pub(crate) const LUMP_LEAFWATERDATA: usize = 36;
    pub(crate) const LUMP_PRIMITIVES: usize = 37;
    pub(crate) const LUMP_PRIMVERTS: usize = 38;
    pub(crate) const LUMP_PRIMINDICES: usize = 39;
    pub(crate) const LUMP_PAKFILE: usize = 40;
    pub(crate) const LUMP_CLIPPORTALVERTS: usize = 41;
    pub(crate) const LUMP_CUBEMAPS: usize = 42;
    pub(crate) const LUMP_TEXDATA_STRING_DATA: usize = 43;
    pub(crate) const LUMP_TEXDATA_STRING_TABLE: usize = 44;
    pub(crate) const LUMP_OVERLAYS: usize = 45;
    pub(crate) const LUMP_LEAFMINDISTTOWATER: usize = 46;
    pub(crate) const LUMP_FACE_MACRO_TEXTURE_INFO: usize = 47;
    pub(crate) const LUMP_DISP_TRIS: usize = 48;
    pub(crate) const LUMP_PHYSCOLLIDESURFACE: usize = 49;
    pub(crate) const LUMP_WATEROVERLAYS: usize = 50;
    pub(crate) const LUMP_LIGHTMAPPAGES: usize = 51;
    pub(crate) const LUMP_LIGHTMAPPAGEINFOS: usize = 52;
    pub(crate) const LUMP_LIGHTING_HDR: usize = 53;
    pub(crate) const LUMP_WORLDLIGHTS_HDR: usize = 54;
    pub(crate) const LUMP_LEAF_AMBIENT_LIGHTING_HDR: usize = 55;
    pub(crate) const LUMP_LEAF_AMBIENT_LIGHTING: usize = 56;
    pub(crate) const LUMP_XZIPPAKFILE: usize = 57;
    pub(crate) const LUMP_FACES_HDR: usize = 58;
    pub(crate) const LUMP_MAP_FLAGS: usize = 59;
    pub(crate) const LUMP_OVERLAY_FADES: usize = 60;
    pub(crate) const LUMP_OVERLAY_SYSTEM_LEVELS: usize = 61;
    pub(crate) const LUMP_PHYSLEVEL: usize = 62;
    pub(crate) const LUMP_DISP_MULTIBLEND: usize = 63;
}

const HEADER_LUMPS: usize = 64;

pub fn parse_bsp(filename: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(filename)?;

    let (version, lumps) = read_header(&mut file)?;

    println!("BSP Version = {version:?}");

    for (index, lump) in lumps.iter().enumerate() {
        println!(
            "{index:>4} {:?} {:<14} {:<14} {:<8}",
            lump.id,
            CommaFormat(lump.offset as usize),
            CommaFormat(lump.length as usize),
            lump.version
        )
    }

    let faces = face::parse_faces(&mut file, lumps[lump_names::LUMP_FACES])?;
    println!("Number of faces: {:}", CommaFormat(faces.len()));
    let planes = plane::parse_plane(&mut file, lumps[lump_names::LUMP_PLANES])?;
    println!("Number of planes: {:}", CommaFormat(planes.len()));
    let verticies = vertex::parse_vertices(&mut file, lumps[lump_names::LUMP_VERTEXES])?;
    println!("Number of verticies: {:}", CommaFormat(verticies.len()));
    let edges = edge::parse_edges(&mut file, lumps[lump_names::LUMP_EDGES])?;
    println!("Number of edges: {:}", CommaFormat(edges.len()));
    let surfedges = surfedges::parse_surf_edges(&mut file, lumps[lump_names::LUMP_SURFEDGES])?;
    println!("Number of surfedges: {:}", CommaFormat(surfedges.len()));
    let texture_info = texinfo::parse_texture_info(&mut file, lumps[lump_names::LUMP_TEXINFO])?;
    let texture_data = texdata::parse_texture_data(&mut file, lumps[lump_names::LUMP_TEXDATA])?;
    let texture_string_array = texture_string_array::parse_texture_data_string_array(
        &mut file,
        lumps[lump_names::LUMP_TEXDATA_STRING_DATA],
    )?;
    let texture_string_table = texture_string_array::parse_texture_data_string_table(
        &mut file,
        lumps[lump_names::LUMP_TEXDATA_STRING_TABLE],
    )?;

    let (glft_verticies, gltf_normals, gltf_uvs, gltf_indicies) = to_primitives(
        faces,
        planes,
        verticies,
        edges,
        surfedges,
        texture_info,
        texture_data,
        texture_string_array,
        texture_string_table,
    );

    gltf_export::save_mesh(
        "out.gltf".to_string(),
        &[gltf_export::GltfObject {
        vertexes: &glft_verticies,
        normals: &gltf_normals,
        uvs: &gltf_uvs,
        indices: &gltf_indicies,
        texture: image::open("cache\\texture.png").unwrap(),
        }]
    )
    .unwrap();

    return Ok(());
}

fn to_primitives(
    bsp_faces: Vec<Face>,
    bsp_planes: Vec<Plane>,
    bsp_vertexes: Vec<Vertex>,
    bsp_edges: Vec<Edge>,
    bsp_surfedges: Vec<SurfEdge>,
    texture_infos: Vec<TextureInfo>,
    bsp_texture_data: Vec<TextureData>,
    bsp_texture_string_array: TextureDataStringArray,
    bsp_texture_string_table: Vec<TextureString>,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>, Vec<usize>) {
    let mut verticies = Vec::<Vec3>::new();
    let mut normals = Vec::<Vec3>::new();
    let mut uvs = Vec::<Vec2>::new();
    let mut indicies = Vec::<usize>::new();

    let get_edge = |surface_edge: &SurfEdge| -> Edge {
        if surface_edge.0 > 0 {
            return bsp_edges[surface_edge.0 as usize];
        } else {
            return bsp_edges[-surface_edge.0 as usize].reverse();
        }
    };

    for (_index, face) in bsp_faces.iter().enumerate() {
        let face_edges: Vec<_> = bsp_surfedges
            [face.first_edge as usize..(face.first_edge + face.num_edges as u32) as usize]
            .iter()
            .map(get_edge)
            .collect();
        let normal = bsp_planes[face.planenum as usize].normal;
        let initial_index = verticies.len();

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

        let mut push_vertex = |vertex: Vertex| {
            verticies.push(vertex.0.to_y_up() * 0.1);
            normals.push(normal.to_y_up());
            uvs.push(texture_info.get_uv(vertex.0, texture_data));
        };
        push_vertex(bsp_vertexes[face_edges[0].first as usize]);

        for (index, edge) in face_edges[1..face_edges.len() - 1].iter().enumerate() {
            indicies.push(initial_index);
            indicies.push(initial_index + 2 + 2 * index);
            indicies.push(initial_index + 1 + 2 * index);

            push_vertex(bsp_vertexes[edge.first as usize]);
            push_vertex(bsp_vertexes[edge.second as usize]);
        }
    }

    return (verticies, normals, uvs, indicies);
}

fn read_header(file: &mut File) -> Result<(u32, Vec<Lump>)> {
    let mut header_bytes = [0u8; 8];
    file.read_exact(&mut header_bytes)?;

    let ident = u32::from_le_bytes(header_bytes[0..4].try_into().unwrap());

    // Magic Number
    assert_eq!(ident, u32::from_le_bytes(*b"VBSP"));
    let version = u32::from_le_bytes(header_bytes[4..8].try_into().unwrap());

    let mut lumps: Vec<Lump> = vec![];

    for _i in 0..HEADER_LUMPS {
        lumps.push(read_lump(file)?);
    }

    return Ok((version, lumps));
}

fn read_lump(file: &mut File) -> Result<Lump> {
    let mut lump_bytes = [0u8; 16];
    file.read_exact(&mut lump_bytes)?;

    return Ok(Lump {
        offset: u32::from_le_bytes(lump_bytes[0..4].try_into().unwrap()),
        length: u32::from_le_bytes(lump_bytes[4..8].try_into().unwrap()),
        version: u32::from_le_bytes(lump_bytes[8..12].try_into().unwrap()),
        id: lump_bytes[12..16].try_into().unwrap(),
    });
}

fn into_bool(x: u8) -> bool {
    match x {
        0 => false,
        1 => true,
        _ => panic!("Expected bool, got {x}"),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Lump {
    offset: u32,
    length: u32,
    version: u32,
    id: [u8; 4],
}

fn parse_vector3(bytes: [u8; 12]) -> Vec3 {
    return Vec3 {
        x: f32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        y: f32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        z: f32::from_le_bytes(bytes[8..12].try_into().unwrap()),
    };
}

fn parse_vector2(bytes: [u8; 8]) -> Vec2 {
    return Vec2 {
        x: f32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        y: f32::from_le_bytes(bytes[4..8].try_into().unwrap()),
    };
}
