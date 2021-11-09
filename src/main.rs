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
    println!("{}", t_N_a(2,1,1));
    println!("{}", t_N_a(2,1,2));
    println!("{}", t_N_a(2,1,4));
    println!("{}", t_N_a(2,1,8));
}
