use std::io::{Read, Seek};

use crate::vector::Vec2;
use crate::vector::Vec3;

use super::parse_split_lump::parse_split_chunks;
use super::texdata::TextureData;
use super::Lump;

#[derive(Copy, Clone)]
pub(super) struct TextureInfo {
    pub texture_vectors: [Vec2; 4],
    pub lightmap_vectors: [Vec2; 4],
    pub flags: u32,
    pub texture_data_index: u32,
}

pub(super) fn parse_texture_info<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<TextureInfo>> {
    parse_split_chunks(file, lump, |bytes: [u8; 72]| TextureInfo {
        texture_vectors: (0..4usize)
            .map(|i| Vec2 {
                x: f32::from_le_bytes(bytes[i * 4..i * 4 + 4].try_into().unwrap()),
                y: f32::from_le_bytes(bytes[i * 4 + 16..i * 4 + 20].try_into().unwrap()),
            })
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap(),
        lightmap_vectors: (0..4usize)
            .map(|i| Vec2 {
                x: f32::from_le_bytes(bytes[i * 4 + 32..i * 4 + 36].try_into().unwrap()),
                y: f32::from_le_bytes(bytes[i * 4 + 48..i * 4 + 52].try_into().unwrap()),
            })
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap(),
        flags: u32::from_le_bytes(bytes[64..68].try_into().unwrap()),
        texture_data_index: u32::from_le_bytes(bytes[68..72].try_into().unwrap()),
    })
}

impl TextureInfo {
    pub fn get_uv(self, coords: Vec3, data: TextureData) -> Vec2 {
        return Vec2 {
            x: self.texture_vectors[0].x * coords.x
                + self.texture_vectors[1].x * coords.y
                + self.texture_vectors[2].x * coords.z
                + self.texture_vectors[3].x,
            y: self.texture_vectors[0].y * coords.x
                + self.texture_vectors[1].y * coords.y
                + self.texture_vectors[2].y * coords.z
                + self.texture_vectors[3].y,
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
