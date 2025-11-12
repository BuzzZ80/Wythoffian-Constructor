mod flag;
mod renderer;

use flag::{Flag, FlagGraph};
use renderer::Renderer;

use std::{f32::consts::PI};
use nalgebra::{ArrayStorage, Const};

// -- CONFIG --
// number of dimensions for polytope
const DIM: usize = 4;
// Coxeter diagram's matrix and ringed nodes
const COXMAT: [[u8; DIM]; DIM] = [
    [1, 3, 2, 2],
    [3, 1, 4, 2],
    [2, 4, 1, 3],
    [2, 2, 3, 1],
];
const RINGS: [bool; DIM] = [true, false, false, false];


// special types used throughout the code
// constant cuz whatever :3
type Vector = nalgebra::Matrix<f32, Const<DIM>, Const<1>, ArrayStorage<f32, DIM, 1>>;
type Matrix = nalgebra::Matrix<f32, Const<DIM>, Const<DIM>, ArrayStorage<f32, DIM, DIM>>;

#[macroquad::main("wireframe")]
async fn main() {
    // attempt to load flags from cached result. otherwise, calculate them
    let flags = if let Ok(graph) = FlagGraph::deserialize(coxmat_to_name(COXMAT)) {
        println!("cached flag file found, loading...");
        graph.flags
    } else {
        println!("no cached flag file found, generating...");

        // convert coxeter matrix to matrix 
        // whose entries are cosines of angles between generators
        let dot_matrix: Matrix = Matrix::from_fn(
            |r, c| {
                if r == c {1.} // diagonals should be 1 always, since reflections are unit vectors
                else {f32::cos(PI / COXMAT[r][c] as f32)} // other entries are angles based on coxeter matrix
            }
        );

        // find mirrors, then one flag from those. use these to generate all fundamental flags
        let mirrors: Matrix = dot_matrix.cholesky().expect("invalid coxeter diagram").l().transpose();
        let start_flag = Flag::from_mirrors(mirrors);

        let flag_graph = FlagGraph::generate(start_flag, mirrors);

        // cache the result
        flag_graph.serialize(coxmat_to_name(COXMAT)).expect("couldn't cache file");

        flag_graph.flags
    };
    println!("{} flags computed", flags.len());
    

    // find polytope elements
    // vertex list without duplicates
    let mut vertices: Vec<Vector> = Vec::new();
    let mut flag_vertex: Vec<usize> = Vec::new();
    for s in flags.iter() {
        let v = s.rings_to_point(RINGS);
        if let Some((i, _)) = vertices.iter().enumerate().find(|(_, f)| (*f - v).magnitude_squared() < 0.0001) {
            flag_vertex.push(i)
        }
        else {
            vertices.push(v);
            flag_vertex.push(vertices.len() - 1);
        }
    }
    println!("{} vertices computed", vertices.len());

    //edge list without self-connections
    let mut edges: Vec<(usize, usize)> = Vec::new();
    for (s, i) in flags.iter().zip(flag_vertex.iter()) {
        flags.iter().zip(flag_vertex.iter()).filter(|(f, _)| s.compare(f) == DIM as u32 - 1).take(DIM).for_each(|(_, j)| {
            let (i, j) = {
                if i < j {(*i, *j)}
                else {(*j, *i)}
            };
            if i != j && !edges.contains(&(i, j)) {
                edges.push((i, j));
            }
        })
    }
    println!("{} edges computed", edges.len());

    // Start renderer
    println!("rendering");
    let mut renderer = Renderer::new(vertices, edges);
    loop {
        renderer.handle_controls();
        renderer.draw();
        macroquad::prelude::next_frame().await
    }
}

pub fn coxmat_to_name(m: [[u8; DIM]; DIM]) -> String {
    let mut name = String::new();
    for r in 0..DIM {
        for c in 0..r+1 {
            let n = m[r][c];
            let c = char::from_digit(n as u32, 36).expect("invalid CD (too big)");
            name.push(c);
        }
    }
    name.push_str(".flag");
    name
}
