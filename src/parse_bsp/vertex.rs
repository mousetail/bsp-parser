use super::Lump;
use std::io::Seek;
use std::{fs::File, io::Read};

#[derive(Copy, Clone, PartialEq, Debug)]
pub(super) struct Vertex(pub crate::vector::Vec3);

pub(super) fn parse_vertices(file: &mut File, lump: Lump) -> std::io::Result<Vec<Vertex>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 12 == 0);

    let mut out = Vec::with_capacity((lump.length / 12) as usize);

    for _ in 0..lump.length / 12 {
        let mut bytes = [0u8; 12];
        file.read_exact(&mut bytes)?;

        out.push(Vertex(super::parse_vector3(bytes)));
    }

    Ok(out)
}
