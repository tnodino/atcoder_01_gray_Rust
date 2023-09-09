// https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (a, b): (usize, usize),
        p: [usize; a],
        q: [usize; b],
    }
    let N = 10;
    let mut flg = vec!['x'; 10];
    for i in 0..a {
        flg[(p[i] + N - 1) % N] = '.';
    }
    for i in 0..b {
        flg[(q[i] + N - 1) % N] = 'o';
    }
    println!("{} {} {} {}", flg[6], flg[7], flg[8], flg[9]);
    println!(" {} {} {}", flg[3], flg[4], flg[5]);
    println!("  {} {}", flg[1], flg[2]);
    println!("   {}", flg[0]);
}