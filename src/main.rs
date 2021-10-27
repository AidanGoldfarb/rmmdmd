mod a;
use a::{*};

fn main() {
    let (_a,_b,c) = (2,2,4);
    let  t = my_atn(c);
    println!("{}", t);
    // t = my_atn(a,b,c);
    // println!("{}", t);
}
