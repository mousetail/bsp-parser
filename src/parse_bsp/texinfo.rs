use std::fs::File;
use std::{io::{Read, Seek}};

use crate::vector::Vec3;
use crate::{parse_bsp::{parse_vector2, parse_vector3}, vector::Vec2};

use super::Lump;

#[derive(Copy, Clone)]
pub(super) struct TextureInfo {
    pub texture_vectors: [Vec2; 4],
    pub lightmap_vectors: [Vec2; 4],
    pub flags: u32,
    pub texture_data_index: u32
}

pub(super) fn parse_texture_info(file: &mut File, lump: Lump) -> std::io::Result<Vec<TextureInfo>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 72 == 0);

    let mut out = Vec::with_capacity((lump.length / 72) as usize);

    for _ in 0..lump.length / 72 {
        let mut bytes = [0u8; 72];
        file.read_exact(&mut bytes)?;

        out.push(TextureInfo {
            texture_vectors: (0..4usize).map(|i|parse_vector2(bytes[i*8..i*8+8].try_into().unwrap())).collect::<Vec<_>>().as_slice().try_into().unwrap(),
            lightmap_vectors: (0..4usize).map(|i|parse_vector2(bytes[32 + i*8..32 + i*8+8].try_into().unwrap())).collect::<Vec<_>>().as_slice().try_into().unwrap(),
            flags: u32::from_le_bytes(bytes[64..68].try_into().unwrap()),
            texture_data_index: u32::from_le_bytes(bytes[68..72].try_into().unwrap())
        });
    }

    Ok(out)
}

impl TextureInfo {
    pub fn get_uv(self, coords: Vec3 ) -> Vec2 {
        return Vec2 {
            x: self.texture_vectors[0].x * coords.x + self.texture_vectors[1].x * coords.y + self.texture_vectors[2].x * coords.z + self.texture_vectors[3].x,
            y: self.texture_vectors[0].y * coords.x + self.texture_vectors[1].y * coords.y + self.texture_vectors[2].y * coords.z + self.texture_vectors[3].y
        } * (1.0 / 2048.0)
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