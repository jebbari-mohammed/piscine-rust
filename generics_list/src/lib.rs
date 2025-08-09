#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        let new = List {
            head: None,
        };
        new
    }

    pub fn push(&mut self, value: T) {
        let old_head = self.head.take();
        let new_node = Node {
            value,
            next: if let Some(value) = old_head {
                Some(Box::new(value))
            } else {
                None
            },
        };
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        if let Some(node) = self.head.take() {
            if let Some(boxed_node) = node.next {
                self.head = Some(*boxed_node);
            } else {
                self.head = None;
            }
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            count += 1;
            if let Some(ref boxed_node) = node.next {
                current = Some(boxed_node.as_ref());
            } else {
                break;
            }
        }

        count
    }
}
