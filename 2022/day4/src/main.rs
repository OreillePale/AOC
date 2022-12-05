use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// look for an excuse to implement classes
struct Interval{
    a: i32,
    b: i32
}

impl Interval{
    fn contains(&self, other: &Interval) -> bool{
        self.a <= other.a && self.b >= other.b
    }

    fn overlaps(&self, other: &Interval) -> bool{
        !(self.a > other.b || other.a > self.b)
    }

    fn from_problem_string(sstr: &str) -> Interval{
        let parts: Vec<&str> = sstr.split("-").collect();
        let a: i32 = parts[0].parse().unwrap();
        let b: i32 = parts[1].parse().unwrap();

        Interval{a:a, b:b}
    }
}

fn main(){
    // read command line arguments
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("** opening {path}");

    // read file
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    // loop over lines
    let mut n1: u32 = 0;
    let mut n2: u32 = 0;
    for line in lines{
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(",").collect();
        let x = &parts[0];
        let y = &parts[1];
        let a = Interval::from_problem_string(&parts[0]);
        let b = Interval::from_problem_string(&parts[1]);

        if a.contains(&b) || b.contains(&a){
            n1 += 1;
        }

        if a.overlaps(&b){
            n2 += 1;
        }
    }

    println!("Answer 1: {n1}");
    println!("Answer 2: {n2}");
}