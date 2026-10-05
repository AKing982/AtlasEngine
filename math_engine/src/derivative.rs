use crate::limit::Limit;

pub(crate) struct Derivative {
    limit: Limit,
}

impl Derivative {
    pub fn derivative(lim: Limit) -> f64 {
        let h = 1e-6;
        let left = lim.func.eval(lim.approach - h);
        let right = lim.func.eval(lim.approach + h);
        (right - left) / (2.0 * h) // central difference derivative
    }
}