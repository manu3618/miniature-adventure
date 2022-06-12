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
    println!("XXX 1400 parent:{:?}", parent);
    parent.set_item(3);
    println!("XXX 1600 parent:{:?}", parent);
    parent.add_child(5);
    println!("XXX 1800 parent:{:?}", parent);
    let mut children = &parent.clone().get_children();
    println!("XXX 2000 parent:{:?}", parent);
    let child = &children[0];
    println!("parent:{:?}", parent);
    println!("child :{:?}", child);

    //let item = child.get_item();
    //println!("{}", item.unwrap());
}
