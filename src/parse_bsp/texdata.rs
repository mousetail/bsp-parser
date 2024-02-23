use std::fs::File;
use std::{io::{Read, Seek}};

use crate::vector::Vec3;
use crate::{parse_bsp::{parse_vector3}};

use super::Lump;

#[derive(Copy, Clone)]
pub(super) struct TextureData {
    pub reflectivity: Vec3,
    pub name_index: u32,
    pub width: u32,
    pub height: u32,
    pub view_width: u32,
    pub view_height: u32
}

pub(super) fn parse_texture_data(file: &mut File, lump: Lump) -> std::io::Result<Vec<TextureData>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 32 == 0);

    let mut out = Vec::with_capacity((lump.length / 32) as usize);

    for _ in 0..lump.length / 32 {
        let mut bytes = [0u8; 32];
        file.read_exact(&mut bytes)?;

        out.push(TextureData {
            reflectivity: parse_vector3(bytes[0..12].try_into().unwrap()),
            name_index: u32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            width: u32::from_le_bytes(bytes[16..20].try_into().unwrap()),
            height: u32::from_le_bytes(bytes[20..24].try_into().unwrap()),
            view_width: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            view_height: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
        });
    }

    Ok(out)
}