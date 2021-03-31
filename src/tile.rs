#[allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum TileType {
    Point,
    Flat
}
#[allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum Direction {
    PlusX,
    PlusY,
    PlusZ,
    MinusX,
    MinusY,
    MinusZ
}
#[derive(Clone,Debug,PartialEq)]
pub struct Tile {
    pub tile_type: TileType,
    pub tile_id: usize
}
impl Tile {
    pub fn new(tile_type: TileType, face_size: usize, face_id: usize, index_id: usize) -> Tile {
        Tile {
            tile_id: face_id * usize::pow(face_size, 2) + index_id,
            tile_type
        }
    }
}