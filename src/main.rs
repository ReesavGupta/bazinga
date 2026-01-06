mod prng;
mod platform;
mod arena;
mod matrix;

use crate::arena::Arena;
use crate::prng::Pcg32;
use crate::matrix::{Matrix, mat_add, mat_mul, matrix_alloc};

const MI_B:u64 = 1024 * 1024;

fn main() {
    let mut arena = Arena::new(65 * MI_B as usize, 32 * MI_B as usize).expect("Arena could not be created");    
    let mut rng = Pcg32::new(42, 54);
    
    let mut input = matrix_alloc(&mut arena, 1, 2);
    let mut weights = matrix_alloc(&mut arena, 2, 2);
    let mut bias = matrix_alloc(&mut arena, 1, 2);

    input.fill_rand(&mut rng);
    weights.fill_rand(&mut rng);
    bias.fill_rand(&mut rng);

    let dot = mat_mul(&mut arena, &input, &weights);
    let mut activated = mat_add(&mut arena, &dot, &bias);
    activated.sigmoid();

    println!("input: {:?}", input.data);
    println!("weights: {:?}", weights.data);
    println!("bias: {:?}", bias.data);
    println!("forward pass result : {:?}", activated.data)
}