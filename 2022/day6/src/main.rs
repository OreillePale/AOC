use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_buffer_index(s: &String, l: usize) -> usize{
    for i in 0..s.len()-l{
        if all_different(&s[i..i+l]){
            return i+l;
        }
    }

    panic!("Could not find buffer index");
}

fn all_different(s: &str) -> bool{
    let mut ord: Vec<char> = s.chars().collect();
    //println!("{:?}", ord);
    ord.sort();
    //println!("{:?}", ord);
    for i in 0..ord.len()-1{
        if ord[i] == ord[i+1]{
            return false;
        }
    }

    true
}

fn main() {
    // read command line arguments
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("** opening {path}");

    // read file
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("could not parse line")).collect();

    let idx1 = find_buffer_index(&lines[0], 4);
    println!("Answer1: {idx1}");

    let idx2 = find_buffer_index(&lines[0], 14);
    println!("Answer2: {idx2}");


}
