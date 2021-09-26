/*
 * @Author: Lin Bowen
 * @Date: 2021-09-24 23:50:02
 * @LastEditTime: 2021-09-25 00:49:05
 * @LastEditors: Lin Bowen
 * @Description:
 * @FilePath: \rust\loops\src\main.rs
 */
fn main() {
    loop {
        println!("again!");
        break;
    }

    //循环标签
    let mut count = 0;
    'conting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'conting_up;
            }
            remaining -= 1
        }
        count += 1
    }
    println!("End count = {}", count);

    //从循环返回
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {}", result);

    //while循环

    //使用for循环遍历集合
    let a = [10, 20, 30, 40, 50, 60];

    for item in a {
        println!("item is {}", item)
    }

    //斐波那契数列
    let res = generate_fibonacci(15);

    for num in res {
        println!("fibonacci:{}",num)
    }
}
fn generate_fibonacci(n: u32) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();

    if n == 0 {
        return result;
    }

    for i in 0..n {
        if i <= 1 {
            result.push(i);
            continue;
        }
        let val = result[i as usize - 1] + result[i as usize -2];
        result.push(val)
    }

    return result;
}
