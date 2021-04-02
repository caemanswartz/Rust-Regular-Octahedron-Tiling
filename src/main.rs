mod tile;
mod octo;

fn main() {
    let size = 10;
    use {octo::Octo, tile::Direction};
    let octo = Octo::new(size);
    octo.display();
    for i in 0..(usize::pow(size, 2) * 8) {
        let start_tile = i;
        let start_direction = Direction::NegZ;
        println!("start: {} {:?}", start_tile, start_direction);
        let mut current = octo.step((start_tile, start_direction.clone()));
        while current.0 != start_tile {
            println!("{:?}",current);
            current = octo.step(current);
        }
        assert_eq!(current, (start_tile, start_direction));
    }
}