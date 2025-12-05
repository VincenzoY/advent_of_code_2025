use std::fs::File;
use std::io::Read;

fn main() {
    let filename = "../../input.txt";

    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => panic!("Could not open file: {}", err),
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => panic!("Could not read file: {}", err),
    }

    let ranges: Vec<&str> = contents.split(",").collect();
    let mut sum: u64 = 0;

    for range in &ranges {
        if let [first, second] = &range.split('-').collect::<Vec<&str>>()[..] {
            let first = first.parse::<u64>().unwrap();
            let second = second.parse::<u64>().unwrap();
            for cur in first..(second + 1) {
                if is_invalid_id(cur) {
                    sum += cur;
                }
            }
        }
    }

    println!("{}", sum);
}

fn is_invalid_id(id: u64) -> bool {
    let id_as_str = id.to_string();
    let id_digits = id_as_str.len();

    for pattern_digits in 1..(id_digits / 2 + 1) {
        if id_digits % pattern_digits != 0 { continue; }

        let match_value = &id_as_str[0..pattern_digits];

        for group in 0..(id_digits / pattern_digits) {
            let cur_value = &id_as_str[
                (group * pattern_digits)..
                ((group + 1) * pattern_digits)
            ];

            if match_value != cur_value { break; }
            if group + 1 == id_digits / pattern_digits { return true; }
        }
    }

    return false;
}
