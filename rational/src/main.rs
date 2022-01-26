use crate::rational::Rational;

mod rational;

fn main() {
    let range = &Rational {
        numerator_min: -5,
        numerator_max: 5,
        denominator_min: 1,
        denominator_max: 5,
    };
    println!("{:?}", 2f64.powf(rational::decimal_create((1, 2))));
    println!("{:?}", rational::lcm(6, 8));
    println!("{:?}", rational::rational(range));
    println!("{:?}", rational::rational_add((-1, 2), (1, 3)));
    println!("{:?}", rational::rational_create(-1.25f64));
    println!("{:?}", rational::decimal_create((1, 3)));
    println!("{:?}", rational::decimal_create(rational::rational_create(-1.25f64)));
    println!("{:?}", rational::decimal_div(1.25, 0.5));
    println!("{:?}", rational::rational_prod(rational::rational_div((1, 1), (3, 1)), (3, 1)));
    println!("{:?}", rational::root_create(rational::rational_create(1.5), rational::rational_create(0.5)));
    println!(
        "{:?}",
        rational::root_mul(rational::rational_create(2.0),
        rational::rational_create(0.25),
        rational::rational_create(0.25)
    ));
    println!(
        "{:?}",
        rational::root_mul_decimal(
            rational::rational_create(2.0),
            rational::rational_create(0.25),
            rational::rational_create(0.25)
        )
    );
    println!(
        "{:?}",
        rational::root_mul_decimal(
            rational::rational_create(2.0),
            rational::rational_create(0.5),
            rational::rational_create(0.5)
        )
    );
}
