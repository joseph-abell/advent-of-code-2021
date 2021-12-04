use std::fs;
use std::str;

pub fn main() {
    let filename = "inputs/002.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let contents = contents.split("\n");

    let mut length = 0;
    let mut depth = 0;

    for rule in contents {
        let mut rule_iter = str::split_whitespace(rule);
        let text = rule_iter.next().unwrap();
        let num = rule_iter.next().unwrap();
        let n: i32 = num.parse().unwrap();

        if text.eq("forward") {
            length = length + n;
        } else if text.eq("up") {
            depth = depth - n;
        } else {
            depth = depth + n;
        }
    }

    println!("{}", length * depth);
}
