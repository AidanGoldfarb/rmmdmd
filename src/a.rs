#[allow(unused)]
pub fn atn(i:usize, j:usize, n: usize) -> f64{
    let mut i = i as f64;
    let mut j = j as f64;
    let n = n as f64;
    let mut t: f64 = 0.0;
    let mut n_temp: f64 = n as f64;

    if j > n {
        t = n*n;
    }
    while n_temp > 1.0 {
        if j > n_temp {
            t += n*n;
        }
        if i > n_temp {
            t +=  2.0 * (n/2.0)*(n/2.0)
        }
        j = ((j-1.0)%n_temp) + 1.0;
        i = ((i-1.0)%n_temp) + 1.0;
        n_temp /= 2.0;
    }
    return t;
}
#[allow(unused)]
pub fn my_atn(n: usize) -> f64{
    let n = n as f64;
    
    let t1 = t(n);
    let t2 = 2.0*n*n;
    let t3 = 8.0*t(n/2.0);
    let mut t4 = 0.0;
    let mut count = n;
    let mut i = 1.0;
    while count > 1.0 {
        t4 += 2.0*t(2.0_f64.powf(i-1.0));
        count /= 2.0;
        i += 1.0;
    }
    return t1 + t2 + t3 - t4;
}

#[allow(unused)]
fn t(n: f64) -> f64{
    return n*n*(2.0*n-1.0);
}
#[allow(unused)]
fn d(n: f64) -> f64{
    return t(n) + 2.0*n*n;
}