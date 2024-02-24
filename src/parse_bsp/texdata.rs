use std::io::{Read, Seek};

use crate::parse_bsp::parse_vector3;
use crate::vector::Vec3;

use super::parse_split_lump::parse_split_chunks;
use super::Lump;

#[derive(Copy, Clone)]
pub(super) struct TextureData {
    pub reflectivity: Vec3,
    pub name_index: u32,
    pub width: u32,
    pub height: u32,
    pub view_width: u32,
    pub view_height: u32,
}

pub(super) fn parse_texture_data<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<TextureData>> {
    parse_split_chunks(file, lump, |bytes: [u8; 32]| TextureData {
        reflectivity: parse_vector3(bytes[0..12].try_into().unwrap()),
        name_index: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
        width: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
        height: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
        view_width: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
        view_height: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
    })
}
