pub fn a_t_n(i:usize, j:usize, n: usize) -> f64{
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