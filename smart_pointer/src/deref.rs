use std::ops::Deref;

pub fn run(){
    // 解引用运算符
    let x = 5;
    let y = &x;
    // Box智能指针也可以解引用
    let z = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    let my_box = MyBox::new(x);
    assert_eq!(5, *my_box);

    // 实现Deref trait可以实现将一种类型的引用转换成另一种类型的引用
    // 以下为例: &String -> &str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // 假设String没有实现Deref trait,那这里就需要这么写了
    // *m 解出String, 然后&String[..]变成slice
    hello(&(*m)[..]);

    // Rust提供DerefMut trait用于重载可变引用的*运算符
    // Rust在发现类型和trait满足一下三种情况时会进行Deref强制转换：
    // 1. 当 T: Deref<Target=U>时，从&T到&U
    // 2. 当 T: DerefMut<Target=U>时，从&mut T到&mut U
    // 3. 当 T: Deref<Target=U>时，从&mut T到&U
}

fn hello(name: &str) {
    println!("hello,{}", name);
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// 实现了Deref trait才可以解引用
// 本质是 *(y.deref())
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // .0用来访问元组结构体的第一个元素
        &self.0
    }
}
