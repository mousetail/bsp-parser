use std::io::Seek;
use std::{io::Read};

use super::parse_split_lump::parse_split_chunks;
use super::Lump;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct SurfEdge(pub i32);

pub(super) fn parse_surf_edges<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<SurfEdge>> {
    parse_split_chunks(file, lump, |bytes: [u8; 4]| {
        SurfEdge(i32::from_le_bytes(bytes))
    })
}
