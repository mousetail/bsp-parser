use std::io::{Read, Seek};

use super::{parse_split_lump::parse_split_chunks, Lump};

pub(super) struct VisNode {
    plane_id: u32,
    children: [i32; 2],
    min: [u16; 3],
    max: [u16; 3],
    first_face: u16,
    num_faces: u16,
    area: u16,
    _padding: u16,
}

pub(super) struct VisLeaf {
    contents: u32,
    cluster: u16,
    area_and_flags: u16,
    min: [u16; 3],
    max: [u16; 3],
    first_leaf_face: u16,
    num_leaf_faces: u16,
    first_leaf_brush: u16,
    num_leaf_brushes: u16,
    water_data: u16,
    _padding: u16,
}

pub(super) fn parse_vis_node<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<VisNode>> {
    parse_split_chunks(file, lump, |bytes: [u8; 32]| VisNode {
        plane_id: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        children: [0, 1]
            .map(|k| i32::from_le_bytes(bytes[k * 4 + 4..k * 4 + 8].try_into().unwrap())),
        min: [0, 1, 2]
            .map(|k| u16::from_le_bytes(bytes[k * 2 + 12..k * 2 + 14].try_into().unwrap())),
        max: [0, 1, 2]
            .map(|k| u16::from_le_bytes(bytes[k * 2 + 18..k * 2 + 20].try_into().unwrap())),
        first_face: u16::from_le_bytes(bytes[24..26].try_into().unwrap()),
        num_faces: u16::from_le_bytes(bytes[26..28].try_into().unwrap()),
        area: u16::from_le_bytes(bytes[28..30].try_into().unwrap()),
        _padding: u16::from_le_bytes(bytes[30..32].try_into().unwrap()),
    })
}

pub(super) fn parse_vis_leaf<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<VisLeaf>> {
    parse_split_chunks(file, lump, |bytes: [u8; 32]| VisLeaf {
        contents: u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        cluster: u16::from_le_bytes(bytes[4..6].try_into().unwrap()),
        area_and_flags: u16::from_le_bytes(bytes[6..8].try_into().unwrap()),
        min: [0, 1, 2]
            .map(|k| u16::from_le_bytes(bytes[k * 2 + 8..k * 2 + 10].try_into().unwrap())),
        max: [0, 1, 2]
            .map(|k| u16::from_le_bytes(bytes[k * 2 + 14..k * 2 + 16].try_into().unwrap())),
        first_leaf_face: u16::from_le_bytes(bytes[20..22].try_into().unwrap()),
        num_leaf_faces: u16::from_le_bytes(bytes[22..24].try_into().unwrap()),
        first_leaf_brush: u16::from_le_bytes(bytes[24..26].try_into().unwrap()),
        num_leaf_brushes: u16::from_le_bytes(bytes[26..28].try_into().unwrap()),
        water_data: u16::from_le_bytes(bytes[28..30].try_into().unwrap()),
        _padding: u16::from_le_bytes(bytes[30..32].try_into().unwrap()),
    })
}
