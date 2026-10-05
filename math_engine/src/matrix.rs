use crate::vector::Vector;

struct Matrix {
    row: i32,
    col: i32,
    dim: i32,
    matrix: Vec<Vec<i32>>
}

impl Matrix {
    pub(crate) fn eye(dim: &i32) -> Self {
        if *dim < 0 {
            panic!("The dimension must be greater than zero");
        }
        let row = *dim;
        let col = *dim;
        for i in 0..row {
            for j in col {

            }
        }
    }
}