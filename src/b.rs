use crate::util::*;

#[allow(unused, non_snake_case)]
pub fn F_B(i:usize,j:usize,n:usize) -> f64{
    g_T(i,j,n) + g_AB(i as f64,j as f64,n as f64)
}


/*
    (46)
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
    (47) SUM BOUNDS
*/
#[allow(unused, non_snake_case)]
pub fn t_N(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let n = n as f64;

    let mut t1 = 0.0;
    let (mut count,t1_upper_bound) = (0.0,(i % (2.0 * n)).log2().ceil() - 1.0);
    println!("count: {}, bound: {}", count, t1_upper_bound);
    while count <= t1_upper_bound {
        t1 += 4.0_f64.powf(count)
            * I(((((i % (2.0*n)) - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count));
        println!("HII{}", I(((((i % (2.0*n)) - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count)));
        count += 1.0;
    }

    let mut t2 = 0.0;
    let mut t2_upper_bound = (j % n).log2().ceil() - 1.0;
    count = 0.0;
    while count <= t2_upper_bound {
        t2 += 4.0_f64.powf(count)
            * I(((((j % n) - 1.0) % 2.0_f64.powf(count + 1.0)) + 1.0) > 2.0_f64.powf(count));
        count += 1.0;
    }

    t1 + t2
}

/*
    (48)
*/
#[allow(unused, non_snake_case)]
pub fn g_T(i: usize, j: usize, n: usize) -> f64 {
    let i = i as f64;
    let j = j as f64;
    let n = n as f64;

    let t1 = 4.0 * T(n) + 2.0 * n * n;

    let mut t2 = 0.0;
    let t2_upper_bound = n.log2() - 1.0;
    let mut k = 0.0;
    while k < t2_upper_bound {
        t2 += 4.0 * T(2.0_f64.powf(k));
        k += 1.0;
    }

    let mut t3 = 0.0;
    let t3_upper_bound = ((i % 2.0 * n).log2()).ceil() - 1.0;
    k = 0.0;
    while k < t3_upper_bound {
        t3 += 4.0_f64.powf(k)
            * I((((i % 2.0 * n) - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0 > 2.0_f64.powf(k));
        k += 1.0;
    }

    let mut t4 = 0.0;
    let t4_upper_bound = ((j % 2.0 * n).log2()).ceil() - 1.0;
    k = 0.0;
    while k < t3_upper_bound {
        t4 += 4.0_f64.powf(k)
            * I((((j % 2.0 * n) - 1.0) % 2.0_f64.powf(k + 1.0)) + 1.0 > 2.0_f64.powf(k));
        k += 1.0;
    }

    t1 + t2 + t3 + t4
}

/*
    (56)
*/
#[allow(unused, non_snake_case)]
pub fn g_AB(i: f64, j: f64, n: f64) -> f64 {
    if n < 1.0 {
        return 0.0;
    }

    let t1 = 3.0 * 2.0_f64.powf(2.0 * n.log2() + 1.0)
        + (2.0_f64.powf(2.0 * n.log2() - 1.0)) * I(i <= n / 2.0 && n / 4.0 < j && j <= n / 2.0);
    let t2 = (3.0 * 2.0_f64.powf(2.0 * n.log2() - 1.0))
        * I(i <= n / 2.0 && n / 2.0 < j && j <= 3.0 * n / 4.0);
    let t3 = (3.0 * 2.0_f64.powf(2.0 * n.log2() - 1.0))
        * I(n / 2.0 < i && i <= n && n / 4.0 < j && j <= n / 2.0);
    let t4 = (2.0_f64.powf(2.0 * n.log2() - 1.0))
        * I(n / 2.0 < i && i < n && n / 2.0 < j && j <= 3.0 * n / 4.0);
    let t5 = I(i <= n / 2.0 && j <= n / 4.0) * g_AB(i, j, n / 2.0);
    let t6 = I(n / 2.0 < i && i <= n && j <= n / 4.0) * (n * n + g_AB(i - n / 2.0, j, n / 2.0));
    let t7 =
        I(i <= n / 2.0 && 3.0 * n / 4.0 < j && j <= n) * (n * n + g_AB(i, j - n / 2.0, n / 2.0));
    let t8 = I(n / 2.0 < i && i <= n && 3.0 * n / 4.0 < j && j <= n)
        * g_AB(i - n / 2.0, j - n / 2.0, n / 2.0);

    t1 + t2 + t3 + t4 + t5 + t6 + t7 + t8
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

    //tn (47)
    #[test]
    fn verify_47_sz_1(){ //TODO: test whole matrix
        let bt1 = vec![vec![6.0],vec![7.0]];
        for (i,r) in bt1.iter().enumerate(){
            for (j,c) in r.iter().enumerate(){
                assert_eq!(t_N(i,j,1),c - btn(1))
            }
        }
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
                assert_eq!(t_N(i,j,2),c - btn(2))
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
                assert_eq!(t_N(i,j,4),c - btn(4))
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
                assert_eq!(t_N(i,j,8),c - btn(8))
            }
        }
    }

    // fn str_to_vec(input: String) -> Vec<Vec<usize>>{
    //     let res = Vec::new();
    //     let mut split = input.split("\n");
    //     for r in split{
    //         let row = Vec::new();
    //         let tmp = r.split(" ");
    //         for ele in tmp{
    //             row.push(ele.parse::<usize>().unwrap());
    //         }
    //         res.push(row);
    //     }
    //     res
    // }
}