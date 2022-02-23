use std::{collections::HashMap, iter::FromIterator, vec};

fn main() {
    //learning_vector();
    //learning_string();
    //learning_map();
    let test_vec = vec![
        5, -5, 8, 7, 9, 12, -9, -11, -6, 2, 7, 4, 3, 6, 5, 8, 7, 9, 4, 11, 2, 3, 5,
    ];
    let median = get_median(&test_vec);
    println!("median is {}", median);

    get_mode(&test_vec);
}

fn learning_vector() {
    {
        let v: Vec<i32> = Vec::new(); //初始化Vec

        let v = vec![1, 2, 3]; //vec!宏,提供类型推断

        //更新Vector
        let mut v = Vec::new(); //要想插入值必须是mut
        v.push(1); //插入值后会推断类型
        v.push(2);
        v.push(3);
        v.push(4);
    }

    {
        //丢弃Vector时也会丢弃其所有元素
        let v = vec![1, 2, 4, 3];
    }
    //println!("{}", &v); //error

    {
        //获取元素
        //1.用slice获取一个引用
        let v = vec![1, 2, 3, 4];

        let slice = &v[2];

        //2.用get方法获取一个Option<&T>
        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }

        //第一种如果索引越界会panic,第二种会返回None
    }

    {
        //遍历
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50; // *为解引用,见15章
        }
    }

    {
        //用枚举来储存不同类型
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![
            SpreadsheetCell::Int(1),
            SpreadsheetCell::Float(2.2),
            SpreadsheetCell::Text(String::from("fuck")),
        ];
    }

    //如果不知道运行时Vec里的数据类型,就不能用枚举了,就需要用trait,见17章
}

fn learning_string() {
    // String和slice都是UTF-8编码的
    // 标准库里有其他编码或者不同内存表现形式的字符串，如：OsString, OsStr, CString, CStr等

    {
        //初始化字符串
        let data = String::from("initial contents");

        let data = "initial contents";

        let s = data.to_string(); //to_string可用于任何被实现了Display trait的类型

        let s = "initial contents".to_string(); //字符串字面值也实现了该trait
    }
    {
        //更新字符串
        let mut s = String::from("foo");
        let s2 = "bar";
        s.push_str(s2);
        println!("s2 is {}", s2); //push_str使用的是slice,所以不需要交出所有权

        s.push('l'); //push方法只能传一个字符

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; //注意 s1 被移动了，不能继续使用

        /*
        之所以s2需要借用,因为+运算符这里使用了add函数,它的签名大概是这样的
            fn add(self, s:&str) -> String {}
        但是我们传的是&String 而不是 &s2,却仍然能通过编译
        因为&String可以被强转(deref coerced)成&str,编译器把&s2强转成了&s2[..]
        又因为签名中是self而不是&self，所以这里的s1所有权移动到了add里
        */

        //使用format!拼接复杂字符串
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3); //format!使用slice,所以不会夺取所有权
    }

    {
        //Rust的字符串不支持索引
        let s1 = String::from("hello");
        //let h = s1[0]; error

        /*
            关于为什么rust字符串不支持索引
                1. 根据UTF-8,一个索引可能会返回字节、标量值、字形簇,而Rust 提供了多种不同的方式来解释计算机储存的原始字符串数据，这样程序就可以选择它需要的表现方式，而无所谓是何种人类语言。
                2. 索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
        */

        // 字符串slice
        let hello = "Здравствуйте"; //这些字母都是两个字节长度
        let s = &hello[0..4]; // 它包含字符串的头四个字节
        println!("{}", &s); //所以这里是Зд

        //如果是&hello[0..1]  程序就会panic
    }

    {
        // 字符串的遍历
        // 操作字符串每一部分的最好的方法是明确表示需要字符还是字节
        for c in "卧槽".chars() {
            println!("{}", c);
        }

        for b in "卧槽".bytes() {
            println!("{}", b);
        }

        //从字符串中获取字形簇是很复杂的，所以标准库并没有提供这个功能。
    }
}

fn learning_map() {
    use std::collections::HashMap;
    {
        // 新建
        let mut scores = HashMap::new();

        scores.insert("Blue".to_string(), 10);
        scores.insert("Yellow".to_string(), 50);

        // 通过元组的Vector使用迭代器和collect创建（见13章）
        let teams = vec!["Blue".to_string(), "Yellow".to_string()];
        let initial_scores = vec![10, 50];

        //使用下划线占位 会根据其中的值自动推断类型
        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    }
    {
        // 实现了Copy trait类型的值 可以直接拷贝进hashmap,对于String这种有所有权的值,会被移动
        let field_name = "Favorite age".to_string();
        let field_value = 10;

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        //println!("{},{}",field_name,field_value); error
    }
    {
        //访问hashmap中的值
        let mut scores = HashMap::new();

        scores.insert("Blue", 10);
        scores.insert("Yellow", 50);

        let team_name = "Blue".to_string();
        let score = scores.get("Blue");
        if let Some(10) = score {
            println!("{}", &score.unwrap());
        }

        // 遍历hashmap

        for (key, value) in scores {
            println!("{},{}", key, value);
        }
    }
    {
        //更新hashmap

        //覆盖一个值
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 50);
        scores.insert("Blue".to_string(), 10);

        //没有键时才插入
        let mut scores = HashMap::new();
        scores.insert("Blue".to_string(), 50);

        //entry返回一个枚举Entry 代表可能存在或不存在的值
        scores.entry("Blue".to_string()).or_insert(50);
        scores.entry("Yellow".to_string()).or_insert(20);
        println!("{:?}", scores);

        //根据单词出现的次数构建hashmap
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            //or_insert会返回这个值的一个可变引用
            let count = map.entry(word).or_insert(0);
            *count += 1;
            println!("word '{}' appears {}  times", word, count);
        }
        println!("{:?}", map);
    }
}

//练习
//返回一个vec的中位数
fn get_median(vec: &Vec<i32>) -> i32 {
    let mut slice = Vec::from_iter(vec[..].iter().cloned());
    slice.sort();
    let mut median_index = slice.len() / 2;
    return slice[median_index];
}
//返回一个vec的众数
fn get_mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for &num in vec {
        *map.entry(num).or_insert(0) += 1;
    }

    return map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute the mode of zero numbers");
}
// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。