use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let path = std::env::args().nth(1).expect("No path given");
    let optional_lines = read_lines(path).expect("Invalid file");
    let lines = optional_lines.map(|x| x.unwrap());
    let numbers : Vec<i32> = lines.map(|x| x.parse::<i32>().unwrap()).collect();
    
    let previous = numbers.iter();
    let current = numbers.iter().skip(1);
    let is_increased = previous.zip(current).map(|x| { let (prev, curr) = x; prev < curr} );

    let increased = is_increased.filter(|x|*x).count();

    println!("Increased: {}", increased);
}