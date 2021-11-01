mod a;
mod b;
mod temp;
mod util;
//use a::{*};
use temp::{*};

fn main() {
    let n = 8;
    let dt1 = DT1(n);
    let _dt2 = DT2(n);
    for r in dt1{
        println!("{:?}", r);
    }
    // println!("\n\n\n");
    // for r in dt2{
    //     println!("{:?}", r);
    // }
    
}
