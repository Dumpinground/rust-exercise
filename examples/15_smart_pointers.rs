use std::{ops::Deref, rc::Rc, cell::{Cell, RefCell}};

fn main() {
    println!("Hello Smart Pointers");
    println!("reference:\nhttps://course.rs/advance/smart-pointer/drop.html");
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

#[test]
fn rc() {
    let s = String::from("hello, world");
    let a = Rc::new(s);
    println!("{}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("{}", Rc::strong_count(&b));
}

#[test]
fn cell() {
    let c = Cell::new("asdf");
    let one = c.get();
    c.set("qwer");
    let tow = c.get();
    println!("{}, {}", one, tow);
}

#[test]
fn ref_cell() {
    let s = Rc::new(RefCell::new("In 1".to_string()));
    println!("{}", Rc::strong_count(&s));
    let s1 = s.clone();
    println!("{}", Rc::strong_count(&s));
    let s2 = s.clone();
    println!("{}", Rc::strong_count(&s));

    s2.borrow_mut().push_str(" out 2");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

#[derive(Debug)]
struct Org {
    name: String,
    depth: usize,
    child: Vec<Org>,
}

impl Org {
    fn new(name: String, depth: usize) -> Org {
        Org { name, depth, child: Vec::new() }
    }
}

#[test]
fn vec_ref() {
    let s = "1".to_string();
    let texts = vec![("1".to_owned(), 1), ("2".to_owned(), 2), ("3".to_owned(), 2)];
    let mut stack = Vec::new();

    for (text, indent) in texts {
        let last = stack.last_mut();
        match last {
            None => {
                stack.push(Org::new(text, indent));
            }
            Some(last) => {
                last.child.push(Org::new(text, indent))
            }
        }
        println!("len: {}, {:?}", stack.len(), stack.last());
    }
}