mod guess_game;

use crate::leetcode_solution::Solution;
use ferris_says::say;
use std::io;
use std::io::{stdout, BufWriter};

mod leetcode_solution;

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    print!(
        "Can rect1 hold rect2? {}\nCan rect2 hold rect3? {}\n",
        rect1.can_hold(&rect2),
        rect2.can_hold(&rect3)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn first_word(s: &str) -> &str {
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
        .trim()
        .parse()
        .expect("Index entered was not a nubmer");

    let element = a[index];

    println!("The value of the element at index {} is {}", index, element);
}

fn say_simply(message: &str) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());

    say(message, width, &mut writer).unwrap();
}
