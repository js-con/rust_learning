// 在一些情况下，一个值可能有多个所有者，比如图数据结构
// 为了启用多所有权，需要使用Rc<T>,叫做引用计数
// 但是,Rc<T>只允许可读的共享数据

use std::rc::Rc;
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn run() {
    // 这个例子中，如果使用Box而不是Rc，则无法通过编译，因为b和c同时获取a的所有权
    // 使用Rc时，每次调用Rc::clone,Rc<List>中数据的引用计数都会增加，直到有零个引用之前其数据都不会被清理
    // 通过Rc::strong_count()获取引用计数的值
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}",Rc::strong_count(&a));
    // a.clone() 等同于 Rc::clone(&a)
    let b = Cons(3, a.clone());
    println!("count after creating b = {}",Rc::strong_count(&a));
    {
      let c = Cons(4, Rc::clone(&a));
      println!("count after creating c = {}",Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}",Rc::strong_count(&a));
}
