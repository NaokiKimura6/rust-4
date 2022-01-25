use crate::rational::Rational;

mod rational;

fn main() {
    let range = &Rational {
        numerator_min: -5,
        numerator_max: 5,
        denominator_min: 1,
        denominator_max: 5,
    };
    println!("{:?}", rational::rational(range));
    println!("{:?}", rational::rational_add((-1, 2), (1, 3)));
}
