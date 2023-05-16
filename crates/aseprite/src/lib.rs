use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub frames: Vec<FrameItem>,
}

impl Output {
    pub fn from<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(
            std::fs::read_to_string(path)?.as_str(),
        )?)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FrameItem {
    pub filename: String,
    pub frame: Frame,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}
