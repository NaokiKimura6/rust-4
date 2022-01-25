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

pub fn rational_add((a, b): (i32, i32), (c, d): (i32, i32)) -> (i32, i32) {
    ((a * d + b * c) / gcd(a * d + b * c, b * d), b * d / gcd(a * d + b * c, b * d))
}
