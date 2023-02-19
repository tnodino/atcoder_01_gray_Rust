// https://atcoder.jp/contests/arc024/tasks/arc024_1

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        L: usize,
        R: usize,
        l: [usize; L],
        r: [usize; R],
    }
    let mut cntl = [0; 50];
    let mut cntr = [0; 50];
    for i in 0..L {
        cntl[l[i]] += 1;
    }
    for i in 0..R {
        cntr[r[i]] += 1;
    }
    let mut ans = 0;
    for i in 0..50 {
        ans += cntl[i].min(cntr[i]);
    }
    println!("{}", ans);
}