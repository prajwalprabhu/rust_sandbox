#![allow(dead_code)]
#[derive(Debug)]
pub struct LinkedList <T>{
    head: Option<Box<Node<T>>>,
}
// type Link = Option<Box<Node<T>>>;
#[derive(Debug)]
pub struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}
impl <T> LinkedList<T>
where T:std::fmt::Debug {
    pub fn new(element: T) -> LinkedList<T> {
        let new:LinkedList<T> =LinkedList{
            head: Some(Box::new(Node {
                element,
                next: None,
            })),
        };
        new
    }
    pub fn insert_big(&mut self, element: T) {
        let new_node = Node {
            element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
    }
    pub fn count(&self) -> usize{
        let mut p: &Option<Box<Node<T>>> = &self.head;
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
    pub fn push(&mut self, element: T) {
        let new_node = Node {
            element,
            next: None,
        };
        let mut count = self.count();
        let mut p: &mut Option<Box<Node<T>>> = &mut self.head;
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
    // pub fn insert_pos()
    pub fn pop(&mut self) {
        let count = self.count();
        let mut p: &mut Option<Box<Node<T>>> = &mut self.head;
        for _ in 0..count - 2 {
            match p {
                Some(n) => {
                    // count += 1;
                    p = &mut n.next
                }
                None => break,
            }
        }
        match p {
            Some(n) => {
                n.next.take();
            }
            None => return,
        }
    }
    pub fn popp(&mut self, count: usize) {
        let p: &mut Option<Box<Node<T>>> = &mut self.head;
        // if !count==0let count = count-1;
        if count == 0 {
            match p {
                Some(n) => {
                    self.head = n.next.take();
                        return
                }
                None => return,
            }
        }
        let capacity = self.count();
        let mut p: &mut Option<Box<Node<T>>> = &mut self.head;
        
        if count>=capacity{
            println!("Out Of Range");
            return
        }
        for _ in 0..count-1 {
            match p {
                Some(n) => {
                    p = &mut n.next
                }
                None => break,
            }
        }
        match p {
            Some(n) => {
                let next = n.next.take();
                n.next = next.unwrap().next;
            }
            None => return,
        }
    }
    pub fn insert_pos(&mut self,position:usize,element:T){
        let capacity=self.count();
        if position>=capacity+2{
            println!("Out of range expected {} found {}",capacity,position);
            return
        }
        if position==capacity || position == capacity+1{
            self.push(element);
            println!("Called push");
            return
        }
        let mut new_node = Node{element,next:None};
        let mut p:&mut Option<Box<Node<T>>>=&mut self.head;
        for _ in 0..position{
            match p{
                Some(n)=>{
                    p=&mut n.next
                }
                None=>break
                
            }
        }
        match p {
            Some(n)=>{
                let last = n.next.take();
                new_node.next=last;
                n.next = Some(Box::new(new_node));
            }
            None=>return
        }
        

    }
    pub fn join(&mut self, other:LinkedList<T>){
        let capacity = self.count();
        let mut p = &mut self.head;
        for _ in 0..capacity-1{
            match p {
                Some(n)=>{
                    p = &mut n.next;
                }
                None=>break
            }
        }
        println!("{:?}",p);
        match p{
            Some(n)=>{
                n.next=other.head;
            }
            None=>return
        }
    }
}
// impl <T> Iterator for LinkedList<T>{
//     type Item=Box<Node<T>>;
//     fn next(&mut self) -> Option<Self::Item>{
//         let p =&mut self.head;
//         // p.next
//         match p{
//             Some(n)=>n.next,
//             None=> None
//         }


//     }
// }

pub fn run() {
    let mut a = LinkedList::new(Some("Hi"));
    a.insert_big(None);
    a.insert_big(Some("Hello"));
    a.insert_big(None);
    println!("{:?}", a.head);

}
