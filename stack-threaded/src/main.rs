use std::sync::{Arc, Mutex};



#[derive(Debug, Clone)]
struct Node {
    obj: u8,
    tail: Option<Arc<Node>>
}

#[derive(Debug, Clone)]
struct Stack { tail: Option<Arc<Node>>}


impl Stack {
    fn new() -> Self {
        Stack{tail: None}
    }
    fn push(&mut self, obj: u8) {
        let link = self.tail.take();
        match link {
            Some(x) => {
                self.tail = Some(Arc::new(Node{obj, tail: Some(x)}));
            },
            None => {
                self.tail = Some(Arc::new(Node{obj, tail: None}));
            }
        };
    }

    fn pop(&mut self) -> Option<u8> {
        let link = self.tail.take();
        match link {
            Some(x) => {
                let ret = Some(x.obj);
                match &x.tail {
                    Some(y) => self.tail = Some(y.clone()),
                    None => self.tail = None,
                }
                ret
            },
            None => None,
        }

    }
}
fn main() {
    let  stack = Arc::new(Mutex::new(Stack::new()));
    let x_shared = Arc::clone(&stack);
    let y_shared = Arc::clone(&stack);
    let x_thread = std::thread::spawn(move || {
        for i in 1..=100 {
            x_shared.lock().unwrap().push(i);
        };
    });
    let y_thread = std::thread::spawn(move || {
        for i in 101..=200 {
            y_shared.lock().unwrap().push(i);
        };
    });
    x_thread.join().unwrap();
    y_thread.join().unwrap();

    let x_shared = Arc::clone(&stack);
    let y_shared = Arc::clone(&stack);

    let x_thread = std::thread::spawn(move || {
        for _ in 1..=100 {
            println!("Thread x took: {}", x_shared.lock().unwrap().pop().unwrap());
        };
    });
    let y_thread = std::thread::spawn(move || {
        for _ in 101..=200 {
            println!("Thread y took: {}", y_shared.lock().unwrap().pop().unwrap());
        };
    });
    x_thread.join().unwrap();
    y_thread.join().unwrap();
}
