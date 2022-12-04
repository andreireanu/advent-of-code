use elves::read_records;

fn main() {
    let values: Vec<Vec<i64>> = read_records("/home/andrei/Programming/Rust/advent-of-code/01/input_short");
    println!("{:?}", values);
}