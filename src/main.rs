


mod gltf_export;
mod vector;
mod parse_bsp;


fn main() -> std::io::Result<()> {
    parse_bsp::parse_bsp("C:\\Users\\Admin\\Documents\\cp_border\\cp_border_011.bsp")
}