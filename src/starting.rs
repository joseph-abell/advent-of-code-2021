use std::fs;

pub fn main() {
    let filename = "inputs/_test.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let contents = contents.split("\n");
}
