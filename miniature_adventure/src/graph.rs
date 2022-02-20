/// minimal implementation of directed graphs

enum Node<T> {
    None,
    Leave {
        item: T,
    },
    Node {
        item: T,
        parent: Box<Node<T>>,
        children: Box<Vec<Node<T>>>,
    },
}

impl<T> Node<T> {
    pub fn add_child(&mut self, element: T) {
        /// add new node with element
        match self {
            self::None => {
                //create new node}
            }
            self::Leave => {
                // add new edges
            }
            sel::Node => {
                //push new node
            }
        }
    }
    pub fn iter(self) {}
    pub fn get_parent(self) {}
    pub fn get_children(self) {}
}
