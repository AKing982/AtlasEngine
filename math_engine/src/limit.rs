use crate::fraction::Fraction;
#[derive(Debug, Clone)]
pub(crate) struct Limit {
    pub(crate) func: Fraction,
    pub(crate) approach: f64,
}

impl Limit {
    
    pub fn new(func: Fraction, approach: f64) -> Self {
        Self {func, approach}
    }
    fn lim(&self) -> f64 {
        let h = 1e-6;
        let left = self.func.eval(self.approach - h);
        let right = self.func.eval(self.approach + h);
        (left + right) / 2.0
    }

    fn to_string(&self) -> String {
        let numerator = &self.func.numerator.expr_str;
        let denominator = &self.func.denominator.expr_str;
        format!("lim as h -> {} (({}) / ({})) = {}",self.approach, numerator, denominator, self.lim())
    }
}