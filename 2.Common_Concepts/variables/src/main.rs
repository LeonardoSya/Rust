fn main() {
    let mut x = 5;
    // println!("The value of x is: {}", x);
    x = 6;
    // println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let space = "   ";
    let space = space.len();
    // println!("{}", space);

    let heart_eyed_cat = "😻😻😻😻😻😻";
    // println!("{}", heart_eyed_cat);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    // println!("{}", y);

    let a = [1, 2, 3, 4, 5];
    let b = [3; 5];
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("a: {:?}", a);
    // println!("b: {:?}", b);
    // println!("c: {:?}", c);

    let m = five();
    let n = plus_one(1);
    println!("{}", n);
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
