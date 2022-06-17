struct LL<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LL<T> {
    fn new() -> LL<T> {
        LL::<T> { head: None }
    }
    fn append(&mut self, data: T) {
        let new_node = Some(Box::new(Node::<T> { data, next: None }));
        match self.head {
            None => self.head = new_node,
            Some(ref mut i) => {
                let mut node = i;
                loop {
                    match { node }.next {
                        None => {
                            //node.next = new_node;
                            break;
                        }
                        Some(ref mut j) => {
                            node = j;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

fn main() {
    let mut yay = LL::<i8>::new();
    yay.append(5);
    println!("{:?}", yay.head);
}
