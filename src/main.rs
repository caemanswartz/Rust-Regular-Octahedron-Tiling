mod tile;
mod octo;

fn main() {
    let size = 5;
    use octo::Octo;
    let octo = Octo::new(size);
    octo.display();
}