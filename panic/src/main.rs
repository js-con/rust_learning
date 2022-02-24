use std::{
    fs::{File, self},
    io::{self, ErrorKind, Read},
};

fn main()  {
    //nonrenewable_panic();
    //open_file();
/*     let result = return_error();
    match result {
        Ok(str) => println!("{}", &str),
        Err(err) => panic!("{:?}", err),
    }; */
    
    let result = return_error_operator();
    match result {
        Ok(str) => println!("{}", &str),
        Err(err) => panic!("{:?}", err),
    };
}
//命令RUST_BACKTRACE=1 cargo run 可以看到错误栈信息
//panic!是不可恢复的错误
fn direct_panic() {
    panic!("crash and burn");
}
fn nonrenewable_panic() {
    let v = vec![1, 2, 3];

    v[99];
}

// 处理Result枚举来解决可能出现的错误
fn open_file() {
    let f = File::open("hello.txt");

    // 通过match处理不同的错误
    // 未找到文件时 创建文件
    // 其他情况,panic
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件失败：{:?}", e),
            },
            other_error => {
                panic!("打开文件失败: {:?}", other_error)
            }
        },
    };

    //通过闭包(13章)和unwrap_or_else处理不同的错误,代码更简洁
    let f = File::open("world.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("创建文件失败:{:?}", error);
            })
        } else {
            panic! {"打开文件失败: {:?}",error};
        }
    });
}

fn unwrap_and_expect() {
    //match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好的表明其意图。Result<T, E> 类型定义了很多辅助方法来处理各种情况
    // unwrap是其中一种,如果Result值为Ok,则返回Ok中的值,如果Result是Err,则自动panic!
    let f = File::open("hello.txt").unwrap();

    //如果想在panic!时打印自己的错误信息,则可以使用expect
    let f = File::open("hello.txt").expect("打开文件失败");
}

fn return_error() -> Result<String, io::Error> {
    //传播错误：函数操作失败时返回错误，由调用者决定如何处理错误
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn return_error_operator() -> Result<String, io::Error> {
    //传播错误的简写运算符 ? ,逻辑与上面的match一致：有错误就提前返回，否则就取Ok值
    // 注意： ?只能用于返回值为Result或Option或其它实现了FromResidual的类型的函数中,比如在main函数中就用不了?，因为main函数返回值是()，当然，也有办法可以修改main函数返回值，见17章
    // 如果用在返回值是Option的函数中，其None将会被提前返回，等同于Err
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}

fn return_error_chain() -> Result<String, io::Error>{
    // 还可以对?操作符链式调用,逻辑与上面完全相同
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    return Ok(s);
}

fn shortest_version() -> Result<String, io::Error>{
    //以上逻辑的最简写法
    //fs::read_to_string 函数会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它
    fs::read_to_string("hello.txt")
}

//?用于Option返回值的例子
fn last_chat_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// 创建自定义类型进行有效性验证
fn guess_game(){
    //如果在猜猜看游戏中要保证用户输入的数字在1-100之间，则代码的每一处逻辑都要检查，这样十分繁琐
    //我们可以创建一个新类型来将验证放在其new函数中，而不是到处重复检查

    pub struct Guess{
        value: i32
    }
    impl Guess{
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100{
                panic!("Guess value must be between 1 and 100, got {}.", value)
            }
            Guess { value }
        }
        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

//错误处理指导原则
/* 
在当有可能会导致有害状态的情况下建议使用 panic! —— 在这里，有害状态是指当一些假设、保证、协议或不可变性被打破的状态，例如无效的值、自相矛盾的值或者被传递了不存在的值 —— 外加如下几种情况：

1. 有害状态是非预期的行为，与偶尔会发生的行为相对，比如用户输入了错误格式的数据。
2. 在此之后代码的运行依赖于不处于这种有害状态，而不是在每一步都检查是否有问题。
3. 没有可行的手段来将有害状态信息编码进所使用的类型中的情况。我们会在第十七章 “将状态和行为编码为类型” 部分通过一个例子来说明我们的意思。

如果别人调用你的代码并传递了一个没有意义的值，最好的情况也许就是 panic! 并警告使用你的库的人他的代码中有 bug 以便他能在开发时就修复它。类似的，如果你正在调用不受你控制的外部代码，并且它返回了一个你无法修复的无效状态，那么 panic! 往往是合适的。

然而当错误预期会出现时，返回 Result 仍要比调用 panic! 更为合适。这样的例子包括解析器接收到格式错误的数据，或者 HTTP 请求返回了一个表明触发了限流的状态。在这些例子中，应该通过返回 Result 来表明失败预期是可能的，这样将有害状态向上传播，调用者就可以决定该如何处理这个问题。使用 panic! 来处理这些情况就不是最好的选择。

当代码对值进行操作时，应该首先验证值是有效的，并在其无效时 panic!。这主要是出于安全的原因：尝试操作无效数据会暴露代码漏洞，这就是标准库在尝试越界访问数组时会 panic! 的主要原因：尝试访问不属于当前数据结构的内存是一个常见的安全隐患。函数通常都遵循 契约（contracts）：他们的行为只有在输入满足特定条件时才能得到保证。当违反契约时 panic 是有道理的，因为这通常代表调用方的 bug，而且这也不是那种你希望所调用的代码必须处理的错误。事实上所调用的代码也没有合理的方式来恢复，而是需要调用方的 程序员 修复其代码。函数的契约，尤其是当违反它会造成 panic 的契约，应该在函数的 API 文档中得到解释。

虽然在所有函数中都拥有许多错误检查是冗长而烦人的。幸运的是，可以利用 Rust 的类型系统（以及编译器的类型检查）为你进行很多检查。如果函数有一个特定类型的参数，可以在知晓编译器已经确保其拥有一个有效值的前提下进行你的代码逻辑。例如，如果你使用了一个并不是 Option 的类型，则程序期望它是 有值 的并且不是 空值。你的代码无需处理 Some 和 None 这两种情况，它只会有一种情况就是绝对会有一个值。尝试向函数传递空值的代码甚至根本不能编译，所以你的函数在运行时没有必要判空。另外一个例子是使用像 u32 这样的无符号整型，也会确保它永远不为负。
*/