/// minimal implementation of directed graphs

/// Node of a graph.
///
/// the node is linked to zero, one, or several parents
/// the node links to zero, one, or several childs
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[derive(Debug, Clone)]
pub enum Node<T: Copy> {
    None,
    Node {
        item: Option<T>,
        id: String,
        parents: Box<Vec<Node<T>>>,
        children: Box<Vec<Node<T>>>,
    },
}

impl<T: Copy> Node<T> {
    pub fn new() -> Self {
        Self::None
    }
    /// add new node with an item
    pub fn add_child(&mut self, new_item: T) {
        let mut child = Self::Node {
            item: Some(new_item),
            id: "".to_string(),
            parents: Box::new(Vec::new()),
            children: Box::new(Vec::new()),
        };

        match self {
            Self::None => {}
            Self::Node { id, children, .. } => {
                let parent_id = id.clone();
                match child {
                    Self::Node { ref mut id, .. } => {
                        let child_id =
                            format!("{}_{}_{}", parent_id, children.capacity(), children.len());
                        *id = String::from(child_id);
                    }
                    _ => {
                        panic!("Child not initialized as expected")
                    }
                }
                children.push(child)
            }
        }
    }
    /// link to new parent
    pub fn add_parent(self, parent_node: Node<T>) {}
    /// returns vector of parents
    pub fn get_parents(self) {}
    pub fn get_item(self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Node { item, .. } => match item {
                None => None,
                Some(current_item) => Some(current_item),
            },
        }
    }
    pub fn set_item(&mut self, new_item: T) {
        match self {
            Self::None => {
                *self = Self::Node {
                    item: Some(new_item),
                    id: "".to_string(),
                    parents: Box::new(Vec::new()),
                    children: Box::new(Vec::new()),
                }
            }
            Self::Node { item, .. } => {
                let item = Some(new_item); // XXX
            }
        }
    }
    pub fn iter_children(self) {}
    pub fn get_children(self) -> Vec<Node<T>> {
        match self {
            Self::None => Vec::new(),
            Self::Node { children, .. } => *children,
        }
    }
    fn to_node(self) {}
    fn to_none(&mut self) {
        *self = std::mem::replace(self, Node::None);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    fn create_node() {
        let mut node: Node<i32> = Node::new();
        node.set_item(3);
        assert_eq!(node.get_item().unwrap(), 3)
    }
    fn add_child() {
        let mut parent: Node<i32> = Node::new();
        parent.set_item(3);
        parent.add_child(5);
        let children = &parent.clone().get_children();
        let child = &children[0];
        let content = child.get_item();
        assert_eq!(content.unwrap(), 5);
    }
}
