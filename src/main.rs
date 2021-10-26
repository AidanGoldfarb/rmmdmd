mod a;
use a::{my_atn};

fn main() {
    let (_a,_b,c) = (1,2,8);
    let  t = my_atn(c);
    println!("{}", t);
    // t = my_atn(a,b,c);
    // println!("{}", t);
}
