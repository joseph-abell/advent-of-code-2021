use std::fs;

pub fn main() {
    let filename = "inputs/001.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let contents = contents.split("\n");

    let mut index = 0;
    let mut n: i32 = 0;
    let mut larger_nums: Vec<i32> = Vec::new();

    for t in contents {
        let t = t.parse::<i32>().expect("Failed to parse string to integer");

        if index > 0 && t > n {
            larger_nums.push(t);
        }

        n = t;
        index = index + 1;
    }

    println!("Final Count: {}", larger_nums.len());
}
