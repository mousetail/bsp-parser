
use std::{fs::File, io::Read};
use std::io::Seek;

use super::Lump;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct SurfEdge (pub i32);

pub(super) fn parse_surf_edges(file: &mut File, lump: Lump) -> std::io::Result<Vec<SurfEdge>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 4 == 0);

    let mut out = Vec::with_capacity((lump.length / 4) as usize);

    for _ in 0..lump.length / 4 {
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)?;

        out.push(SurfEdge(i32::from_le_bytes(bytes)));
    }

    Ok(out)
}