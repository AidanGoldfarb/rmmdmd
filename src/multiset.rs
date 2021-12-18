use crate::a::{*};
use crate::b::{*};
use crate::temp::{*};
use crate::util::{*};
use std::collections::HashMap;

#[allow(unused)]
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
                            //println!("DT1({}): {}",*l, val);
                            for _ in 0..(Ln(*l as f64,n as f64)/2.0) as usize{
                                res.push(val);
                            }
                            
                        },
                        2 => { //DT2
                            let tmp = DT2(*l);
                            let val = tmp[i-1][j-1];
                            //println!("DT2({}): {}",l, val);
                            for _ in 0..(Ln(*l as f64,n as f64)/2.0) as usize{
                                res.push(val);
                            }
                        },
                        _ => {},
                    }
                }
            }
        }
    }
    for l in gen(n).iter(){
        for i in 1..*l+1{
            for j in 1..(*l*2)+1{
                for _ in 0..(n*n*n/(4*l*l*l)){
                    res.push(F_A(i,j,*l));
                }  
            }
        }
    }   
    for l in gen(n).iter(){
        for i in 1..(*l*2)+1{
            for j in 1..*l+1{
                for _ in 0..(n*n*n/(4*l*l*l)){
                    res.push(F_B(i,j,*l));
                } 
            }
        }
    }
    res
}

#[allow(unused)]
pub fn gen(n:usize) -> Vec<usize>{
    let mut res = Vec::new();
    let mut v = 1;
    while v <= n {
        res.push(v);
        v *= 2;
    }
    res
}

pub fn process(input: Vec<f64>) -> HashMap<usize,usize>{
    let mut dist = HashMap::new();
    for v in input{
        let v = v as usize;
        if dist.contains_key(&v){
            dist.insert(v,dist.get(&v).unwrap()+1);
        }
        else{
            dist.insert(v,1);
        }
    }
    dist
}

pub fn reuse_distance(trace: Vec<f64>) -> HashMap<usize, usize> {
    let mut stack = Vec::new();
    let mut freq_map: HashMap<usize,usize> = HashMap::new();

    // a b c a b c a b a
    for val in trace.iter(){
        let val = *val as usize;
        if stack.contains(&val){ //resuse
            //println!("val: {}", val);
            let position = stack.iter().position(|&x| x == val).unwrap();  //get position in stack
            if position == stack.len()-1{ //top of stack
                let freq = freq_map.entry(1).or_insert(0);
                *freq += 1;
            }
            else{
                let item = stack.remove(position);    //remove element and place at top
                stack.push(item);
                let temp_dist = stack.len()-position;

                let freq = freq_map.entry(temp_dist).or_insert(0);
                *freq += 1;
            }
        }
        else if stack.contains(&val){
            let position = stack.iter().position(|&x| x == val).unwrap();
            let item = stack.remove(position);    //remove element and place at top
            stack.push(item);
        }
        else{
            stack.push(val);
            // freq_map.insert(temp_dist, 0);
        }
    }
    freq_map
}
