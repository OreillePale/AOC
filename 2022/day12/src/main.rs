use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod graph;
use crate::graph::Graph;

//this will return the heigth of each pixel in a matrix
fn parse_input(lines: &Vec<String>) -> (Vec<Vec<i32>>,usize,usize){
    let mut ret: Vec<Vec<i32>> = Vec::new();
    let mut start: Option<usize> = None;
    let mut stop: Option<usize> = None;
    let mut k: usize = 0;
    for i in 0..lines.len(){
        ret.push(Vec::new());
        
        for c in lines[i].chars(){
            if c == 'S'{
                start = Some(k);
                ret[i].push('a' as i32);
            }
            else if c == 'E'{
                stop = Some(k);
                ret[i].push('z' as i32);
            }
            else{
                ret[i].push(c as i32);
            }

            k+=1;
        }
    }

    // return 
    (ret,start.unwrap(),stop.unwrap())
}

fn build_graph(mat: &Vec<Vec<i32>>) -> graph::Graph{
    let n_nodes: usize = mat[0].len()*mat.len();
    let mut m2: Vec<Vec<Option<f32>>> = vec![vec![None; n_nodes]; n_nodes];
    let deltas: Vec<(i32,i32)> = vec![(-1,0),(1,0),(0,-1),(0,1)];
    let mut k1: usize = 0;
    let mut k2: usize = 0;

    for i in 0..mat.len(){
        for j in 0..mat[0].len(){
            k1 = i*mat[0].len() + j;
            for delta in &deltas{
                let a: i32 = i as i32 + delta.0;
                let b: i32 = j as i32 + delta.1;
                if valid_index(a, b, mat.len() as i32, mat[i].len() as i32){
                    let d = mat[a as usize][b as usize] - mat[i][j];
                    k2 = (a as usize)*mat[0].len() + (b as usize);
                    if d <= 1{
                        m2[k1][k2] = Some(1.);
                    }
                }
            }
        }
    }

    Graph::from_matrix(&m2)
}

fn valid_index(i: i32, j: i32, n: i32, m: i32) -> bool{
    i >= 0 && j >= 0 && i < n && j < m
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

        let (mat, start, stop) = parse_input(&lines);

        // solve part 1
        println!("start = {start}, end = {stop}");
        let graph = build_graph(&mat);
        let route = graph.dijkstra(start, stop);
        println!("Answer 1 = {}", route.len()-1);

        // solve part 2, we can be lazy because it's rust; but there is a more efficient way of doing that
        let n = mat.len()*mat[0].len();
        let mut visited = vec![false; n];
        let mut costs = vec![0; n];
        let a = 'a' as i32;
        let mut k = 0;
        let mut fcost: Option<usize> = None;
        for i in 0..mat.len(){
            for j in 0..mat[0].len(){
                if mat[i][j] == a{
                    let k = i*mat[0].len() + j;
                    let route = graph.dijkstra(k, stop);
                    if route.len() > 0{
                        if fcost == None {
                            fcost = Some(route.len());
                        }
                        else if fcost.unwrap() > route.len(){
                            fcost = Some(route.len());
                        }
                    }
                } 
            }
        }
        println!("Anser 2 = {}", fcost.unwrap()-1);
}
