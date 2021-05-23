use crate::blocks::Block;
use crate::neighbours::Neighbours;
use cgmath::Vector3;
use specs::Component;
use specs::DenseVecStorage;

pub const CHUNK_SIZE: usize = 16;
#[derive(Debug, Component)]
pub struct Chunk {
    pub position: Vector3<i32>,
    pub blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    /// whether the mesh of this chunk has to be regenerated
    pub regenerate_mesh: bool,
    /// all the neighbours that have to updated
    pub update_neighbours: Neighbours<()>,
}

impl Chunk {
    pub fn empty(position: Vector3<i32>) -> Self {
        Self {
            position,
            blocks: Box::new([[[Block::Empty; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
            regenerate_mesh: false,
            update_neighbours: Neighbours::new(),
        }
    }
    pub fn get_block(&self, pos: Vector3<u16>) -> Block {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let z = pos.z as usize;
        self.blocks[x][y][z]
    }
    pub fn set_block(&mut self, pos: Vector3<u16>, block: Block) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let z = pos.z as usize;
        let b = &mut self.blocks[x][y][z];
        if *b != block {
            *b = block;
            self.regenerate_mesh = true;
            if x == 0 {
                self.update_neighbours.west = Some(());
            } else if x == CHUNK_SIZE - 1 {
                self.update_neighbours.east = Some(());
            }
            if y == 0 {
                self.update_neighbours.down = Some(());
            } else if y == CHUNK_SIZE - 1 {
                self.update_neighbours.up = Some(());
            }
            if z == 0 {
                self.update_neighbours.south = Some(());
            } else if z == CHUNK_SIZE - 1 {
                self.update_neighbours.north = Some(());
            }
        }
    }
}
