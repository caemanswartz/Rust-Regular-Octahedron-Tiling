#[cfg(test)]
#[path = "./octo_tests.rs"]
mod octo_tests;
use crate::tile::{
    Tile,
    TileType,
    Direction
};
#[derive(Clone,Debug,PartialEq)]
pub struct Octo {
    face_size: usize,
    tile_grid: Vec<Tile>
}
#[allow(dead_code)]
impl Octo {
    pub fn new(face_size: usize) -> Octo {
        let mut tile_grid = Vec::new();
        for i in 0..8 {
            let (mut h, mut d) = (0, true);
            for j in 0..usize::pow(face_size, 2) {
                if j >= usize::pow(h, 2) {
                    h += 1;
                } else {
                    d = !d;
                }
                tile_grid.push(Tile::new(
                    match d {
                        false => TileType::Flat,
                        _ => TileType::Point
                    }, face_size, i, j));
            }
        }
        Octo {
            face_size,
            tile_grid
        }
    }
    pub fn display(&self) {
        for i in 0..self.face_size {
            for j in 0..4 {
                for _ in 0..(self.face_size - i - 1) {
                    print!("     ");
                }
                for k in (usize::pow(i, 2))..(usize::pow(i + 1, 2)) {
                    print!("{:4} ", usize::pow(self.face_size, 2) * j + k);
                }
                for _ in 0..(self.face_size - i - 1) {
                    print!("     ");
                }
            }
            println!("");
        }
        for i in (0..self.face_size).rev() {
            for j in (4..8).rev() {
                for _ in 0..(self.face_size - i - 1) {
                    print!("     ");
                }
                for k in ((usize::pow(i, 2))..(usize::pow(i + 1, 2))).rev() {
                    print!("{:4} ", usize::pow(self.face_size, 2) * j + k);
                }
                for _ in 0..(self.face_size - i - 1) {
                    print!("     ");
                }
            }
            println!("");
        }
    }
    fn get_adjacent(&self, direction: Direction, tile_id: usize) -> usize {
        let face_base = usize::pow(self.face_size, 2);
        let face_id = tile_id / face_base;
        let orientation = match face_id {
            _ if face_id > 3 => -direction,
            _ => direction
        };
        let index_id = tile_id - face_id * face_base;
        let h = (index_id as f32).sqrt() as usize;
        match orientation {
            Direction::PosX =>
                match index_id {
                    0 => match face_id {
                        7 => 5 * face_base,
                        6 => 4 * face_base,
                        5 => 7 * face_base,
                        4 => 6 * face_base,
                        3 => face_base,
                        2 => 0,
                        1 => 3* face_base,
                        _ => 2 * face_base
                    },
                    _ if (index_id as f32 + 1.0).sqrt() % 1.0 == 0.0 =>
                        face_base * match face_id {
                            _ if face_id == 3 => 0,
                            _ if face_id == 7 => 4,
                            _ => face_id + 1
                    } + usize::pow(h - 1, 2),
                    _ if (index_id as f32).sqrt() % 1.0 == 0.0 =>
                        face_base * match face_id {
                            _ if face_id == 0 => 3,
                            _ if face_id == 4 => 7,
                            _ => face_id - 1
                    } + usize::pow(h, 2) - 1,
                    _ => tile_id - 2 * h
                },
            Direction::NegX => 
                match index_id {
                    _ if index_id >= usize::pow(self.face_size - 1, 2) =>
                        (8 - face_id) * face_base - index_id + usize::pow(self.face_size - 1, 2) - 1,
                    _ => tile_id + 2 * (h + 1)
                },
            Direction::PosY => 
                match self.tile_grid[tile_id].tile_type {
                    TileType::Point => match index_id {
                        0 => face_base * match face_id {
                            3 => 0,
                            7 => 4,
                            _ => face_id + 1
                        },
                        _ if (index_id as f32 + 1.0).sqrt() % 1.0 == 0.0 =>
                            face_base * match face_id {
                                3 => 0,
                                7 => 4,
                                _ => face_id + 1
                            } + usize::pow(h, 2),
                        _ => tile_id + 1,
                    },
                    _ => match index_id {
                        _ if (index_id as f32 + 2.0).sqrt() % 1.0 == 0.0 =>
                            face_base * match face_id {
                                3 => 0,
                                7 => 4,
                                _ => (face_id + 1)
                            } + usize::pow(h, 2) + 1,
                        _ => tile_id - 2 * h + 2
                    }
                },
            Direction::NegY => 
                match self.tile_grid[tile_id].tile_type {
                    TileType::Flat => tile_id - 1,
                    _ => match index_id {
                        _ if index_id == usize::pow(self.face_size - 1,2) =>
                            face_base * match face_id {
                                0 => 4,
                                4 => 0,
                                _ => 8 - face_id
                            } + usize::pow(h, 2),
                        _ if h == self.face_size - 1 =>
                            face_base * (8 - face_id) - index_id + usize::pow(self.face_size - 1, 2) + 1,
                        _ if index_id == usize::pow(h,2) => face_base * match face_id {
                                0 => 3,
                                4 => 7,
                                _ => face_id - 1,
                            } + usize::pow(h + 1, 2) + 2 * (h + 1),
                        _ => tile_id + 2 * (h + 1) - 2
                    }
                },
            Direction::PosZ => 
                match self.tile_grid[tile_id].tile_type {
                    TileType::Point => match index_id {
                        _ if index_id == face_base - 1 => face_base * match face_id {
                                0 => 6,
                                3 => 7,
                                7 => 3,
                                _ if face_id > 3 => 6 - face_id,
                                _ => 6 - face_id
                            } + usize::pow(h, 2) + 2 * h,
                        _ if index_id == usize::pow(h + 1, 2) - 1 => face_base * match face_id {
                                3 => 0,
                                7 => 4,
                                _ => face_id + 1
                            } + index_id + 1,
                        _ if h == self.face_size - 1 =>
                            (8 - face_id) * face_base - index_id + usize::pow(h, 2) - 3,
                        _ => tile_id + 2 * (h + 1) + 2
                    },
                    _ => tile_id + 1
                },
            Direction::NegZ => 
                match self.tile_grid[tile_id].tile_type {
                    TileType::Point => match index_id {
                        0 => face_base * match face_id {
                                0 => 3,
                                4 => 7,
                                _ => face_id - 1
                            },
                        _ if index_id == usize::pow(h,2) => face_base * match face_id {
                                0 => 3,
                                4 => 7,
                                _ => face_id - 1
                            } + usize::pow(h + 1, 2) - 1,
                        _ => tile_id - 1
                    },
                    TileType::Flat => match index_id {
                        _ if index_id == usize::pow(h, 2) + 1 => face_base * match face_id {
                            4 => 7,
                            0 => 3,
                            _ => face_id - 1
                        } + usize::pow(h, 2) + 2 * (h - 1) + 1,
                        _ => tile_id - 2 * h - 2
                    }
                }
        }
    }
}