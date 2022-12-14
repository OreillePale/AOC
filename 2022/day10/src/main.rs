use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn print_CRT(crt: &Vec<char>){
    let mut ctn = 0;
    for i in 0..crt.len(){
        let c = crt[i];
        print!("{c}");
        
        ctn += 1;
        if ctn == 40{
            ctn = 0;
            println!("");
        }
    }
}

fn solve(lines: &Vec<String>) -> i32{
    let mut X = 1;
    let mut n: i32;
    let mut dcycles = 0;
    let mut current_cycle = 1;
    let mut j = 0;
    let mut tot_strength: i32 = 0;
    let mut pixel = 0;

    let mut CRT: Vec<char> = Vec::new();
    for i in 0..40*6{
        CRT.push(' ');
    }

    for line in lines{
        if line.eq("noop"){
            dcycles = 1;
            n = 0;
        }
        else{
            let parts = line.split(" ").collect::<Vec<&str>>();
            n = parts[1].parse().unwrap();
            dcycles = 2;
        }

        for _ in 0..dcycles as usize{
            // part 1
            if current_cycle == 20+40*j{
                j += 1;
                tot_strength += current_cycle*X;
            }

            // part 2
            let pos: usize = (pixel % 240).try_into().unwrap();
            if pixel % 40 >= X-1 && pixel % 40 <= X+1{
                CRT[pos] = '#';
            }
            pixel += 1;

            // end of cycle
            current_cycle += 1;
        }

        X += n;
    }

    print_CRT(&CRT);
    tot_strength
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

    // solve
    let ans1 = solve(&lines);
    println!("");
    println!("Answer 1 = {ans1}");
}
