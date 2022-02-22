use std::vec;

fn main() {
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
            None => println!("There is no third element")
        }

        //第一种如果索引越界会panic,第二种会返回None
    }

    {
        //遍历
        let mut v = vec![100,32,57];
        for i in &mut v {
            *i += 50; // *为解引用,见15章
        }
    }

    {
        //用枚举来储存不同类型
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String)
        }
        let row = vec![
            SpreadsheetCell::Int(1),
            SpreadsheetCell::Float(2.2),
            SpreadsheetCell::Text(String::from("fuck"))
        ];
    }

    //如果不知道运行时Vec里的数据类型,就不能用枚举了,就需要用trait,见17章
}

fn learning_string(){
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
        println!("s2 is {}",s2);//push_str使用的是slice,所以不需要交出所有权

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

        let s = format!("{}-{}-{}",s1,s2,s3); //format!使用slice,所以不会夺取所有权
    }
    
    {
        //Rust的字符串不支持索引
        let s1 = String::from("hello");
        //let h = s1[0]; error

        /* 
            关于为什么rust字符串不支持索引
        */
    }
}