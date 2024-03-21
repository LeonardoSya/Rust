fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];  // 使用引用返回一个元素
    println!("The third element is {}", third);

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("hello")),
        SpreadsheetCell::Float(10.12),
    ];
    
    match v.get(2) {   // 使用get方法返回一个元素
        Some(third) => println!("The third ele is {}", third), 
        None => println!("there is no third ele"),
    }

    // 遍历vector的每个元素的 不可变引用
    for i in &v {
        println!("{}", i);
    }

    // 遍历vector的每个元素的 可变引用
    for i in &mut v {
        *i += 50;  // * 解引用 因为i是v中的可变引用，所以要修改元素值的话就要*解引用i，从而获得对实际值的可变访问
        println!("{}", i);
    }

    for i in row.iter() {
        println!("{:?}", i);
    }
}
