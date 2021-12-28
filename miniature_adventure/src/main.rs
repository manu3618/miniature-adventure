mod place;

fn main() {
    let room = place::build_empty_place();
    println!("Hello, world!");
    println!("{}", room.description)
}
