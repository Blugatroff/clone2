use cgmath::Vector2;
use image::DynamicImage;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct UvRect {
    top_left_y: f32,
    top_left_x: f32,
    down_right_x: f32,
    down_right_y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Texture {
    pub name: String,
    pub rect: UvRect,
}

pub struct UvBlock {
    pub name: String,
    pub base: usize,
    pub top: Option<usize>,
    pub side: Option<usize>,
    pub index: usize,
}

pub struct Atlas {
    pub atlas: DynamicImage,
    pub blocks: Vec<UvBlock>,
    pub all_uvs: Vec<[f32; 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Block {
    Empty = 0,
    Dirt = 1,
    Stone = 2,
    Sand = 3,
    Water = 4,
    Grass = 5,
}

impl From<u32> for Block {
    fn from(n: u32) -> Self {
        match n {
            0 => Self::Empty,
            1 => Self::Dirt,
            2 => Self::Stone,
            3 => Self::Sand,
            4 => Self::Water,
            5 => Self::Grass,
            _ => {
                panic!("conversion from u32 ({}) to Block failed", n)
            }
        }
    }
}

impl Atlas {
    pub fn load() -> Self {
        let atlas = image::open("atlas.png").unwrap();
        let textures: Vec<Texture> =
            serde_json::from_str(&std::fs::read_to_string("map.json").unwrap()).unwrap();

        let mut dir: HashMap<String, usize> = HashMap::new();
        for (i, texture) in textures.iter().enumerate() {
            dir.insert(texture.name.clone(), i);
        }

        let mut blocks = vec![
            UvBlock {
                name: "emtpy".into(),
                base: *dir.get("stone").unwrap(), // just a filler for empty
                top: None,
                side: None,
                index: 0,
            },
            UvBlock {
                name: "dirt".into(),
                base: *dir.get("dirt").unwrap(),
                top: None,
                side: None,
                index: 0,
            },
            UvBlock {
                name: "stone".into(),
                base: *dir.get("stone").unwrap(),
                top: None,
                side: None,
                index: 0,
            },
            UvBlock {
                name: "sand".into(),
                base: *dir.get("sand").unwrap(),
                top: None,
                side: None,
                index: 0,
            },
            UvBlock {
                name: "water".into(),
                base: *dir.get("water").unwrap(),
                top: None,
                side: None,
                index: 0,
            },
            UvBlock {
                name: "grass".into(),
                base: *dir.get("dirt").unwrap(),
                top: Some(*dir.get("grass_top").unwrap()),
                side: Some(*dir.get("grass_side").unwrap()),
                index: 0,
            },
        ];

        let mut all_uvs: Vec<[f32; 2]> = vec![];
        for block in &mut blocks {
            block.index = all_uvs.len();
            let mut add = |i: usize| {
                let rect = &textures[i].rect;
                all_uvs.push(Vector2::new(rect.top_left_x, rect.top_left_y).into());
                all_uvs.push(Vector2::new(rect.down_right_x, rect.top_left_y).into());
                all_uvs.push(Vector2::new(rect.top_left_x, rect.down_right_y).into());
                all_uvs.push(Vector2::new(rect.down_right_x, rect.down_right_y).into());
            };
            {
                add(block.base);
            }
            if let Some(side) = block.side {
                add(side);
            } else {
                add(block.base);
            }
            if let Some(top) = block.top {
                add(top);
            } else {
                add(block.base);
            }
        }
        Self {
            atlas,
            blocks,
            all_uvs,
        }
    }
    pub fn uvs_of_block_index(&self, block: usize, side: Side) -> [u16; 4] {
        let block = &self.blocks[block];
        let offset = match side {
            Side::Base => 0,
            Side::Top => 8,
            Side::Side => 4,
        };
        [
            block.index as u16 + offset,
            block.index as u16 + offset + 1,
            block.index as u16 + offset + 2,
            block.index as u16 + offset + 3,
        ]
    }
}

pub enum Side {
    Base,
    Top,
    Side,
}
