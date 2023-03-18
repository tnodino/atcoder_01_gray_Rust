// https://atcoder.jp/contests/abc192/tasks/abc192_c

use proconio::input;
use proconio::fastout;

fn g1(mut x: usize) -> usize {
    if x == 0 {
        return 0;
    }
    let mut vec = Vec::new();
    while x > 0 {
        vec.push(x % 10);
        x /= 10;
    }
    vec.sort_by(|a, b| b.cmp(a));
    let mut ret = 0;
    for i in 0..vec.len() {
        ret *= 10;
        ret += vec[i];
    }
    return ret;
}

fn g2(mut x: usize) -> usize {
    if x == 0 {
        return 0;
    }
    let mut vec = Vec::new();
    while x > 0 {
        vec.push(x % 10);
        x /= 10;
    }
    vec.sort();
    let mut ret = 0;
    for i in 0..vec.len() {
        ret *= 10;
        ret += vec[i];
    }
    return ret;
}

fn f(x: usize) -> usize {
    return g1(x) - g2(x);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        mut N: usize,
        K: usize,
    }
    for _ in 0..K {
        N = f(N);
    }
    println!("{}", N);
}