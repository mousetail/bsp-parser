mod bsp_to_primitives;
mod displacement;
mod edge;
mod face;
mod parse_split_lump;
mod plane;
mod texdata;
mod texinfo;
mod texture_string_array;
mod vertex;
mod vis_node_leaf;

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
    brush_model::BrushModel,
    displacement::{DisplacementInfo, DisplacementVertex},
    edge::Edge,
    face::Face,
    plane::Plane,
    surfedges::SurfEdge,
    texdata::TextureData,
    texinfo::TextureInfo,
    texture_string_array::{TextureDataStringArray, TextureString},
    vertex::Vertex,
};

mod brush_model;
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

struct ParsedBspFile {
    faces: Vec<Face>,
    planes: Vec<Plane>,
    vertexes: Vec<Vertex>,
    edges: Vec<Edge>,
    surface_edges: Vec<SurfEdge>,
    texture_infos: Vec<TextureInfo>,
    texture_data: Vec<TextureData>,
    texture_string_array: TextureDataStringArray,
    texture_string_table: Vec<TextureString>,
    displacement_info: Vec<DisplacementInfo>,
    displacement_vertexes: Vec<DisplacementVertex>,
    brush_models: Vec<BrushModel>,
}

pub fn parse_bsp(filename: &str) -> Result<()> {
    let mut file = OpenOptions::new().read(true).open(filename)?;

    let (version, lumps) = read_header(&mut file)?;

    println!("BSP Version = {version:?}");

    for (index, lump) in lumps.iter().enumerate() {
        println!(
            "{index:>4} id={:03?} offset={:<14} length={:<14} version={:<8}",
            lump.id,
            CommaFormat(lump.offset as usize),
            CommaFormat(lump.length as usize),
            lump.version
        )
    }

    let parsed_file = ParsedBspFile {
        faces: face::parse_faces(&mut file, lumps[lump_names::LUMP_FACES])?,
        planes: plane::parse_planes(&mut file, lumps[lump_names::LUMP_PLANES])?,
        vertexes: vertex::parse_vertices(&mut file, lumps[lump_names::LUMP_VERTEXES])?,
        edges: edge::parse_edges(&mut file, lumps[lump_names::LUMP_EDGES])?,
        surface_edges: surfedges::parse_surf_edges(&mut file, lumps[lump_names::LUMP_SURFEDGES])?,
        texture_infos: texinfo::parse_texture_info(&mut file, lumps[lump_names::LUMP_TEXINFO])?,
        texture_data: texdata::parse_texture_data(&mut file, lumps[lump_names::LUMP_TEXDATA])?,
        texture_string_array: texture_string_array::parse_texture_data_string_array(
            &mut file,
            lumps[lump_names::LUMP_TEXDATA_STRING_DATA],
        )?,
        texture_string_table: texture_string_array::parse_texture_data_string_table(
            &mut file,
            lumps[lump_names::LUMP_TEXDATA_STRING_TABLE],
        )?,
        displacement_info: displacement::parse_displacements(
            &mut file,
            lumps[lump_names::LUMP_DISPINFO],
        )?,
        displacement_vertexes: displacement::parse_displacement_vertexes(
            &mut file,
            lumps[lump_names::LUMP_DISP_VERTS],
        )?,
        brush_models: brush_model::parse_bush_model(&mut file, lumps[lump_names::LUMP_MODELS])?,
    };
    println!("Number of faces: {:}", CommaFormat(parsed_file.faces.len()));
    println!(
        "Number of planes: {:}",
        CommaFormat(parsed_file.planes.len())
    );
    println!(
        "Number of vertexes: {:}",
        CommaFormat(parsed_file.vertexes.len())
    );
    println!("Number of edges: {:}", CommaFormat(parsed_file.edges.len()));
    println!(
        "Number of surfedges: {:}",
        CommaFormat(parsed_file.surface_edges.len())
    );
    println!(
        "String data table size: {:}",
        CommaFormat(parsed_file.texture_string_table.len())
    );

    let primitive_groups = bsp_to_primitives::to_primitives(parsed_file);

    for key in primitive_groups.keys() {
        println!("{key}")
    }

    gltf_export::save_mesh(
        "out.gltf".to_string(),
        primitive_groups
            .iter()
            .map(|(name, primitive)| gltf_export::GltfObject {
                vertexes: &primitive.verticies,
                normals: &primitive.normals,
                uvs: &primitive.uvs,
                indices: &primitive.indices,
                texture: image::open(format!("cache\\textures\\{name}.png"))
                    .expect(&format!("{name}")),
                name,
            })
            .collect::<Vec<_>>()
            .as_slice(),
    )
    .unwrap();

    return Ok(());
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

impl Lump {
    fn is_compressed(self) -> bool {
        return self.id != [0; 4];
    }
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
