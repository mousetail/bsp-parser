use std::io::Seek;
use std::{fs::File, io::Read};

use super::Lump;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(super) struct Edge {
    pub first: u16,
    pub second: u16,
}

impl Edge {
    pub fn reverse(self) -> Self {
        return Edge {
            first: self.second,
            second: self.first,
        };
    }
}

pub(super) fn parse_edges(file: &mut File, lump: Lump) -> std::io::Result<Vec<Edge>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 4 == 0);

    let mut out = Vec::with_capacity((lump.length / 4) as usize);

    for _ in 0..lump.length / 4 {
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)?;

        out.push(Edge {
            first: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
            second: u16::from_le_bytes(bytes[2..4].try_into().unwrap()),
        });
    }

    Ok(out)
}
