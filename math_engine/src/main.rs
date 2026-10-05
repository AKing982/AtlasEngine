mod vector;
mod function;
mod fraction;
mod limit;
mod derivative;
mod matrix;

use fraction::Fraction;
use limit::Limit;
use derivative::Derivative;

fn main() {
    let frac = Fraction::new(
        "3*t^3 - 2*t^2 + 1",
        "t - 2"
    );

    let lim = Limit::new(frac, 3.0);

    let derivative = Derivative::derivative(lim);

    println!("Derivative = {}", derivative);
}
