pub struct Rational {
    pub numerator_min: i32,
    pub numerator_max: i32,
    pub denominator_min: i32,
    pub denominator_max: i32,
}

#[warn(dead_code)]
pub fn rational(q: &Rational) -> Vec<(i32, i32)> {
    let mut r: Vec<(i32, i32)> = vec![];

    if q.denominator_min < 1 {
        r
    } else {
        for a in q.numerator_min..q.numerator_max {
            for b in q.denominator_min..=q.denominator_max {
                let mut count = 0;
                for (c, d) in r.iter() {
                    if a * d  == b * c {
                        count = count + 1;
                    }
                }
                if count == 0 {
                    r.push((a / gcd(a, b), b / gcd(a, b)));
                }
            }
        }
        r
    }
}

pub fn gcd(a: i32, b: i32) -> i32 {
    if a % b == 0 {
        b.abs()
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: i32, b: i32) -> i32 {
    (a * b) / gcd(a, b)
}

//分数足し算
pub fn rational_add((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
    ((a * d + b * c) / gcd(a * d + b * c, b * d), b * d / gcd(a * d + b * c, b * d))
}

//分数かけ算
pub fn rational_prod((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
    ((a * c) / gcd(a * c, b * d), b * d / gcd(a * c, b * d))
}

//分数割り算
pub fn rational_div((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
    rational_prod((a, b), (d, c))
}

//小数→分数
pub fn rational_create(float: f64) -> (i32, i32) {
    if float != 0f64 {
        let mut power: i32 = 0;
        let mut f: f64 = float;
        loop {
            if (f as i32) as f64 == f || power == 9 {
                break;
            }
            power += 1;
            f *= 10f64;
        }
        (
            (f as i32) / gcd(f as i32, 10i32.pow(power as u32)),
            10i32.pow(power as u32) / gcd(f as i32, 10i32.pow(power as u32))
        )
    } else {
        (0, 1)
    }
}

//分数→小数
pub fn decimal_create((a, b): (i32, i32)) -> f64 {
    (a / gcd(a, b)) as f64 / (b / gcd(a, b)) as f64
}

//小数足し算
pub fn decimal_add(r: f64, s: f64) -> f64 {
    decimal_create(rational_add(rational_create(r), rational_create(s)))
}

//小数かけ算
pub fn decimal_prod(r: f64, s: f64) -> f64 {
    decimal_create(rational_prod(rational_create(r), rational_create(s)))
}

//小数割り算
pub fn decimal_div(r: f64, s: f64) -> f64 {
    decimal_create(rational_div(rational_create(r), rational_create(s)))
}

//根号
pub fn root_create(float: (i32, i32), root: (i32, i32)) -> ((i32, i32), String, (i32, i32)) {
    (float, String::from("^"), root)
}

//根号の積
pub fn root_mul(float: (i32, i32), (a1, b1): (i32, i32), (a2, b2): (i32, i32)) -> ((i32, i32), String, (i32, i32)) {
    root_create(float, rational_add((a1, b1), (a2, b2)))
}

//根号の積(小数化)
pub fn root_mul_decimal(float: (i32, i32), (a1, b1): (i32, i32), (a2, b2): (i32, i32)) -> f64 {
    let ((a, b), _str, (x, y)) = root_mul(float, (a1, b1), (a2, b2));
    ((a as f64).powi(x) / (b as f64).powi(x)).powf(decimal_create((1, y)))
}
