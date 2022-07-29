use core::fmt::Display;
use std::fmt::Debug;
fn generic_learning() {
    //结构体使用泛型
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let wrond_point = Point { x: 5, y: 4.0 }; //error

    //方法中使用泛型
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    //可以仅仅为某个类型的实例定义方法
    impl Point<f32> {
        //其他类型的Point实例就没有这个方法
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    //使用与结构体定义中不同类型的泛型
    struct Point2<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Point2<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 {
        x: "Hello".to_string(),
        y: 'c',
    };
    let p3 = p1.mixup(p2);
    println!("p3.x = {},p3.y = {}", p3.x, p3.y);

    //enum使用泛型
    enum Option<T> {
        Some(T),
        None,
    }
}

fn trait_learning() {
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    //trait类似于其他语言中的"接口"概念
    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        pub trait Summary {
            //也可以提供一个默认实现
            fn summarize(&self) -> String {
                "Read more".to_string()
            }
        }
        //定义一个空的块来使用默认实现（有默认实现的方法可以不用再次实现）
        impl Summary for NewsArticle {}

        let news_article = NewsArticle {
            headline: "Penguins win the Stanley Cup championship!".to_string(),
            location: "Pittsburgh, PA, USA".to_string(),
            author: "Iceburgh".to_string(),
            content: "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL."
                .to_string(),
        };
        println!("New article available! {}", news_article.summarize());
    }

    {
        //trait中的方法可以互相调用
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }
        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }
        let tweet = Tweet {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            retweet: false,
        };
        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        //trait作为参数
        //此处参数为实现了Summary的任何类型
        pub fn notify(item: &impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
    }

    {
        //trait bound语法
        pub trait Summary {
            fn summarize(&self) -> String;
        }
        //配合泛型使用
        pub fn notify<T: Summary>(item: &T) {
            println!("Breaking news! {}", item.summarize());
        }
        //复杂场景
        pub fn notify2<T: Summary>(item1: &T, item2: &T) {}

        //指定多个trait bound
        pub fn notify3(item: &(impl Summary + Display)) {}
        pub fn notify4<T: Summary + Display>(item: &T) {}

        //遇到每种泛型的trait bound不同时，使用where提高可读性
        fn some_function<T, U>(t: &T, u: &U) -> i32
        where
            T: Display + Clone,
            U: Clone + Debug,
        {
            1
        }
    }

    {
        pub trait Summary {
            fn summarize(&self) -> String {
                "Read more".to_string()
            }
        }
        impl Summary for Tweet {}
        impl Summary for NewsArticle {}
        //返回实现了trait的类型
        fn returns_summarizable() -> impl Summary {
            Tweet {
                username: "horse_ebooks".to_string(),
                content: "of course,as you probably already know, people".to_string(),
                reply: false,
                retweet: false,
            }
        }

        //但是这只适用于返回单一类型的情况，比如下面这个就无法编译
        //如果要完成这种功能，见17章
        /*         fn returns_summarizable2(switch: bool) -> impl Summary {
            if switch {
                NewsArticle {}
            }else {
                Tweet {}
            }
        } */
    }

    {
        //使用trait bounds来修复largest函数
        //原本 > 运算符不能直接比较T泛型的值
        //> 运算符是标准库std::cmp::PartialOrd中的一个方法
        //所以要给T加上trait bounds
/*         fn largest<T: PartialOrd>(list: &[T]) -> T{
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }

            return largest;
        } */

        //但是这样会有另一个错误 cannot move out of type `[T]`, a non-copy slice
        // 因为T可能是没有实现Copy的类型,所以不能直接这样移动
        // 所以还要增加一个Copy约束
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
    }
}
fn lifetime_learning(){
    {
        // 生命周期避免了悬垂引用
        let r;
        {
            let x = 5;
            r = &x;
        }
        println!("r:{}",r)
    }

    {
        // 函数中的泛型生命周期
        fn longest<'a>(x: &'a str, y:&'a str) -> &'a str{
            if x.len() > y.len() {
                x
            }else {
                y
            }
        } 

        let long_string = "long string is long".to_string();
        let result;
        {
            let short_string = "xyz".to_string();
            result = longest(long_string.as_str(), short_string.as_str());
        }
        println!("The longest string is {}", result)
    }

    {
        // 包含引用的结构体
        struct ImportantExcerpt<'a>{
            part: &'a str
        }
        let novel = "Call me Ishmael. Some years ago".to_string();
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");

        // 该实例不能比part字短中的引用存在的更久
        let i = ImportantExcerpt{
            part: first_sentence
        };
    }
}
fn main() {
    generic_learning();
    trait_learning();
}
