mod tile;
mod octo;

fn main() {
    let size = 3;
    use octo::Octo;
    let octo = Octo::new(size);
    println!("{:?}", octo);
    for i in 0..4 {
        octo.print_face(i);
        octo.print_face(7 - i);
        println!("");
    }
}