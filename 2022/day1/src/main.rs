use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn max(x: u32, y: u32) -> u32{
    if x > y{
        x
    }
    else{
        y
    }
}

// naive sort, the least effective way
fn sort(v: &mut Vec<u32>){
    let mut a = 0;
    for n in 0..v.len()-1{
        if v[n] > v[n+1]{
            // switch(v, n, n+1);
            v.swap(n, n+1);
            a += 1;
        }
    }

    if a > 0{
        sort(v);
    }
}

// no need for this, new version of rust has built-in swap function
fn switch(v: &mut Vec<u32>, i: usize, j:usize){
    let vi = v[i]; 
    v[i] = v[j];
    v[j] = vi;
}

fn main() {
    // read command line arguments
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("** opening {path}");

    // read file
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    // aggregate 
    let mut n: u32 = 0;
    let mut sums: Vec<u32> = Vec::new();
    for line in lines{
        let line = line.unwrap();
        if line.eq(""){
            sums.push(n);
            n = 0;
        }
        else{
            n += line.trim().parse::<u32>().expect("Could not convert {line} to u32");
        }
    }
    sums.push(n);

    // sort vec
    sort(&mut sums);

    // output answers
    let s1 = &sums[sums.len()-1];
    let s2 = &sums[sums.len()-1] + &sums[sums.len()-2] + &sums[sums.len()-3];
    
    println!("answer part 1: {s1}");
    println!("answer part 2: {s2}");
}
