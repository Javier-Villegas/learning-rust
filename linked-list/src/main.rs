use std::mem;


#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

#[derive(Debug)]
struct Node {
    obj: u8,
    tail: Link
}

#[derive(Debug)]
struct List {
    head: Link
}

impl List {
    fn new() -> Self{
        List{head: Link::Empty}
    }

    fn insert(&mut self, obj: u8, pos: u8) {
        match &mut self.head {
            Link::Empty => {
                self.head = Link::More(Box::new(Node{obj,tail: Link::Empty}));
            },
            Link::More(_) if pos == 0 => {
                let old_tail = mem::replace(&mut self.head, Link::Empty);
                _ = mem::replace(&mut self.head, Link::More(Box::new(Node{obj, tail: old_tail})));
            },
            Link::More(node) => node.insert(obj, pos-1),
        };
    }

    fn delete(&mut self, obj: u8) -> bool{
        match &mut self.head {
            Link::Empty => false,
            Link::More(node) if node.obj == obj => {
                let new_tail = mem::replace(&mut node.tail, Link::Empty);
                _ = mem::replace(&mut self.head, new_tail);
                true
            }
            Link::More(node) => node.delete(obj)
        }
    }
}

impl Node {
    fn insert(&mut self, obj: u8, pos: u8) {
        match &mut self.tail {
            Link::Empty => {
                self.tail = Link::More(Box::new(Node{obj, tail: Link::Empty}));
            },
            Link::More(_) if pos == 0 => {
                let old_tail = mem::replace(&mut self.tail, Link::Empty);
                _ = mem::replace(&mut self.tail, Link::More(Box::new(Node{obj, tail: old_tail})));
            },
            Link::More(node) => node.insert(obj, pos-1),
        };
    }

    fn delete(&mut self, obj: u8) -> bool {
        match &mut self.tail {
            Link::Empty => false,
            Link::More(node) if node.obj == obj => {
                let new_tail = mem::replace(&mut node.tail, Link::Empty);
                _ = mem::replace(&mut self.tail, new_tail);
                true
            },
            Link::More(node) => node.delete(obj)
        }
    }

}

fn main() {
    println!("Hello, world!");
    let mut l = List::new();
    l.insert(10,0);
    l.insert(12,0);
    l.insert(13,2);
    l.insert(1,20);
    l.insert(2,2);

    l.delete(13);
    println!("{:?}", l);
}
