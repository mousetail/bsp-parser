use std::io::{Read, Seek};

use crate::vector::Vec3;

use super::{parse_split_lump::parse_split_chunks, parse_vector3, Lump};

pub(super) struct DisplacementInfo {
    start_position: Vec3,
    vertex_start: u32,
    triangle_start: u32,
    power: u32,
    minimum_tesselation: u32,
    smoothing_angle: f32,
    contents: u32,
    face: u16,
    lightmap_alpha_start: u32,
    lightmap_sample_start: u32,
    rest: [u8; 130],
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
        rest: bytes[46..176].try_into().unwrap(),
    })
}
