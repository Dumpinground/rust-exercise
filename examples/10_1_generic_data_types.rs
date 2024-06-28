fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn run_compare() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}

fn run_struct() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1., y: 4. };
    let integer_and_float = Point2 { x: 5, y: 4. };
}

fn main() {
    run_compare();
    run_struct();
}
