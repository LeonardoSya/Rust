fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    enum Message {
        Quit,
        Move {  // 包含一个匿名结构体
            x: i32,
            y: i32,
        },
        Write(String),  // 包含单独一个String
        ChangeColor(i32, i32, i32),
    }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let x:i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y;
    //* Option<i8>和i8类型不同，所以不能相加 */
    //* 当Rust中拥有一个像i8这样类型的值时，编译器确保它总是一个有效的值，因为只有当用Option<T>时才需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况 */
    //* 因此，在对Option<T>进行T的运算之前必须将其转换为T，这通常能捕获空值的错误 */
}
