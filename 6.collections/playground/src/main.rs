use std::collections::HashMap;

fn main() {
    let v = vec![1, 4, 5, 2, 1, 5, 3, 7, 2, 1, 7];

    println!("{}", v_avrg(&v));
    println!("{}", v_median(&v));
    println!("{:?}", v_mode(&v).unwrap());
}

fn v_avrg(v: &Vec<i32>) -> f64 {
    let sum: i32 = v.iter().sum();
    (sum as f64) / (v.len() as f64)
}

fn v_median(v: &Vec<i32>) -> f64 {
    let mut sorted = v.clone();
    sorted.sort_unstable();
    let mid = sorted.len() / 2;

    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] as f64) + (sorted[mid] as f64) / 2.0
    } else {
        sorted[mid] as f64
    }
}

fn v_mode(v: &Vec<i32>) -> Option<i32> {
    // Option<i32> 表示可能存在一个整数(Some(i32))，也可能没有(None)
    let mut hashmap = HashMap::new();

    for &i in v {
        *hashmap.entry(i).or_insert(0) += 1;
    }

    hashmap
        .into_iter() // 转换成一个迭代器，这允许我们遍历键值对
        .max_by_key(|&(_, count)| count) // 这里的闭包 |&(_, count)| count 用于提取出每个键值对中的值（即出现次数）作为比较的基准。
        .map(|(val, _)| val)
}
