use crate::complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Complex(Complex),
}