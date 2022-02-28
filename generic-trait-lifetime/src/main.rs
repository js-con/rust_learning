fn main() {
    generic_learning();
}

fn generic_learning() {
    //结构体使用泛型
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let wrond_point = Point { x: 5, y: 4.0 }; //error

    //方法中使用泛型
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    //可以仅仅为某个类型的实例定义方法
    impl Point<f32> {
        //其他类型的Point实例就没有这个方法
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    //使用与结构体定义中不同类型的泛型
    struct Point2<X1, Y1> {
        x: X1,
        y: Y1,
    }
    impl<X1, Y1> Point2<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {},p3.y = {}", p3.x, p3.y);

    //enum使用泛型
    enum Option<T> {
        Some(T),
        None,
    }
}
