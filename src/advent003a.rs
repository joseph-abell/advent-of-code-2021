use std::fs;

pub fn main() {
    let filename = "inputs/003_test.txt";
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.split("\n");

    let mut a = String::from("");
    let mut b = String::from("");
    let mut c = String::from("");
    let mut d = String::from("");
    let mut e = String::from("");

    for line in lines {
        println!("{}", line);
        let nums = line.chars();

        nums.enumerate().for_each(|(i, num)| {
            if i == 0 {
                a.push(num);
            }
            if i == 1 {
                b.push(num);
            }
            if i == 2 {
                c.push(num);
            }
            if i == 3 {
                d.push(num);
            }
            if i == 4 {
                e.push(num);
            }
        });
    }

    // TODO: count 1s in here, use maths to figure out the other count.
    let a_g = a.scan("1");
    let b_g = b.matches("1").count() > b.matches("0").count();
    let c_g = c.matches("1").count() > c.matches("0").count();
    let d_g = d.matches("1").count() > d.matches("0").count();
    let e_g = e.matches("1").count() > e.matches("0").count();

    let g = format!("{}{}{}{}{}", a_g, b_g, c_g, d_g, e_g);

    println!("{} {}", a_g, e);
}
