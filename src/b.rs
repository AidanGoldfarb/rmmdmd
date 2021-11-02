use crate::util::*;

#[allow(unused, non_snake_case)]
pub fn F_B(i:usize,j:usize,n:usize) -> f64{
    g_T(i,j,n) + g_AB(i as f64,j as f64,n as f64)
}


/*
    (46) careful about sum bounds
*/
#[allow(unused)]
pub fn btn(n: usize) -> f64 {
    let mut n = n as f64;

    let mut t3 = 0.0;
    let mut count = 0.0;
    while n > 2.0 {
        t3 += 4.0 * T(2.0_f64.powf(count));
        n /= 2.0;
        count += 1.0;
    }

    4.0 * T(n) + 2.0 * n * n - t3
}

/*
    (47)
*/
#[allow(unused, non_snake_case)]
pub fn t_N(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let n = n as f64;

    let mut t1 = 0.0;
    let mut t1_upper_bound = (i % (2.0 * n)).ceil() - 1.0;
    let mut count = 0.0;
    while count < t1_upper_bound {
        t1 += 4.0_f64.powf(count)
            * I((((i % (n / 2.0)) - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0 > 2.0_f64.powf(count));
        count += 1.0;
    }

    let mut t2 = 0.0;
    let mut t2_upper_bound = (j % (2.0 * n)).ceil() - 1.0;
    count = 0.0;
    while count < t2_upper_bound {
        t2 += 4.0_f64.powf(count)
            * I((((j % (n / 2.0)) - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0 > 2.0_f64.powf(count));
        count += 1.0;
    }

    t1 + t2
}

/*
    (48)
*/
#[allow(unused, non_snake_case)]
pub fn g_T(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let n = n as f64;

    let t1 = 4.0 * T(n) + 2.0 * n * n;

    let mut t2 = 0.0;
    let t2_upper_bound = n.log2() - 1.0;
    let mut k = 0.0;
    while k < t2_upper_bound {
        t2 += 4.0 * T(2.0_f64.powf(k));
        k += 1.0;
    }

    let mut t3 = 0.0;
    let t3_upper_bound = ((i % 2.0 * n).log2()).ceil() - 1.0;
    k = 0.0;
    while k < t3_upper_bound {
        t3 += 4.0_f64.powf(k)
            * I((((i % 2.0 * n) - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0 > 2.0_f64.powf(k));
        k += 1.0;
    }

    let mut t4 = 0.0;
    let t4_upper_bound = ((j % 2.0 * n).log2()).ceil() - 1.0;
    k = 0.0;
    while k < t3_upper_bound {
        t4 += 4.0_f64.powf(k)
            * I((((j % 2.0 * n) - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0 > 2.0_f64.powf(k));
        k += 1.0;
    }

    t1 + t2 + t3 + t4
}

/*
    (56)
*/
#[allow(unused, non_snake_case)]
pub fn g_AB(i: f64, j: f64, n: f64) -> f64 {
    if n < 1.0 {
        return 0.0;
    }

    let t1 = 3.0 * 2.0_f64.powf(2.0 * n.log2() + 1.0)
        + (2.0_f64.powf(2.0 * n.log2() - 1.0)) * I(i <= n / 2.0 && n / 4.0 < j && j <= n / 2.0);
    let t2 = (3.0 * 2.0_f64.powf(2.0 * n.log2() - 1.0))
        * I(i <= n / 2.0 && n / 2.0 < j && j <= 3.0 * n / 4.0);
    let t3 = (3.0 * 2.0_f64.powf(2.0 * n.log2() - 1.0))
        * I(n / 2.0 < i && i <= n && n / 4.0 < j && j <= n / 2.0);
    let t4 = (2.0_f64.powf(2.0 * n.log2() - 1.0))
        * I(n / 2.0 < i && i < n && n / 2.0 < j && j <= 3.0 * n / 4.0);
    let t5 = I(i <= n / 2.0 && j <= n / 4.0) * g_AB(i, j, n / 2.0);
    let t6 = I(n / 2.0 < i && i <= n && j <= n / 4.0) * (n * n + g_AB(i - n / 2.0, j, n / 2.0));
    let t7 =
        I(i <= n / 2.0 && 3.0 * n / 4.0 < j && j <= n) * (n * n + g_AB(i, j - n / 2.0, n / 2.0));
    let t8 = I(n / 2.0 < i && i <= n && 3.0 * n / 4.0 < j && j <= n)
        * g_AB(i - n / 2.0, j - n / 2.0, n / 2.0);

    t1 + t2 + t3 + t4 + t5 + t6 + t7 + t8
}
