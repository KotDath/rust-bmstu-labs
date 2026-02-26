# Лабораторная работа №3: Транспонирование матрицы

## Теоретическая часть

### 1. Массивы

Массив в Rust - это коллекция фиксированного размера, хранящая элементы одного типа. Массивы располагаются в стеке.

#### Создание массивов

```rust
// Явное указание элементов
let arr = [1, 2, 3, 4, 5];

// С явным типом: [тип; длина]
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// Инициализация одинаковым значением
let zeros = [0; 10];        // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
let threes = [3; 4];        // [3, 3, 3, 3]
```

#### Доступ к элементам

```rust
let arr = [10, 20, 30, 40, 50];

let first = arr[0];         // 10
let second = arr[1];        // 20
let last = arr[arr.len() - 1];  // 50

// Проверка границ происходит во время выполнения!
// arr[10] вызовет panic!("index out of bounds")
```

#### Длина массива

```rust
let arr = [1, 2, 3, 4, 5];
println!("Длина: {}", arr.len());  // 5
println!("Пустой? {}", arr.is_empty());  // false
```

#### Итерация по массиву

```rust
let arr = [1, 2, 3, 4, 5];

// По значению (копии)
for elem in arr {
    println!("{}", elem);
}

// По индексу
for i in 0..arr.len() {
    println!("arr[{}] = {}", i, arr[i]);
}

// С индексом и значением
for (i, &elem) in arr.iter().enumerate() {
    println!("arr[{}] = {}", i, elem);
}
```

---

### 2. Изменяемые массивы

Массивы immutable по умолчанию. Для изменения используйте `mut`:

```rust
let mut arr = [1, 2, 3, 4, 5];

arr[0] = 100;       // OK
arr[2] = arr[2] * 2;  // OK

println!("{:?}", arr);  // [100, 2, 6, 4, 5]
```

#### Изменение через итератор

```rust
let mut arr = [1, 2, 3, 4, 5];

for elem in arr.iter_mut() {
    *elem *= 2;  // разыменование и умножение на 2
}

println!("{:?}", arr);  // [2, 4, 6, 8, 10]
```

---

### 3. Многомерные массивы и вложенные массивы

#### Двумерные массивы (матрицы)

```rust
// Матрица 3x3
let matrix: [[i32; 3]; 3] = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

// Доступ к элементу: matrix[строка][столбец]
let center = matrix[1][1];     // 5
let top_right = matrix[0][2];  // 3
```

#### Создание матрицы с одинаковыми значениями

```rust
// Нулевая матрица 4x4
let zeros: [[i32; 4]; 4] = [[0; 4]; 4];

// Единичная матрица 3x3 (построчно)
let identity: [[i32; 3]; 3] = [
    [1, 0, 0],
    [0, 1, 0],
    [0, 0, 1],
];
```

#### Итерация по матрице

```rust
let matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

// По строкам
for row in matrix {
    println!("{:?}", row);
}

// По всем элементам
for row in matrix {
    for &elem in row {
        print!("{} ", elem);
    }
    println!();
}

// С индексами
for (i, row) in matrix.iter().enumerate() {
    for (j, &elem) in row.iter().enumerate() {
        println!("matrix[{}][{}] = {}", i, j, elem);
    }
}
```

#### Изменение матрицы

```rust
let mut matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9],
];

// Изменение элемента
matrix[0][0] = 100;

// Умножение всех элементов на 2
for row in matrix.iter_mut() {
    for elem in row.iter_mut() {
        *elem *= 2;
    }
}
```

#### Трёхмерные массивы

```rust
// 3D массив: 2x3x4
let cube: [[[i32; 4]; 3]; 2] = [
    [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
    ],
    [
        [13, 14, 15, 16],
        [17, 18, 19, 20],
        [21, 22, 23, 24],
    ],
];

let value = cube[1][2][3];  // 24
```

---

### 4. Матрицы

#### Типичное представление

```rust
type Matrix3x3 = [[i32; 3]; 3];

fn print_matrix(m: Matrix3x3) {
    for row in m {
        for &elem in row {
            print!("{:4}", elem);
        }
        println!();
    }
}

fn main() {
    let m: Matrix3x3 = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    print_matrix(m);
}
```

#### Размерность матрицы

```rust
fn matrix_dimensions<T, const ROWS: usize, const COLS: usize>(m: [[T; COLS]; ROWS]) -> (usize, usize) {
    (ROWS, COLS)
}

let m = [[1, 2, 3], [4, 5, 6]];
let (rows, cols) = matrix_dimensions(m);
println!("{}x{}", rows, cols);  // 2x3
```

---

### 5. Кортежи

Кортеж (tuple) - это коллекция фиксированного размера, которая может хранить элементы **разных типов**.

#### Создание кортежей

```rust
// Разные типы
let tuple: (i32, f64, &str) = (42, 3.14, "hello");

// Без явного типа
let person = ("Иван", 25, true);

// Пустой кортеж (unit type)
let unit = ();

// Кортеж из одного элемента (нужна запятая!)
let single = (5,);  // тип: (i32,)
let not_tuple = (5); // просто i32 в скобках
```

#### Доступ к элементам

```rust
let tuple = (10, 20.5, "тридцать");

// По индексу (через точку)
let first = tuple.0;   // 10
let second = tuple.1;  // 20.5
let third = tuple.2;   // "тридцать"
```

#### Изменяемые кортежи

```rust
let mut tuple = (1, 2, 3);

tuple.0 = 100;
tuple.1 += 5;

println!("{:?}", tuple);  // (100, 7, 3)
```

#### Вложенные кортежи

```rust
let nested = ((1, 2), (3, 4));

let a = nested.0.0;  // 1
let b = nested.1.1;  // 4
```

#### Кортежи и функции

```rust
// Возврат нескольких значений
fn divide(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)  // (частное, остаток)
}

let (quotient, remainder) = divide(17, 5);
println!("17 / 5 = {} остаток {}", quotient, remainder);
```

---

### 6. Шаблоны и деструктуризация

#### Деструктуризация кортежей

```rust
let tuple = (1, 2.0, "три");

// Полная деструктуризация
let (a, b, c) = tuple;
println!("{} {} {}", a, b, c);

// С игнорированием элементов (_)
let (first, _, last) = tuple;
println!("{} {}", first, last);

// Вложенная деструктуризация
let nested = ((1, 2), (3, 4));
let ((a, b), (c, d)) = nested;
```

#### Деструктуризация массивов

```rust
let arr = [1, 2, 3, 4, 5];

// Полная
let [a, b, c, d, e] = arr;

// С игнорированием
let [first, _, _, _, last] = arr;

// Первый и остальные (с ..)
let [first, rest @ ..] = arr;
println!("first: {}, rest: {:?}", first, rest);  // first: 1, rest: [2, 3, 4, 5]

// Последний и остальные
let [rest @ .., last] = arr;
println!("rest: {:?}, last: {}", rest, last);  // rest: [1, 2, 3, 4], last: 5

// Первый, последний и середина
let [first, middle @ .., last] = arr;
println!("{} {:?} {}", first, middle, last);  // 1 [2, 3, 4] 5
```

#### Деструктуризация в let

```rust
// Обмен значений без временной переменной
let (mut a, mut b) = (1, 2);
(a, b) = (b, a);
println!("a={}, b={}", a, b);  // a=2, b=1
```

#### Деструктуризация в match

```rust
let tuple = (2, 3);

match tuple {
    (0, y) => println!("первый = 0, второй = {}", y),
    (x, 0) => println!("первый = {}, второй = 0", x),
    (x, y) if x == y => println!("равны: {}", x),
    (x, y) => println!("{} и {}", x, y),
}
```

```rust
let arr = [1, 2, 3];

match arr {
    [0, _, _] => println!("начинается с 0"),
    [a, b, c] if a + b == c => println!("{} + {} = {}", a, b, c),
    [first, ..] => println!("первый элемент: {}", first),
}
```

#### Деструктуризация в параметрах функции

```rust
fn print_point((x, y): (i32, i32)) {
    println!("x = {}, y = {}", x, y);
}

fn print_coords([x, y, z]: [i32; 3]) {
    println!("({}, {}, {})", x, y, z);
}

fn main() {
    print_point((10, 20));
    print_coords([1, 2, 3]);
}
```

#### Деструктуризация в цикле for

```rust
let pairs = [(1, 2), (3, 4), (5, 6)];

for (a, b) in pairs {
    println!("{} + {} = {}", a, b, a + b);
}

let matrix = [
    [1, 2, 3],
    [4, 5, 6],
];

for (i, row) in matrix.iter().enumerate() {
    for (j, &elem) in row.iter().enumerate() {
        println!("[{}][{}] = {}", i, j, elem);
    }
}
```

---

### 7. Операции с массивами

#### Срезки (Slices)

Срезка - это ссылка на часть массива:

```rust
let arr = [1, 2, 3, 4, 5];

let slice = &arr[1..4];   // [2, 3, 4]
let first_two = &arr[..2]; // [1, 2]
let last_two = &arr[3..];  // [4, 5]
let all = &arr[..];        // [1, 2, 3, 4, 5]
```

#### Методы массивов (через итераторы)

```rust
let arr = [1, 2, 3, 4, 5];

// Поиск
let first_even = arr.iter().find(|&&x| x % 2 == 0);  // Some(&2)
let any_negative = arr.iter().any(|&x| x < 0);       // false
let all_positive = arr.iter().all(|&x| x > 0);       // true

// Фильтрация и преобразование (возвращает Vec)
let evens: Vec<i32> = arr.iter().filter(|&&x| x % 2 == 0).copied().collect();
let doubled: Vec<i32> = arr.iter().map(|&x| x * 2).collect();

// Сумма, мин, макс
let sum: i32 = arr.iter().sum();
let max = arr.iter().max();
let min = arr.iter().min();

// Проверка содержания
let contains = arr.contains(&3);  // true
```

#### Сортировка и переворот (для мутабельных массивов)

```rust
let mut arr = [3, 1, 4, 1, 5, 9, 2, 6];

arr.sort();           // [1, 1, 2, 3, 4, 5, 6, 9]
arr.reverse();        // [9, 6, 5, 4, 3, 2, 1, 1]

// Сортировка по ключу
let mut words = ["banana", "apple", "cherry"];
words.sort_by_key(|w| w.len());  // ["apple", "banana", "cherry"]
```

---

## Условие задачи

### Транспонирование матрицы

Транспонирование матрицы - это операция, при которой строки матрицы становятся столбцами, а столбцы - строками.

Для матрицы A размером m×n транспонированная матрица A^T имеет размер n×m:

```
A = | 1  2  3 |      A^T = | 1  4  7 |
    | 4  5  6 |             | 2  5  8 |
                            | 3  6  9 |
```

Элемент `A^T[i][j] = A[j][i]`

### Задание

1. Реализуйте функцию `transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3]`, которая возвращает транспонированную матрицу
2. Реализуйте обобщённую функцию для матрицы любого размера (используя const generics)
3. Программа должна запрашивать у пользователя элементы матрицы 3x3 и выводить исходную и транспонированную матрицы

### Требования

- Используйте вложенные циклы `for` для транспонирования
- Форматируйте вывод матрицы в виде таблицы
- Добавьте тесты для проверки корректности

### Пример работы программы

```
Введите элементы матрицы 3x3:
matrix[0][0]: 1
matrix[0][1]: 2
matrix[0][2]: 3
matrix[1][0]: 4
matrix[1][1]: 5
matrix[1][2]: 6
matrix[2][0]: 7
matrix[2][1]: 8
matrix[2][2]: 9

Исходная матрица:
   1    2    3
   4    5    6
   7    8    9

Транспонированная матрица:
   1    4    7
   2    5    8
   3    6    9
```

### Подсказки

- Используйте `swap` для обмена элементов: `matrix[i][j]` ↔ `matrix[j][i]`
- Обратите внимание, что для квадратной матрицы достаточно пройти только верхний треугольник
- Для вывода используйте форматирование ширины: `print!("{:4}", elem)`
