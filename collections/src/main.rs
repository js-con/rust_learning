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
