use std::io::{Read, Seek};

use crate::vector::Vec2;
use crate::vector::Vec3;

use super::parse_split_lump::parse_split_chunks;
use super::parse_vector3;
use super::texdata::TextureData;
use super::Lump;

#[derive(Copy, Clone)]
pub(super) struct TextureInfo {
    pub texture_vectors: [(Vec3, f32); 2],
    pub lightmap_vectors: [(Vec3, f32); 2],
    pub flags: u32,
    pub texture_data_index: u32,
}

pub(super) fn parse_texture_info<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<TextureInfo>> {
    parse_split_chunks(file, lump, |bytes: [u8; 72]| TextureInfo {
        texture_vectors: [0, 1].map(|i| {
            (
                parse_vector3(bytes[i * 16..i * 16 + 12].try_into().unwrap()),
                f32::from_le_bytes(bytes[i * 16 + 12..i * 16 + 16].try_into().unwrap()),
            )
        }),
        lightmap_vectors: [0, 1].map(|i| {
            (
                parse_vector3(bytes[i * 16..i * 16 + 12].try_into().unwrap()),
                f32::from_le_bytes(bytes[i * 16 + 12..i * 16 + 16].try_into().unwrap()),
            )
        }),
        flags: u32::from_le_bytes(bytes[64..68].try_into().unwrap()),
        texture_data_index: u32::from_le_bytes(bytes[68..72].try_into().unwrap()),
    })
}

impl TextureInfo {
    pub fn get_uv(self, coords: Vec3, data: TextureData) -> Vec2 {
        return Vec2 {
            x: self.texture_vectors[0].0.dot(&coords) + self.texture_vectors[0].1,
            y: self.texture_vectors[1].0.dot(&coords) + self.texture_vectors[1].1,
        } * Vec2 {
            x: 1.0 / data.width as f32,
            y: 1.0 / data.height as f32,
        };
    }
}

#[allow(unused)]
pub mod surface_flags {
    pub const SURF_LIGHT: u32 = 0x0001;
    pub const SURF_SKY2D: u32 = 0x0002;
    pub const SURF_SKY: u32 = 0x0004;
    pub const SURF_WARP: u32 = 0x0008;
    pub const SURF_TRANS: u32 = 0x0010;
    pub const SURF_NOPORTAL: u32 = 0x0020;
    pub const SURF_TRIGGER: u32 = 0x0040;
    pub const SURF_NODRAW: u32 = 0x0080;
    pub const SURF_HINT: u32 = 0x0100;
    pub const SURF_SKIP: u32 = 0x0200;
    pub const SURF_NOLIGHT: u32 = 0x0400;
    pub const SURF_BUMPLIGHT: u32 = 0x0800;
    pub const SURF_NOSHADOWS: u32 = 0x1000;
    pub const SURF_NODECALS: u32 = 0x2000;
    pub const SURF_NOCHOP: u32 = 0x4000;
    pub const SURF_HITBOX: u32 = 0x8000;
}
