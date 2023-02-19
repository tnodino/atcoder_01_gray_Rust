// https://atcoder.jp/contests/code-formula-2014-quala/tasks/code_formula_2014_qualA_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        a: usize,
        b: usize,
        p: [usize; a],
        q: [usize; b],
    }
    let mut st = vec!["x"; 10];
    for x in 0..10 {
        for i in 0..a {
            if x == p[i] {
                st[x] = ".";
            }
        }
        for i in 0..b {
            if x == q[i] {
                st[x] = "o";
            }
        }
    }
    println!("{} {} {} {}", st[7], st[8], st[9], st[0]);
    println!(" {} {} {}", st[4], st[5], st[6]);
    println!("  {} {}", st[2], st[3]);
    println!("   {}", st[1]);
}