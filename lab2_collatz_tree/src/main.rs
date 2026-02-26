use std::io;

fn collatz_length(mut n: u32) -> u32 {
    let mut counter = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }

        counter += 1;
    }

    counter
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Введите число:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Ошибка чтения");

    let n: u32 = input.trim().parse().expect("Введите корректное число");

    println!("Длина последовательности Коллатца для {}: {}", n, collatz_length(n));
}
