#[derive(Debug)] // 增加属性来派生 Debug trait
struct Rectangle {
    width: u32,
    height: u32,
}
//* 所有在impl块中定义的函数被称为【关联函数】，因为它们与impl后面命名的类型相关 */
//* 关联函数经常被用作返回一个结构体新实例的构造函数 */
impl Rectangle {
    // &self 是 self:&self 的缩写，选择&self的原因是我们不想获取所有权，只希望读取而不是写入
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(10 * scale),
        height: 20,
    };
    //  XXX::fn() 调用关联函数
    let rect_sq = Rectangle::square(3);

    println!("{}", rect1.area()); // 希望借用rect1的实例，而不是获取所有权
    println!("{}", rect1.can_hold(&rect2));
    println!("{:#?}", rect1);
    dbg!(&rect1); // dbg!宏接收一个表达式的所有权，打印调用dbg!宏时所在的文件和行号和结果值，并返回该值的所有权
}

// 入参rectangle的类型是 结构体Rectangle实例的不可变借用，因为我们希望借用结构体而不是获取它的所有权
