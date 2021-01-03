#[derive(Debug)]
pub struct ll {
    head: Link,
}
type Link = Option<Box<Node>>;
#[derive(Debug)]
pub struct Node {
    element: i32,
    next: Link,
}
impl ll {
    pub fn new(element: i32) -> ll {
        ll {
            head: Some(Box::new(Node {
                element,
                next: None,
            })),
        }
    }
    pub fn insert_big(&mut self, element: i32) {
        let new_node = Node {
            element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
    pub fn count(&self) -> i32 {
        let mut p: &Link = &self.head;
        let mut count = 0;
        loop {
            match p {
                Some(n) => {
                    count += 1;
                    p = &n.next
                }
                None => return count,
            }
        }
    }
    pub fn insert_end(&mut self, element: i32) {
        let new_node = Node {
            element,
            next: None,
        };
        let mut p: &mut Link = &mut self.head;
        let mut count = self.count();
        let mut p: &mut Link = &mut self.head;
        for _ in 0..count - 1 {
            match p {
                Some(n) => {
                    count += 1;
                    p = &mut n.next
                }
                None => break,
            }
        }
        match p {
            Some(n) => {
                n.next = Some(Box::new(new_node));
            }
            None => return,
        }
    }
}
pub fn run() {
    let mut a = ll::new(10);
    a.insert_big(11);
    a.insert_big(12);
    a.insert_big(13);
    a.insert_end(13);
    println!("{:?}", a.head.unwrap());
}
