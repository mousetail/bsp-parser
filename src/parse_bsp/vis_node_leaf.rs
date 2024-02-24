struct VisNode {
    plane_id: u32,
    children: [i32; 2],
    min: [u16;3],
    max: [u16;3],
    first_face: u16,
    num_faces: u16,
    _padding: u16
}

struct VisLeaf {
    contents: u32,
    cluster: u16,
    area: u16, // these two share a 16 bit field
    flags: u16,
    min: [u16;3],
    max: [u16;3],
    first_leaf_face: u16,
    num_leaf_faces: u16,
    first_leaf_brush: u16,
    num_leaf_brushes: u16,
    water_data: u16,
    _padding: u16
}

// pub(super) fn parse_texture_data(file: &mut File, lump: Lump) -> std::io::Result<Vec<TextureData>> {
//     file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

//     assert!(lump.length % 32 == 0);

//     let mut out = Vec::with_capacity((lump.length / 32) as usize);

//     for _ in 0..lump.length / 32 {
//         let mut bytes = [0u8; 32];
//         file.read_exact(&mut bytes)?;
//     }
// }