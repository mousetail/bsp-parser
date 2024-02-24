use parse_bsp::parse_bsp;

mod comma_format;
mod gltf_export;
mod parse_bsp;
mod vector;

fn main() -> std::io::Result<()> {
    //parse_bsp("D:\\steam\\steamapps\\common\\Team Fortress 2\\tf\\maps\\cp_gorge.bsp")
    parse_bsp("D:\\steam\\steamapps\\common\\Team Fortress 2\\tf\\maps\\plr_pipeline.bsp")
    //parse_bsp::parse_bsp("C:\\Users\\Admin\\Documents\\cp_border\\cp_border_011.bsp")
    //parse_bsp("D:\\steam\\steamapps\\common\\Team Fortress 2\\tf\\maps\\cp_junction_final.bsp")
}
