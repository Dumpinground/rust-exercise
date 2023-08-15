use std::ops::Deref;

fn main() {
    println!("Hello Smart Pointers");
}

#[test]
fn normal_deref() {
    let x = 5;
    let y = &x;

    let z = *y;
}

fn smart_deref() {
    let x = Box::new(1);
    let sum = *x + 1;
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn my_smart_deref() {
    let y = MyBox::new(5);
    let z = *y;
}