use crate::util::*;

/*
    (29)
*/
#[allow(unused)]
pub fn my_atn(n: usize) -> f64 {
    let n = n as f64;

    let t1 = T(n);
    let t2 = 2.0 * n * n;
    let t3 = 8.0 * T(n / 2.0);
    let mut t4 = 0.0;
    let mut count = n;
    let mut i = 1.0;
    while count > 1.0 {
        t4 += 2.0 * T(2.0_f64.powf(i - 1.0));
        count /= 2.0;
        i += 1.0;
    }
    t1 + t2 + t3 - t4
}

/*
    (29)
*/
#[allow(unused)]
pub fn atn(i: usize, j: usize, n: usize) -> f64 {
    let mut i = i as f64;
    let mut j = j as f64;
    let n = n as f64;
    let mut t: f64 = my_atn(n as usize); //0.0;
    let mut n_temp: f64 = n as f64;

    if j > n {
        t = n * n;
    }
    while n_temp > 1.0 {
        if j > n_temp {
            t += n * n;
        }
        if i > n_temp {
            t += 2.0 * (n / 2.0) * (n / 2.0)
        }
        j = ((j - 1.0) % n_temp) + 1.0;
        i = ((i - 1.0) % n_temp) + 1.0;
        n_temp /= 2.0;
    }
    t
}

/*
    (30) 
*/
#[allow(unused)]
pub fn atn_c(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let mut n = n as f64;
    let (mut t1, mut t2) = (0.0, 0.0);
    let mut c = 0.0;
    t1 += (2.0_f64.powf(c).powf(2.0))
        * I((j - 1.0 % 2.0 * n) % 2.0_f64.powf(n + 1.0) + 1.0 > 2.0_f64.powf(n));
    while n > 1.0 {
        t1 += (2.0_f64.powf(c).powf(2.0))
            * I((j - 1.0 % 2.0 * n) % 2.0_f64.powf(c + 1.0) + 1.0 > 2.0_f64.powf(c));
        t2 += (2.0_f64.powf(c).powf(2.0))
            * I((i - 1.0 % 2.0 * n) % 2.0_f64.powf(c + 1.0) + 1.0 > 2.0_f64.powf(c));
        n -= 1.0;
        c += 1.0;
    }
    t1 + t2
}

/*
    (23)
*/
#[allow(unused, non_snake_case)]
pub fn F_A(i: usize, j: usize, n: usize) -> f64 {
    f_T(i, j, n) + f_AB(i as f64, j as f64, n as f64)
}

/*
    (31)
*/
#[allow(unused, non_snake_case)]
pub fn f_T(i: usize, j: usize, n: usize) -> f64 {
    my_atn(n) + atn_c(i, j, n)
}

/*
    (41)
*/
#[allow(unused, non_snake_case)]
pub fn f_AB(i: f64, j: f64, n: f64) -> f64 {
    if n < 1.0 {
        return 0.0;
    }

    let t1 = 4.0*n*n * (n/2.0).powf(2.0) * I((n/4.0 < i && i <= n/2.0) && j <= n/2.0);
    let t2 =
        2.0 * (n / 2.0).powf(2.0) * I((n / 4.0 < i && i <= n / 2.0) && (n / 2.0 < j && j <= n));
    let t3 =
        (n / 2.0).powf(2.0) * I((n / 2.0 < i && i <= 3.0 * n / 4.0) && (n / 2.0 < j && j <= n));
    let t4 = 2.0 * (n / 2.0).powf(2.0) * I((n / 2.0 < i && i <= 3.0 * n / 4.0) && (j <= n / 2.0));
    let t5 = ((n / 2.0).powf(2.0) + f_AB(i - n / 2.0, j, n / 2.0))
        * I(i > 3.0 * n / 4.0 && j <= n / 2.0);
    let t6 = f_AB(i - n / 2.0, j - n / 2.0, n / 2.0) * I(i > 3.0 * n / 4.0 && j > n / 2.0);
    let t7 = ((n / 2.0).powf(2.0) + f_AB(i, j - n / 2.0, n / 2.0)) * I(i < n / 4.0 && j > n / 2.0);
    let t8 = f_AB(i, j, n / 2.0) * I(i < n / 4.0 && j <= n / 2.0);

    t1 + t2 + t3 + t4 + t5 + t6 + t7 + t8
}

/*
*/

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq};
    #[test]
    fn test_atn() {
        for i in 0..50 {
            assert_eq!(atn(0, 0, i), my_atn(i));
        }
    }

    //atn (29)
    #[test]
    fn verify_29(){
        let gt = vec![3.0,26.0,214.0,1734.0];
        let depth = vec![1,2,4,8];
        for (t,n) in gt.iter().zip(depth.iter()){
            assert_eq!(atn(0,0,*n), *t);
        }
    }

    #[test]
    fn verify_table() {
        let n = vec![1.0, 2.0, 4.0, 8.0];
        let tn_correct = vec![1.0, 12.0, 112.0, 960.0];
        let dn_correct = vec![3.0, 20.0, 144.0, 1088.0];
        for (i, val) in n.iter().enumerate() {
            assert_eq!(T(*val), tn_correct[i]);
            assert_eq!(D(*val), dn_correct[i]);
        }
    }

    // #[test]
    // fn verify_atn_size_1() {
    //     let gt = vec![3.0,4.0];
    //     let res = 
    // }

    // #[test]
    // fn verify_atn_size_2() {
        
    // }

    // #[test]
    // fn verify_atn_size_4() {
        
    // }

    // #[test]
    // fn verify_atn_size_8() {
        
    // }
}
