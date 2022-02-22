/*
 * @Author: Lin Bowen
 * @Date: 2021-09-27 10:23:11
 * @LastEditTime: 2021-09-27 15:50:02
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \rust_learning\matches\src\main.rs
 */
fn main() {
    {
        #[derive(Debug)]
        enum UsState{
            Alabama,
            Alaska
        }
        enum Coin{
            Penny,
            Nickel,
            Dime,
            Quarter(UsState)
        }
        fn value_in_cents(coin:Coin)->u8{
            match coin{
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!",state);
                    25
                }
            }
        }
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }
    {
        //编写一个函数，获取一个Option<32>，如果其中有值，则值加一，否则返回None
        fn plus_one(x:Option<i32>)->Option<i32>{
            match x {
                None => None,
                Some(i)=>Some(i+1)
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
    {
        //match必须覆盖所有可能的值
        //可以用通配符_覆盖其余的值:
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            2 => println!("two"),
            _ => ()
        }

        //如果只想处理一种情况，可以使用if let
        if let 0u8 = some_u8_value {
            println!("one!")
        }else{
            println!("others")
        };

        //2021-9-27 我的基础代谢率估算
        let basic = 67.0 + 13.73 * 73.0 + 5.0 * 170.0 - 6.9 * 24.0;
        println!("{}",basic) //1753.69
    }
}
