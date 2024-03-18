fn main() {
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> &String {  // 返回一个字符串的引用
    let s = String::from("hello");

    &s
}  // s离开作用域就被丢弃，内存被释放，而返回了s的引用，这意味着这个引用会指向一个无效的String。危险！