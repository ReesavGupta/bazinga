use crate::arena::Arena;
use crate::prng::Pcg32;
pub struct Matrix<'a> {
    rows: usize,
    cols: usize,
    pub data: &'a mut [f32]
}

impl<'a> Matrix<'a> {
    pub fn fill_rand(&mut self, rng: &mut Pcg32 ) {
        for val in self.data.iter_mut() {
            let next_f32 = 2.0 * rng.next_f32() - 1.0;
            *val = next_f32;
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        let index = row * self.cols + col;
        self.data[index]
    }

    pub fn set(&mut self, row: usize, col: usize, val:f32) {
        let index = row * self.cols + col;
        self.data[index] = val;
    }

    pub fn sigmoid(&mut self) {
        for val in self.data.iter_mut() {
            *val = 1.0 / (1.0 + (-*val).exp());
        }
    }
}

pub fn matrix_alloc<'a>(arena: &mut Arena, rows: usize, cols: usize) -> Matrix<'a> {
    let count = rows * cols;
    let bytes = count * std::mem::size_of::<f32>();
    
    let ptr = arena.push_zeroed(bytes);

    if ptr.is_null() {
        panic!("Out of memory in Arena!");
    }

    let slice = unsafe{
        std::slice::from_raw_parts_mut(ptr as *mut f32, count)
    };

    Matrix { 
        rows,
        cols, 
        data: slice,
    }
}

pub fn mat_mul<'a>(arena: &mut Arena, a: &Matrix, b: &Matrix) -> Matrix<'a>{
    assert_eq!(a.cols, b.rows, "matrix dimensions do not match for multiplication :(");
    let mut res = matrix_alloc(arena, a.rows, b.cols);

    for i in 0..a.rows {
        for j in 0..b.cols {
            let mut sum = 0.0;
            // sum += A[i, k] * B[k, j]
            for k in 0..a.cols {
                sum += a.get(i, k) * b.get(k, j)
            }
            res.set(i, j, sum);
        }
    }    
    res
}

pub fn mat_add<'a>(arena: &mut Arena, a: &Matrix, b: &Matrix) -> Matrix<'a> {
    assert_eq!(a.cols, b.cols, "matrix cols are not matching for addition");
    assert_eq!(a.rows, b.rows, "matrix rows are not matching for addition");

    let res = matrix_alloc(arena, a.rows, a.cols);
    
    for i in 0..a.data.len() {
        res.data[i] = a.data[i] + b.data[i];
    }

    res
}

pub fn mat_mse(pred: &Matrix, target: &Matrix) -> f32 {
    assert_eq!(pred.rows, target.rows);
    assert_eq!(pred.cols, target.cols);

    let mut error = 0.0;
    let n = (pred.rows * pred.cols) as f32;

    for i in 0..pred.data.len() {
        let diff = pred.data[i] - target.data[i];
        error += diff * diff;
    }

    error / n
}

// hen we "nudge" a weight to see if the error goes down, we need to be able to undo that nudge if the error actually gets worse. 
// To do that, we need to copy the state of our weights.
pub fn mat_copy(dst: &mut Matrix, src: &Matrix) {
    assert_eq!(dst.rows, src.rows);
    assert_eq!(dst.cols, src.cols);
    for i in 0..src.data.len() {
        dst.data[i] = src.data[i];
    }
}
