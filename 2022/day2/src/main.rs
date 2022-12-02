use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

// assume we only have ASCII chars
fn index_of(c: &str, v: &Vec<&str>) -> i32{
    let mut ctn: i32 = 0;
    let a = c.as_bytes();
    for i in 0..v.len(){
        let bytes = v[i].as_bytes();
        if a[0] == bytes[0] {
            return ctn;
        }
        ctn += 1;
    }
    panic!("{c} not in vector !");
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

    // points
    let mut map = HashMap::new();
    map.insert(0,3);
    map.insert(1,0);
    map.insert(2,6);
    //map.insert(-1,6);
    //map.insert(-2,0);

    // let's be naive, A,B,C = X,Y,Z = Rock,Paper,Scissors, = 0,1,2
    let elf_tags: Vec<&str> = Vec::from(["A","B","C"]);
    let me_tags : Vec<&str> = Vec::from(["X","Y","Z"]); //loose, draw, win, -1,0,+1
    let mut points: i32 = 0;
    let mut points2: i32 = 0;
    for line in lines{
        // that's part one
        let line = line.unwrap();
        let elf_i: i32 = index_of(&line[0..1], &elf_tags);
        let me_i: i32 = index_of(&line[2..3], &me_tags);
        let delta = (elf_i - me_i + 9)%3;
        //println!("delta = {delta}");

        points += map[&delta] + me_i + 1;

        // part two
        let outcome = (-me_i + 1 + 9)%3; // the difference we need to have
        //println!("outcome = {outcome}");
        let play = (elf_i-outcome+9) % 3; // how to get it
        
        let out = map[&outcome];
        let ddelta =  out + play + 1;
        points2 += ddelta;

        println!("{elf_i} {me_i} ==> {play} {outcome} ==> {out} + {play} + 1 ==> {points2}");
    } 

    println!("Part 1: {points}");
    println!("Part 2: {points2}");
    
}
