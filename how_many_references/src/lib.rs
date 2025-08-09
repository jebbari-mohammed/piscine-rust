use std::rc::Rc;

pub struct Node {
    pub ref_list: Vec<Rc<String>>,
}

impl Node {
    pub fn new(ref_list: Vec<Rc<String>>) -> Self {
        Node { ref_list }
    }

    pub fn add_element(&mut self, element: Rc<String>) {
        self.ref_list.push(element);
    }

    pub fn rm_all_ref(&mut self, element: Rc<String>) {
        let mut i = 0;
        while i < self.ref_list.len() {
            if Rc::ptr_eq(&self.ref_list[i], &element) {
                self.ref_list.remove(i);
                // Do NOT increment i; the list has shifted
            } else {
                i += 1;
            }
        }
    }
}

pub fn how_many_references(rc_string: &Rc<String>) -> usize {
    Rc::strong_count(rc_string)
}
