fn main() {
    {
        struct User{
            username:String,
            email:String,
            sign_in_count:u64,
            active:bool
        }
        //构建可变结构体
        let mut user1 = User {
            username:String::from("lbw"),
            email:String::from("543422342@qq.com"),
            sign_in_count:1,
            active:true
        };
        user1.email = String::from("ss.lbw.ss@gmail.com");

        //复用已有结构体成员
        let user2 = User{
            username:String::from("xjx"),
            email:String::from("fuck"),
            ..user1
        };

        //简写(类似ES6)
        fn build_user(email:String,username:String) -> User{
            User{
                username,
                email,
                active:true,
                sign_in_count:1
            }
        }
        
        //元组结构体  赋予元组语义
        struct Dirty_words (String,String);
        let dirty_words = Dirty_words(String::from("fuck"),String::from("shit"));
        println!("{}!,{}!",dirty_words.0,dirty_words.1);

        //应用
        struct Rectangle{
            width:u32,
            height:u32
        }
        let rect = Rectangle{
            width:10,
            height:20
        };
        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        };
        let area = area(&rect);
        println!("{}",area);

        //增加trait来打印结构体
        #[derive(Debug)]
        struct Power{
            magic:u32,
            physical:u32
        }
        let my_power = Power{
            magic:35,
            physical:100
        };
        //两种格式
        println!("my power is {:?}",my_power); 
        println!("my power is {:#?}",my_power);
    }
    {
        //方法
        #[derive(Debug)]
        struct Rectangle{
            width:u32,
            height:u32
        }
        impl Rectangle {
            //只希望读取数据,而不是写入,所以这里加上引用,不拿走所有权
            fn area(&self) -> u32 {
                self.width * self.height
            }
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
            //关联函数,不接受self为参数,而是与结构体相关联,不是方法,而是函数
            fn square(size: u32) -> Rectangle {
                Rectangle{width: size, height: size}
            }
        }
        //多个impl块是可行的
        impl Rectangle{
            //虽然这个关联函数没什么卵用
            fn sayHi(s: String){
                println!("{}",s)
            }
        }
        let rect1 = Rectangle {width: 30, height: 50};
        println!("area is {}",rect1.area());

        //rust可以自动引用和解引用,以下写法等价
        let area = rect1.area();
        let area = (&rect1).area();

        //调用多参数方法
        let rect1 = Rectangle{width:35,height:35};
        let rect2 = Rectangle{width:50,height:50};
        println!("is rect1 can hold rect2? {}",rect1.can_hold(&rect2));

        //调用关联函数,创建一个正方形
        let square = Rectangle::square(30);
        println!("{:#?}",square);
    }
    
}
