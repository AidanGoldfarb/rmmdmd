mod a;
mod b;
mod temp;
mod util;
//use a::{*};
use temp::{*};

fn main() {
    let n = 4;
    let t = DT2(n);
    for r in t{
        println!("{:?}", r);
    }
    
}
