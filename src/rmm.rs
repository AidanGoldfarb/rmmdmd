use std::collections::HashMap;

pub struct Data{
    trace: Vec<String>,
    temp: usize,
    n_start: usize,
}


pub fn mm_dist(a: &mut Vec<Vec<usize>>, b: &mut Vec<Vec<usize>>) -> HashMap<usize, usize>{
    let trace: Vec<String> = Vec::new();
    let temp = 1;
    let n_start = 0;
    let mut dta = Data{trace,temp,n_start};
    let res = mat_mul(a,b, &mut dta);
    rdd(&dta.trace)
}

/*Assuming square matrix & dim is a power of 2  
    https://shivathudi.github.io/jekyll/update/2017/06/15/matr-mult.html
*/
pub fn mat_mul(a: &mut Vec<Vec<usize>>, b: &mut Vec<Vec<usize>>, dta: &mut Data) -> Vec<Vec<usize>>{
    let n = a.len();
    let mut c11 = vec![vec![0;n/2];n/2]; 
    let mut c12 = vec![vec![0;n/2];n/2];
    let mut c21 = vec![vec![0;n/2];n/2];
    let mut c22 = vec![vec![0;n/2];n/2];
    
    if n == 1 {   
        dta.trace.push(a[0][0].to_string());
        dta.trace.push(b[0][0].to_string());
        //c11[0][0] = dta.temp;
        dta.trace.push(dta.temp.to_string()); 
        dta.temp += 1;
    }
    else{
        let (a11, a12, a21, a22) = corners(&a); //deal with temp memory
        let (b11, b12, b21, b22) = corners(&b);
    
        let mut a11_times_b11 = mat_mul(&mut a11.to_vec(), &mut b11.to_vec(),dta);
        let mut a12_times_b21 = mat_mul(&mut a12.to_vec(), &mut b21.to_vec(),dta);
        c11 = matrix_add(&mut a11_times_b11, &mut a12_times_b21).to_vec();

        let mut a11_times_b12 = mat_mul(&mut a11.to_vec(), &mut b12.to_vec(),dta);
        let mut a12_times_b22 = mat_mul(&mut a12.to_vec(), &mut b22.to_vec(),dta);
        c12 = matrix_add(&a11_times_b12, &mut a12_times_b22).to_vec();

        let mut a21_times_b11 = mat_mul(&mut a21.to_vec(), &mut b11.to_vec(),dta);
        let mut a22_times_b21 = mat_mul(&mut a22.to_vec(), &mut b21.to_vec(),dta);
        c21 = matrix_add(&mut a21_times_b11, &mut a22_times_b21).to_vec();

        let mut a21_times_b12 = mat_mul(&mut a21.to_vec(), &mut b12.to_vec(),dta);
        let mut a22_times_b22 = mat_mul(&mut a22.to_vec(), &mut b22.to_vec(),dta);
        c22 = matrix_add(&mut a21_times_b12, &mut a22_times_b22).to_vec();
    }
    stitch(&c11,&c12,&c21,&c22)
}


pub fn rdd(trace: &Vec<String>) -> HashMap<usize, usize> {
    let mut stack = Vec::new();
    let mut freq_map: HashMap<usize,usize> = HashMap::new();

    for val in trace.iter(){
        if stack.contains(&val){ //resuse
            let position = stack.iter().position(|&x| x == val).unwrap();  //get position in stack
            if position == stack.len()-1{ //top of stack
                let freq = freq_map.entry(1).or_insert(0);
                *freq += 1;
            }
            else{
                let item = stack.remove(position);    //remove element and place at top
                stack.push(item);
                let temp_dist = stack.len()-position;

                let freq = freq_map.entry(temp_dist).or_insert(0);
                *freq += 1;
            }
        }
        else if stack.contains(&val){
            let position = stack.iter().position(|&x| x == val).unwrap();
            let item = stack.remove(position);    //remove element and place at top
            stack.push(item);
        }
        else{
            stack.push(val);
        }
    }
    freq_map
}



fn matrix_add(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>) -> Vec<Vec<usize>>{
    let n = a.len();
    let mut c = vec![vec![0 as usize; n]; n];

    for i in 0..c.len(){
        for j in 0..c.len(){
            c[i][j] = a[i][j] + b[i][j];
        }
    }
    c
}

fn stitch(tl: &Vec<Vec<usize>>, tr: &Vec<Vec<usize>>, bl: &Vec<Vec<usize>>, br: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = tl.len();
    let mut c = Vec::new();
    for i in 0..(2*n) {
        let mut row = Vec::new();
        for j in 0..(2*n) {
            if i <= n/2 && j <= n/2{ //tl
                row.push(tl[i/2][j/2]);
            }
            else if i <= n/2 && j >= n/2{ //tr
                row.push(tr[i/2][j/2]);
            }
            else if i >= n/2 && j <= n/2{ //bl
                row.push(bl[i/2][j/2]);
            }
            else if i >= n/2 && j >= n/2{ //br
                row.push(br[i/2][j/2]);
            }
            else{
                break; //unreachable
            }
        }
        c.push(row);
    }
    c
}

fn corners(a: &Vec<Vec<usize>>) -> (Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>, Vec<Vec<usize>>){

    let n = a.len();

    let mut tl: Vec<Vec<usize>> = Vec::new();
    let mut tr: Vec<Vec<usize>> = Vec::new();
    let mut bl: Vec<Vec<usize>> = Vec::new();
    let mut br: Vec<Vec<usize>> = Vec::new();

    if n == 1 {
        tl.push(vec![a[0][0]]);
        return (tl,tr,bl,br);
    }

    for i in 0..n{
        let mut left: Vec<usize> = Vec::new();
        let mut right: Vec<usize> = Vec::new();
        for j in 0..n{
            if i < n/2 && j < n/2{ //tl
                left.push(a[i][j]);
            }
            else if i < n/2 && j > n/2{ //tr
                right.push(a[i][j]);
            }
            else if i > n/2-1 && j < n/2{ //bl
                left.push(a[i][j]);
            }
            else{ //br
                right.push(a[i][j]);
            }
        }
        if i < n/2 {
            tl.push(left);
            tr.push(right);
        }
        else{
            bl.push(left);
            br.push(right);
        }
    }

    return (tl,tr,bl,br);
}
