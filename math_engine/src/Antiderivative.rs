use std::f64::consts::E;
struct Antiderivative;

impl Antiderivative {
    pub(crate) fn power_rule(k: f64, n: f64, x: f64) -> f64 {
        if n == -1.0 {
            return k * x.abs().ln();
        }
        k / (n + 1.0) * x.powf(n + 1.0)
    }

    pub(crate) fn exp_rule(x: f64) -> f64 {
        E.powf(x)
    }

    pub(crate) fn constant(c: f64) -> String {
        "c*x + C".to_string()
    }

    pub(crate) fn constant_eval(c: f64, x:f64) -> f64 {
        c * x
    }

    pub(crate) fn cos_eval(x: f64, k: f64) -> f64 {
        if k == 1.0 {
            x.sin()
        }else if k == 2.0{
            return x / 2.0 + (2.0 * x).sin() / 4.0;
        }else {
            let term1 = x.sin() * x.cos().powf(k - 1.0) / k;
            let term2 = (k - 1.0) / k * Self::cos_eval(x, k - 2.0);
            return term1 + term2;
        }
    }

    pub(crate) fn sin_eval(x: f64, k: f64) -> f64 {
        if k == 1.0 {
            -x.cos()
        }
        else if k == 2{
            return x / 2.0 - (2.0 * x).sin() / 4.0;
        }else {
            let term1 = -(x.sin().powf(k - 1.0) * x.cos()) / k;
            let term2
        }
    }

    pub(crate) fn sin(k: f64) -> String {
        if k == 1.0 {
            return "-cos(x) + C".to_string();
        }else if k == 2.0 {
            return "(x / 2) - (sin(2x) / 4) + C".to_string();
        }else {
            return format!(
                "-(sin^{:.2}(x) * cos(x) / {:.2}) + (({:.2} - 1) / {:.2}) * ∫ sin^{:.2}(x) dx + C",
                k - 1.0, k, k, k, k - 2.0
            );
        }
    }

    pub(crate) fn secant(k: f64) -> String {
        if k == 1.0 {
            "ln|sec(x) + tan(x)| + C".to_string()
        }else if k == 2.0 {
            return "tan(x) + C".to_string();
        }else {
            return format!(
                "[sec^{:.2}(x) * tan(x) / ({:.2} - 1)] + [({:.2}-2)/({:.2}-1) * ∫ sec^({:.2}-2)(x) dx] + C",
                k - 2.0, k, k, k, k - 2.0
            );
        }
    }

    pub(crate) fn secant_eval(x: f64, k: f64) -> f64 {
        if k == 1.0 {
            return (1.0 / x.cos()).abs().ln() + x.tan().abs().ln();
        }else if k == 2.0 {
            return x.tan();
        }else {
            let term1 = (1.0 / x.cos()).powf(k - 2.0) * x.tan() / (k - 1.0);
            let term2 = (k - 2.0) / (k - 1.0) * Self::secant_eval(x, k - 2.0);
            return term1 + term2;
        }
    }

    pub(crate) fn tan(k: f64) -> String {
        if k == 1.0 {
            return "-ln|cos(x)| + C".to_string();
        }else if k == 2.0 {
            return "tan(x) - x + C".to_string();
        }else {
            return format!(
                "(tan^{:.2}(x) / ({:.2} - 1)) - ∫ tan^{:.2}(x) dx + C",
                k - 1.0, k, k - 2.0
            );
        }
    }

    pub(crate) fn tan_eval(x: f64, k: f64) -> f64 {
        if k == 1.0 {
            -(x.cos().abs().ln())
        }else if k == 2.0 {
            return x.tan() - x;
        }else {
            let term1 = x.tan().powf(k - 1.0) / (k - 1.0);
            let term2 = Self::tan_eval(x, k - 2.0);
            return term1 - term2;
        }
    }

    pub(crate) fn ln() -> String {
        return "x*ln(x)-x + C".to_string()
    }

    pub(crate) fn ln_eval(x: f64) -> f64 {
        if x <= 0.0 {
            panic!("ln(x) is undefined for x <= 0");
        }
        x * x.ln() - x
    }

    pub(crate) fn u_rule(f: impl Fn(f64) -> f64,
                         u: impl Fn(f64) -> f64,
                         du_dx: impl Fn(f64) -> f64,
                         x: f64) -> f64 {


    }


}