// https://atcoder.jp/contests/abc260/tasks/abc260_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        X: usize,
        Y: usize,
        Z: usize,
        A: [usize; N],
        B: [usize; N],
    }
    let mut flg = vec![false; N];
    let mut vec = Vec::new();
    for i in 0..N {
        vec.push((A[i], B[i], A[i] + B[i], i));
    }
    vec.sort_by(|a, b| {
        if a.0 == b.0 {
            return a.3.cmp(&b.3);
        }
        else {
            return b.0.cmp(&a.0);
        }
    });
    let mut cnt = 0;
    for i in 0..N {
        if cnt == X {
            break;
        }
        if flg[vec[i].3] {
            continue;
        }
        flg[vec[i].3] = true;
        cnt += 1;
    }
    vec.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.3.cmp(&b.3);
        }
        else {
            return b.1.cmp(&a.1);
        }
    });
    let mut cnt = 0;
    for i in 0..N {
        if cnt == Y {
            break;
        }
        if flg[vec[i].3] {
            continue;
        }
        flg[vec[i].3] = true;
        cnt += 1;
    }
    vec.sort_by(|a, b| {
        if a.2 == b.2 {
            return a.3.cmp(&b.3);
        }
        else {
            return b.2.cmp(&a.2);
        }
    });
    let mut cnt = 0;
    for i in 0..N {
        if cnt == Z {
            break;
        }
        if flg[vec[i].3] {
            continue;
        }
        flg[vec[i].3] = true;
        cnt += 1;
    }
    for i in 0..N {
        if flg[i] {
            println!("{}", i + 1);
        }
    }
}