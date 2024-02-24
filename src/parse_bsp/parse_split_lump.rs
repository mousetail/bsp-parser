use std::io::{Read, Seek};

use super::Lump;

pub(super) fn parse_split_chunks<
    T,
    FileType: Read + Seek,
    const LENGTH: usize,
    Function: FnMut([u8; LENGTH]) -> T,
>(
    file: &mut FileType,
    lump: Lump,
    mut f: Function,
) -> std::io::Result<Vec<T>> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64));

    assert!(lump.length % (LENGTH as u32) == 0);

    let mut out: Vec<T> = Vec::with_capacity(lump.length as usize / LENGTH);
    for _i in 0..lump.length / LENGTH as u32 {
        let mut data = [0u8; LENGTH];
        file.read_exact(&mut data)?;

        out.push(f(data));
    }

    return Ok(out);
}
