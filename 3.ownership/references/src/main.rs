fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // ==> len = &s1.len()

    let mut s2 = String::from("hello");

    change(&mut s2);

    println!("{}", s2);
}

fn calculate_length(s: &String) -> usize {   // s是对 String的引用
    s.len()
}  // 这里s离开了作用域，但因为它并不拥有引用值的所有权，所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}