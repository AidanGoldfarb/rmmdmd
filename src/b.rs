use crate::util::*;

#[allow(unused, non_snake_case)]
pub fn F_B(i:usize,j:usize,n:usize) -> f64{
    //println!("{} + {}", g_T(i,j%n,n), g_AB((i as f64),j as f64,n as f64));
    g_T(i,j%n,n) + g_AB((i as f64),j as f64,n as f64)

}
/*
    (48)
*/
#[allow(unused)]
pub fn btn(n: usize) -> f64 {
    let mut n = n as f64;

    let mut t3 = 0.0;
    let (mut count,t3_upper_bound) = (0.0,n.log2()-1.0);
    while count <= t3_upper_bound {
        t3 += 4.0 * T(2.0_f64.powf(count));
        count += 1.0;
    }

    (4.0 * T(n)) + (2.0 * n * n) - (t3)
}

/*
    (49)
*/
#[allow(unused, non_snake_case)]
pub fn t_N(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let n = n as f64;

    let mut t1 = 0.0;
    let (mut count,t1_upper_bound) = (0.0,(n).log2().ceil());
    while count <= t1_upper_bound {
        t1 += 4.0_f64.powf(count)
            * I((((i  - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count));
        count += 1.0;
    }

    let mut t2 = 0.0;
    let mut t2_upper_bound = (n).log2().ceil();
    count = 0.0;
    while count <= t2_upper_bound {
        t2 += 4.0_f64.powf(count)
            * I((((j - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count));
        count += 1.0;
    }

    t1 + t2
}

//VERIFY ME
pub fn g_T(i: usize, j: usize, n: usize) -> f64 {
    println!("{} + {}", btn(n) , t_N(i,j,n));
    btn(n) + t_N(i,j,n)
}

/*
    (50)
*/
#[allow(unused, non_snake_case)]
pub fn g_T_helper(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let n = n as f64;

    let mut t1 = 0.0;
    let (mut count,t1_upper_bound) = (0.0,(n).log2().ceil());
    // println!("bound: {}", t1_upper_bound);
    while count <= t1_upper_bound {
        //println!("hiiiiiiii");
        t1 += 4.0_f64.powf(count)
            * I((((i  - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count));
        //println!("HII{}", I(((((i % (2.0*n)) - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count)));
        count += 1.0;
    }

    let mut t2 = 0.0;
    //let mut t2_upper_bound = (j % n).log2().ceil();
    let mut t2_upper_bound = (n).log2().ceil();
   // println!("bound: {}", t2_upper_bound);
    count = 0.0;
    while count <= t2_upper_bound {
        t2 += 4.0_f64.powf(count)
            * I((((j - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count));
            //println!("j:{}, n:{}, k:{}, IND:{} ",j,n,count,I(((((j % n) - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count)));
        count += 1.0;
        //println!("t2: {}", t2);
    }

    t1 + t2
    // let i = i as f64;
    // let j = j as f64;
    // let n = n as f64;

    // let t1 = 4.0 * T(n) + 2.0 * n * n;

    // let mut t2 = 0.0;
    // let t2_upper_bound = n.log2() - 1.0;
    // let mut k = 0.0;
    // while k < t2_upper_bound {
    //     t2 += 4.0 * T(2.0_f64.powf(k));
    //     k += 1.0;
    // }

    // let mut t3 = 0.0;
    // // let t3_upper_bound = ((i % 2.0 * n).log2()).ceil() - 1.0;
    // let t3_upper_bound = (n.log2()).ceil();
    // k = 0.0;
    // while k < t3_upper_bound {
    //     t3 += 4.0_f64.powf(k)
    //         * I(((i - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0 > 2.0_f64.powf(k));
    //     k += 1.0;
    // }

    // let mut t4 = 0.0;
    // //let t4_upper_bound = ((j % 2.0 * n).log2()).ceil() - 1.0;
    // let t4_upper_bound = (n.log2()).ceil();
    // k = 0.0;
    // while k < t3_upper_bound {
    //     t4 += 4.0_f64.powf(k)
    //         * I(((j - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0 > 2.0_f64.powf(k));
    //     k += 1.0;
    // }

    // t1 + t2 + t3 + t4
}


/*
    (58)
*/
#[allow(unused, non_snake_case,illegal_floating_point_literal_pattern)]
pub fn g_AB(i: f64, j: f64, n: f64) -> f64 {
    (6.0*n*n) + g_AB_helper(i,j,n)
}

/*
    (58)
*/
#[allow(unused, non_snake_case,illegal_floating_point_literal_pattern)]
pub fn g_AB_helper(i: f64, j: f64, n: f64) -> f64 {
    let ii = i as usize;
    let jj = j as usize;
    //println!("n: {}\ni: {}\nj: {}\n",n, ii,jj);
    if n == 1.0 {
        return match (ii,jj){
            (1,1) => 1.0,
            (1,2) => 1.0,
            (2,1) => 2.0,
            (2,2) => 0.0,
            (0,_) | (_,0) => 0.0,
            (_,_) => -100000.0,
        }
    }
    let t1 = n*n/2.0 * I(i<=n && n/2.0<j && j<=n);
    
    let t2 = (3.0*n*n/2.0) * I(i<=n && n<j && j<=3.0*n/2.0);

    let t3 = (3.0*n*n/2.0) * I(n<i && i<=2.0*n && n/2.0<j && j<=n);
    
    let t4 = (n*n/2.0) * I(n<i && i<=2.0*n && n<j && j<=3.0*n/2.0);

    let t5 = g_AB_helper(i,j,n/2.0) * I(i<=n && j<=n/2.0);

    let t6 = (n*n + g_AB_helper(i-n,j,n/2.0)) * I(n<i && i<= 2.0*n && j<=n/2.0);

    let t7 = (n*n + g_AB_helper(i,j-n,n/2.0)) * I(i<=n && 3.0*n/2.0<j && j<=2.0*n);

    let t8 = g_AB_helper(i-n,j-n,n/2.0) * I(n<i && i<=2.0*n && 3.0*n/2.0<j && j<= 2.0*n);

    t1+t2+t3+t4+t5+t6+t7+t8
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::{assert_eq};
    use crate::util::str_to_vec;

    #[test]
    fn test_vec_parse(){
        let gt = vec![vec![6.0],vec![7.0]];
        let input = "6
        7";
        let res = str_to_vec(input);
        assert_eq!(res,gt);
    }

    #[test]
    //Tested from mats (42)[0][0] .. (45)[0][0]
    fn verify_46(){
        let gt = vec![6.0,52.0,428.0,3468.0];
        let depth = vec![1,2,4,8];
        for (t,n) in gt.iter().zip(depth.iter()){
            assert_eq!(btn(*n), *t);
        }
    }

    //tn (49)
    #[test]
    fn verify_47_sz_1(){ //TODO: test whole matrix
        let bt1 = vec![vec![6.0],vec![7.0]];
        for (i,r) in bt1.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N(i+1,j+1,1),c - btn(1))
            }
        }
    }

    #[test]
    #[ignore]
    fn verify_47_edge(){
        let bt4 = str_to_vec("428 429 432 433
        429 430 433 434
        432 433 436 437
        433 434 437 438
        444 445 448 449
        445 446 449 450
        448 449 452 453
        449 450 453 454");
        assert_eq!(t_N(1,40,4),bt4[0][3] - btn(4));
    }

    #[test]
    #[ignore]
    fn verify_47_sz_8_compl(){ //TODO: test whole matrix
        let bt8 = str_to_vec("3468 3469 3472 3473 3484 3485 3488 3489
        3469 3470 3473 3474 3485 3486 3489 3490
        3472 3473 3476 3477 3488 3489 3492 3493
        3473 3474 3477 3478 3489 3490 3493 3494
        3484 3485 3488 3489 3500 3501 3504 3505
        3485 3486 3489 3490 3501 3502 3505 3506
        3488 3489 3492 3493 3504 3505 3508 3509
        3489 3490 3493 3494 3505 3506 3509 3510
        3532 3533 3536 3537 3548 3549 3552 3553
        3533 3534 3537 3538 3549 3550 3553 3554
        3536 3537 3540 3541 3552 3553 3556 3557
        3537 3538 3541 3542 3553 3554 3557 3558
        3548 3549 3552 3553 3564 3565 3568 3569
        3549 3550 3553 3554 3565 3566 3569 3570
        3552 3553 3556 3557 3568 3569 3572 3573
        3553 3554 3557 3558 3569 3570 3573 3574");
        let mut correctness = vec![vec![false;8];16];
        for (i,r) in bt8.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                if t_N(i+1,j+1,8) == c - btn(8){
                    correctness[i][j] = true;
                }
            }
        }
        for r in correctness{
            println!("{:?}", r);
        }
        assert!(false);
    }

    //tn (47)
    #[test]
    fn verify_47_sz_2(){ //TODO: test whole matrix
        let bt2 = str_to_vec("52 53
        53 54
        56 57
        57 58");
        for (i,r) in bt2.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N(i+1,j+1,2),c - btn(2), "failed on [{}][{}]", i,j);
            }
        }
    }

    //tn (47)
    #[test]
    fn verify_47_sz_4(){ //TODO: test whole matrix
        let bt4 = str_to_vec("428 429 432 433
        429 430 433 434
        432 433 436 437
        433 434 437 438
        444 445 448 449
        445 446 449 450
        448 449 452 453
        449 450 453 454");
        for (i,r) in bt4.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N(i+1,j+1,4),c - btn(4), "failed at [{}][{}]", i,j)
            }
        }
    }

    //tn (47)
    #[test]
    fn verify_47_sz_8(){ //TODO: test whole matrix
        let bt8 = str_to_vec("3468 3469 3472 3473 3484 3485 3488 3489
        3469 3470 3473 3474 3485 3486 3489 3490
        3472 3473 3476 3477 3488 3489 3492 3493
        3473 3474 3477 3478 3489 3490 3493 3494
        3484 3485 3488 3489 3500 3501 3504 3505
        3485 3486 3489 3490 3501 3502 3505 3506
        3488 3489 3492 3493 3504 3505 3508 3509
        3489 3490 3493 3494 3505 3506 3509 3510
        3532 3533 3536 3537 3548 3549 3552 3553
        3533 3534 3537 3538 3549 3550 3553 3554
        3536 3537 3540 3541 3552 3553 3556 3557
        3537 3538 3541 3542 3553 3554 3557 3558
        3548 3549 3552 3553 3564 3565 3568 3569
        3549 3550 3553 3554 3565 3566 3569 3570
        3552 3553 3556 3557 3568 3569 3572 3573
        3553 3554 3557 3558 3569 3570 3573 3574");
        for (i,r) in bt8.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N(i+1,j+1,8),c - btn(8))
            }
        }
    }

    #[test]
    fn verify_50_sz_4(){
        let gt = str_to_vec("428 429 432 433
        429 430 433 434
        432 433 436 437
        433 434 437 438
        444 445 448 449
        445 446 449 450
        448 449 452 453
        449 450 453 454");
        let sz = 4;
        let mut values = vec![vec![0.0;sz];2*sz];
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                values[i][j] = g_T((i+1),(j+1),sz);
                assert_eq!(g_T((i+1),(j+1),sz), *c);
            }
        }
        for r in values{
            println!("{:?}", r);
        }
        //assert!(false);
    }

    #[test]
    fn verify_58_sz_1(){
        let gt = str_to_vec("7 7
        8 6");
        let sz = 1;
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(g_AB((i+1) as f64,(j+1) as f64,sz as f64), *c);
            }
        }
    }

    #[test]
    fn verify_58_sz_2(){
        let gt = str_to_vec("25 26 30 29
        26 26 30 28
        29 30 26 25
        30 30 26 24");
        let sz = 2;
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(g_AB((i+1) as f64,(j+1) as f64,sz as f64), *c);
            }
        }
    }

    #[test]
    fn verify_58_sz_4(){
        let gt = str_to_vec("97 98 104 104 120 120 118 117
        98 98 104 104 120 120 118 116
        101 102 104 104 120 120 114 113
        102 102 104 104 120 120 114 112
        113 114 120 120 104 104 102 101
        114 114 120 120 104 104 102 100
        117 118 120 120 104 104 98 97
        118 118 120 120 104 104 98 96");
        let sz = 4;
        for (i,r) in gt.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(g_AB((i+1) as f64,(j+1) as f64,sz as f64), *c);
            }
        }
    }

    // #[test]
    // fn verify_58_sz_4_compl(){
    //     let gt = str_to_vec("97 98 104 104 120 120 118 117
    //     98 98 104 104 120 120 118 116
    //     101 102 104 104 120 120 114 113
    //     102 102 104 104 120 120 114 112
    //     113 114 120 120 104 104 102 101
    //     114 114 120 120 104 104 102 100
    //     117 118 120 120 104 104 98 97
    //     118 118 120 120 104 104 98 96");
    //     let sz = 4;
    //     let mut correct = vec![vec![false;2*sz];2*sz];
    //     let mut values = vec![vec![0.0;2*sz];2*sz];
    //     for (i,r) in gt.iter().enumerate(){
    //         for (j,c) in r.iter().enumerate(){
    //             values[i][j] = g_AB((i+1) as f64,(j+1) as f64,sz as f64);
    //             if g_AB((i+1) as f64,(j+1) as f64,sz as f64) == *c{
    //                 correct[i][j] = true;
    //             }
    //             //assert_eq!(f_AB((i+1) as f64,(j+1) as f64,sz as f64), gt[i][j]);
    //         }
    //     }
    //     for r in correct{
    //         println!("{:?}", r);
    //     }
    //     for r in values{
    //         println!("{:?}", r);
    //     }
    //     assert!(false);
    // }
}