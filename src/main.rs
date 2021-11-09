mod a;
mod b;
mod temp;
mod util;
mod multiset;
use a::{*};
//use b::{*};
//use temp::{*};
//use multiset::*;


fn main() {
    // let n = 4;
    // let _r = multiset(n);
    // println!("{:#?}", _r);
    println!("{}", atn_c(2,1,1));
    println!("{}", atn_c(2,1,2));
    println!("{}", atn_c(2,1,4));
    println!("{}", atn_c(2,1,8));
}
