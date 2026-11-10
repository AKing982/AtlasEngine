use crate::complex::Complex;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal{
    Number(f64),
    String(String),
    Nil(),
    Boolean(bool),
    Complex(Complex),
}