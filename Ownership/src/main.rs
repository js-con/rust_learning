/*
 * @Author: Lin Bowen
 * @Date: 2021-09-25 01:08:50
 * @LastEditTime: 2021-09-26 01:23:35
 * @LastEditors: Lin Bowen
 * @Description:
 * @FilePath: \rust\Ownership\src\main.rs
 */
fn main() {
    //rust采用类似Resource Acquisition Is Initialization (RAII)的模式释放内存
    //即在变量离开作用域时，调用内部的drop函数释放内存
    {
        let s = String::from("hello");
        println!("{}", s);
    }
    //println!("{}",s);  error

    {
        //String在堆上,值可变;字符串字面值在栈上,值不可变
        let mut s = String::from("The!");
        s.push_str("World!");
        println!("{}", s);

        //栈
        let x = 5;
        let y = x;
        println!("x is {},y is {}", x, y);

        //堆
        // s1给s2的只有栈上的指针、容量、长度三个值的拷贝
        // rust并不会把堆上储存的s1的内容也拷贝给s2，因为这样会对运行时性能造成很大的影响(与其他语言类似)
        let s1 = String::from("hello");
        let s2 = s1;

        //现在，当离开作用域时，s1和s2都将被释放内存，但是他们的指针指向的是同一块内存，这会导致了二次释放错误(double free)
        //rust的处理是，当变量移动后，之前的变量就无效化:

        //println!("s1 is {}", s1);   error[E0382]: use of moved value: `s1`

        //这个过程类似于浅拷贝，但是由于拷贝后之前的变量无效了，所以这个过程被称作 移动(move)
        //这里还隐含了一个设计选择：Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何 自动 的复制可以被认为对运行时性能影响较小。
    }
    {
        //函数也会夺取所有权
        let s = String::from("hello");
        fn takes_ownership(str: String) {
            println!("{}", str)
        }
        takes_ownership(s);
        //println!("{}",s);   //value borrowed here after moverustc(E0382)

        //如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用
        //rust中，以下类型实现了copy
        /*
        所有整数类型，比如 u32。
        所有浮点数类型，比如 f64。
        布尔类型，bool，它的值是 true 和 false。
        字符类型，char。
        元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。 */
        let x = 5;
        fn makes_copy(num: u32) {
            println!("{}", num)
        }
        makes_copy(x);
        println!("{}", x)
    }
    {
        fn gives_ownership() -> String {
            let str = String::from("hello");
            str
        }
        fn takes_and_gives_back(str: String) -> String {
            str
        }
        let s1 = gives_ownership();
        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        println!("{}", s3)
    }
    {
        //借用
        //接受一个引用值作为函数参数称为借用，这样就不会夺取所有权
        fn get_str_length(str: &String) -> usize {
            //str.push_str("world"); //但引用值不可被改变，因为是借的
            str.len()
        }
        let s1 = String::from("hello");
        let len = get_str_length(&s1);
        println!("s1 is {},len is {}", s1, len);

        //如果想要改变引用值，则应使用mut
        let mut s2 = String::from("hello"); //值必须是mut的
        println!("s2 is {}", s2);
        fn change_str(str: &mut String) {
            //接收的参数需要标注mut
            str.push_str(" world");
        }
        change_str(&mut s2); //传入时也需要标注mut
        println!("now s2 is {}", s2);

        //但是，在一个作用域中只能有一个可变引用，这是为了编译时避免数据竞争（data race）
        let mut s = String::from("hello");
        let r1 = &mut s;
        //let r2 = &mut s;    error!
        //println!("{},{}",r1,r2);

        //而且，也不能在拥有不可变引用的同时 拥有可变引用，这是为了防止数据意外改变
        let mut s = String::from("hello");
        let r1 = &s;
        //let r2 = &mut s; //error:cannot borrow `s` as mutable because it is also borrowed as immutable
        //println!("{},{}", r1, r2);

        //但是可以同时拥有多个不可变引用
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{},{}", r1, r2); //ok

        //注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止。
        //例如，因为最后一次使用不可变引用在声明可变引用之前，所以如下代码是可以编译的
        let mut r3 = &mut s;
        println!("{}", r3);
    }
    {
        //dangling references悬垂引用
        //有些语言释放内存后，保留了指向它的指针，这个就叫悬垂指针（dangling pointer）
        //rust 确保永远不会出现悬垂引用

/*      fn dangle() -> &String {
            //error:missing lifetime specifier
            let s = String::from("hello");
            &s                              //返回了字符串的引用,但离开函数作用域时,字符串内存会被释放
        } */
        //let reference_to_nothing = dangle();

        //解决方法是直接移出所有权
        fn dangle() -> String{
            let s = String::from("hello");
            s
        }
        let reference = dangle();
        println!("{}",reference);
    }
    //概括一下之前对引用的讨论：
    //1.在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    //2.引用必须总是有效的。
}
