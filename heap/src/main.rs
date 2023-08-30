use std::mem;

#[derive(Debug)]
enum Tail {
    Empty,
    More(Box<Node>)
}

#[derive(Debug)]
struct Queue {
    head: Tail
}

    
#[derive(Debug)]
struct Node {
    obj: u8,
    tail: Tail
}

impl Node {
    fn push(&mut self, obj: u8) {
        match &mut self.tail {
            Tail::Empty => {
                let new_node = Tail::More(Box::new(Node{obj, tail: Tail::Empty}));
                //_ = mem::replace(&mut self.tail, new_node);
                self.tail = new_node;
            },
            Tail::More(node) => node.push(obj),
        }
    }


    fn pop(&mut self) -> Option<u8> {
        match &mut self.tail {
            Tail::Empty => {
                None
            },
            Tail::More(node) => match &mut node.tail {
                Tail::Empty => {
                    let ret = node.obj;
                    _ = mem::replace(&mut self.tail, Tail::Empty);
                    Some(ret)
                },
                Tail::More(_) => {
                    node.pop()
                }
            }
        }
    }
}

impl Queue {
    fn new() -> Self {
        Queue{head: Tail::Empty}
    }

    fn push(&mut self, obj: u8) {
        match &mut self.head {
            Tail::Empty => {
                let new_node = Tail::More(Box::new(Node{obj, tail: Tail::Empty}));
                _ = mem::replace(&mut self.head, new_node);
                },
            Tail::More(node) => node.push(obj),
        };
    }

    fn pop(&mut self) -> Option<u8> {
        match &mut self.head {
            Tail::Empty => None,
            Tail::More(node) => {
                    match &mut node.tail {
                    Tail::Empty => {
                        let ret = node.obj;
                        _ = mem::replace(&mut self.head, Tail::Empty);
                        Some(ret)
                    },
                    Tail::More(_) => {
                        node.pop()
                    }
                }
            }
        }
    }
}

fn main() {
    let mut q = Queue::new();
    println!("{:?}",q.pop());
    q.push(1);
    q.push(2);
    q.push(3);
    q.push(4);
    q.push(5);
    println!("{:?}", q);
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q.pop());
    println!("{:?}", q);
}
