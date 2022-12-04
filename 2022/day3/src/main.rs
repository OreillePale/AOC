use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn char_value(c: &char) -> u32{
    // find right value
    let mut digits = *c as u32;
    if digits >= 65 && digits <= 90{
        digits -= 65;
        digits += 27;
    }
    else if digits >= 97 && digits <= 122{
        digits -= 96;
    }
    else{
        panic!("I should not ever receive this: {c}");
    }

    digits
}

fn count_chars(s: &str) -> Vec<u32>{
    //println!("counting chars for: {s}");

    // put zeros in array
    let mut ret: Vec<u32> = Vec::new();
    for i in 0..52{
        ret.push(0);
    }

    // fill array
    for c in s.chars(){
        let priority = char_value(&c);
        //println!("{c} is priority {priority}");
        ret[priority as usize - 1] += 1;
    }

    // return
    ret
}

fn find_common_priority(a: &Vec<u32>, b: &Vec<u32>) -> u32{
    for i in 0..a.len(){
        let ai = &a[i];
        let bi = &b[i];
        //println!("{i}: {ai} {bi}");
        if a[i] != 0 && b[i] != 0{
            return i as u32 + 1;
        }
    }

    panic!("could not find priority for shared item");
}

fn find_priority(a: &Vec<u32>) -> u32{
    for i in 0..a.len(){
        if a[i] != 0{
            return i as u32 + 1;
        }
    }
    panic!("could not find priority for shared item");
}

fn mult(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32>{
    let mut c: Vec<u32> = Vec::new();
    assert!(a.len() == b.len(), "The two vectors do not have the same size.");

    for i in 0..a.len(){
        c.push(&a[i] * &b[i]);
    }

    c

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

    // answer part 1
    let mut res1: u32 = 0;
    let mut things: Vec<Vec<u32>> = Vec::new();
    for line in lines{
        let line = line.unwrap();
        things.push(count_chars(&line));
        let a: usize = line.len()/2;
        let left = count_chars(&line[0..a]);
        let right = count_chars(&line[a..line.len()]);
        let c = mult(&left, &right);

        res1 += find_priority(&c); //find_common_priority(&left, &right);

        //break
    }

    println!("Answer 1 is {res1}");

    // answer part 2
    let mut res2: u32 = 0;
    for i in 0..things.len()/3{
        let c = mult(&things[3*i], &things[3*i+1]);
        let d = mult(&things[3*i+2], &c);

        res2 += find_priority(&d);
    }

    println!("Answer 2 is {res2}");
}
