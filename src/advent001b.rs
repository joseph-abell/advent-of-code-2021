use std::fs;

pub fn main() {
    let filename = "inputs/001.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let split_text = contents.split("\n");

    let mut index = 0;
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut n3: i32 = 0;
    let mut old_total: i32 = 0;
    let mut larger_nums: Vec<i32> = Vec::new();

    for s in split_text {
        let s_num = s.parse::<i32>().expect("Failed to parse string to integer");

        if index == 0 {
            n1 = s_num;
        } else if index == 1 {
            n2 = s_num;
        } else if index == 2 {
            n3 = s_num;
            old_total = n1 + n2 + n3;
        } else {
            n1 = n2;
            n2 = n3;
            n3 = s_num;
            let total = n1 + n2 + n3;

            if total > old_total {
                larger_nums.push(total);
            }
            old_total = total;
        }

        index = index + 1;
    }

    println!("Final Count: {}", larger_nums.len());
}
