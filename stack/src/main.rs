use std::mem;


#[derive(Debug)]
enum Tail<T> {
    Empty,
    More(Box<Node<T>>)
}

#[derive(Debug)]
struct Stack<T> {
    head: Tail<T>,
}

#[derive(Debug)]
struct Node<T> {
    obj: T,
    next: Tail<T>
}



impl <T>Stack<T>{
    fn new() -> Self {
        Stack{head: Tail::Empty}
    }

    fn push(&mut self, obj: T) {
        self.head = Tail::More(Box::new(Node{obj: obj, next: mem::replace(&mut self.head, Tail::Empty)}));
    }
    
    fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Tail::Empty) {
            Tail::Empty => None,
            Tail::More(node) => {
                self.head = node.next;
                Some(node.obj)
            }

        }
    }
}

 


fn main() {
    println!("Hello, world!");
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("{:?}", stack);
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
    println!("{:?}", stack.pop());
}
