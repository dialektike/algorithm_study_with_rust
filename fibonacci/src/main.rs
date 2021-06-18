fn main() {
    println!("Hello, world!");

    println!("{:?}", fibonacci(21));
    println!("{:?}", fibonacci_numbers(21));
}

fn fibonacci_numbers(n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for item in 0..n + 1 {
        result.push(fibonacci(item))
    }
    //println!("{:?}", result);
    result
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
