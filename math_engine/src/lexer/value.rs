use crate::complex::Complex;
use crate::vector::Vector;
use crate::matrix::Matrix;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Complex(Complex),
    Vector(Vector),
    Matrix(Matrix)
}