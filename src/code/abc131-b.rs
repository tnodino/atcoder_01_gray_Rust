// https://atcoder.jp/contests/abc131/tasks/abc131_b

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: isize,
        L: isize,
    }
    let sum = (L..L+N).sum::<isize>();
    let mut mi: isize = -999;
    for i in L..L+N {
        if i.abs() < mi.abs() {
            mi = i;
        }
    }
    println!("{}", sum - mi);
}