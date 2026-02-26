use std::io;

fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 1) + fib(n - 2) }
}

fn main() {
    println!("Введите число:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка чтения");

    let n: u32 = input.trim().parse().expect("Введите корректное число");

    println!("fib({}) = {}", n, fib(n));
}
