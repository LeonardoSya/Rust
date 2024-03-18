fn fib(n: usize) -> Vec<u64> {
    let mut sequence = Vec::new();

    if n > 0 {
        sequence.push(0);
    }
    if n > 1 {
        sequence.push(1);
    }

    while sequence.len() < n {
        let last = *sequence.last().unwrap();
        let second_last = *sequence.get(sequence.len() - 2).unwrap_or(&0);
        sequence.push(last + second_last);
    }

    sequence
}

fn main() {
    let n = 10;
    let fib_sequence = fib(n);
    for num in fib_sequence {
        println!("{} ", num)
    }
}
