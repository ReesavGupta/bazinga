mod prng;
mod platform;
mod arena;
mod matrix;

struct Matrix {
    rows : u32,
    cols : u32,
    data : Vec<f32>
}

fn main() {
    let mat: Matrix = Matrix {
        rows: 1,
        cols: 2,
        data: Vec::new(),
    };

    println!("Hello, world!");
}
