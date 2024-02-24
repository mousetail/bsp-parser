use std::{
    fs::File,
    io::{Read, Seek},
};

use crate::{parse_bsp::parse_vector3, vector::Vec3};

use super::{parse_split_lump::parse_split_chunks, Lump};

pub(super) struct Plane {
    pub normal: Vec3,
    pub distance: f32,
    pub axis: u32,
}

pub(super) fn parse_planes<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<Plane>> {
    parse_split_chunks(file, lump, |bytes: [u8; 20]| Plane {
        normal: parse_vector3(bytes[0..12].try_into().unwrap()),
        distance: f32::from_le_bytes(bytes[12..16].try_into().unwrap()),
        axis: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
    })
}
