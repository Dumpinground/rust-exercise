use std::{fs::File, io::{BufRead, BufReader, Error, Read}};

use encoding_rs::{GBK, SHIFT_JIS, UTF_8};

fn main() {
    let bytestring = b"this is a bytestring";
    println!("A bytestring: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = std::str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }
}

#[test]
fn shift_jis_test() {
    // 字节串可以不使用 UTF-8 编码
    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82"; // SHIFT-JIS 编码的 "ようこそ"
    let shift_jis_bytes = b"\x83n\x83\x8D\x81[\x81E\x83\x8F\x81[\x83\x8B\x83h";
    let expectation = "\u{30CF}\u{30ED}\u{30FC}\u{30FB}\u{30EF}\u{30FC}\u{30EB}\u{30C9}";

    // 但这样的话它们就无法转换成 &str 了
    match std::str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    };

    let (cow, encoding_used, had_errors) = SHIFT_JIS.decode(shift_jis);
    println!("text: {:?}", cow);
}

#[test]
fn gbk_test() -> Result<(), Error> {
    let path = "E:/desktop/JYJCHRJK20240513.log";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut v = Vec::new();

    for line in buffered.bytes() {
        v.push(line?);
    }

    let (text, _, _) = GBK.decode(&v);

    // println!("{}", text);

    let mut stack = Vec::new();
    for line in text.lines() {

        if !line.contains("DBTIME") {
            continue;
        }

        if let Some(last_line) = stack.last() {
            if !line.eq(*last_line) {
                println!("{line}");
            }
        }

        if stack.is_empty() {
            stack.push(line);
        } else {
            stack.pop();
        }
    }

    Ok(())
}