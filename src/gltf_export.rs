use crate::vector::{Vec2, Vec3};
use image::{DynamicImage, ImageError};
use json::{array, object, JsonError};
use std::fs::File;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::{fs, result};
use thiserror::Error;

fn float_max<T>(it: T) -> f32
where
    T: Iterator<Item = f32>,
{
    it.fold(-1. / 0., f32::max)
}

fn float_min<T>(it: T) -> f32
where
    T: Iterator<Item = f32>,
{
    it.fold(1. / 0., f32::min)
}

fn pad_length(x: usize) -> usize {
    ((x + 3) / 4) * 4
}

#[derive(Debug, Error)]
pub enum SaveMeshError {
    #[error("Json Error")]
    JsonError(#[from] JsonError),
    #[error("IO Error")]
    IOError(#[from] std::io::Error),
    #[error("Image Error")]
    ImageError(#[from] ImageError),
}

pub fn save_mesh(
    filename: String,
    vertexes: &Vec<Vec3>,
    normals: &Vec<Vec3>,
    uvs: &Vec<Vec2>,
    indices: &Vec<usize>,
    texture: DynamicImage,
) -> result::Result<(), SaveMeshError> {
    let min_vertex = [
        float_min(vertexes.iter().map(|i| i.x)),
        float_min(vertexes.iter().map(|i| i.y)),
        float_min(vertexes.iter().map(|i| i.z)),
    ];
    let max_vertex = [
        float_max(vertexes.iter().map(|i| i.x)),
        float_max(vertexes.iter().map(|i| i.y)),
        float_max(vertexes.iter().map(|i| i.z)),
    ];

    let mut image_bytes: Vec<u8> = Vec::new();
    texture.write_to(
        &mut Cursor::new(&mut image_bytes),
        image::ImageOutputFormat::Png,
    )?;

    let gltf_json_part = object! {
        "asset"=> object!{
            "generator": "None",
            "version": "2.0"
        },
        "scene"=> 0,
        "scenes"=>array![
            object!{
                "name"=> "Scene0",
                "nodes" => array![0]
            }
        ],
        "nodes"=>array![
            object!{
                "mesh"=>0,
                "name"=>"curve"
            }
        ],
        "meshes"=> array![
            object!{
                "primitives"=> array![
                    object!{
                        "attributes"=>object!{
                            "NORMAL"=> 0,
                            "POSITION"=>1,
                            "TEXCOORD_0"=>2
                        },
                        "indices"=>3,
                        "material"=>0
                    }
                ],
            }
        ],
        "textures"=>array![
            object!{
                "source"=>0,
                "sampler"=>0
            }
        ],
        "images"=>array![
            object!{
                "bufferView"=>4,
                "mimeType"=>"image/png",
                "name"=>"texture0"
            }
        ],
        "materials"=>array![
            object!{
                "pbrMetallicRoughness" => object!{
                    "baseColorTexture" => object!{
                        "index" => 0,
                        "texCoord" => 0
                    }
                }
            }
        ],
        "samplers"=>array![
            object!{
                "magFilter"=>9728,
                "minFilter"=>9728
            }
        ],
        "accessors"=>array![
            object!{
                "bufferView"=>0,
                "componentType"=> 5126_u32, // Float
                "count"=> normals.len(),
                "type"=> "VEC3"
            },
            object!{
                "bufferView"=>1,
                "componentType"=> 5126_u32, // Float
                "count"=> vertexes.len(),
                "type"=> "VEC3",
                "min"=>array![min_vertex[0], min_vertex[1], min_vertex[2]],
                "max"=>array![max_vertex[0], max_vertex[1], max_vertex[2]],
            },
            object!{
                "bufferView"=>2,
                "componentType"=> 5126_u32, // Float
                "count"=> uvs.len(),
                "type"=> "VEC2"
            },
            object!{
                "bufferView"=>3,
                "componentType"=> 5125_u32, // Unsigned Int
                "count"=> indices.len(),
                "type"=> "SCALAR"
            }
        ],
        "bufferViews"=> array![
            object!{
                "buffer"=>0,
                "byteLength"=>4 *3 * normals.len(),
                "byteOffset"=>0,
            },
            object!{
                "buffer"=>0,
                "byteOffset"=>4 * 3 * normals.len(),
                "byteLength"=>4 * 3 * vertexes.len(),
            },

            object!{
                "buffer"=>0,
                "byteOffset"=>4 * 3 * normals.len() + 4 * 3 * vertexes.len(),
                "byteLength"=>4 * 2 * uvs.len(),
            },
            object!{
                "buffer"=>0,
                "byteOffset"=>4 * 3 * normals.len() + 4 * 3 * vertexes.len() + 4 * 2 * uvs.len(),
                "byteLength"=>4 * indices.len(),
            },
            // Texture
            object!{
                "buffer"=>0,
                "byteLength"=>image_bytes.len(),
                "byteOffset"=>4 * 3 * normals.len() + 4 * 3 * vertexes.len() + 4 * 2 * uvs.len() + 4 * indices.len()
            }
        ],
        "buffers"=>array![
            object!{
                "byteLength"=>4 * 3 * normals.len() + 4 * 3 * vertexes.len() + 4 * 2 * uvs.len() + 4 * indices.len() + image_bytes.len()
            },
        ]
    };

    fs::create_dir_all("cache")?;
    let mut jsfile = File::create(format!("cache/{:}.json", filename)).unwrap();
    jsfile.write_all(json::stringify_pretty(gltf_json_part.clone(), 2).as_bytes())?;

    let mut data = json::stringify(gltf_json_part);
    while data.len() % 4 != 0 {
        data += " "
    }

    let buffer_normals: Vec<u8> = normals
        .iter()
        .map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes(), x.z.to_le_bytes()])
        .flatten()
        .flatten()
        .collect();
    let buffer_positions: Vec<u8> = vertexes
        .iter()
        .map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes(), x.z.to_le_bytes()])
        .flatten()
        .flatten()
        .collect();
    let buffer_uvs: Vec<u8> = uvs
        .iter()
        .map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes()])
        .flatten()
        .flatten()
        .collect();
    let buffer_indices: Vec<u8> = indices
        .iter()
        .map(|x| (*x as u32).to_le_bytes())
        .flatten()
        .collect();

    let mut file = File::create(format!("cache/{:}.glb", filename)).unwrap();
    file.write_all("glTF".as_bytes())?;
    file.write_all(&2_u32.to_le_bytes())?;
    file.write_all(
        &((
            pad_length(data.len() +
                    buffer_normals.len() +
                    buffer_positions.len() +
                    buffer_uvs.len() +
                    buffer_indices.len() +
                    image_bytes.len()
                ) +
                    16 + // Chunk headers
                    12
            // Top header
        ) as u32)
            .to_le_bytes(),
    )?;

    file.write_all(&(data.len() as u32).to_le_bytes())?;
    file.write_all("JSON".as_bytes())?;
    file.write(data.as_bytes())?;

    file.write_all(
        &(pad_length(
            buffer_positions.len()
                + buffer_normals.len()
                + buffer_uvs.len()
                + buffer_indices.len()
                + image_bytes.len(),
        ) as u32)
            .to_le_bytes(),
    )?;
    file.write_all("BIN".as_bytes())?;
    file.write(&[0])?;
    file.write_all(buffer_normals.as_slice())?;
    file.write_all(buffer_positions.as_slice())?;
    file.write_all(buffer_uvs.as_slice())?;
    file.write_all(buffer_indices.as_slice())?;
    file.write_all(image_bytes.as_slice())?;

    let cursor_position = file.seek(SeekFrom::Current(0))?;
    for _i in 0..((4 - cursor_position % 4) % 4) {
        file.write(&[0])?;
    }

    let mut img_file = File::create(format!("cache/{}.png", filename))?;
    img_file.write_all(image_bytes.as_slice())?;
    return result::Result::Ok(());
}
