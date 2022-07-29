mod front_of_house{
    pub mod hosting{
        pub fn add_to_waitlist(){}

        fn seat_at_table(){}
    }
}

fn serve_order(){}
mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        //super关键字
        super::serve_order();
    }
    fn cook_order(){}

    //带公私字段的结构体
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }
    //由于有私有字段，虽然结构体是公共的，但外部无法创建实例
    //所以要提供一个构造函数（关联函数）
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    //和结构体相反，如果给枚举设置为共有，则它的所有成员都将是公有的
    pub enum Appetizer {
        Soup,
        Salad
    }
}
pub fn eat_at_restaurant(){
    //绝对路径和相对路径
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    //注意 不能直接构造一个Breakfast，因为seasonal_fruit字段是私有的
/*     let shit = back_of_house::Breakfast{
        toast:String::from("shit"),
        seasonal_fruit:String::from("fuck")
    }; */

    //这里只能通过公共构造函数来创建实例
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);

    //公有结构体的所有成员也会自动变成公有
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

