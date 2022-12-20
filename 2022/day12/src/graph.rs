use std::collections::VecDeque;
use std::cmp;

// a graph is necessary f32 weigth (easier)
struct Edge{
    to: usize,
    weight: f32,
}
 
struct Node{
    edges: Vec<Edge>,
    parents: Vec<usize>,
}

impl Node{  
    fn new() -> Node{
        Node{edges: Vec::new(), parents: Vec::new()}
    }

    fn add_edge(&mut self, to: usize, weight: f32){
        self.edges.push(Edge{to: to, weight: weight});
    }
}

pub struct Graph{
    nodes: Vec<Node>
}

impl Graph{
    pub fn new() -> Graph{
        Graph{nodes: Vec::new()}
    }

    pub fn add_node(&mut self) -> usize{
        self.nodes.push(Node::new());
        self.nodes.len() - 1
    }

    pub fn add_edge(&mut self, i: usize, j: usize, weight: f32){
        self.nodes[i].edges.push(Edge{to: j, weight: weight}); // overkill if graph is symmetric
        self.nodes[j].parents.push(i);
    }

    pub fn from_matrix(mat: &Vec<Vec<Option<f32>>>) -> Graph{
        // build graph with mat.len() empty nodes
        let mut graph = Graph::new();
        for i in 0..mat.len(){
            graph.add_node();
        }

        for i in 0..mat.len(){
            for j in 0..mat[i].len(){
                if mat[i][j] != None{
                    graph.add_edge(i, j, mat[i][j].unwrap());
                }
            }
        }

        // return
        graph
    }

    pub fn dijkstra(&self, left: usize, right: usize) -> Vec<usize>{
        // create vector with costs and visited flags
        let mut costs: Vec<f32> = vec![f32::INFINITY; self.nodes.len()];
        costs[left] = 0.;

        let mut visited: Vec<bool> = vec![false; self.nodes.len()];
        let mut cid = left;

        let mut n = 0;
        while cid != right{
            // visit neighbours to current node
            for i in 0..self.nodes[cid].edges.len(){
                let r = self.nodes[cid].edges[i].to;
                let w = self.nodes[cid].edges[i].weight;
                
                // if neighbour was never visited
                if !visited[r] && costs[cid] + w < costs[r]{ 
                    costs[r] = costs[cid] + w;
                }
            }

            // set current node to visited
            visited[cid] = true;

            // find node with lowest distance for next step
            let mut ncid = None;
            for i in 0..costs.len(){
                if !visited[i] && ncid == None{
                    ncid = Some(i);
                }
                if !visited[i] && costs[i] < costs[ncid.unwrap()]{
                    ncid = Some(i);
                }
            }

            if ncid == None{
                //panic!("search stopped because all nodes were visited");
                return Vec::new();
            }
            if costs[ncid.unwrap()] == f32::INFINITY{
                //panic!("current cost is infinity");
                return Vec::new();
            }
            cid = ncid.unwrap();

            n += 1;
        }

        let mut m = 0;
        for i in 0..visited.len(){
            if visited[i]{
                m += 1;
            }
        }

        // backtrack graph
        let mut cid = right;
        let mut ret: Vec<usize> = Vec::new();
        ret.push(right);
        let mut n = 0;
        while cid != left{
            // find lowest cost node and add it to return
            let mut minid: usize = self.nodes[cid].parents[0];
            let mut mmin = costs[self.nodes[cid].parents[0]];
            for i in 1..self.nodes[cid].parents.len(){
                let l = self.nodes[cid].parents[i];
                if costs[l] < mmin{
                    mmin = costs[self.nodes[cid].parents[i]];
                    minid = l;
                }
            }
            ret.push(minid); // this works because we work on the stack ?
            cid = minid;
        }

        // return 
        ret.reverse();
        ret
    }
}