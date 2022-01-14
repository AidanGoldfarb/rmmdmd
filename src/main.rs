#![allow(warnings, unused)] //delete this
mod a;
mod b;
mod temp;
mod util;
mod multiset;
mod rmm;

use a::{*};
use b::{*};
use rmm::{*};

use multiset::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use itertools::Itertools;


fn main() {
    let args: Vec<String> = env::args().collect();
    let sz = args[1].parse::<usize>().unwrap();
    
    // let res = multiset(sz);
    // write_to_file(process(res), "ms.txt".to_string());

    let (mut a,mut b) = init_mats(&sz);
    write_to_file(mm_dist(&mut a,&mut b), "gt.txt".to_string());
}

pub fn write_to_file(hist: HashMap<usize,usize>, filename: String){
    let mut output = File::create(filename).unwrap();
    for k in hist.keys().sorted(){
        write!(
            output,
            "{}: {}\n",
            k,hist[k]
        ).unwrap();
    }
}

fn init_mats(sz: &usize) -> (Vec<Vec<usize>>,Vec<Vec<usize>>){
    let sz = *sz;
    let mut a = vec![vec![0;sz];sz];
    let mut b = vec![vec![0;sz];sz];
    for i in 0..sz{
        for j in 0..sz{
            a[i][j] = sz*i+j;
            b[i][j] = sz*i+j;
        }
    }
    for i in 0..sz{
        for j in 0..sz{
            a[i][j] += 1;
            b[i][j] += sz*sz+1
        }
    }
    (a,b)
} 