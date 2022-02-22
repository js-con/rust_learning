/*
 * @Author: Lin Bowen
 * @Date: 2021-09-27 01:08:53
 * @LastEditTime: 2021-09-27 10:23:23
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \rust_learning\enumeration\src\main.rs
 */
fn main() {
    {
        #[derive(Debug)]
        enum IpAddrKind{
            V4,
            V6
        }

        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        fn route(ip_type: IpAddrKind){}

        route(four);
        route(six);

        //用在结构体中
        #[derive(Debug)]
        struct IpAddr{
            kind:IpAddrKind,
            address:String
        }
        let home = IpAddr{
            kind:IpAddrKind::V4,
            address:String::from("127.0.0.1")
        };
        let loopback = IpAddr{
            kind:IpAddrKind::V6,
            address:String::from("::1")
        };
        println!("home:{:?}",home);
        println!("loopback:{:?}",loopback);

    }
    {
        //使用枚举代替结构体
        //枚举成员可以处理不同类型不同数量的数据，比结构体更加灵活
        enum IpAddr{
            V4(u8,u8,u8,u8),
            V6(String)
        }
        let home = IpAddr::V4(127,0,0,1);
        let loopback = IpAddr::V6(String::from("::1"));

        //在枚举中使用impl
        enum Message{
            Quit,
            Move {x:i32,y:i32}, //匿名结构体
            Write(String),
            ChangeColor(i32,i32,i32),
        }
        impl Message {
            fn call(&self){
                println!("{}","Call!")
            }
        }
        let m = Message::Write(String::from("hello"));
        m.call();

    }
    {
        //Option枚举
        //rust没有空值，但是有一个可以编码存在或不存在概念的枚举，他就是标准库中的Option<T>
        //它被包含在了prelude中，不需要显示引入
        //它的成员也不需要使用Option::前缀来调用

/*         enum Option<T>{  //泛型
            Some(T),
            None,
        } */

        let some_number = Some(5);
        let some_string = Some("a string");

        //如果使用None，则需要标注类型
        let absent_number:Option<i32> = None;

        //Option类型不能直接参与运算
        let x = 5;
        //let sum = x + some_number; //cannot add `Option<{integer}>` to `{integer}`

    }
}
