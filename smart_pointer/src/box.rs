// Box允许你将一个值放在栈上而不是堆上
// 多用于如下场景:
// - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

// 递归类型无法通过编译
// 需要使用Box包装成指针使其大小固定
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

pub fn run() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
