mod place;

fn main() {
    let room = place::Place::new();
    println!("Hello, world!");
    println!("{}", room.describe())
}
