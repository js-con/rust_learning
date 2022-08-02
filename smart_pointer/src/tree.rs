use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
struct Node {
    value: i32,
    // 父节点应该拥有子节点，父节点被丢弃，子节点也应该被丢弃
    // 而子节点不应该拥有父节点，子节点被丢弃，父节点仍然存在
    // 所以parent应该是个弱引用
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>
}

fn run(){
  let leaf = Rc::new(Node {
    value: 3,
    children: RefCell::new(vec![]),
    parent: RefCell::new(Weak::new())
  });

  let branch = Rc::new(Node {
    value: 5,
    children: RefCell::new(vec![Rc::clone(&leaf)]),
    parent: RefCell::new(Weak::new())
  });

  // *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
  println!("leaf parent is {:?}", leaf.parent.borrow().upgrade());
}
