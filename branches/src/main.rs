/*
 * @Author: Lin Bowen
 * @Date: 2021-09-24 23:46:09
 * @LastEditTime: 2021-09-24 23:48:55
 * @LastEditors: Lin Bowen
 * @Description: 
 * @FilePath: \rust\branches\src\main.rs
 */
fn main() {
    let number = 3;

    if number < 5 {
        println!("小于5")
    }else{
        println!("大于5")
    }

    //if是个表达式，可以在let语句中使用
    let condition = true;
    let number = if condition {
        5
    }else{
        6
    };
    println!("the number is {}",number)
    
}
