
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
                if j >= h * h {
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

    pub fn print_face(&self, face_id: usize) {
        let face_base = usize::pow(self.face_size, 2);
        let start = face_id * face_base;
        if face_id <= 3 {
            for i in start..(start + face_base) {
                let index_id = i - face_id * face_base;
                let h = (index_id as f32).sqrt() as usize;
                if (index_id as f32).sqrt() % 1.0 == 0.0 {
                    println!("");
                    for _ in 0..(self.face_size - h - 1) {
                        print!("     ");
                    }
                }
                let tile_type = &self.tile_grid[i].tile_type;
                if tile_type == &TileType::Point { print!("/");}
                else {print!(" ");}
                print!{"{:3} ", self.tile_grid[i].tile_id};
                if tile_type == &TileType::Point { print!("\\");}
                else {print!(" ");}
            }
        }
        else {
            for i in (start..(start + face_base)).rev() {
                let index_id = i - face_id * face_base;
                let h = (index_id as f32).sqrt() as usize;
                if (index_id as f32 + 1.0).sqrt() % 1.0 == 0.0 {
                    println!("");
                    for _ in 0..(self.face_size - h - 1) {
                        print!("     ");
                    }
                }
                let tile_type = &self.tile_grid[i].tile_type;
                if tile_type == &TileType::Point { print!("\\");}
                else {print!(" ");}
                print!{"{:3} ", self.tile_grid[i].tile_id};
                if tile_type == &TileType::Point { print!("/");}
                else {print!(" ");}
            }
        }
    }

    fn get_adjacent(&self, direction: Direction, tile_id: usize) -> usize {
        let face_base = self.face_size * self.face_size;
        let face_id = tile_id / face_base;
        print!("t:{} {:?} -> ", tile_id, direction);
        let orientation = match face_id {
            _ if face_id > 3 => match direction {
                    Direction::PlusX => Direction::MinusX,
                    Direction::PlusY => Direction::MinusY,
                    Direction::PlusZ => Direction::MinusZ,
                    Direction::MinusX => Direction::PlusX,
                    Direction::MinusY => Direction::PlusY,
                    Direction::MinusZ => Direction::PlusZ
                },
            _ => direction
        };
        println!("{:?}", orientation);
        match orientation {
            Direction::PlusX => self.get_x_plus(tile_id, face_base, face_id),
            Direction::PlusY => self.get_y_plus(tile_id, face_base, face_id),
            Direction::PlusZ => self.get_z_plus(tile_id, face_base, face_id),
            Direction::MinusX => self.get_x_minus(tile_id, face_base, face_id),
            Direction::MinusY => self.get_y_minus(tile_id, face_base, face_id),
            Direction::MinusZ => self.get_z_minus(tile_id, face_base, face_id)
        }
    }
    fn get_x_plus(&self, tile_id: usize, face_base: usize, face_id: usize) -> usize {
        let index_id = tile_id - face_id * face_base;
        let h = (index_id as f32).sqrt() as usize;
        println!("x plus t:{} f:{} i:{} h:{}", tile_id, face_id, index_id, h);
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
            _ if (index_id as f32 + 1.0).sqrt() % 1.0 == 0.0 => face_base * match face_id {
                _ if face_id == 3 => 0,
                _ if face_id == 7 => 4,
                _ => face_id + 1
            } + usize::pow(h - 1, 2),
            _ if (index_id as f32).sqrt() % 1.0 == 0.0 => face_base * match face_id {
                _ if face_id == 0 => 3,
                _ if face_id == 4 => 7,
                _ => face_id - 1
            } + usize::pow(h, 2) - 1,
            _ => tile_id - 2 * h
        }
    }
    fn get_x_minus(&self, tile_id: usize, face_base: usize, face_id: usize) -> usize {
        let index_id = tile_id - face_id * face_base;
        let h = (index_id as f32).sqrt() as usize;
        let b = usize::pow(self.face_size - 1, 2);
        println!("x minus t:{} f:{} i:{} h:{} b:{}", tile_id, face_id, index_id, h, b);
        match index_id {
            _ if index_id >= b => (8 - face_id) * face_base - index_id + b - 1,
            _ => tile_id + 2 * (h + 1)
        }
    }
    fn get_y_plus(&self, tile_id: usize, face_base: usize, face_id: usize) -> usize {
        let index_id = tile_id - face_id * face_base;
        let h = (index_id as f32).sqrt() as usize;
        println!("y plus t:{} f:{} i:{} h:{} t: {:?}", tile_id,face_id,index_id,h,self.tile_grid[tile_id].tile_type);
        match self.tile_grid[tile_id].tile_type {
            TileType::Point => match index_id {
                // top corner
                0 => face_base * match face_id {
                    3 => 0,
                    7 => 4,
                    _ => face_id + 1
                },
                // y plus side point
                _ if (index_id as f32 + 1.0).sqrt() % 1.0 == 0.0 => face_base * match face_id {
                        3 => 0,
                        7 => 4,
                        _ => face_id + 1
                    } + usize::pow(h, 2),
                _ => tile_id + 1,
            },
            _ => match index_id {
                // y plus side flat
                _ if (index_id as f32 + 2.0).sqrt() % 1.0 == 0.0 => face_base * match face_id {
                        3 => 0,
                        7 => 4,
                        _ => (face_id + 1)
                    } + usize::pow(h, 2) + 1,
                _ => tile_id - 2 * h + 2
            }
        }
    }
    fn get_y_minus(&self, tile_id: usize, face_base: usize, face_id: usize) -> usize {
        let index_id = tile_id - face_id * face_base;
        let h = (index_id as f32).sqrt() as usize;
        println!("y minus t:{} f:{} i:{} h:{} t: {:?}", tile_id,face_id,index_id,h,self.tile_grid[tile_id].tile_type);
        match self.tile_grid[tile_id].tile_type {
            TileType::Flat => tile_id - 1,
            _ => match index_id {
                // y minus corner
                _ if index_id == usize::pow(self.face_size - 1,2) => face_base * match face_id {
                        0 => 4,
                        4 => 0,
                        _ => 8 - face_id
                    } + usize::pow(h, 2),
                // pyramid change
                _ if h == self.face_size - 1 =>
                    face_base * (8 - face_id) - index_id + usize::pow(self.face_size - 1, 2) + 1,
                // side change
                _ if index_id == usize::pow(h,2) => face_base * match face_id {
                        0 => 3,
                        4 => 7,
                        _ => face_id - 1,
                    } + usize::pow(h + 1, 2) + 2 * (h + 1),
                _ => tile_id + 2 * (h + 1) - 2
            }
        }
    }
    fn get_z_plus(&self, tile_id: usize, face_base: usize, face_id: usize) -> usize {
        let index_id = tile_id - face_id * face_base;
        let h = (index_id as f32).sqrt() as usize;
        println!("z plus t:{} f:{} i:{} h:{} t: {:?}", tile_id,face_id,index_id,h,self.tile_grid[tile_id].tile_type);
        match self.tile_grid[tile_id].tile_type {
            TileType::Point => match index_id {
                // z plus corner
                _ if index_id == face_base - 1 => face_base * match face_id {
                        0 => 6,
                        3 => 7,
                        7 => 3,
                        _ if face_id > 3 => 6 - face_id,
                        _ => 6 - face_id
                    } + usize::pow(h, 2) + 2 * h,
                // point side
                _ if index_id == usize::pow(h + 1, 2) - 1 => face_base * match face_id {
                        3 => 0,
                        7 => 4,
                        _ => face_id + 1
                    } + index_id + 1,
                // pyramid change
                _ if h == self.face_size - 1 => (8 - face_id) * face_base - index_id + usize::pow(h, 2) - 3,
                _ => tile_id + 2 * (h + 1) + 2
            },
            _ => tile_id + 1
        }
    }
    fn get_z_minus(&self, tile_id: usize, face_base: usize, face_id: usize) -> usize {
        let index_id = tile_id - face_id * face_base;
        let h = (index_id as f32).sqrt() as usize;
        println!("z minus t:{} f:{} i:{} h:{} t: {:?}", tile_id,face_id,index_id,h,self.tile_grid[tile_id].tile_type);
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