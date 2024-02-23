use std::{
    fs::File,
    io::{Read, Seek},
};

use super::Lump;

pub(super) struct TextureDataStringArray(Vec<u8>);

impl TextureDataStringArray {
    pub fn get_str(&self, index: usize) -> Result<&str, std::str::Utf8Error> {
        let start = index;
        let mut end = index;
        while self.0[end] > 0 {
            end += 1;
        }
        return std::str::from_utf8(&self.0[start..end]);
    }
}

pub(super) fn parse_texture_data_string_array(
    file: &mut File,
    lump: Lump,
) -> std::io::Result<TextureDataStringArray> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    let mut out = vec![0; lump.length as usize];
    file.read_exact(out.as_mut_slice())?;

    return Ok(TextureDataStringArray(out));
}

pub(super) struct TextureString(pub u32);

pub(super) fn parse_texture_data_string_table(
    file: &mut File,
    lump: Lump,
) -> std::io::Result<Vec<TextureString>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 4 == 0);

    let mut out = Vec::with_capacity((lump.length / 4) as usize);

    for _ in 0..lump.length / 4 {
        let mut bytes = [0u8; 4];
        file.read_exact(&mut bytes)?;

        out.push(TextureString(u32::from_le_bytes(bytes)));
    }

    Ok(out)
}
