use std::time::Instant;

fn main() {
    println!("Hello, world!");

    fn factorial(n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            n * factorial(n -1)
        }
    }

    println!("{:?}", factorial(10));

    fn power(a: i32, n: i32) -> i32 {
        if n == 0 {
            1
        } else {
            a * power(a, n-1)
        }
    }

    println!("{:?}", power(2, 8));

    fn gcd(a: i32, b: i32) -> i32{

        let mut x: i32 = a;
        let mut y: i32 = b;
        
        while y != 0 {
            let r: i32 = x % y;
            x = y;
            y = r;
        }
        x
    }

    let now = Instant::now();
    println!("{:?}", gcd(414, 662));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));

    fn gcd_recursive(a: i32, b: i32) -> i32{

        if a == 0 {
            b
        } else {
            gcd_recursive(b % a, a)
        }
    }

    let now = Instant::now();
    println!("{:?}", gcd_recursive(414, 662));
    let new_now = Instant::now();
    println!("{:?}", new_now.checked_duration_since(now));

}

