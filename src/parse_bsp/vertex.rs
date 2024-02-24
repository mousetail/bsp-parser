use super::parse_split_lump::parse_split_chunks;
use super::Lump;
use std::io::Read;
use std::io::Seek;

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct Vertex(pub crate::vector::Vec3);

pub(super) fn parse_vertices<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<Vertex>> {
    parse_split_chunks(file, lump, |bytes: [u8; 12]| {
        Vertex(super::parse_vector3(bytes))
    })
}
