#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal{
    Number(f64),
    String(String),
}