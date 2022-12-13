use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// build a matrix starting with the input
fn build_matrix(lines: &Vec<String>) -> Vec<Vec<u32>>{
    let mut mat: Vec<Vec<u32>> = Vec::new();

    for i in 0..lines.len(){
        mat.push(Vec::new());
        for j in 0..lines[i].len(){
            mat[i].push(lines[i][j..j+1].parse().unwrap());
        }
    }

    mat
}

fn transpose(m: &Vec<Vec<u32>>) -> Vec<Vec<u32>>{
    let mut ret: Vec<Vec<u32>> = Vec::new();
    for j in 0..m[0].len(){
        ret.push(Vec::new());
        for i in 0..m.len(){
            ret[j].push(m[i][j]);
        }
    }

    ret
}

fn print_mat(m: &Vec<Vec<u32>>){
    for i in 0..m.len(){
        for j in 0..m[i].len(){
            let v = &m[i][j];
            print!("{v}");
        }
        println!("");
    }
}

fn is_visible(line: &Vec<u32>, idx: usize, ileft: usize, iright: usize, rev: bool, score: &mut u32) -> bool{
    *score = 0;
    let mut visible = true;
    // check left
    for i in ileft..iright{
        let mut k = i;
        if rev{
            k = iright-i-1;
        }
        *score += 1; // update score anyway
        if line[k] >= line[idx]{
            visible = false;
            break
        }
    }

    visible
}

fn count_trees(mat: &Vec<Vec<u32>>) -> (u32,u32){
    let mut ctn: u32 = 0;
    let tmat = transpose(&mat);
    let mut max_score: u32 = 0;
    let mut score11: u32 = 0;
    let mut score12: u32 = 0;
    let mut score21: u32 = 0;
    let mut score22: u32 = 0;
    let mut b1 = false;
    let mut b2 = false;
    let mut b3 = false;
    let mut b4 = false;
    let mut score: u32 = 0;

    // count in i direction
    for i in 1..mat.len()-1{
        for j in 1..mat[0].len()-1{
            let mut score11: u32 = 0;
            let mut score12: u32 = 0;
            let mut score21: u32 = 0;
            let mut score22: u32 = 0;
            b1 = is_visible(&mat[i], j, 0, j, true, &mut score11);
            b2 = is_visible(&mat[i], j, j+1, mat[i].len(), false, &mut score12);
            b3 = is_visible(&tmat[j], i, 0, i, true, &mut score21);
            b4 = is_visible(&tmat[j], i, i+1, tmat[j].len(), false, &mut score22);
            score = score11*score12*score21*score22;
            if score > max_score{
                max_score = score;
            }
            let m = mat[i][j];
            if b1 || b2 || b3 || b4  {
                ctn += 1;
            }
            else{
            }
        }
    }   
    // return
    (ctn + 2*(mat.len() as u32 - 1) + 2*(mat[0].len() as u32 - 1), max_score)
}

struct Score{
    height: u32,
    occurence: u32
}

fn main(){
    // read command line arguments
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    println!("** opening {path}");

    // read file
    let file = File::open(path).expect("input file not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.expect("could not parse line")).collect();

    // parse
    let mat = build_matrix(&lines);

    // count
    let (ans1, ans2) = count_trees(&mat);
    println!("Answer 1: {ans1}");
    println!("Answer 2: {ans2}");
}
