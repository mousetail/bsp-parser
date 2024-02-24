use std::{
    fs::File,
    io::{Read, Seek},
};

use super::{
    parse_split_lump::{decompress_stream, parse_split_chunks},
    Lump,
};

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
    println!("About to parse string data");
    let (mut stream, length) = decompress_stream(file, lump)?;
    println!("Parsed string data");

    let mut out = vec![0; length as usize];
    stream.read_exact(out.as_mut_slice())?;

    return Ok(TextureDataStringArray(out));
}

pub(super) struct TextureString(pub u32);

pub(super) fn parse_texture_data_string_table<T: Read + Seek>(
    file: &mut T,
    lump: Lump,
) -> std::io::Result<Vec<TextureString>> {
    parse_split_chunks(file, lump, |bytes: [u8; 4]| {
        TextureString(u32::from_le_bytes(bytes))
    })
}
