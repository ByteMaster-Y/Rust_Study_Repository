fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    return curr;
}

fn main() {
    let n = 10; // 여기에 n값을 변경하여 다른 피보나치 수열을 얻을 수 있습니다.
    println!("The {}th Fibonacci number is: {}", n, fibonacci(n));
}