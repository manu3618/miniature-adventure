#[macro_use]
extern crate lazy_static;

mod graph;
mod place;

fn main() {
    let room = place::Place::new();
    println!("Hello, world!");
    println!("{}", room.describe());
    println!("{}", room.describe());
}
