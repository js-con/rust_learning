/*
 * @Author: Lin Bowen
 * @Date: 2021-09-23 23:59:37
 * @LastEditTime: 2021-09-24 23:37:26
 * @LastEditors: Lin Bowen
 * @Description:
 * @FilePath: \rust\variables\src\main.rs
 */
fn main() {
    //隐藏变量 可改变类型
    let x = 5;
    let x = x + 2;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {}", x);
    }
    println!("The value of x is: {}", x);

    let space = "     ";
    let space = space.len();

    println!("The value of space is {}", space);

    //mut变量 不可改变类型
    let mut space = "     ";
    //space = space.len(); // Error

    //元组解构
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of tup is ({},{},{})", x, y, z);
    println!("The value of tup is ({},{},{})", tup.0, tup.1, tup.2);

    //当想在栈上分配空间
    //或者确保元素数量固定时
    //用数组
    //否则用vector
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [u32; 5] = [1, 2, 3, 4, 5];

    //等同于let b = [3,3,3,3,3];
    let b = [3;5];
}
