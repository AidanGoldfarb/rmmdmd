#![allow(warnings, unused)] //delete this
mod a;
mod b;
mod temp;
mod util;
mod multiset;
use a::{*};
use b::{*};
// use temp::{*};
use multiset::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
//use std::env;
use itertools::Itertools;


fn main() {
    // let args: Vec<String> = env::args().collect();
    // let sz = args[1].parse::<usize>().unwrap();
    //println!("{}", F_B(4,2,2));
    let sz = 16;
    let res = multiset(sz);
    //println!("{:?}", _res);
    write_to_file(process(res));
    //println!("{}", F_A(1,1,1));
}

pub fn write_to_file(hist: HashMap<usize,usize>){
    let mut output = File::create("rust_output.txt").unwrap();
    for k in hist.keys().sorted(){
        write!(
            output,
            "{}: {}\n",
            k,hist[k]
        ).unwrap();
    }
}
