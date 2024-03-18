fn main() {
    let s = String::from("hello world");

    // let hello = &s[0..5]; // 对部分 String 的引用，包含一个指针和一个长度
    // let world = &s[6..11];

    let word = first_word(&s);

    s.clear(); // clear需要清空String，它尝试获取一个可变引用

    println!("{}", word); // word中的引用被使用，所以这个不可变引用在此时必须仍然有效
                          // rust不允许clear中的可变引用和word中的不可变引用同时存在，因此编译失败
}

fn first_word(s: &str) -> &str {
    //* 传递String的整个slice或对String的引用更好 */
    // 字符串slice的类型声明写作&str
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
