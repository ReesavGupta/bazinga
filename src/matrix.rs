use crate::arena::Arena;

pub struct Matrix<'a> {
    rows: usize,
    cols: usize,
    data: &'a mut [f32]
}

pub fn matrix_alloc<'a>(arena: &'a mut Arena, rows: usize, cols: usize) -> Matrix<'a> {
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