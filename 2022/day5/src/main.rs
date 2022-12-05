use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// another excuse to play with classes
struct Instruction{
    mmove: u32,
    from: usize,
    to: usize
}

fn parse_input(lines: &Vec<String>) -> (Vec<Vec<char>>, Vec<Instruction>){
    let n_stacks = (lines[0].len()+1)/4;
    println!("n_stacks = {n_stacks}");
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); n_stacks];
    let mut instructions: Vec<Instruction> = Vec::new();
    
    for line in lines{
        if line.contains("["){ // this is a stack line
            for i in 0..n_stacks{
                let c = line.chars().nth(4*i+1).unwrap();
                if c != ' '{
                    stacks[i].push(c);
                }
            }
        }
        else if line.contains("move"){ // this is an instruction line
            let args: Vec<&str> = line.split(" ").collect();
            let mmove: u32 = args[1].parse().unwrap();
            let mut from: usize = args[3].parse().unwrap();
            let mut to: usize = args[5].parse().unwrap();
            from -= 1;
            to -= 1;
            
            instructions.push(Instruction{mmove:mmove,from:from,to:to});
        }
    }

    // reverse stack order and return
    for i in 0..stacks.len(){
        stacks[i].reverse();
    }
    (stacks, instructions)
}

fn mover9000(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>){
    for inst in instructions{
        for i in 0..inst.mmove{
            let llen = stacks[inst.from].len();
            if llen > 0{
                let c = stacks[inst.from].remove(llen-1);
                stacks[inst.to].push(c);
            }
        }
    }
}

fn mover9001(stacks: &mut Vec<Vec<char>>, instructions: &Vec<Instruction>){
    for inst in instructions{
        let idx = stacks[inst.from].len() - inst.mmove as usize;
        for _i in 0..inst.mmove as usize{
            let c = stacks[inst.from].remove(idx);
            stacks[inst.to].push(c);
        }
    }
}

fn extract_sol(stacks: &Vec<Vec<char>>) -> String{
    let mut res1: String = String::new();
    for s in stacks{
        res1.push(s[s.len()-1]);
    }
    res1
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

    // parse input
    let (mut stacks, instructions) = parse_input(&lines);
    let mut stacks2 = stacks.clone();

    // solve part 1
    mover9000(&mut stacks, &instructions);
    let res1 = extract_sol(&stacks);
    println!("Answer 1: {res1}");

    // solve part 2
    mover9001(&mut stacks2, &instructions);
    let res2 = extract_sol(&stacks2);
    println!("Answer 2: {res2}");
}
