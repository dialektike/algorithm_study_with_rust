use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let num: i64 = 21;

    let now = Instant::now();
    println!("{}번째 피보나치 수: {:?}", num, fibonacci(num));
    let new_now = Instant::now();
    println!(
        "피보나치 함수로 구한 시간: {:?}",
        new_now.checked_duration_since(now)
    );

    let now = Instant::now();
    println!("{}번째 피보나치 수: {:?}", num, recursive_fibonacci(num));
    let new_now = Instant::now();
    println!(
        "재귀 피보나치 함수로 구한 시간: {:?}",
        new_now.checked_duration_since(now)
    );

    let now = Instant::now();
    println!("21 번째 피보나치 수: {:?}", fibonacci_dp(21));
    let new_now = Instant::now();
    println!(
        "동적계획법 피보나치 함수로 구한 시간: {:?}",
        new_now.checked_duration_since(now)
    );

    let now = Instant::now();
    println!("{:?}", fibonacci_numbers(fibonacci, num));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));

    let now = Instant::now();
    println!("{:?}", fibonacci_numbers(recursive_fibonacci, num));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));
}

fn fibonacci_numbers(f: fn(i64) -> i64, n: i64) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for item in 0..n + 1 {
        result.push(f(item))
    }
    //println!("{:?}", result);
    result
}

// fn recursive_fibonacci(n: i32) -> i32 {
//     if n == 0 {
//         0
//     } else if n == 1 {
//         1
//     } else {
//         recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
//     }
// }

fn recursive_fibonacci(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2),
    }
}

fn fibonacci(n: i64) -> i64 {
    let mut x: i64 = 0;
    let mut y: i64 = 1;
    let mut z: i64 = 0;

    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        for _i in 1..n {
            z = x + y;
            x = y;
            y = z;
        }
        z
    }
}

fn fibonacci_dp(n: usize) -> usize {
    let mut memo = vec![0; n + 1];
    memo[1] = 1;
    for i in 2..n + 1 {
        memo[i] = memo[i - 1] + memo[i - 2];
    }
    return memo[n];
}
