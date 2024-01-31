fn main() {
    let file_path = "./input.txt";
    let contents = std::fs::read_to_string(file_path).expect("Failed to read file");
    let strings: Vec<&str> = contents.split(',').collect();

    let mut total_sum = 0;

    for str in strings {
        total_sum += hash(str);
    }

    println!("Verification number: {}", total_sum)

    // println!("{}", hash("qp-"))
}

fn hash (str: &str) -> i32 {
    let mut current_value = 0;
    for ch in str.chars() {
        current_value += ch as i32;
        current_value *= 17;
        current_value %= 256;
    }
    return current_value;
}