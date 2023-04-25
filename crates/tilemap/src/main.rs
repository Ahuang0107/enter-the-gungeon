use std::io::Write;
use tilemap::TilemapWorld;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let world = TilemapWorld::from_ldtk("assets/level.ldtk")?;
    let world_json = serde_json::to_string(&world)?;
    std::fs::File::create("assets/level_output.json")?.write_all(world_json.as_bytes())?;
    Ok(())
}
