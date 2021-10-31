use crate::util::{*};
use itertools::iproduct;

#[allow(unused,non_snake_case)]
pub fn DT2(n:usize) -> Vec<Vec<f64>>{
    let mut res = vec![vec![0.0;n];n];
    //let n = n as f64;
    /*
        Defines the barriers at which the cur formula is relevant

        #  #  #  #  b3  #  #  #  #  b4  #  #  #  #  b5  #  #  #  # 
        #  #  #  #  b3  #  #  #  #  b4  #  #  #  #  b5  #  #  #  #
        b1 b1 b1 b1 --  b1 b1 b1 b1 --  b1 b1 b1 b1 --  b1 b1 b1 b1
        #  #  #  #  b3  #  #  #  #  b4  #  #  #  #  b5  #  #  #  #
        #  #  #  #  b3  #  #  #  #  b4  #  #  #  #  b5  #  #  #  #
        #  #  #  #  b3  #  #  #  #  b4  #  #  #  #  b5  #  #  #  #
        b2 b2 b2 b2 --  b2 b2 b2 b2 --  b2 b2 b2 b2 --  b2 b2 b2 b2
        #  #  #  #  b3  #  #  #  #  b4  #  #  #  #  b5  #  #  #  #

        Can be abstracted as (zero indexed ofc):

        P0  b3  P1  b4  P2  b5  P3
        ----||------b1------||----  
        P4  b3  P5  b4  P6  b5  P7
        ----||------b2------||---- 
        P8  b3  P9  b4 P10 b5  P11
    */
    
    // let (p0_x,p0_y,p1_x,p1_y,p2_x,p2_y,p3_x,p3_y,p4_x,p4_y,p5_x,p5_y,
    //     p6_x,p6_y,p7_x,p7_y,p8_x,p8_y,p9_x,p9_y,p10_x,p10_y,p11_x,p11_y) = init_ranges(&n);
    let ranges = init_ranges(&n);
    let mut part = 0;
    for (px,py) in ranges{
        for (x,y) in iproduct!(py,px){
            let n = n as f64;
            match part {
                0 => { res[x][y] = d3(n); res[x][y+1] = d3(n) + n;},
                1 => { res[x][y] = d3(n); res[x][y+1] = d3(n) + n },
                2 => { res[x][y] = d4(n); res[x][y+1] = d4(n) + 6.0*2.0_f64.powf(n.log2()-2.0) },
                3 => { res[x][y] = d4(n); res[x][y+1] = d4(n) + 6.0*2.0_f64.powf(n.log2()-2.0) },
                4 => { res[x][y] = d3(n) + n*(n/2.0 - 1.0); res[x][y+1] = d3(n) - lambda(n); res[x][y+2] = d3(n) - lambda(n) + n},
                5 => { res[x][y] = d3(n) + n*(n/2.0 - 1.0); res[x][y+1] = d3(n) - lambda(n); res[x][y+2] = d3(n) - lambda(n) + n},
                6 => { res[x][y] = d4(n) + (n/2.0 - 1.0)*6.0*2.0_f64.powf(n.log2()-2.0); res[x][y+1] = d4(n) - omega(n); res[x][y+2] = d4(n) - omega(n) + n + 6.0*2.0_f64.powf(n.log2()-2.0)},
                7 => { res[x][y] = d4(n) + (n/2.0 - 1.0)*6.0*2.0_f64.powf(n.log2()-2.0); res[x][y+1] = d4(n) - omega(n); res[x][y+2] = d4(n) - omega(n) + n + 6.0*2.0_f64.powf(n.log2()-2.0)},
                8 => { res[x][y] = d3(n) - lambda(n) + n*(n/2.0 -1.0) },
                9 => { res[x][y] = d3(n) - lambda(n) + n*(n/2.0 -1.0) },
                10 => { res[x][y] = d4(n) - omega(n) + (n/2.0 - 1.0) + 6.0*2.0_f64.powf(n.log2()-2.0) }, 
                11 => { res[x][y] = d4(n) - omega(n) + (n/2.0 - 1.0) + 6.0*2.0_f64.powf(n.log2()-2.0) },
                _ => break,
            }
        }
        part += 1;
    }
    
    
    
    res
}

use std::ops::Range;
#[allow(unused)]
// pub fn init_ranges(n:&usize) -> (Range<usize>,Range<usize>,Range<usize>,Range<usize>,
//                                 Range<usize>,Range<usize>,Range<usize>,Range<usize>,
//                                 Range<usize>,Range<usize>,Range<usize>,Range<usize>,
//                                 Range<usize>,Range<usize>,Range<usize>,Range<usize>,
//                                 Range<usize>,Range<usize>,Range<usize>,Range<usize>,
//                                 Range<usize>,Range<usize>,Range<usize>,Range<usize>){
pub fn init_ranges(n:&usize) -> Vec<(Range<usize>,Range<usize>)>{
    let n = *n;
    let (b1,b2,b3,b4,b5) = (n/3,5*n/6,n/4,n/2,3*n/4);
    let mut res = Vec::new();

    let p0_x = 0..b3;
    let p0_y = 0..b1;
    res.push((p0_x,p0_y));

    let p1_x = b3..b4;
    let p1_y = 0..b1;
    res.push((p1_x,p1_y));

    let p2_x = b4..b5;
    let p2_y = 0..b1;
    res.push((p2_x,p2_y));

    let p3_x = b5..n;
    let p3_y = 0..b1;
    res.push((p3_x,p3_y));

    let p4_x = 0..b3;
    let p4_y = b1..b2;
    res.push((p4_x,p4_y));

    let p5_x = b3..b4;
    let p5_y = b1..b2;
    res.push((p5_x,p5_y));

    let p6_x = b4..b5;
    let p6_y = b1..b2;
    res.push((p6_x,p6_y));

    let p7_x = b5..n;
    let p7_y = b1..b2;
    res.push((p7_x,p7_y));

    let p8_x = 0..b3;
    let p8_y = b2..n;
    res.push((p8_x,p8_y));

    let p9_x = b3..b4;
    let p9_y = b2..n;
    res.push((p9_x,p9_y));

    let p10_x = b4..b5;
    let p10_y = b2..n;
    res.push((p10_x,p10_y));

    let p11_x = b5..n;
    let p11_y = b2..n;
    res.push((p11_x,p11_y));

    res

    // (p0_x,p0_y,p1_x,p1_y,p2_x,p2_y,p3_x,p3_y,p4_x,p4_y,p5_x,p5_y,
    // p6_x,p6_y,p7_x,p7_y,p8_x,p8_y,p9_x,p9_y,p10_x,p10_y,p11_x,p11_y)
}

// pub fn init_ranges(n:&usize) -> (Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,
//     Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,
//     Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,
//     Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,
//     Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,
//     Vec<usize>,Vec<usize>,Vec<usize>,Vec<usize>,){
// let n = *n;
// let (b1,b2,b3,b4,b5) = (n/3,5*n/6,n/4,n/2,3*n/4);

// let p0_x = (0..b3).collect();//vec![0..b3].collect();
// let p0_y = (0..b1).collect();//vec![0..b1];

// let p1_x = (b3..b4).collect();//vec![b3..b4];
// let p1_y = (0..b1).collect();//vec![0..b1];

// let p2_x = (b4..b5).collect();//vec![b4..b5];
// let p2_y = (0..b1).collect();//vec![0..b1];

// let p3_x = (b5..n).collect();//vec![b5..n];
// let p3_y = (0..b1).collect();//vec![0..b1];

// let p4_x = (0..b3).collect();//vec![0..b3];
// let p4_y = (b1..b2).collect();//vec![b1..b2];

// let p5_x = ().collect();//vec![b3..b4];
// let p5_y = ().collect();//vec![b1..b2];

// let p6_x = ().collect();//vec![b4..b5];
// let p6_y = ().collect();//vec![b1..b2];

// let p7_x = ().collect();//vec![b5..n];
// let p7_y = ().collect();//vec![b1..b2];

// let p8_x = ().collect();//vec![0..b3];
// let p8_y = ().collect();//vec![b2..n];

// let p9_x = ().collect();//vec![b3..b4];
// let p9_y = ().collect();//vec![b2..n];

// let p10_x = ().collect();//vec![b4..b5];
// let p10_y = ().collect();//vec![b2..n];

// let p11_x = ().collect();//vec![b5..n];
// let p11_y = ().collect();//vec![b2..n];

// (p0_x,p0_y,p1_x,p1_y,p2_x,p2_y,p3_x,p3_y,p4_x,p4_y,p5_x,p5_y,
// p6_x,p6_y,p7_x,p7_y,p8_x,p8_y,p9_x,p9_y,p10_x,p10_y,p11_x,p11_y)
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn check_range() {
        
    }

}