use std::{fs::File, io::{Read, Seek}};

use crate::{parse_bsp::parse_vector3, vector::Vec3};

use super::Lump;

pub(super) struct Plane {
    pub normal: Vec3,
    pub distance: f32,
    pub axis: u32,
}

pub(super) fn parse_plane(file: &mut File, lump: Lump) -> std::io::Result<Vec<Plane>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 20 == 0);

    let mut out = Vec::with_capacity((lump.length / 20) as usize);

    for _ in 0..lump.length / 20 {
        let mut bytes = [0u8; 20];
        file.read_exact(&mut bytes)?;

        out.push(Plane {
            normal: parse_vector3(bytes[0..12].try_into().unwrap()),
            distance: f32::from_le_bytes(bytes[12..16].try_into().unwrap()),
            axis: u32::from_le_bytes(bytes[16..20].try_into().unwrap())
        })
    }

    Ok(out)
}