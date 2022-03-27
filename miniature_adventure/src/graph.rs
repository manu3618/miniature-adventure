/// minimal implementation of directed graphs

enum Node<T> {
    None,
    Node {
        item: Option<T>,
        id: String,
        parents: Box<Vec<Node<T>>>,
        children: Box<Vec<Node<T>>>,
    },
}

impl<T> Node<T> {
    pub fn new() -> Self {
        Self::None
    }
    /// add new node with element
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
    pub fn add_parent(self, parent: Node<T>) {}
    /// return vector of parents
    pub fn get_parents(self) {}
    pub fn get_item(self) -> Option<T> {
        match self {
            Self::None => None,
            Self::Node { item, .. } => item,
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
                let item = Some(new_item);
            }
        }
    }
    pub fn iter_children(self) {}
    pub fn get_children(self) {}
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
}
