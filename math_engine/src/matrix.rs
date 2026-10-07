use crate::vector::Vector;

#[derive(Debug, Clone)]
struct Matrix {
    rows: usize,
    cols: usize,
    matrix: Vec<Vec<f64>>,
}

impl Matrix {

    pub(crate) fn new(cols: usize, rows: usize, fill: f64) -> Self {
        Self {
            matrix: vec![vec![fill; cols]; rows],
            rows,
            cols,
        }
    }

    pub(crate) fn get(&self, row: usize, col: usize) -> f64 {
        self.matrix[row][col]
    }

    pub(crate) fn rows(&self) -> usize{
        self.matrix.len()
    }

    pub(crate) fn cols(&self) -> usize {
        self.matrix.first().map_or(0, |r| r.len())
    }

    pub(crate) fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub(crate) fn swap(&self, row: usize, col: usize, element: f64) -> Self {
        let mut modified = self.matrix.clone();
        modified[row][col] = element;
        Self {
            matrix: modified,
            cols: self.cols,
            rows: self.rows,
        }
    }

    pub(crate) fn subtract(&self, other: &Self) -> Self {
        if self.size() != other.size(){
            panic!("Cannot perform matrix subtraction on matrices of different sizes");
        }

        let subtracted_data: Vec<Vec<f64>> = self.matrix
            .iter()
            .zip(other.matrix.iter())
            .map(|(row_a, row_b)| {
                row_a.iter()
                    .zip(row_b.iter())
                    .map(|(&a, &b)| a - b)
                    .collect()
            })
            .collect();
        Self {
            matrix: subtracted_data,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        if self.size() != other.size() {
            panic!("Cannot perform matrix addition on matrices of different sizes");
        }

        // sum each row component-wise
        let summed_rows: Vec<Vec<f64>> = self.matrix
            .iter()
            .zip(other.matrix.iter())
            .map(|(row_a,row_b)|{
                row_a.iter()
                    .zip(row_b.iter())
                    .map(|(&a, &b)| a + b)
                    .collect()
            })
            .collect();

        // reconstruct a new matrix with the new added components for each entry
        Self {
            matrix: summed_rows,
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub(crate) fn trace(&self) -> f64 {
        let rows = self.matrix.len();
        let cols = self.matrix[0].len();
        if rows != cols {
            panic!("Cannot perform the trace on a non square matrix");
        }
        let mut sum = 0.0;
        for i in 0..rows {
            sum += self.matrix[i][i];
        }
        sum
    }

    pub(crate) fn diagonal(&self, is_left: bool) -> f64 {
        let rows = self.matrix.len();
        let cols = self.matrix[0].len();
        if rows != cols || rows < 2{
            panic!("Cannot perform diagonal operation on square matrix of dimension less than 2");
        }
        let mut product: f64 = 1.0;
        if is_left {
            for i in 0..rows {
                product *= self.matrix[i][i]
            }
        }else{
            for i in 0..rows {
                product *= self.matrix[i][cols - i - 1];
            }
        }
        product
    }

    pub(crate) fn determinant(&self) -> f64 {
        let rows = self.size().0;
        let cols = self.size().1;
        if rows != cols {
            panic!("Unable to perform determinant operation on non-square matrix");
        }
        if rows == 2 {
            let main_diagonal = self.diagonal(true);
            let anti_diagonal = self.diagonal(false);
            return main_diagonal - anti_diagonal;
        }
        else {
            //TODO: Write out code that uses the cofactor expansion for the general n x n case.

        }
        panic!("Determinant calculation only implemented for 2x2 matrices");
    }


    pub(crate) fn multiply(&self, other: &Self) -> Self {
        if self.cols != other.rows {
            panic!("Matrix multiplication not possible: left.cols ({}) != right.rows ({})", self.cols, other.rows);
        }
        let mut result = vec![vec![0.0; other.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    let a = self.matrix[i][j];
                    let b = other.matrix[k][j];
                    sum += a * b;
                }
                result[i][j] = sum;
            }
        }
        Self {
            rows: self.rows,
            cols: other.cols,
            matrix: result,
        }
    }

    pub(crate) fn inverse(&self) -> Self {
        let rows = self.size().0;
        let cols = self.size().1;
        if rows != cols {
            panic!("Unable to perform inversion of a non square matrix");
        }

        if rows == 2 {
            // first get the determinant
            let determinant = self.determinant();
            if determinant == 0.0 {
                panic!("Determinant is zero. Cannot calculate the inverse matrix.");
            }
            let one_over_determinant = (1.0 / determinant);
            let a = self.matrix[0][0];
            let b = self.matrix[0][1];
            let c = self.matrix[1][0];
            let d = self.matrix[1][1];
            let inverse_matrix = vec![vec![d * one_over_determinant, -b * one_over_determinant,
            ],
                                      vec![-c * one_over_determinant, a * one_over_determinant, ], ];
            return Self {
                rows: 2,
                cols: 2,
                matrix: inverse_matrix,
            };
        }
        panic!("No implementation for n x n matrices greater than n = 2 yet.");
    }

    pub(crate) fn eye(dim: usize) -> Self {
        if dim < 0 || dim == 0{
            panic!("The dimension must be greater than zero");
        }
        let rows = dim;
        let cols = dim;
        let mut identity: Vec<Vec<f64>> = Vec::with_capacity(rows);
        for i in 0..rows {
            let mut row = Vec::with_capacity(cols);
            for j in 0..cols {
                if i == j {
                    row.push(1.0);
                }else{
                    row.push(0.0);
                }
            }
            identity.push(row);
        }
        Self {
            cols,
            rows,
            matrix: identity
        }
    }

    pub(crate) fn is_square(&self) -> bool {
        return self.rows == self.cols
    }

    pub(crate) fn transpose(&self) -> Self {
        let size = self.size();
        let rows = size.0;
        let cols = size.1;
        if (rows < 0 && cols < 0) || (rows == 1 && cols == 1) {
            panic!("Unable to perform transpose operation on matrix of dimension less than zero or equal to one");
        }

        for i in 0..rows {
            for j in 0..cols {

            }
        }

        if rows == 2 {
            let a = self.matrix[0][0];
            let b = self.matrix[0][1];
            let c = self.matrix[1][0];
            let d = self.matrix[0][0];
            let transposed_matrix: Vec<Vec<f64>> = vec![vec![a, c,], vec![b, d],];
            return Self {
                matrix: transposed_matrix,
                rows: self.rows,
                cols: self.cols,
            }
        }
        panic!("No implementation for transpose for dimensions n > 2 yet");
    }

    pub(crate) fn is_orthogonal(&self) -> bool {
        if !self.is_square() {
            panic!("Unable to perform operation on a non square matrix");
        }


    }

    pub(crate) fn print_size(&self) {
        println!("({}, {})", self.rows, self.cols);
    }

    pub(crate) fn print(&self) {
        print!("[");
        for i in 0..self.rows {
            print!("[");
            for j in 0..self.cols {
                print!("{}", self.matrix[i][j]);
                if j < self.cols - 1 {
                    print!(", ");
                }
            }
            print!("]");
            if i < self.rows - 1 {
                println!(",");
            }
        }
        println!("]");
    }
}