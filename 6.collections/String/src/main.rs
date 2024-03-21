fn main() {
    let mut s = String::from("hello");
    let data = "initial contents".to_string();

    // push_str采用字符串slice，因此不需要获取参数的所有权
    s.push_str(" world");

    // push 获取一个单独的字符作为参数，并附加到String中
    s.push('!');

    // println!("{}", data);
    // println!("{}", s);

    let s1 = String::from("hello ");
    let s2 = String::from("world");
    // let s3 = s1 + &s2;
    // println!("{}", s3);

    // 不获取参数所有权
    let s4 = format!("{}-{}-{}", s1, s2, s2);
    // println!("{}", s4);

    // 遍历字符串的方法
    // chars() 将字符串分开并返回char类型的值
    for c in s1.chars() {
        println!("{}", c);
    }
    // bytes() 返回每个原始字节
    for b in s1.bytes() {
        println!("{}", b);
    }
}

// 在这里，s2的&String被强转为&str（解引用强制转换）
// 签名中add获取了self的所有权，s1的所有权被移动到add调用中
// fn add(self, s: &str) -> String {}
