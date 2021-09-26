/*
 * @Author: Lin Bowen
 * @Date: 2021-09-24 23:37:15
 * @LastEditTime: 2021-09-24 23:43:58
 * @LastEditors: Lin Bowen
 * @Description:
 * @FilePath: \rust\functions\src\main.rs
 */
fn main() {
    println!("Hello, world!");

    another_function();

    let x = five();
    println!("value of x is {}",x);

    let x = plus_one(x);
    println!("value of x is {}",x);
}
fn another_function() {
    println!("Another function!")
}
//表达式返回值
fn five() -> i32 {
    5
}
fn plus_one(x:i32) -> i32 {
    x + 1
    //语句不能返回(加上引号会报错)
    // x + 1;    error!
}

