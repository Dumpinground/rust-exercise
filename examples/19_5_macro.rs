fn main() {
    println!("Hello macro");
}

macro_rules! myvec {
    [ $( $x:expr ),* ] => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[test]
fn test_myvec() {
    let v = myvec![1, 2, 3];
    for i in v {
        print!("{} ", i);
    }
    println!();

    let a = vec![1, 2];
}
