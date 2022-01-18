#[allow(unused, non_snake_case)]
pub fn T(n: f64) -> f64 {
    n * n * (2.0 * n - 1.0)
}
#[allow(unused, non_snake_case)]
pub fn D(n: f64) -> f64 {
    T(n) + 2.0 * n * n
}

#[allow(unused, non_snake_case)]
pub fn I(cond: bool) -> f64 {
    cond as i64 as f64
}

#[allow(unused, non_snake_case)]
pub fn Ln(n: f64, big_n: f64) -> f64 {
    8.0_f64.powf(big_n.log2() - n.log2())
}

/*
Temporary utils
*/
#[allow(unused)]
pub fn delta(n: f64) -> f64 {
    8.0_f64.powf(n.log2())
}

#[allow(unused)]
pub fn phi(n: f64) -> f64 {
    8.0_f64.powf(n.log2()) - 2.0_f64.powf(2.0 * n.log2() - 1.0)
}

#[allow(unused)]
pub fn lambda(n: f64) -> f64 {
    8.0_f64.powf(n.log2()) - 2.0_f64.powf(2.0 * n.log2())
}

#[allow(unused)]
pub fn omega(n: f64) -> f64 {
    8.0_f64.powf(n.log2()) - 2.0_f64.powf(2.0 * n.log2() - 1.0)
}

#[allow(unused)]
pub fn d1(n: f64) -> f64 {
    (2.0 * D(n) - (2.0 * T(n / 2.0) - 2.0 * ((n / 2.0).powf(2.0) - 1.0))).floor()
}
#[allow(unused)]
pub fn d2(n: f64) -> f64 {
    (2.0 * D(n)
        - (4.0 * T(n / 2.0) + 2.0 * (n / 2.0).powf(2.0)
            - (2.0 * (n / 2.0) - 1.0)
            - (2.0 * (n / 2.0).powf(2.0) - (n / 2.0))))
        .floor()
}
#[allow(unused)]
pub fn d3(n: f64) -> f64 {
    (D(n) - (2.0 * T(n / 2.0) - (2.0 * (n / 2.0).powf(2.0) - 1.0))).floor()
}
#[allow(unused)]
pub fn d4(n: f64) -> f64 {
    (D(n) - (2.0*(n/2.0).powf(2.0)) - (4.0 * T(n / 2.0) - 2.0 * (n / 2.0).powf(2.0) + 2.0) - ((n / 2.0).powf(2.0) - n)
        + (n / 2.0 + 1.0))
        .floor()

    // (D(n) - ((4.0*T(n/2.0) - 2.0*(n/2.0).powf(2.0)+2.0)-
    // (((n/2.0).powf(2.0)-n) + ((n/2.0) + 1.0)))).floor()
}

#[allow(unused)]
pub fn str_to_vec(input: &str) -> Vec<Vec<f64>>{
    let mut res = Vec::new();
    let split = input.split("\n").collect::<Vec<&str>>();
    for r in split{
        let mut row = Vec::new();
        let tmp = r.split_whitespace().collect::<Vec<&str>>();
        for ele in tmp{
            row.push(ele.parse::<f64>().unwrap());
        }
        res.push(row);
    }
    res
}

#[allow(unused)]
pub fn str_to_uvec(input: &str) -> Vec<Vec<usize>>{
    let mut res = Vec::new();
    let split = input.split("\n").collect::<Vec<&str>>();
    for r in split{
        let mut row = Vec::new();
        let tmp = r.split_whitespace().collect::<Vec<&str>>();
        for ele in tmp{
            row.push(ele.parse::<usize>().unwrap());
        }
        res.push(row);
    }
    res
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq};

    #[test]
    fn verify_d1() {
        let correct = vec![4.0, 38.0, 270.0, 1982.0];
        let depth = vec![1.0, 2.0, 4.0, 8.0];
        for (c, index) in correct.iter().zip(depth.iter()) {
            assert_eq!(*c, d1(*index), "input: {}", *index);
        }
    }

    #[test]
    fn verify_d3() {
        let correct = vec![2.0, 19.0, 127.0, 895.0];
        let depth = vec![1.0, 2.0, 4.0, 8.0];
        for (c, index) in correct.iter().zip(depth.iter()) {
            assert_eq!(*c, d3(*index), "input: {}", *index);
        }
    }

    #[test]
    fn verify_d4() {
        let correct = vec![17.0, 97.0, 635.0];
        let depth = vec![2.0, 4.0, 8.0];
        for (c, index) in correct.iter().zip(depth.iter()) {
            assert_eq!(*c, d4(*index), "input: {}", *index);
        }
    }

    #[test]
    fn verify_I(){
        let i = 3.0;
        assert_eq!(0.0, I(false));
        assert_eq!(1.0, I(true));
        assert_eq!(1.0, I(i<=3.0));
        assert_eq!(0.0, I(i>3.0));
    }
}
