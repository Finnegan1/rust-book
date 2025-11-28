fn main() {
    let numbers: [u64; 5] = [3, 1, 3, 5, 4];

    let i = if numbers.len() < 5 { 3 } else { 4 };

    for element in numbers.iter() {
        println!("{}", element)
    }

    let mut input = String::new();

    println!("Please input number index.");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Reading input failed");

    let input: usize = input.trim().parse().expect("input was no number");

    let output = numbers[input];
    println!("{}", output)
}
