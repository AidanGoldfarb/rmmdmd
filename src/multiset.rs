use crate::a::{*};
use crate::b::{*};
use crate::temp::{*};

pub fn multiset(n:usize) -> Vec<f64>{
    let mut res = Vec::new();
    for l in gen(n).iter(){
        for i in 1..*l+1{
            for j in 1..*l+1{
                for k in 1..3{
                    match k{
                        1 => { //DT1
                            let tmp = DT1(*l);
                            let val = tmp[i-1][j-1];
                            res.push(val);
                        },
                        2 => { //DT2
                            let tmp = DT2(*l);
                            let val = tmp[i-1][j-1];
                            res.push(val);
                        },
                        _ => {},
                    }
                }
            }
        }
    }
    for i in gen(n).iter(){
        for j in 1..n{
            for _k in 1..n{
                let tmp = F_A(*i,j,n);
                res.push(tmp);
            }
        }
    }
    for i in gen(n).iter(){
        for j in 1..n{
            for _k in 1..n{
                let tmp = F_B(*i,j,n);
                res.push(tmp);
            }
        }
    }
    res
}

pub fn gen(n:usize) -> Vec<usize>{
    let mut res = Vec::new();
    let mut v = 2;
    while v <= n {
        res.push(v);
        v *= 2;
    }
    res
}

