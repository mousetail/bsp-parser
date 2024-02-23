use std::{fs::{File, OpenOptions}, io::*};

const HEADER_LUMPS: usize = 64;

#[allow(unused)]
mod lump_names {
    pub(crate) const LUMP_ENTITIES: usize = 0;
    pub(crate) const LUMP_PLANES: usize = 1;
    pub(crate) const LUMP_TEXDATA: usize = 2;
    pub(crate) const LUMP_VERTEXES: usize = 3;
    pub(crate) const LUMP_VISIBILITY: usize = 4;
    pub(crate) const LUMP_NODES: usize = 5;
    pub(crate) const LUMP_TEXINFO: usize = 6;
    pub(crate) const LUMP_FACES: usize = 7;
    pub(crate) const LUMP_LIGHTING: usize = 8;
    pub(crate) const LUMP_OCCLUSION: usize = 9;
    pub(crate) const LUMP_LEAFS: usize = 10;
    pub(crate) const LUMP_FACEIDS: usize = 11;
    pub(crate) const LUMP_EDGES: usize = 12;
    pub(crate) const LUMP_SURFEDGES: usize = 13;
    pub(crate) const LUMP_MODELS: usize = 14;
    pub(crate) const LUMP_WORLDLIGHTS: usize = 15;
    pub(crate) const LUMP_LEAFFACES: usize = 16;
    pub(crate) const LUMP_LEAFBRUSHES: usize = 17;
    pub(crate) const LUMP_BRUSHES: usize = 18;
    pub(crate) const LUMP_BRUSHSIDES: usize = 19;
    pub(crate) const LUMP_AREAS: usize = 20;
    pub(crate) const LUMP_AREAPORTALS: usize = 21;
    pub(crate) const LUMP_PORTALS: usize = 22;
    pub(crate) const LUMP_CLUSTERS: usize = 23;
    pub(crate) const LUMP_PORTALVERTS: usize = 24;
    pub(crate) const LUMP_CLUSTERPORTALS: usize = 25;
    pub(crate) const LUMP_DISPINFO: usize = 26;
    pub(crate) const LUMP_ORIGINALFACES: usize = 27;
    pub(crate) const LUMP_PHYSDISP: usize = 28;
    pub(crate) const LUMP_PHYSCOLLIDE: usize = 29;
    pub(crate) const LUMP_VERTNORMALS: usize = 30;
    pub(crate) const LUMP_VERTNORMALINDICES: usize = 31;
    pub(crate) const LUMP_DISP_LIGHTMAP_ALPHAS: usize = 32;
    pub(crate) const LUMP_DISP_VERTS: usize = 33;
    pub(crate) const LUMP_DISP_LIGHTMAP_SAMPLE_POSITIONS: usize = 34;
    pub(crate) const LUMP_GAME_LUMP: usize = 35;
    pub(crate) const LUMP_LEAFWATERDATA: usize = 36;
    pub(crate) const LUMP_PRIMITIVES: usize = 37;
    pub(crate) const LUMP_PRIMVERTS: usize = 38;
    pub(crate) const LUMP_PRIMINDICES: usize = 39;
    pub(crate) const LUMP_PAKFILE: usize = 40;
    pub(crate) const LUMP_CLIPPORTALVERTS: usize = 41;
    pub(crate) const LUMP_CUBEMAPS: usize = 42;
    pub(crate) const LUMP_TEXDATA_STRING_DATA: usize = 43;
    pub(crate) const LUMP_TEXDATA_STRING_TABLE: usize = 44;
    pub(crate) const LUMP_OVERLAYS: usize = 45;
    pub(crate) const LUMP_LEAFMINDISTTOWATER: usize = 46;
    pub(crate) const LUMP_FACE_MACRO_TEXTURE_INFO: usize = 47;
    pub(crate) const LUMP_DISP_TRIS: usize = 48;
    pub(crate) const LUMP_PHYSCOLLIDESURFACE: usize = 49;
    pub(crate) const LUMP_WATEROVERLAYS: usize = 50;
    pub(crate) const LUMP_LIGHTMAPPAGES: usize = 51;
    pub(crate) const LUMP_LIGHTMAPPAGEINFOS: usize = 52;
    pub(crate) const LUMP_LIGHTING_HDR: usize = 53;
    pub(crate) const LUMP_WORLDLIGHTS_HDR: usize = 54;
    pub(crate) const LUMP_LEAF_AMBIENT_LIGHTING_HDR: usize = 55;
    pub(crate) const LUMP_LEAF_AMBIENT_LIGHTING: usize = 56;
    pub(crate) const LUMP_XZIPPAKFILE: usize = 57;
    pub(crate) const LUMP_FACES_HDR: usize = 58;
    pub(crate) const LUMP_MAP_FLAGS: usize = 59;
    pub(crate) const LUMP_OVERLAY_FADES: usize = 60;
    pub(crate) const LUMP_OVERLAY_SYSTEM_LEVELS: usize = 61;
    pub(crate) const LUMP_PHYSLEVEL: usize = 62;
    pub(crate) const LUMP_DISP_MULTIBLEND: usize = 63;
}

fn main() -> Result<()> {
    let mut file = OpenOptions::new().read(true).open("C:\\Users\\Admin\\Documents\\cp_border\\cp_border_011.bsp")?;

    let (version, lumps) = read_header(&mut file)?;

    println!("{version:?}");

    for lump in &lumps {
        println!("{:?} {:<8} {:<8} {:<8}", lump.id, lump.offset, lump.length, lump.version)
    }

    let faces = parse_faces(&mut file, lumps[lump_names::LUMP_FACES])?;
    println!("Number of faces: {:?}", faces.len());

    return Ok(())
}

fn read_header(file: &mut File) -> Result<(u32, Vec<Lump>)> {
    let mut header_bytes = [0u8; 8];
    file.read_exact(&mut header_bytes)?;

    let ident =u32::from_le_bytes(header_bytes[0..4].try_into().unwrap());

    // Magic Number
    assert_eq!(ident, u32::from_le_bytes(*b"VBSP"));
    let version = u32::from_le_bytes(header_bytes[4..8].try_into().unwrap());

    let mut lumps: Vec<Lump> = vec![];

    for i in 0..64 {
        lumps.push(read_lump(file)?);
    }

    return Ok((version, lumps));
}

fn read_lump(file: &mut File) -> Result<Lump> {
    let mut lump_bytes = [0u8; 16];
    file.read_exact(&mut lump_bytes)?;

    return Ok(Lump {
        offset: u32::from_le_bytes(lump_bytes[0..4].try_into().unwrap()),
        length: u32::from_le_bytes(lump_bytes[4..8].try_into().unwrap()),
        version: u32::from_le_bytes(lump_bytes[8..12].try_into().unwrap()),
        id: lump_bytes[12..16].try_into().unwrap()
    })
}

fn into_bool(x: u8) -> bool {
    match x {
        0 => false,
        1 => true,
        _ => panic!("Expected bool, got {x}")
    }
}

fn parse_faces(file: &mut File, lump: Lump) -> Result<Vec<Face>> {
    file.seek(SeekFrom::Start(lump.offset as u64))?;

    assert!(lump.length % 56 == 0);

    let mut out = vec![];

    for i in 0..lump.length / 56 {
        let mut data = [0u8; 56];
        file.read_exact(&mut data);

        out.push(
            Face {
                planenum: u16::from_be_bytes(data[0..2].try_into().unwrap()),
                side: into_bool(data[2]),
                on_node: into_bool(data[3]),
                first_edge: u32::from_be_bytes(data[4..8].try_into().unwrap()),
                num_edges: u16::from_be_bytes(data[8..10].try_into().unwrap()),
                tex_info: u16::from_be_bytes(data[10..12].try_into().unwrap()),
                displacement_info: u16::from_be_bytes(data[12..14].try_into().unwrap()),
                volume_id: u16::from_be_bytes(data[14..16].try_into().unwrap()),
                styles: data[16..20].try_into().unwrap(),
                lightmap_offset: u32::from_be_bytes(data[20..24].try_into().unwrap()),
                area: f32::from_be_bytes(data[24..28].try_into().unwrap()),
                lightmap_texture_mins_in_luxels: [
                    u32::from_be_bytes(data[28..32].try_into().unwrap()),
                    u32::from_be_bytes(data[32..36].try_into().unwrap()),
                ],
                lightmap_texture_size_in_luxels: [
                    u32::from_be_bytes(data[36..40].try_into().unwrap()),
                    u32::from_be_bytes(data[40..44].try_into().unwrap()),
                ],
                original_face: u32::from_be_bytes(data[44..48].try_into().unwrap()),
                number_of_primitives: u16::from_be_bytes(data[48..50].try_into().unwrap()),
                first_primitive_id: u16::from_be_bytes(data[50..52].try_into().unwrap()),
                smoothing_groups: u32::from_be_bytes(data[52..56].try_into().unwrap()),
            }
        )
    }

    Ok(out)
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Lump {
    offset: u32,
    length: u32,
    version: u32,
    id: [u8; 4]
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Edge {
    first: u16,
    second: u16
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vertex(Vector3);

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

struct Surfedge(i32);

struct Face {
    planenum: u16,
    side: bool,
    on_node: bool,
    first_edge: u32,
    num_edges: u16,
    tex_info: u16,
    displacement_info: u16,
    volume_id: u16,
    styles: [u8; 4],
    lightmap_offset: u32,
    area: f32,
    lightmap_texture_mins_in_luxels: [u32; 2],
    lightmap_texture_size_in_luxels: [u32; 2],
    original_face: u32,
    number_of_primitives: u16,
    first_primitive_id: u16,
    smoothing_groups: u32
}

