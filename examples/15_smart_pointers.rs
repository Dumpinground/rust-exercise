use std::{
    cell::{Cell, RefCell},
    cmp::Ordering,
    fmt::{write, Display},
    ops::Deref,
    rc::Rc,
};

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
    child: RefCell<Vec<Rc<Org>>>,
}

impl Display for Org {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, [", self.name)?;
        let a = self.child.borrow();
        for c in a.iter() {
            write!(f, " {} ", c)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl Org {
    fn new(name: String, depth: usize) -> Org {
        Org {
            name,
            depth,
            child: RefCell::new(Vec::new()),
        }
    }
}

fn generate_texts(str_v: Vec<(&str, usize)>) -> Vec<(String, usize)> {
    let mut v = Vec::new();
    for (t, i) in str_v {
        v.push((t.to_owned(), i));
    }
    v
}

#[test]
fn vec_ref() {
    let s = "1".to_string();
    let texts = generate_texts(vec![
        ("root", 0),
        ("1", 1),
        ("2", 2),
        ("3", 2),
        ("4", 1),
        ("5", 2),
    ]);
    let mut stack = Vec::new();

    for (text, indent) in texts {
        loop {
            let last = stack.last_mut();
            match last {
                None => {
                    stack.push(Rc::new(Org::new(text, indent)));
                }
                Some(last) => {
                    match last.depth.cmp(&indent) {
                        Ordering::Less => {
                            let org = Rc::new(Org::new(text, indent));
                            last.child.borrow_mut().push(org.clone());
                            stack.push(org.clone());
                        }
                        Ordering::Equal => {
                            let org = Rc::new(Org::new(text, indent));
                            stack.pop();
                            let last = stack.last_mut().unwrap();
                            last.child.borrow_mut().push(org.clone());
                            stack.push(org.clone());
                        }
                        Ordering::Greater => {
                            stack.pop();
                            continue;
                        }
                    }
                    // last.child.push(Org::new(text, indent));
                }
            }
            break;
        }
        println!("len: {}, {}", stack.len(), stack.first().unwrap());
    }
}
