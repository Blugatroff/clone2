use crate::dir::Dir;
#[derive(Debug, Clone)]
pub struct Neighbours<T> {
    pub north: Option<T>,
    pub south: Option<T>,
    pub west: Option<T>,
    pub east: Option<T>,
    pub up: Option<T>,
    pub down: Option<T>,
}
impl<'a, T> Neighbours<T> {
    pub fn iter(&'a self) -> NeighboursIter<'a, T> {
        NeighboursIter {
            current: 0,
            neighbours: &self,
        }
    }
    pub fn clear(&mut self) {
        self.north = None;
        self.south = None;
        self.west = None;
        self.east = None;
        self.up = None;
        self.down = None;
    }
    pub fn new() -> Self {
        Self {
            north: None,
            south: None,
            west: None,
            east: None,
            up: None,
            down: None,
        }
    }
}
pub struct NeighboursIter<'a, T> {
    current: u8,
    neighbours: &'a Neighbours<T>,
}
impl<'a, T> Iterator for NeighboursIter<'a, T> {
    type Item = (Dir, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;
            match self.current - 1 {
                0 => {
                    if let Some(north) = &self.neighbours.north {
                        return Some((Dir::North, north));
                    }
                }
                1 => {
                    if let Some(south) = &self.neighbours.south {
                        return Some((Dir::South, south));
                    }
                }
                2 => {
                    if let Some(west) = &self.neighbours.west {
                        return Some((Dir::West, west));
                    }
                }
                3 => {
                    if let Some(east) = &self.neighbours.east {
                        return Some((Dir::East, east));
                    }
                }
                4 => {
                    if let Some(up) = &self.neighbours.up {
                        return Some((Dir::Up, up));
                    }
                }
                5 => {
                    if let Some(down) = &self.neighbours.down {
                        return Some((Dir::Down, down));
                    }
                }
                _ => return None,
            }
        }
    }
}

#[test]
fn neighbour_test() {
    let mut neighbours = Neighbours {
        north: Some(0),
        south: None,
        west: Some(45),
        east: Some(-24345),
        up: None,
        down: Some(349934),
    }
    .iter();
    assert_eq!(neighbours.next(), Some((Dir::North, &0)));
    assert_eq!(neighbours.next(), Some((Dir::West, &45)));
    assert_eq!(neighbours.next(), Some((Dir::East, &-24345)));
    assert_eq!(neighbours.next(), Some((Dir::Down, &349934)));
    assert_eq!(neighbours.next(), None);
}
