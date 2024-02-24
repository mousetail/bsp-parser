use std::io::{BufReader, Cursor, Read, Seek, Write};

use super::Lump;

pub(super) enum ChunkReader<'a, T: Read> {
    Uncompressed(&'a mut T),
    Compressed(Cursor<Vec<u8>>),
}

impl<'a, T: Read> Read for ChunkReader<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Self::Uncompressed(k) => k.read(buf),
            Self::Compressed(k) => k.read(buf),
        }
    }
}

pub(super) fn decompress_stream<FileType: Read + Seek>(
    file: &mut FileType,
    lump: Lump,
) -> std::io::Result<(ChunkReader<'_, FileType>, usize)> {
    file.seek(std::io::SeekFrom::Start(lump.offset as u64))?;
    if lump.is_compressed() {
        let mut header_bytes = [0u8; 17];
        file.read_exact(&mut header_bytes)?;

        assert_eq!(&header_bytes[0..4], b"LZMA");

        let actual_size = u32::from_le_bytes(header_bytes[4..8].try_into().unwrap());
        let compressed_size = u32::from_le_bytes(header_bytes[8..12].try_into().unwrap());

        let properties: [u8; 5] = header_bytes[12..17].try_into().unwrap();

        println!("original_size={actual_size} compressed_size={compressed_size}");

        let mut standard_header: [u8; 13] = [0; 13];
        let mut standard_writer: &mut [u8] = &mut standard_header;
        standard_writer.write_all(&properties)?;
        standard_writer.write_all(&(actual_size as u64).to_le_bytes())?;

        let mut file = BufReader::new(standard_header.chain(file.take(compressed_size as u64)));

        // todo: A more efficient stream adapter
        let mut output = vec![];
        lzma_rs::lzma_decompress(&mut file, &mut output).unwrap();

        return Ok((
            ChunkReader::Compressed(Cursor::new(output)),
            actual_size as usize,
        ));
    } else {
        return Ok((ChunkReader::Uncompressed(file), lump.length as usize));
    }
}

pub(super) fn parse_split_chunks<
    T,
    FileType: Read + Seek,
    const LENGTH: usize,
    Function: FnMut([u8; LENGTH]) -> T,
>(
    file: &mut FileType,
    lump: Lump,
    f: Function,
) -> std::io::Result<Vec<T>> {
    let (mut decompressed_stream, length) = decompress_stream(file, lump)?;
    chunks_from_uncompressed_file(&mut decompressed_stream, length, f)
}

fn chunks_from_uncompressed_file<
    T,
    FileType: Read,
    const LENGTH: usize,
    Function: FnMut([u8; LENGTH]) -> T,
>(
    file: &mut FileType,
    length: usize,
    mut f: Function,
) -> std::io::Result<Vec<T>> {
    assert!(length % LENGTH == 0);

    let mut out: Vec<T> = Vec::with_capacity(length as usize / LENGTH);
    for _i in 0..length / LENGTH {
        let mut data = [0u8; LENGTH];
        file.read_exact(&mut data)?;

        out.push(f(data));
    }

    return Ok(out);
}
