use cgmath::Vector3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Dir {
    /// +Z
    North = 0,
    /// -Z
    South = 1,
    /// +X
    East = 2,
    /// -X
    West = 3,
    /// +Y
    Up = 4,
    /// -Y
    Down = 5,
}
impl Dir {
    pub fn iter() -> impl Iterator<Item = Dir> {
        DirIter::new()
    }
}
impl From<Dir> for Vector3<i32> {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::North => Vector3::new(0, 0, 1),
            Dir::South => Vector3::new(0, 0, -1),
            Dir::East => Vector3::new(1, 0, 0),
            Dir::West => Vector3::new(-1, 0, 0),
            Dir::Up => Vector3::new(0, 1, 0),
            Dir::Down => Vector3::new(0, -1, 0),
        }
    }
}
impl From<u8> for Dir {
    fn from(n: u8) -> Self {
        match n {
            0 => Dir::North,
            1 => Dir::South,
            2 => Dir::East,
            3 => Dir::West,
            4 => Dir::Up,
            5 => Dir::Down,
            _ => {
                panic!()
            }
        }
    }
}
struct DirIter {
    current: u8,
}
impl DirIter {
    pub fn new() -> Self {
        DirIter { current: 0 }
    }
}
impl Iterator for DirIter {
    type Item = Dir;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        match self.current - 1 {
            0 => Some(Dir::North),
            1 => Some(Dir::South),
            2 => Some(Dir::East),
            3 => Some(Dir::West),
            4 => Some(Dir::Up),
            5 => Some(Dir::Down),
            _ => None,
        }
    }
}
