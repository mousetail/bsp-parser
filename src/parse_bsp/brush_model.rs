use std::io::{Read, Seek};

use crate::vector::Vec3;

use super::{parse_split_lump::parse_split_chunks, Lump};

pub(super) struct BrushModel {
    pub min: Vec3,
    pub max: Vec3,
    pub origin: Vec3,
    pub head_node: u32,
    pub first_face: u32,
    pub num_faces: u32,
}

pub(super) fn parse_bush_model<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<BrushModel>> {
    parse_split_chunks(file, lump, |bytes: [u8; 48]| BrushModel {
        min: super::parse_vector3(bytes[0..12].try_into().unwrap()),
        max: super::parse_vector3(bytes[12..24].try_into().unwrap()),
        origin: super::parse_vector3(bytes[24..36].try_into().unwrap()),
        head_node: u32::from_le_bytes(bytes[36..40].try_into().unwrap()),
        first_face: u32::from_le_bytes(bytes[40..44].try_into().unwrap()),
        num_faces: u32::from_le_bytes(bytes[44..48].try_into().unwrap()),
    })
}
