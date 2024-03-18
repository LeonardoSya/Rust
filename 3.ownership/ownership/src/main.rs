// fn main() {
//     //* 字符串字面量是不可变的 ("hello")  */
//     //* Rust的第二个字符串类型 String 这个类型管理被分配到堆上的数据，所以能够存储在编译时未知大小的文本 */
//     let mut s1 = String::from("hello");  // s1进入作用域

//     s1.push_str(", world!");

//     println!("{}", s1);

//     let s2 = s1;  // s1被移动到s2

//     println!("{}", s2);

// }

// fn main() {
//     let s1 = gives_ownership();

//     let s2 = String::from("hello");

//     let s3 = takes_and_gives_back(s2);
// }

fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string  // 返回值也可以转移所有权，会移出给调用函数
}

fn takes_and_gives_back(a_string: String) -> String  {
    a_string
}

fn main() { 
    let s = String::from("hello");  // s进入作用域

    takes_ownership(s);  // s的值移动到函数里，所以这里的s不再有效
    
    let x = 5;

    makes_copy(x);  // x应该移动到函数里，但是i32是Copy，所以在后面可以继续使用x

} // x先移出作用域，然后是s。但是s已经移走，所以不做特殊操作

fn takes_ownership(some_string:String) {
    println!("{}", some_string);
}  // some_string移出作用域并调用drop方法，内存被释放


fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}  // some_integer移出作用域，不会有特殊操作因为i32实现了Copy trait