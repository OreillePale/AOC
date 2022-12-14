use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Position{
    x: i32,
    y: i32
}

impl Position{
    fn copy(&self) -> Position{
        Position{x: self.x, y: self.y}
    }
    
    fn add(&mut self, rhs: &Vector){
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Vector{
    x: i32,
    y: i32
}

impl Vector{
    fn from_positions(A: &Position, B: &Position) -> Vector{
        Vector{x: A.x-B.x, y: A.y-B.y}
    }

    fn norm2(&self) -> i32{
        self.x.pow(2) + self.y.pow(2)
    }

    fn unitary(&self) -> Vector{
        let mut x = self.x as f32;
        let mut y = self.y as f32;

        let norm = (x.powf(2.)+y.powf(2.)).powf(0.5);
        x /= norm;
        y /= norm;

        Vector{x: Vector::s_ceil(x), y: Vector::s_ceil(y)} // that's what a unitary vector looks like in L1
    }

    fn sign(x: f32) -> i32{
        if x > 0.{
            return 1;
        }
        else if x < 0.{
            return -1;
        }
        0
    }

    fn s_ceil(v: f32) -> i32{
        Vector::sign(v)*v.abs().ceil() as i32
    }
}

fn solve(lines: &Vec<String>, n_nodes: usize) -> usize{
    // define hashmap for line parsing
    let mut dirs: HashMap<&str,(i32,i32)> = HashMap::new();
    dirs.entry("R").or_insert((1,0));
    dirs.entry("L").or_insert((-1,0));
    dirs.entry("U").or_insert((0,1));
    dirs.entry("D").or_insert((0,-1));

    let mut positions: HashMap<Position,u32> = HashMap::new(); // position of last node
    let mut nodes: Vec<Position> = Vec::new();
    for i in 0..n_nodes+1{
        nodes.push(Position{x:0, y:0}) // all nodes start at the same place, first "node" is the head
    }

    for line in lines{
        // parse line
        let parts = line.split(" ").collect::<Vec<&str>>();
        let (dx, dy) = dirs.get(parts[0]).unwrap(); // first movement
        let vec = Vector{x: *dx, y: *dy};
        let n: u32 = parts[1].parse().unwrap();

        // update the positions
        for i in 0..n{
            nodes[0].add(&vec);
            for j in 0..n_nodes{
                let v = Vector::from_positions(&nodes[j], &nodes[j+1]);
                if v.norm2() > 2{ 
                    let un = v.unitary();
                    nodes[j+1].add(&un);
                }
                else{
                    break
                }
            }
            // store position of last node, always
            positions.insert(nodes[nodes.len()-1].copy(), 0);
            let l = positions.keys().len();
        }
    }
    positions.keys().len()
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
    let ans1 = solve(&lines, 1);
    let ans2 = solve(&lines, 9);

    println!("ans1(1)  = {ans1}");
    println!("ans2(10) = {ans2}");
}
