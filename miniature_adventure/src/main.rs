#[macro_use]
extern crate lazy_static;

mod graph;
mod place;

fn main() {
    let room = place::Place::new();
    println!("Hello, world!");
    println!("{}", room.describe());
    println!("{}", room.describe());

    let mut parent: graph::Node<i32> = graph::Node::new();
    parent.set_item(4);
    parent.add_child(5);
    let mut children = &parent.clone().get_children();
    let child = &children[0];
    println!("parent:{:?}", parent);
    println!("child :{:?}", child);
}
