use crate::vector::{Vec2, Vec3};
use image::{DynamicImage, ImageError};
use json::{array, object, JsonError, JsonValue};
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

pub struct GltfObject<'a> {
    pub vertexes: &'a [Vec3],
    pub normals: &'a [Vec3],
    pub uvs: &'a [Vec2],
    pub indices: &'a [usize],
    pub texture: DynamicImage,
    pub name: &'a str,
}

impl<'a> GltfObject<'a> {
    fn byte_length_excluding_texture(&self) -> usize {
        return self.vertexes.len() * 4 * 3
            + self.normals.len() * 4 * 3
            + self.uvs.len() * 4 * 2
            + self.indices.len() * 4;
    }
}

pub fn save_mesh<'a, 'b>(
    filename: String,
    meshes: &'a [GltfObject<'b>],
) -> result::Result<(), SaveMeshError> {
    let images: Vec<Vec<u8>> = meshes
        .iter()
        .map(|mesh| -> Result<Vec<u8>, ImageError> {
            let mut image_bytes: Vec<u8> = Vec::new();
            mesh.texture.write_to(
                &mut Cursor::new(&mut image_bytes),
                image::ImageOutputFormat::Png,
            )?;
            Ok(image_bytes)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let buffer_offsets: Vec<usize> = (0..meshes.len())
        .map(|k| {
            (0..k)
                .map(|l| meshes[l].byte_length_excluding_texture() + pad_length(images[l].len()))
                .sum::<usize>()
        })
        .collect();
    let total_buffer_length = meshes
        .iter()
        .map(|i| i.byte_length_excluding_texture())
        .sum::<usize>()
        + images.iter().map(|i| pad_length(i.len())).sum::<usize>();

    let gltf_json_part = object! {
        "asset"=> object!{
            "generator": "None",
            "version": "2.0"
        },
        "scene"=> 0,
        "scenes"=>array![
            object!{
                "name"=> "Scene0",
                "nodes" => JsonValue::Array(
                    (0..meshes.len()).map(|i|JsonValue::Number(i.into())).collect::<Vec<_>>()
                )
            }
        ],
        "nodes"=>JsonValue::Array(
            meshes.iter().enumerate().map(|(index, mesh)|{
                object!{
                    "mesh"=>index,
                    "name"=>mesh.name
                }
            }).collect()
        ),
        "meshes"=> JsonValue::Array(
            meshes.iter().enumerate().map(|(index, _mesh)|{
                object! {
                    "primitives" => array![
                        object!{
                            "attributes"=>object!{
                                "NORMAL"=> index * 4,
                                "POSITION"=>index * 4 + 1,
                                "TEXCOORD_0"=>index * 4 + 2
                            },
                            "indices"=>index * 4 + 3,
                            "material"=>index
                        }
                    ]
                }
            }).collect()
        ),
        "textures"=>JsonValue::Array(
            meshes.iter().enumerate().map(|(index, _mesh)|
                object! {
                    "source"=>index,
                    "sampler"=>0
                }
            ).collect()
        ),
        "images"=>JsonValue::Array(
            meshes.iter().enumerate().map(|(index, _mesh)|
                object! {
                    "bufferView"=>4 + 5 * index,
                    "mimeType"=>"image/png",
                    "name"=>format!("texture{index}")
                }
            ).collect()
        ),
        "materials"=>JsonValue::Array(
            meshes.iter().enumerate().map(|(index, _mesh)|
                object! {
                    "pbrMetallicRoughness" => object!{
                        "baseColorTexture" => object!{
                            "index" => index,
                            "texCoord" => 0
                        }
                    }
                }
            ).collect()
        ),
        "samplers"=>array![
            object!{
                "magFilter"=>9728,
                "minFilter"=>9728
            }
        ],
        "accessors"=>JsonValue::Array(
            meshes.iter().enumerate().flat_map(|(index, mesh)|{

            let min_vertex = [
                float_min(mesh.vertexes.iter().map(|i| i.x)),
                float_min(mesh.vertexes.iter().map(|i| i.y)),
                float_min(mesh.vertexes.iter().map(|i| i.z)),
            ];
            let max_vertex = [
                float_max(mesh.vertexes.iter().map(|i| i.x)),
                float_max(mesh.vertexes.iter().map(|i| i.y)),
                float_max(mesh.vertexes.iter().map(|i| i.z)),
            ];

            return [
                object!{
                    "bufferView"=>0 + index * 5,
                    "componentType"=> 5126_u32, // Float
                    "count"=> mesh.normals.len(),
                    "type"=> "VEC3"
                },
                object!{
                    "bufferView"=>1 + index * 5,
                    "componentType"=> 5126_u32, // Float
                    "count"=> mesh.vertexes.len(),
                    "type"=> "VEC3",
                    "min"=>array![min_vertex[0], min_vertex[1], min_vertex[2]],
                    "max"=>array![max_vertex[0], max_vertex[1], max_vertex[2]],
                },
                object!{
                    "bufferView"=>2 + index * 5,
                    "componentType"=> 5126_u32, // Float
                    "count"=> mesh.uvs.len(),
                    "type"=> "VEC2"
                },
                object!{
                    "bufferView"=>3 + index * 5,
                    "componentType"=> 5125_u32, // Unsigned Int
                    "count"=> mesh.indices.len(),
                    "type"=> "SCALAR"
                }
            ]}).collect()
        ),
        "bufferViews"=> JsonValue::Array(meshes.iter().enumerate().flat_map(|(index, mesh)| [
            object!{
                "buffer"=>0,
                "byteOffset"=>buffer_offsets[index],
                "byteLength"=>4 *3 * mesh.normals.len(),
            },
            object!{
                "buffer"=>0,
                "byteOffset"=>buffer_offsets[index] + 4 * 3 * mesh.normals.len(),
                "byteLength"=>4 * 3 * mesh.vertexes.len(),
            },

            object!{
                "buffer"=>0,
                "byteOffset"=>buffer_offsets[index] + 4 * 3 * mesh.normals.len() + 4 * 3 * mesh.vertexes.len(),
                "byteLength"=>4 * 2 * mesh.uvs.len(),
            },
            object!{
                "buffer"=>0,
                "byteOffset"=>buffer_offsets[index] +4 * 3 * mesh.normals.len() + 4 * 3 * mesh.vertexes.len() + 4 * 2 * mesh.uvs.len(),
                "byteLength"=>4 * mesh.indices.len(),
            },
            // Texture
            object!{
                "buffer"=>0,
                "byteOffset"=>buffer_offsets[index] +4 * 3 * mesh.normals.len() + 4 * 3 * mesh.vertexes.len() + 4 * 2 * mesh.uvs.len() + 4 * mesh.indices.len(),
                "byteLength"=>images[index].len(),
            }
        ]).collect()),
        "buffers"=>array![
            object!{
                "byteLength"=>total_buffer_length
            },
        ]
    };

    fs::create_dir_all("cache")?;
    let mut jsfile = File::create(format!("cache/{:}.json", filename)).unwrap();
    jsfile.write_all(json::stringify_pretty(gltf_json_part.clone(), 2).as_bytes())?;

    let mut data = json::stringify(gltf_json_part);
    while data.len() % 4 != 0 {
        data += " ";
    }

    let mut file = File::create(format!("cache/{:}.glb", filename)).unwrap();
    file.write_all("glTF".as_bytes())?;
    file.write_all(&2_u32.to_le_bytes())?;
    file.write_all(
        &((
            pad_length(data.len() +
            total_buffer_length
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

    file.write_all(&(pad_length(total_buffer_length) as u32).to_le_bytes())?;
    file.write_all("BIN".as_bytes())?;
    file.write(&[0])?;

    for (mesh, image) in meshes.iter().zip(images.iter()) {
        let buffer_normals: Vec<u8> = mesh
            .normals
            .iter()
            .map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes(), x.z.to_le_bytes()])
            .flatten()
            .flatten()
            .collect();
        let buffer_positions: Vec<u8> = mesh
            .vertexes
            .iter()
            .map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes(), x.z.to_le_bytes()])
            .flatten()
            .flatten()
            .collect();
        let buffer_uvs: Vec<u8> = mesh
            .uvs
            .iter()
            .map(|x| [x.x.to_le_bytes(), x.y.to_le_bytes()])
            .flatten()
            .flatten()
            .collect();
        let buffer_indices: Vec<u8> = mesh
            .indices
            .iter()
            .map(|x| (*x as u32).to_le_bytes())
            .flatten()
            .collect();

        file.write_all(buffer_normals.as_slice())?;
        file.write_all(buffer_positions.as_slice())?;
        file.write_all(buffer_uvs.as_slice())?;
        file.write_all(buffer_indices.as_slice())?;
        file.write_all(image.as_slice())?;

        let cursor_position = file.seek(SeekFrom::Current(0))?;
        for _i in 0..((4 - cursor_position % 4) % 4) {
            file.write(&[0])?;
        }

        //let mut img_file = File::create(format!("cache/{}.png", filename))?;
        //img_file.write_all(image_bytes.as_slice())?;
    }
    return result::Result::Ok(());
}
