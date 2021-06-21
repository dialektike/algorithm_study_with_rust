use std::time::Instant;

fn main() {
    println!("Hello, world!");

    let now = Instant::now();
    println!("{:?}", fibonacci(21));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));

    let now = Instant::now();
    println!("{:?}", recursive_fibonacci(21));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));


    let now = Instant::now();
    println!("{:?}", fibonacci_numbers(fibonacci, 21));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));

    let now = Instant::now();
    println!("{:?}", fibonacci_numbers(recursive_fibonacci, 21));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));

}

fn fibonacci_numbers(f: fn(i32) -> i32, n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for item in 0..n + 1 {
        result.push(f(item))
    }
    //println!("{:?}", result);
    result
}

fn recursive_fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        recursive_fibonacci(n - 1) + recursive_fibonacci(n - 2)
    }
}

fn fibonacci(n: i32) -> i32{
    let mut x: i32 = 0;
    let mut y: i32 = 1;
    let mut z: i32 = 0;
    
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
