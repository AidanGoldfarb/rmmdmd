use crate::util::{*};
//use itertools::iproduct;
const DEBUG:bool = false;


#[allow(unused,non_snake_case)]
pub fn DT1(n:usize) -> Vec<Vec<f64>>{
    let mut res = vec![vec![0.0;n];n];
    let ranges = init_ranges(&n);
    let n = n as f64;
    for (q,(xr,yr)) in ranges.iter().enumerate(){
        match q {
            0 => { //q1
                if DEBUG {println!("-----Q1-----");}
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        res[*i][*j] = d1(n) - *j as f64;
                    }
                }
            }
            1 => { //q2
                if DEBUG {println!("-----Q2-----");}
                let factor = d2(n) - (n/2.0)*(n/2.0);
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    let mut col = 0.0;
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        if DEBUG {println!("applying d2 - (n/2)^2 + (n-col)  to: ({},{})", i,j);}
                        let col_factor = n/2.0 - col ;
                        res[*i][*j] = factor + sum_n(n/2.0, *i) + col_factor - 1.0; 
                        col += 1.0;
                    }
                }
            }
            2 => { //q3
                if DEBUG {println!("-----Q3-----");}
                let mut row = 0;
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    let mut col = 0;
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        res[*i][*j] = d1(n) - phi(n) - col as f64;
                        col += 1;
                    }
                }
            }
            3=> { //q4
                if DEBUG {println!("-----Q4-----");}
                let factor = d2(n) - (n/2.0)*(n/2.0);
                let mut row = 0;
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    let mut col = 0.0;
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        if DEBUG {println!("applying   to: ({},{})", i,j);}
                        let col_factor = n/2.0 - col ; 
                        res[*i][*j] = factor - delta(n) + sum_n(n/2.0,row) + col_factor - 1.0;
                        col += 1.0;
                    }
                    row += 1;
                }
            }
            _ => {}
        }
    }
    res
}


#[allow(unused,non_snake_case)]
pub fn DT2(n:usize) -> Vec<Vec<f64>>{
    let mut res = vec![vec![0.0;n];n];
    let ranges = init_ranges(&n);
    let n = n as f64;
    for (q,(xr,yr)) in ranges.iter().enumerate(){
        match q {
            0 => { //q1
                if DEBUG {println!("-----Q1-----");}
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    let mut row = 0;
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        if DEBUG {println!("applying d3 + sum(n) to: ({},{})", i,j);}
                        res[*i][*j] = d3(n)+sum_n(n,row);
                    }
                    row += 1;
                }
            }
            1 => { //q2
                if DEBUG {println!("-----Q2-----");}
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    let mut row = 0;
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        if DEBUG {println!("applying d4 + sum(factor) to: ({},{})", i,j);}
                        let factor = 6.0*2.0_f64.powf(n.log2()-2.0);
                        res[*i][*j] = d4(n)+sum_n(factor,row);
                    }
                    row += 1;
                }
            }
            2 => { //q3
                if DEBUG {println!("-----Q3-----");}
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    let mut row = 0;
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        if DEBUG {println!("applying d3- lambda + sum(n) to: ({},{})", i,j);}
                        res[*i][*j] = d3(n)-lambda(n)+sum_n(n,row);
                    }
                    row += 1;
                }
            }
            3=> { //q4
                if DEBUG {println!("-----Q4-----");} 
                for i in xr.clone().collect::<Vec<usize>>().iter(){
                    let mut row = 0;
                    for j in yr.clone().collect::<Vec<usize>>().iter(){
                        if DEBUG {println!("applying d4 - omega + sum(factor) to: ({},{})", i,j);}
                        let factor = 6.0*2.0_f64.powf(n.log2()-2.0);
                        res[*i][*j] = d4(n)-omega(n)+sum_n(factor,row);
                    }
                    row += 1;
                }
            }
            _ => {}
        }
    }
    res
}






pub fn sum_n(n: f64, mut reps: usize) -> f64{
    let mut sum = 0.0;
    while reps > 0{
        sum += n as f64;
        reps -= 1;
    }
    sum
}


use std::ops::Range;
#[allow(unused)]
pub fn init_ranges(n:&usize) -> Vec<(Range<usize>,Range<usize>)>{
    let n = *n;
    let mut res = Vec::new();
    res.push(((0..n/2),(0..n/2))); //q1
    res.push(((0..n/2),(n/2..n))); //q2
    res.push(((n/2..n),(0..n/2))); //q3
    res.push(((n/2..n),(n/2..n))); //q4
    

    res

    // (p0_x,p0_y,p1_x,p1_y,p2_x,p2_y,p3_x,p3_y,p4_x,p4_y,p5_x,p5_y,
    // p6_x,p6_y,p7_x,p7_y,p8_x,p8_y,p9_x,p9_y,p10_x,p10_y,p11_x,p11_y)
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn verify_DT1_2() {
        let gt2 = vec![vec![38.0,35.0],vec![32.0,27.0]];
        let res2 = DT1(2);

        for (gt,res) in gt2.iter().zip(res2.clone()){
            assert_eq!(res,*gt);
        }
        

    }
    #[test]
    #[allow(non_snake_case)]
    fn verify_DT1_4() {
        let gt4 = vec![vec![270.0,269.0,238.0,237.0],vec![270.0,269.0,240.0,239.0],vec![214.0,213.0,174.0,173.0], vec![214.0,213.0,176.0,175.0] ];
        let res4 = DT1(4);

        for (gt,res) in gt4.iter().zip(res4.clone()){
            assert_eq!(res,*gt);
        }

    }
    #[test]
    #[allow(non_snake_case)]
    fn verify_DT1_8() {
        let gt8 = vec![
                  vec![1982.0,1981.0,1980.0,1979.0,1718.0,1717.0,1716.0,1715.0],
                  vec![1982.0,1981.0,1980.0,1979.0,1722.0,1721.0,1720.0,1719.0],
                  vec![1982.0,1981.0,1980.0,1979.0,1726.0,1725.0,1724.0,1723.0],
                  vec![1982.0,1981.0,1980.0,1979.0,1730.0,1729.0,1728.0,1727.0],
                  vec![1502.0,1501.0,1500.0,1499.0,1206.0,1205.0,1204.0,1203.0],
                  vec![1502.0,1501.0,1500.0,1499.0,1210.0,1209.0,1208.0,1207.0],
                  vec![1502.0,1501.0,1500.0,1499.0,1214.0,1213.0,1212.0,1211.0],
                  vec![1502.0,1501.0,1500.0,1499.0,1218.0,1217.0,1216.0,1215.0]
                  ];
        let res8 = DT1(8);
        for (i,(gt,res)) in gt8.iter().zip(res8.clone()).enumerate(){
            assert_eq!(res,*gt, "failed at row {}", i);
        }
        assert_eq!(res8,gt8);
    }

    // #[test]
    // fn verify_DT2() {
        
    // }

}