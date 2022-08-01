use std::mem;
struct CustomSmartPointer {
    data: String,
}
// 实现drop智能指针将会在结构体实例离开作用域时调用drop方法
// 但是，显示的调用drop是不允许的
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping with data: {}", self.data);
    }
}
pub fn run() {
    let c = CustomSmartPointer {
        data: "my stuff".to_string(),
    };
    let d = CustomSmartPointer {
        data: "other stuff".to_string(),
    };
    // 如果需要显示调用drop，需要用标准库mem的drop函数
    mem::drop(d);

    println!("CustomSmartPointers created");
}
