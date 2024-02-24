use std::io::Seek;
use std::{fs::File, io::Read};

use super::parse_split_lump::parse_split_chunks;
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

pub(super) fn parse_edges<T: Read + Seek>(file: &mut T, lump: Lump) -> std::io::Result<Vec<Edge>> {
    parse_split_chunks(file, lump, |bytes: [u8; 4]| Edge {
        first: u16::from_le_bytes(bytes[0..2].try_into().unwrap()),
        second: u16::from_le_bytes(bytes[2..4].try_into().unwrap()),
    })
}
