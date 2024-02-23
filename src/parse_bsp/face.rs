use super::{into_bool, Lump};
use std::{fs::File, io::*};

pub(super) fn parse_faces(file: &mut File, lump: Lump) -> Result<Vec<Face>> {
    file.seek(SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 56 == 0);

    let mut out = vec![];

    for _i in 0..lump.length / 56 {
        let mut data = [0u8; 56];
        file.read_exact(&mut data)?;

        out.push(Face {
            planenum: u16::from_le_bytes(data[0..2].try_into().unwrap()),
            side: into_bool(data[2]),
            on_node: into_bool(data[3]),
            first_edge: u32::from_le_bytes(data[4..8].try_into().unwrap()),
            num_edges: u16::from_le_bytes(data[8..10].try_into().unwrap()),
            tex_info: u16::from_le_bytes(data[10..12].try_into().unwrap()),
            displacement_info: u16::from_le_bytes(data[12..14].try_into().unwrap()),
            volume_id: u16::from_le_bytes(data[14..16].try_into().unwrap()),
            styles: data[16..20].try_into().unwrap(),
            lightmap_offset: u32::from_le_bytes(data[20..24].try_into().unwrap()),
            area: f32::from_le_bytes(data[24..28].try_into().unwrap()),
            lightmap_texture_mins_in_luxels: [
                u32::from_le_bytes(data[28..32].try_into().unwrap()),
                u32::from_le_bytes(data[32..36].try_into().unwrap()),
            ],
            lightmap_texture_size_in_luxels: [
                u32::from_le_bytes(data[36..40].try_into().unwrap()),
                u32::from_le_bytes(data[40..44].try_into().unwrap()),
            ],
            original_face: u32::from_le_bytes(data[44..48].try_into().unwrap()),
            number_of_primitives: u16::from_le_bytes(data[48..50].try_into().unwrap()),
            first_primitive_id: u16::from_le_bytes(data[50..52].try_into().unwrap()),
            smoothing_groups: u32::from_le_bytes(data[52..56].try_into().unwrap()),
        })
    }

    Ok(out)
}

#[derive(Debug)]
pub struct Face {
    pub planenum: u16,
    pub side: bool,
    pub on_node: bool,
    pub first_edge: u32,
    pub num_edges: u16,
    pub tex_info: u16,
    pub displacement_info: u16,
    pub volume_id: u16,
    pub styles: [u8; 4],
    pub lightmap_offset: u32,
    pub area: f32,
    pub lightmap_texture_mins_in_luxels: [u32; 2],
    pub lightmap_texture_size_in_luxels: [u32; 2],
    pub original_face: u32,
    pub number_of_primitives: u16,
    pub first_primitive_id: u16,
    pub smoothing_groups: u32,
}
