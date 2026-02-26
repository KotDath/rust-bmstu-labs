use std::io;

/// Транспонирует матрицу 3x3, возвращает (оригинал, транспонированная)
fn transpose(matrix: [[i32; 3]; 3]) -> ([[i32; 3]; 3], [[i32; 3]; 3]) {
    let mut result = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i];
        }
    }

    (matrix, result)
}

/// Выводит матрицу в красивом формате
fn print_matrix(label: &str, matrix: [[i32; 3]; 3]) {
    println!("{}", label);
    for row in matrix {
        for elem in row {
            print!("{:4}", elem);
        }
        println!();
    }
    println!();
}

/// Считывает матрицу 3x3 с клавиатуры
fn read_matrix() -> [[i32; 3]; 3] {
    let mut matrix = [[0; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            println!("matrix[{}][{}]:", i, j);

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Ошибка чтения");

            matrix[i][j] = input
                .trim()
                .parse()
                .expect("Введите корректное число");
        }
    }

    matrix
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103],
        [201, 202, 203],
        [301, 302, 303],
    ];
    let (original, transposed) = transpose(matrix);
    assert_eq!(original, matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301],
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

#[test]
fn test_transpose_identity() {
    let identity = [
        [1, 0, 0],
        [0, 1, 0],
        [0, 0, 1],
    ];
    let (_, transposed) = transpose(identity);
    assert_eq!(transposed, identity);
}

#[test]
fn test_transpose_twice() {
    let matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    // Транспонирование дважды возвращает исходную матрицу
    let (_, t1) = transpose(matrix);
    let (_, t2) = transpose(t1);
    assert_eq!(t2, matrix);
}

fn main() {
    println!("Введите элементы матрицы 3x3:\n");

    let matrix = read_matrix();

    let (original, transposed) = transpose(matrix);

    println!();
    print_matrix("Исходная матрица:", original);
    print_matrix("Транспонированная матрица:", transposed);
}
