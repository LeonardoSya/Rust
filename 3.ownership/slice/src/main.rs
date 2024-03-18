fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; // 对部分 String 的引用，包含一个指针和一个长度
    let world = &s[6..11];
}

fn first_word(s: &String) -> &str {
    // 字符串slice的类型声明写作&str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
