use std::io::{Read, Seek};

use crate::vector::Vec3;

use super::{parse_split_lump::parse_split_chunks, parse_vector3, Lump};

#[derive(Copy, Clone, Debug)]
pub(super) struct DisplacementInfo {
    pub start_position: Vec3,
    pub vertex_start: u32,
    pub triangle_start: u32,
    pub power: u32,
    pub minimum_tesselation: u32,
    pub smoothing_angle: f32,
    pub contents: u32,
    pub face: u16,
    pub lightmap_alpha_start: u32,
    pub lightmap_sample_start: u32,
    // rest: [u8; 130],
}

pub(super) fn parse_displacements<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<DisplacementInfo>> {
    parse_split_chunks(file, lump, |bytes: [u8; 176]| DisplacementInfo {
        start_position: parse_vector3(bytes[0..12].try_into().unwrap()),
        vertex_start: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
        triangle_start: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
        power: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
        minimum_tesselation: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
        smoothing_angle: f32::from_le_bytes(bytes[28..32].try_into().unwrap()),
        contents: u32::from_le_bytes(bytes[32..36].try_into().unwrap()),
        face: u16::from_le_bytes(bytes[36..38].try_into().unwrap()),
        lightmap_alpha_start: u32::from_le_bytes(bytes[38..42].try_into().unwrap()),
        lightmap_sample_start: u32::from_le_bytes(bytes[42..46].try_into().unwrap()),
        // rest: bytes[46..176].try_into().unwrap(),
    })
}

#[derive(Copy, Clone)]
pub(super) struct DisplacementVertex {
    pub direction: Vec3,
    pub length: f32,
    pub alpha: f32,
}

pub(super) fn parse_displacement_vertexes<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<DisplacementVertex>> {
    parse_split_chunks(file, lump, |bytes: [u8; 20]| DisplacementVertex {
        direction: parse_vector3(bytes[0..12].try_into().unwrap()),
        length: f32::from_le_bytes(bytes[12..16].try_into().unwrap()),
        alpha: f32::from_le_bytes(bytes[16..20].try_into().unwrap()),
    })
}
