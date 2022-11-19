mod guess_game;

use std::io;
use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@example.com");

    print!("{}", user1.email)
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn first_word(s : &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {

        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn the_slice_type() {

    let mut s = String::from("hello world");

    println!("{}", first_word(&s));
}

fn ownership() {
    let s1 = String::from("hello");
    let s2 = &s1;

    println!("{}, world!", s1);
}

fn nf(a: i16, b: i16, n: i16) {
    if n != 0 {
        print!("{} ", a);
        nf(b, a + b, n - 1);
    }
}

fn break_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);
}

fn plus_one(x: i8) -> i8 {
    x + 1
}

fn five() -> i8 {
    5
}

fn array() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim().parse()
        .expect("Index entered was not a nubmer");

    let element = a[index];

    println!(
        "The value of the element at index {} is {}",
        index, element
    );
}

fn say_simply(message : &str) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(message.as_bytes(), width, &mut writer).unwrap();
}