#[derive(Debug)]
struct Vector {
    components: Vec<f64>,
}

impl Vector {
    pub fn dimension(&self) -> usize {
        self.components.len()
    }

    pub fn new(components: Vec<f64>) -> Self {
        Self {components}
    }

    pub fn zeros(dimension: usize) -> Self {
        Vector {
            components: vec![0.0; dimension],
        }
    }

    pub fn add(&self, other: &Vector) -> Vector {
        if self.components.len() != other.components.len(){
            panic!("Vectors must be of the same dimension");
        }
        let summed_components: Vec<f64> = self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a,b)| a + b)
            .collect();
        Vector::new(summed_components)
    }

    pub fn difference(&self, other: &Vector) -> Vector {
        if self.components.len() != other.components.len(){
            panic!("Vectors must be of the same dimension");
        }
        let difference_components: Vec<f64> = self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a,b)| a - b)
            .collect();
        Vector::new(difference_components)
    }

    pub fn basis(dimension: i32, position: i32) -> Vector {
        if dimension <= 0 {
            panic!("Dimension must be positive");
        }
        if dimension < 0 || position >= dimension {
            panic!("Position must be between 0 and and dimension 1")
        }
        let mut v = Vector::zeros(dimension as usize);
        v.components[position as usize] = 1.0;
        v
    }

    pub fn scalar_multiply(&self, scalar: f64) -> Vector {
        if self.components.len() <= 0 {
            panic!("The vector dimension must be positive");
        }
        let result: Vec<f64> = self.components
            .iter()
            .map(|&x| x * scalar)
            .collect();
        Self::new(result)
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        if self.components.len() != other.components.len() {
            panic!("Vectors must be of the same dimension");
        }
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a,b)| a * b)
            .sum()
    }

    pub fn norm(&self) -> f64 {
        if self.dimension() > 0 {
            self.components
                .iter()
                .map(|a| a * a)
                .sum::<f64<>>()
                .sqrt()
        } else{
            0.0
        }
    }

    pub fn norm_distance(&self, other: &Vector) -> f64 {
        if self.components.len() != other.components.len(){
            panic!("Vectors must be of the same dimension");
        }
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a,b)| (a-b).powi(2))
            .sum::<f64<>>()
            .sqrt()
    }

    pub fn linear_combination(&self, other: &Self, scalar1: f64, scalar2: f64) -> Vector {
        if self.components.len() != other.components.len(){
            panic!("The vectors must have the same dimension");
        }
        let vec1 = self.scalar_multiply(scalar1);
        let vec2 = other.scalar_multiply(scalar2);
        vec1.add(&vec2)
    }
}