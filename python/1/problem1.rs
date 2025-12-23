use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let split = contents.split("\n");
    let mut sum:i32 = 0;
    for s in split {
    	let mut input: i32 = 0;
    	if !s.is_empty() {
	    	input = s.trim().parse().expect("Wanted a number");
	    }
    	sum += input;
	}
	println!("{}", sum);
}