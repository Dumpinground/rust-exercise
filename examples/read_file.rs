use std::{
    env::{self, args}, fs::File, io::{BufReader, Error, Read}
};

use chrono::{NaiveTime, TimeDelta};
use clap::{arg, command};
use encoding_rs::GBK;

struct Item {
    end_time: String,
    name: String,
}

fn main() -> Result<(), Error> {
    let path = args().nth(1).expect("A log file path is needed");

    let keynames = (args().nth(2).expect("keyname 1 is needed"), args().nth(3).expect("keyname 2 is needed"));
    let keynames = (keynames.0.as_str(), keynames.1.as_str());

    let min_interval = args().nth(4);
    
    let min_interval = match min_interval {
        Some(interval) => {
            let i = interval.parse().expect("A number is expected");
            TimeDelta::try_seconds(i)
        }
        None => None,
    };

    let path = "E:/desktop/诊间日志记录20240517.log";
    let text = read_file(path)?;

    // check_interval(&text);
    let items = get_time_item(&text, "2024");
    check_item_interval(items, keynames, min_interval);

    Ok(())
}

// 检查时间间隔
fn check_interval(text: &str) {
    let mut stack: Vec<&str> = Vec::new();
    for line in text.lines() {
        if line.contains("2024") {
            if let Some(last) = stack.last() {
                // if !line[12..].eq(&last[12..]) {
                if !line.eq(*last) {
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

fn check_item_interval(items: Vec<Item>, keyname: (&str, &str), min_interval: Option<TimeDelta>) {
    // let mut stack = Vec::new();
    let (mut start_time, mut end_time) = ("", "");
    let mut sum = TimeDelta::seconds(0);
    let mut length = 0;

    for item in &items {
        if item.name.eq(keyname.0) {
            start_time = &item.end_time;
        } else if item.name.eq(keyname.1) {
            end_time = &item.end_time;

            let start = start_time.split(' ').last().unwrap();
            let start = NaiveTime::parse_from_str(start, "%H:%M:%S").unwrap();
            let end = end_time.split(' ').last().unwrap();
            let end = NaiveTime::parse_from_str(end, "%H:%M:%S").unwrap();
            
            let since = NaiveTime::signed_duration_since;
            let interval = since(end, start);

            if let Some(min_interval) = min_interval {
                if interval < min_interval { continue; }
            }

            sum += interval;
            length += 1;
            println!("{:?} {:?} {}", start, end, since(end, start));
        }
    }

    let average = sum / length;
    println!("Average interval is {}", average)
}

fn get_time_item(text: &str, keyword: &str) -> Vec<Item> {
    let mut time_stack: Vec<&str> = Vec::new();
    let mut item_list = Vec::new();
    for line in text.lines() {
        // 包含时间关键字
        if line.contains(keyword) {
            if time_stack.is_empty() {
                time_stack.push(line);
            } else {
                time_stack.pop();
            }
        } else {
            // 获取项目名
            if !time_stack.is_empty() {
                if let Some(last) = time_stack.last() {
                    item_list.push(Item {
                        end_time: (*last).into(),
                        name: line.into(),
                    });
                }
            }
        }
    }

    item_list
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
