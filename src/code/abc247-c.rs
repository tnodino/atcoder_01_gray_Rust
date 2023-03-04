// https://atcoder.jp/contests/abc247/tasks/abc247_c

use proconio::input;
use proconio::fastout;

fn op(x: usize) -> Vec<usize> {
    if x == 1 {
        return vec![1];
    }
    let mut vec = op(x-1);
    let mut ret = vec.clone();
    ret.push(x);
    ret.append(&mut vec);
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    println!("{}", op(N).iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}