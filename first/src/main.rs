use std::{env, fs, i32};

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", file_path);
    println!("{}", sum_calibration_values(args.get(1).unwrap()));
}

fn sum_calibration_values(file_path: &str) -> i32 {
    let mut result: i32 = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut start: Option<char> = None;
        let mut last: Option<char> = None;
        for c in line.chars() {
            if c.is_digit(10) {
                if start.is_none() {
                    start = Some(c);
                }
                last = Some(c);
            }
        }
        let k: i32 = format!("{}{}", start.unwrap(), last.unwrap())
            .parse()
            .unwrap();
        result += k;
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = sum_calibration_values("assets/test.txt");
        assert_eq!(result, 142);
    }
}
