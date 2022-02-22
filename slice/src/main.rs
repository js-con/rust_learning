fn main() {
    {
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();
        
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }
        
            s.len()
        }
        let mut s = String::from("hello world");
        let word = first_word(&s);
        s.clear(); //此时word仍然是5,但s已经变成了空串
        //然而word是针对s的取值,此时s已经为空串,word却还是5,不符合逻辑
    }
    {
        //slice是字符串中一部分值的引用
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
        
        let hello = &s[..5];
        let world = &s[6..s.len()];

        let s = &s[..];

        fn first_word(s:&String)->&str{
            let bytes = s.as_bytes();

            for (i,&item) in bytes.iter().enumerate() {
                if item == b' '{
                    return &s[0..i]
                }
            }

            &s[..]
        }
        let mut s = String::from("hello world");

        let word = first_word(&s);
    
        //s.clear();   error:cannot borrow `s` as mutable because it is also borrowed as immutable
        //因为.clear()尝试获取s的可变引用,但是s已经被word获取了不可变引用,所以会报错 

        println!("the first word is: {}", word);
    }
    {
        //字符串字面量值就是 slice
        let s = "hello world";
        let s2 = String::from("hello world");
        //改造函数让它可以接受String和字面量两种值
        fn slice_str(s:&str) -> &str {
            &s[..]
        }
        let s = slice_str(s);
        let s = slice_str(&s[..]);
        let s2 = slice_str(&s2);
    }
    {
        let a = [1,2,3,4,5];
        let slice = &a[1..3];
    }
}
