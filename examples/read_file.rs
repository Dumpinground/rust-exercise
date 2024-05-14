use std::{
    fs::File,
    io::{BufReader, Error, Read},
};

use encoding_rs::GBK;

fn main() -> Result<(), Error> {
    let path = "E:/desktop/LZYBLOG20240514.log";
    let text = read_file(path)?;

    check_interval(&text);

    Ok(())
}

// 检查时间间隔
fn check_interval(text: &str) {
    let mut stack: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.contains("时间：") {
            if let Some(last) = stack.last() {
                if !line[12..].eq(&last[12..]) {
                    println!("{}\n{}\n", last, line);
                }
            }

            if stack.is_empty() {
                stack.push(line);
            } else {
                stack.pop();
            }
        }
    }
}

// 从 GBK 编码转换文件
fn read_file(path: &str) -> Result<String, Error> {
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut v = Vec::new();
    for line in buffered.bytes() {
        v.push(line?);
    }

    let (text, _, _) = GBK.decode(&v);

    Ok(text.into_owned())
}
