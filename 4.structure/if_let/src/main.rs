fn main() {
    let some_u8_value = Some(0u8);
    // 匹配一个Option<u8>值并只希望当值为3时执行代码
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // if let的工作方式和match相同（语法糖），但失去穷尽性检查
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
